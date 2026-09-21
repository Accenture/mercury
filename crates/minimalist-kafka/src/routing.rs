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

//! Second-level routing rules for one consumer binding (Java
//! `RoutingRuleSet`) — the `flows` alternative to a binding's single `flow`.
//! Each rule inspects one key-value of the inbound record and picks the target
//! flow or function per message:
//!
//! ```yaml
//! flows:
//!   - 'input.header.type(order) -> flow://order-flow'
//!   - 'input.header.type(order-*) -> flow://order-variant-flow'
//!   - 'input.header.type(regex: ^shipment-(eu|us)$) -> flow://shipment-flow'
//!   - 'input.body.event.kind(refund) -> task://v1.refund.processor'
//!   - 'default -> flow://catch-all-flow'
//! ```
//!
//! **Rule syntax.** `<selector>(<matcher>) -> <target>` plus the mandatory
//! `default -> <target>` fallback. The selector is `input.header.<name>` (a
//! Kafka record header; the header NAME lookup is case-insensitive because
//! Kafka preserves the producer's wire casing) or `input.body` followed by a
//! dot-bracket composite path — a map body via `input.body.order.type`, a
//! top-level list body via `input.body[0].type`, and any nesting of the two.
//! The matcher has three modes: exact (`type(order)`, case-sensitive),
//! wildcard when the value contains `*` (`type(order-*)`), and regex only via
//! the explicit `regex:` prefix (`type(regex: ^a|b$)`) — regex is the
//! exception, not the norm. Wildcard and regex matchers use **full-string**
//! matching (the `topic-pattern` precedent).
//!
//! **Evaluation.** Order matters: the first matching rule wins, in declaration
//! order. A missing header/key, a raw-bytes body for an `input.body` rule, or
//! a non-text value is a non-match — never an error. When no rule matches,
//! the `default` target is used. Body lookups run under a synthetic `body`
//! root, which makes a top-level list addressable with the same dot-bracket
//! convention; a `$`-prefixed key is an ordinary literal segment here.
//!
//! **Targets.** `flow://<flow-id>` dispatches to an Event Script flow exactly
//! as direct `flow` routing does; `task://<route>` invokes a registered
//! function directly (see [`crate::consumer`] for the dispatch contract). Any
//! other scheme is rejected at compile time, the same way CompileFlows
//! rejects a `://` that is not `flow://`.
//!
//! Compiled once at startup by [`crate::adapter`] (fail-fast on any malformed
//! rule) and then read-only on the binding's consumer task.

use std::collections::{BTreeMap, HashMap};

use platform_core::{ConfigValue, MultiLevelMap};
use regex::Regex;

const HEADER_SELECTOR: &str = "input.header.";
const BODY_PREFIX: &str = "input.body";
/// Synthetic root the record body is evaluated under — makes a top-level list
/// addressable (`body[0].type`) and keeps every lookup path a plain composite
/// path (never a `$`-prefixed one).
const BODY_ROOT: &str = "body";
const DEFAULT_RULE: &str = "default";
const ARROW: &str = "->";
const REGEX_PREFIX: &str = "regex:";
const FLOW_PROTOCOL: &str = "flow://";
const TASK_PROTOCOL: &str = "task://";

/// One routing destination: an Event Script flow (`task` false, `destination`
/// = flow id) or a direct function invocation (`task` true, `destination` =
/// route name).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RoutingTarget {
    pub task: bool,
    pub destination: String,
}

impl RoutingTarget {
    pub fn flow(flow_id: &str) -> Self {
        RoutingTarget {
            task: false,
            destination: flow_id.to_string(),
        }
    }

    pub fn task(route: &str) -> Self {
        RoutingTarget {
            task: true,
            destination: route.to_string(),
        }
    }

    /// Display form for logs and error messages, e.g. `flow 'order-flow'` or
    /// `task 'v1.refund'`.
    pub fn label(&self) -> String {
        if self.task {
            format!("task '{}'", self.destination)
        } else {
            format!("flow '{}'", self.destination)
        }
    }
}

#[derive(Clone, Debug)]
enum Selector {
    /// A record header, matched by NAME case-insensitively.
    Header(String),
    /// The precomputed lookup path under the synthetic `body` root, e.g.
    /// `body.event.kind` or `body[0].type`.
    Body(String),
}

#[derive(Clone, Debug)]
enum Matcher {
    /// Case-sensitive exact value comparison.
    Exact(String),
    /// A wildcard or explicit regex, anchored to the full value at compile time.
    Pattern(Regex),
}

impl Matcher {
    fn matches(&self, value: &str) -> bool {
        match self {
            Matcher::Exact(exact) => exact == value,
            Matcher::Pattern(pattern) => pattern.is_match(value),
        }
    }
}

#[derive(Clone, Debug)]
struct Rule {
    selector: Selector,
    matcher: Matcher,
    target: RoutingTarget,
}

/// The compiled, immutable rule list of one binding.
#[derive(Clone, Debug)]
pub struct RoutingRuleSet {
    rules: Vec<Rule>,
    default_target: RoutingTarget,
}

impl RoutingRuleSet {
    /// Compile a binding's `flows` rule list. Fail-fast: any malformed rule, a
    /// missing or duplicate `default`, or an invalid regex is an error at
    /// startup. Messages carry no binding prefix — the adapter adds
    /// `consumer[i] (label)` when it reports one.
    pub fn compile(rule_strings: &[String]) -> Result<RoutingRuleSet, String> {
        if rule_strings.is_empty() {
            return Err("'flows' must be a non-empty list of routing rules".to_string());
        }
        let mut rules = Vec::with_capacity(rule_strings.len());
        let mut default_target = None;
        for raw in rule_strings {
            let rule = raw.trim();
            let Some(arrow) = rule.find(ARROW) else {
                return Err(format!(
                    "routing rule '{raw}' must use the syntax '<selector>(<matcher>) -> <target>' \
                     or 'default -> <target>'"
                ));
            };
            let target = parse_target(rule[arrow + ARROW.len()..].trim(), raw)?;
            let lhs = rule[..arrow].trim();
            if lhs == DEFAULT_RULE {
                if default_target.is_some() {
                    return Err("only one 'default' routing rule is allowed".to_string());
                }
                default_target = Some(target);
            } else {
                rules.push(parse_rule(lhs, target, raw)?);
            }
        }
        let Some(default_target) = default_target else {
            return Err("'flows' must contain a 'default -> <target>' routing rule".to_string());
        };
        Ok(RoutingRuleSet {
            rules,
            default_target,
        })
    }

    /// Select the target for one record: the first matching rule in
    /// declaration order, else the default. A non-match (missing header/key,
    /// raw-bytes body, non-text value) never errors.
    ///
    /// `body` is the record value decoded under `serializer: 'json'` — a JSON
    /// object or array; `None` (raw bytes) makes every `input.body` rule a
    /// non-match by design (route on headers instead).
    pub fn select(
        &self,
        headers: &HashMap<String, String>,
        body: Option<&serde_json::Value>,
    ) -> &RoutingTarget {
        let body_map = body
            .filter(|value| value.is_object() || value.is_array())
            .map(|value| {
                let mut root = BTreeMap::new();
                root.insert(BODY_ROOT.to_string(), ConfigValue::from_json(value));
                MultiLevelMap::from_map(root)
            });
        for rule in &self.rules {
            let value = match &rule.selector {
                Selector::Header(name) => header_value(headers, name),
                Selector::Body(path) => body_map
                    .as_ref()
                    .and_then(|map| map.get_element(path))
                    .and_then(ConfigValue::as_text),
            };
            if let Some(value) = value {
                if rule.matcher.matches(value) {
                    return &rule.target;
                }
            }
        }
        &self.default_target
    }

    /// Number of rules excluding the default (for the adapter's binding log line).
    pub fn size(&self) -> usize {
        self.rules.len()
    }

    /// Every target this rule set can route to, including the default — for
    /// startup validation and the poll-interval envelope.
    pub fn all_targets(&self) -> Vec<&RoutingTarget> {
        let mut all: Vec<&RoutingTarget> = self.rules.iter().map(|rule| &rule.target).collect();
        all.push(&self.default_target);
        all
    }
}

/// Parse the `<selector>(<matcher>)` left-hand side of one rule.
fn parse_rule(lhs: &str, target: RoutingTarget, raw: &str) -> Result<Rule, String> {
    let open = lhs.find('(');
    let (Some(open), true) = (open, lhs.ends_with(')')) else {
        return Err(format!(
            "routing rule '{raw}' must use the syntax '<selector>(<matcher>) -> <target>'"
        ));
    };
    if open < 1 {
        return Err(format!(
            "routing rule '{raw}' must use the syntax '<selector>(<matcher>) -> <target>'"
        ));
    }
    let selector = lhs[..open].trim();
    let matcher = lhs[open + 1..lhs.len() - 1].trim();
    let selector = if let Some(name) = selector.strip_prefix(HEADER_SELECTOR) {
        if name.is_empty() {
            return Err(format!("routing rule '{raw}' is missing a header name"));
        }
        Selector::Header(name.to_string())
    } else if selector.starts_with(&format!("{BODY_PREFIX}."))
        || selector.starts_with(&format!("{BODY_PREFIX}["))
    {
        // the remainder starts with '.' or '[' by construction: the record body
        // is evaluated under the synthetic root, so `input.body.event.kind`
        // becomes `body.event.kind` and `input.body[0].type` becomes `body[0].type`
        let relative = &selector[BODY_PREFIX.len()..];
        if relative.len() < 2 {
            return Err(format!("routing rule '{raw}' is missing a body key"));
        }
        Selector::Body(format!("{BODY_ROOT}{relative}"))
    } else {
        return Err(format!(
            "routing rule '{raw}' selector must be 'input.header.<name>', 'input.body.<key>' or \
             'input.body[<index>]...'"
        ));
    };
    if matcher.is_empty() {
        return Err(format!("routing rule '{raw}' is missing a matcher value"));
    }
    Ok(Rule {
        selector,
        matcher: compile_matcher(matcher, raw)?,
        target,
    })
}

/// Resolve the matcher mode: explicit `regex:` prefix > wildcard (contains
/// `*`) > exact. Pattern modes are anchored to the whole value.
fn compile_matcher(matcher: &str, raw: &str) -> Result<Matcher, String> {
    if let Some(expression) = matcher.strip_prefix(REGEX_PREFIX) {
        let expression = expression.trim();
        if expression.is_empty() {
            return Err(format!(
                "routing rule '{raw}' has an empty regex expression"
            ));
        }
        return Regex::new(&format!("^(?:{expression})$"))
            .map(Matcher::Pattern)
            .map_err(|e| format!("routing rule '{raw}' regex is invalid: {e}"));
    }
    if matcher.contains('*') {
        // literal segments escaped, each '*' = any run of characters — including
        // line breaks inside a free-text body value (the Java DOTALL flag)
        let pattern = matcher
            .split('*')
            .map(regex::escape)
            .collect::<Vec<_>>()
            .join(".*");
        return Regex::new(&format!("(?s)^(?:{pattern})$"))
            .map(Matcher::Pattern)
            .map_err(|e| format!("routing rule '{raw}' wildcard is invalid: {e}"));
    }
    Ok(Matcher::Exact(matcher.to_string()))
}

/// Parse a rule's `flow://<flow-id>` or `task://<route>` target.
fn parse_target(target: &str, raw: &str) -> Result<RoutingTarget, String> {
    if let Some(flow_id) = target.strip_prefix(FLOW_PROTOCOL) {
        if !flow_id.is_empty() {
            return Ok(RoutingTarget::flow(flow_id));
        }
    }
    if let Some(route) = target.strip_prefix(TASK_PROTOCOL) {
        if !route.is_empty() {
            return Ok(RoutingTarget::task(route));
        }
    }
    Err(format!(
        "routing rule '{raw}' target must be 'flow://<flow-id>' or 'task://<route>'"
    ))
}

/// Case-insensitive header NAME lookup — Kafka preserves the producer's wire
/// casing, so a rule must not depend on it. Header VALUES stay case-sensitive.
fn header_value<'a>(headers: &'a HashMap<String, String>, name: &str) -> Option<&'a str> {
    headers
        .iter()
        .find(|(key, _)| key.eq_ignore_ascii_case(name))
        .map(|(_, value)| value.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rules(list: &[&str]) -> RoutingRuleSet {
        RoutingRuleSet::compile(&list.iter().map(|s| s.to_string()).collect::<Vec<_>>())
            .expect("rules compile")
    }

    fn compile_error(list: &[&str]) -> String {
        RoutingRuleSet::compile(&list.iter().map(|s| s.to_string()).collect::<Vec<_>>())
            .expect_err("rules rejected")
    }

    fn headers(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    const ORDER: &str = "input.header.type(order) -> flow://order-flow";
    const DEFAULT: &str = "default -> flow://catch-all";

    #[test]
    fn first_match_wins_in_declaration_order() {
        let set = rules(&[
            "input.header.type(order-*) -> flow://variant",
            "input.header.type(order-1) -> flow://exact-later",
            DEFAULT,
        ]);
        assert_eq!(
            &RoutingTarget::flow("variant"),
            set.select(&headers(&[("type", "order-1")]), None),
            "the earlier wildcard rule wins over the later exact one"
        );
    }

    #[test]
    fn header_name_lookup_is_case_insensitive_but_value_stays_case_sensitive() {
        let set = rules(&[ORDER, DEFAULT]);
        assert_eq!(
            &RoutingTarget::flow("order-flow"),
            set.select(&headers(&[("TYPE", "order")]), None)
        );
        assert_eq!(
            &RoutingTarget::flow("catch-all"),
            set.select(&headers(&[("type", "ORDER")]), None)
        );
    }

    #[test]
    fn wildcard_is_anchored_full_match() {
        let set = rules(&["input.header.type(order-*) -> flow://variant", DEFAULT]);
        assert_eq!(
            &RoutingTarget::flow("variant"),
            set.select(&headers(&[("type", "order-1")]), None)
        );
        assert_eq!(
            &RoutingTarget::flow("catch-all"),
            set.select(&headers(&[("type", "my-order-1")]), None),
            "a wildcard never matches a prefix of the value"
        );
    }

    #[test]
    fn wildcard_treats_regex_metacharacters_as_literals() {
        let set = rules(&["input.header.type(a.b*) -> flow://dotted", DEFAULT]);
        assert_eq!(
            &RoutingTarget::flow("dotted"),
            set.select(&headers(&[("type", "a.bc")]), None)
        );
        assert_eq!(
            &RoutingTarget::flow("catch-all"),
            set.select(&headers(&[("type", "axbc")]), None),
            "'.' is a literal dot, not any-character"
        );
    }

    #[test]
    fn wildcard_matches_across_line_breaks() {
        let set = rules(&["input.header.note(a*b) -> flow://multi", DEFAULT]);
        assert_eq!(
            &RoutingTarget::flow("multi"),
            set.select(&headers(&[("note", "a\nb")]), None)
        );
    }

    #[test]
    fn regex_mode_is_explicit_and_full_match() {
        let set = rules(&[
            "input.header.type(regex: ^shipment-(eu|us)$) -> flow://shipment",
            "input.header.kind(regex: ship) -> flow://bare",
            DEFAULT,
        ]);
        assert_eq!(
            &RoutingTarget::flow("shipment"),
            set.select(&headers(&[("type", "shipment-eu")]), None)
        );
        assert_eq!(
            &RoutingTarget::flow("catch-all"),
            set.select(&headers(&[("type", "shipment-eu-2")]), None)
        );
        assert_eq!(
            &RoutingTarget::flow("catch-all"),
            set.select(&headers(&[("kind", "my-ship-1")]), None),
            "an unanchored expression is still a full-string match"
        );
        assert_eq!(
            &RoutingTarget::flow("bare"),
            set.select(&headers(&[("kind", "ship")]), None)
        );
    }

    #[test]
    fn body_rule_matches_composite_path_on_map_body() {
        let set = rules(&["input.body.event.kind(refund) -> task://v1.refund", DEFAULT]);
        let body = serde_json::json!({"event": {"kind": "refund"}, "amount": 10});
        assert_eq!(
            &RoutingTarget::task("v1.refund"),
            set.select(&HashMap::new(), Some(&body))
        );
    }

    #[test]
    fn body_rule_never_matches_bytes_or_a_mismatched_shape() {
        let set = rules(&["input.body.event.kind(refund) -> task://v1.refund", DEFAULT]);
        assert_eq!(
            &RoutingTarget::flow("catch-all"),
            set.select(&HashMap::new(), None),
            "raw bytes: every body rule is a non-match"
        );
        let scalar = serde_json::json!("refund");
        assert_eq!(
            &RoutingTarget::flow("catch-all"),
            set.select(&HashMap::new(), Some(&scalar))
        );
        let wrong_shape = serde_json::json!({"event": "refund"});
        assert_eq!(
            &RoutingTarget::flow("catch-all"),
            set.select(&HashMap::new(), Some(&wrong_shape))
        );
    }

    #[test]
    fn body_rules_address_list_bodies_with_bracket_paths() {
        let set = rules(&[
            "input.body[0].type(batch-order) -> task://batch",
            "input.body.items[1].kind(gift) -> task://gift",
            DEFAULT,
        ]);
        let list = serde_json::json!([{"type": "batch-order"}, {"type": "noise"}]);
        assert_eq!(
            &RoutingTarget::task("batch"),
            set.select(&HashMap::new(), Some(&list))
        );
        let nested = serde_json::json!({"items": [{"kind": "plain"}, {"kind": "gift"}]});
        assert_eq!(
            &RoutingTarget::task("gift"),
            set.select(&HashMap::new(), Some(&nested))
        );
    }

    #[test]
    fn dollar_prefixed_body_keys_are_literal_segments() {
        let set = rules(&["input.body.$meta.kind(x) -> task://meta", DEFAULT]);
        let body = serde_json::json!({"$meta": {"kind": "x"}});
        assert_eq!(
            &RoutingTarget::task("meta"),
            set.select(&HashMap::new(), Some(&body))
        );
    }

    #[test]
    fn non_text_value_is_a_non_match() {
        let set = rules(&["input.body.count(5) -> task://five", DEFAULT]);
        let body = serde_json::json!({"count": 5});
        assert_eq!(
            &RoutingTarget::flow("catch-all"),
            set.select(&HashMap::new(), Some(&body)),
            "a number never equals the text '5' - route on text values"
        );
    }

    #[test]
    fn missing_header_or_key_is_a_non_match_not_an_error() {
        let set = rules(&[ORDER, "input.body.a.b(c) -> task://abc", DEFAULT]);
        let body = serde_json::json!({"other": 1});
        assert_eq!(
            &RoutingTarget::flow("catch-all"),
            set.select(&headers(&[("unrelated", "x")]), Some(&body))
        );
    }

    #[test]
    fn default_may_target_a_task() {
        let set = rules(&[ORDER, "default -> task://sink"]);
        assert_eq!(
            &RoutingTarget::task("sink"),
            set.select(&HashMap::new(), None)
        );
    }

    #[test]
    fn all_targets_includes_every_rule_and_the_default() {
        let set = rules(&[ORDER, "input.body.k(v) -> task://t", DEFAULT]);
        assert_eq!(2, set.size());
        let labels: Vec<String> = set.all_targets().iter().map(|t| t.label()).collect();
        assert_eq!(
            vec!["flow 'order-flow'", "task 't'", "flow 'catch-all'"],
            labels
        );
    }

    #[test]
    fn malformed_rules_are_rejected_by_reason() {
        let cases: [(&[&str], &str); 11] = [
            (
                &["input.header.type(order) flow://x", DEFAULT],
                "must use the syntax",
            ),
            (
                &["input.header.type -> flow://x", DEFAULT],
                "must use the syntax",
            ),
            (
                &["input.header.type() -> flow://x", DEFAULT],
                "is missing a matcher value",
            ),
            (
                &["input.header.(order) -> flow://x", DEFAULT],
                "is missing a header name",
            ),
            (
                &["input.body(order) -> flow://x", DEFAULT],
                "selector must be",
            ),
            (
                &["input.body.(x) -> flow://x", DEFAULT],
                "is missing a body key",
            ),
            (
                &["input.header.type(order) -> http://x", DEFAULT],
                "target must be",
            ),
            (
                &["input.header.type(order) -> flow://", DEFAULT],
                "target must be",
            ),
            (&[DEFAULT, DEFAULT], "only one 'default'"),
            (&[ORDER], "must contain a 'default -> <target>'"),
            (
                &["input.header.type(regex: ) -> flow://x", DEFAULT],
                "empty regex expression",
            ),
        ];
        for (list, reason) in cases {
            let error = compile_error(list);
            assert!(error.contains(reason), "{list:?}: {error}");
        }
        assert!(
            compile_error(&["input.header.type(regex: (unclosed) -> flow://x", DEFAULT])
                .contains("regex is invalid")
        );
        assert!(compile_error(&[]).contains("non-empty list"));
    }
}
