//
// Copyright 2018-2026 Accenture Technology
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

//! Rust port of the Java `ToolCatalog` (`org.platformlambda.agentloop.ToolCatalog`): a loop node's tool
//! catalog (RFC-0011, section 5), the rows the model may call, each mapping an API-safe name to a
//! composable-function route. The catalog is graph data, certified with the graph, so it is checked
//! whole before the first model turn: a name the API would refuse, a duplicate, a schema outside the
//! vocabulary, a write or ask mark that is not true or false, a row that asks a person and has a route
//! or a write mark, a field that is not a row's, or a route nothing serves. Any of them fails before a
//! token is spent - the loop's failure, unlike a name the model invents, which goes back to the model
//! as an error result.
//!
//! A catalog given as JSON text is parsed by `serde_json` into the platform's value type, so its maps
//! keep the order written.

use rmpv::Value;

use crate::tool_schema::{ToolSchema, OBJECT};
use crate::value::{contains_key, copy, get, java_is_blank, java_trim, sorted_keys};

/// The name pattern the model API accepts, as the error quotes it.
pub const NAME_PATTERN: &str = "^[a-zA-Z0-9_-]{1,128}$";
const NAME_KEY: &str = "name";
const DESCRIPTION: &str = "description";
const INPUT_SCHEMA: &str = "input_schema";
const ROUTE: &str = "route";
const STRICT: &str = "strict";
const WRITE_KEY: &str = "write";
const ASK_KEY: &str = "ask";
const NO_DESCRIPTION: &str =
    ".description is missing - the graph writes what the model reads about a tool";
const FIELDS: [&str; 7] = [
    NAME_KEY,
    DESCRIPTION,
    INPUT_SCHEMA,
    ROUTE,
    STRICT,
    WRITE_KEY,
    ASK_KEY,
];

/// One row of the catalog.
#[derive(Debug)]
pub struct Row {
    /// The name advertised to the model.
    pub name: String,
    /// The description written on the graph.
    pub description: String,
    /// The schema as written, advertised to the model.
    pub input_schema: Value,
    /// The compiled schema the loop validates against.
    pub schema: ToolSchema,
    /// The composable-function route, or `None` for a row that asks a person.
    pub route: Option<String>,
    /// Grammar-constrained arguments, or `None` when the row does not say.
    pub strict: Option<bool>,
    /// True when the tool writes: it waits for a person in mode `write`, and is refused in mode
    /// `unattended`.
    pub write: bool,
    /// True when the row asks a person: the call waits, and the person's answer is its result.
    pub ask: bool,
}

/// A loop node's tool catalog, its rows in catalog order.
#[derive(Debug)]
pub struct ToolCatalog {
    rows: Vec<Row>,
}

impl ToolCatalog {
    /// Parse and check a catalog.
    ///
    /// `tools` is the node's `tools` input: a list of rows, or the JSON text of one. `reachable`
    /// answers whether a route is served (local, or an Event-over-HTTP target). The error names the
    /// first problem.
    pub fn parse(
        tools: Option<&Value>,
        reachable: &dyn Fn(&str) -> bool,
    ) -> Result<ToolCatalog, String> {
        let list = as_list(tools)?;
        if list.is_empty() {
            return Err("tools is empty - a loop needs at least one tool \
                        (call llm.chat for a single model turn)"
                .to_string());
        }
        let mut catalog = ToolCatalog { rows: Vec::new() };
        for (i, row) in list.iter().enumerate() {
            let place = format!("tools[{i}]");
            if !matches!(row, Value::Map(_)) {
                return Err(format!("{place} must be a map"));
            }
            for key in sorted_keys(row) {
                if !FIELDS.contains(&key.as_str()) {
                    return Err(format!(
                        "{place}.{key} is not a field of a tool row - the fields are name, \
                         description, input_schema, route, strict, write and ask"
                    ));
                }
            }
            let parsed = parse_row(&place, row, reachable, &catalog)?;
            catalog.rows.push(parsed);
        }
        Ok(catalog)
    }

    /// Find a row by the name the model used; `None` when the catalog has none.
    pub fn get(&self, name: &str) -> Option<&Row> {
        self.rows.iter().find(|row| row.name == name)
    }

    /// The tools as the model sees them: name, description, input schema and strict. The route and the
    /// write mark stay in the catalog. In catalog order.
    pub fn advertised(&self) -> Vec<Value> {
        self.rows
            .iter()
            .map(|row| {
                let mut tool = vec![
                    (Value::from(NAME_KEY), Value::from(row.name.as_str())),
                    (
                        Value::from(DESCRIPTION),
                        Value::from(row.description.as_str()),
                    ),
                    (Value::from(INPUT_SCHEMA), row.input_schema.clone()),
                ];
                if let Some(strict) = row.strict {
                    tool.push((Value::from(STRICT), Value::Boolean(strict)));
                }
                Value::Map(tool)
            })
            .collect()
    }
}

fn parse_row(
    place: &str,
    row: &Value,
    reachable: &dyn Fn(&str) -> bool,
    taken: &ToolCatalog,
) -> Result<Row, String> {
    let name = match get(row, NAME_KEY).and_then(Value::as_str) {
        Some(name) if is_api_name(name) => name.to_string(),
        _ => return Err(format!("{place}.name must match {NAME_PATTERN}")),
    };
    if taken.get(&name).is_some() {
        return Err(format!("{place}.name '{name}' is not unique"));
    }
    let description = match get(row, DESCRIPTION).and_then(Value::as_str) {
        Some(text) if !java_is_blank(text) => text.to_string(),
        _ => return Err(format!("{place}{NO_DESCRIPTION}")),
    };
    let input_schema = copy(row, INPUT_SCHEMA);
    let schema = ToolSchema::compile(&format!("{place}.{INPUT_SCHEMA}"), &input_schema)?;
    if schema.types() != [OBJECT] {
        return Err(format!("{place}.{INPUT_SCHEMA}.type must be 'object'"));
    }
    let ask = get(row, ASK_KEY);
    if ask.is_some_and(|value| !value.is_bool()) {
        return Err(format!("{place}.ask must be true or false"));
    }
    let asks = matches!(ask, Some(Value::Boolean(true)));
    let route = if asks {
        no_route(place, row)?
    } else {
        Some(route(place, row, reachable)?)
    };
    let strict = get(row, STRICT);
    if strict.is_some_and(|value| !value.is_bool()) {
        return Err(format!("{place}.strict must be true or false"));
    }
    let write = get(row, WRITE_KEY);
    if write.is_some_and(|value| !value.is_bool()) {
        return Err(format!("{place}.write must be true or false"));
    }
    let writes = matches!(write, Some(Value::Boolean(true)));
    if asks && writes {
        return Err(format!("{place} asks a person, so it cannot be a write"));
    }
    Ok(Row {
        name,
        description,
        input_schema,
        schema,
        route,
        strict: strict.and_then(Value::as_bool),
        write: writes,
        ask: asks,
    })
}

/// `^[a-zA-Z0-9_-]{1,128}$`, matched whole.
fn is_api_name(name: &str) -> bool {
    (1..=128).contains(&name.len())
        && name
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || c == b'_' || c == b'-')
}

/// A row that asks a person has no route: the loop never calls anything for it.
fn no_route(place: &str, row: &Value) -> Result<Option<String>, String> {
    if contains_key(row, ROUTE) {
        return Err(format!("{place} asks a person, so it has no route"));
    }
    Ok(None)
}

fn route(place: &str, row: &Value, reachable: &dyn Fn(&str) -> bool) -> Result<String, String> {
    let route = match get(row, ROUTE).and_then(Value::as_str) {
        Some(route) if !java_is_blank(route) => route,
        _ => return Err(format!("{place}.route is missing")),
    };
    let target = java_trim(route);
    if !reachable(target) {
        return Err(format!(
            "{place}.route '{target}' is not reachable - no local function, Event-over-HTTP target \
             or mesh route serves it"
        ));
    }
    Ok(target.to_string())
}

fn as_list(tools: Option<&Value>) -> Result<Vec<Value>, String> {
    match tools {
        Some(Value::Array(list)) => return Ok(list.clone()),
        Some(Value::String(text)) => {
            let text = text.as_str().unwrap_or_default();
            if !java_is_blank(text) {
                match serde_json::from_str::<Value>(text) {
                    Ok(Value::Array(list)) => return Ok(list),
                    // JSON null reads as no list, like a value of another kind
                    Ok(Value::Nil) => {}
                    _ => return Err("tools is not a JSON list of tool rows".to_string()),
                }
            }
        }
        _ => {}
    }
    Err(match tools {
        None => "tools is required - the loop's tool catalog".to_string(),
        Some(_) => "tools must be a list of tool rows, or its JSON text".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_catalog_as_json_text_keeps_its_order() {
        let text = Value::from(
            r#"[{"name":"b","description":"B.","input_schema":{"type":"object"},"route":"x.b"},
                {"name":"a","description":"A.","input_schema":{"type":"object"},"ask":true}]"#,
        );
        let catalog = ToolCatalog::parse(Some(&text), &|route| route == "x.b").expect("catalog");
        let names: Vec<_> = catalog
            .advertised()
            .iter()
            .map(|tool| crate::value::value_of(get(tool, "name")))
            .collect();
        assert_eq!(names, ["b", "a"]);
        assert!(catalog
            .get("a")
            .is_some_and(|row| row.ask && row.route.is_none()));
        assert_eq!(
            ToolCatalog::parse(Some(&Value::from("{}")), &|_| true).unwrap_err(),
            "tools is not a JSON list of tool rows"
        );
        assert_eq!(
            ToolCatalog::parse(Some(&Value::from(" ")), &|_| true).unwrap_err(),
            "tools must be a list of tool rows, or its JSON text"
        );
    }
}
