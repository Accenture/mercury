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

//! A minimal OpenAPI 3.0 document of one graph (RFC-0007; Java `OpenApiDocument`), derived from its
//! [`GraphContract`]: the graph's one endpoint `POST /api/graph/{graph-id}`, its request body and
//! header parameters, the `200` response with its body and headers, every status the model stages
//! with `int(N) -> output.status`, and the engine's error shape. The document is a derived
//! artifact, never stored; the shared vector file pins both engines.

use event_script::conversions::to_json_string;
use rmpv::Value;

use crate::contract::{
    map, schema_of, GraphContract, INPUT_BODY, INPUT_HEADER, OUTPUT_BODY, OUTPUT_HEADER,
};

const ERROR_REF: &str = "#/components/schemas/Error";

/// Build the document: `version` is the API version (the deployed set's, else the application's),
/// `server_url` the base URL of the engine that answers (None to omit `servers`).
pub fn document(contract: &GraphContract, version: &str, server_url: Option<&str>) -> Value {
    let id = contract.graph_id();
    let purpose = contract.purpose();
    let mut info = vec![
        ("title", Value::from(id)),
        (
            "version",
            Value::from(if version.trim().is_empty() {
                "1.0.0"
            } else {
                version
            }),
        ),
    ];
    if let Some(p) = purpose {
        info.push(("description", Value::from(p)));
    }
    let mut document = vec![("openapi", Value::from("3.0.3")), ("info", map(info))];
    if let Some(url) = server_url.filter(|u| !u.trim().is_empty()) {
        document.push((
            "servers",
            Value::Array(vec![map(vec![("url", Value::from(url))])]),
        ));
    }
    let mut operation = vec![
        ("operationId", Value::from(id)),
        (
            "summary",
            Value::from(
                purpose
                    .map(str::to_string)
                    .unwrap_or_else(|| format!("Run the graph {id}"))
                    .as_str(),
            ),
        ),
    ];
    let parameters = header_parameters(contract);
    if !parameters.is_empty() {
        operation.push(("parameters", Value::Array(parameters)));
    }
    if let Some(request_schema) = contract.schema(INPUT_BODY) {
        operation.push((
            "requestBody",
            map(vec![
                ("required", Value::from(true)),
                ("content", json_content(request_schema)),
            ]),
        ));
    }
    operation.push(("responses", responses(contract)));
    let paths = map(vec![(
        &format!("/api/graph/{id}"),
        map(vec![("post", map(operation))]),
    )]);
    document.push(("paths", paths));
    document.push((
        "components",
        map(vec![("schemas", map(vec![("Error", error_schema())]))]),
    ));
    map(document)
}

fn header_parameters(contract: &GraphContract) -> Vec<Value> {
    let mut parameters = Vec::new();
    for header in contract.root(INPUT_HEADER).children.values() {
        let mut parameter = vec![
            ("name", Value::from(header.name.as_str())),
            ("in", Value::from("header")),
        ];
        if header.required {
            parameter.push(("required", Value::from(true)));
        }
        let (schema, description) = take_description(schema_of(header));
        if let Some(d) = description {
            parameter.push(("description", d));
        }
        parameter.push(("schema", with_default_type(schema)));
        parameters.push(map(parameter));
    }
    parameters
}

fn responses(contract: &GraphContract) -> Value {
    let mut responses: Vec<(&str, Value)> = Vec::new();
    let mut ok = vec![("description", Value::from("The graph's output.body"))];
    let headers = response_headers(contract);
    if !headers.is_empty() {
        ok.push(("headers", Value::Map(headers)));
    }
    ok.push((
        "content",
        json_content(
            contract
                .schema(OUTPUT_BODY)
                .unwrap_or_else(|| Value::Map(vec![])),
        ),
    ));
    responses.push(("200", map(ok)));
    let staged: Vec<(String, Value)> = contract
        .status_codes()
        .iter()
        .filter(|c| **c != 200)
        .map(|code| {
            (
                code.to_string(),
                map(vec![
                    (
                        "description",
                        Value::from(
                            format!("Staged by the graph (int({code}) -> output.status)").as_str(),
                        ),
                    ),
                    (
                        "content",
                        json_content(map(vec![("type", Value::from("object"))])),
                    ),
                ]),
            )
        })
        .collect();
    let mut all: Vec<(Value, Value)> = responses
        .into_iter()
        .map(|(k, v)| (Value::from(k), v))
        .collect();
    for (code, value) in staged {
        all.push((Value::from(code.as_str()), value));
    }
    all.push((
        Value::from("default"),
        map(vec![
            ("description", Value::from("Error")),
            (
                "content",
                json_content(map(vec![("$ref", Value::from(ERROR_REF))])),
            ),
        ]),
    ));
    Value::Map(all)
}

fn response_headers(contract: &GraphContract) -> Vec<(Value, Value)> {
    let mut headers = Vec::new();
    for header in contract.root(OUTPUT_HEADER).children.values() {
        let mut entry: Vec<(&str, Value)> = Vec::new();
        let (schema, description) = take_description(schema_of(header));
        if let Some(d) = description {
            entry.push(("description", d));
        }
        entry.push(("schema", with_default_type(schema)));
        headers.push((Value::from(header.name.as_str()), map(entry)));
    }
    headers
}

fn take_description(schema: Value) -> (Value, Option<Value>) {
    if let Value::Map(entries) = schema {
        let mut description = None;
        let kept: Vec<(Value, Value)> = entries
            .into_iter()
            .filter(|(k, v)| {
                if k.as_str() == Some("description") {
                    description = Some(v.clone());
                    false
                } else {
                    true
                }
            })
            .collect();
        (Value::Map(kept), description)
    } else {
        (schema, None)
    }
}

fn with_default_type(schema: Value) -> Value {
    if let Value::Map(mut entries) = schema {
        if !entries.iter().any(|(k, _)| k.as_str() == Some("type")) {
            entries.insert(0, (Value::from("type"), Value::from("string")));
        }
        Value::Map(entries)
    } else {
        schema
    }
}

fn json_content(schema: Value) -> Value {
    map(vec![("application/json", map(vec![("schema", schema)]))])
}

fn error_schema() -> Value {
    map(vec![
        ("type", Value::from("object")),
        (
            "properties",
            map(vec![
                ("type", map(vec![("type", Value::from("string"))])),
                ("status", map(vec![("type", Value::from("integer"))])),
                ("message", map(vec![("type", Value::from("string"))])),
            ]),
        ),
        (
            "required",
            Value::Array(vec![
                Value::from("type"),
                Value::from("status"),
                Value::from("message"),
            ]),
        ),
    ])
}

/// The document as YAML, keys in document order.
pub fn to_yaml(document: &Value) -> Result<String, String> {
    serde_yaml::to_string(document).map_err(|e| e.to_string())
}

/// The document as JSON, keys in document order (the ordered value is serialized directly; the
/// engine's generic JSON text would sort the keys).
pub fn to_json(document: &Value) -> String {
    serde_json::to_string(document).unwrap_or_else(|_| to_json_string(document))
}
