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

//! Compile-time data-mapping validation — the syntax-checking half of the
//! Java `DataMappingHelper` (`validInput` / `validOutput` / `validModel`) plus
//! the reserved-state-machine-key guard from `CompileFlows`. The runtime
//! *resolution* half arrives with design increment E-2.

use crate::plugins;
use crate::util::split;

pub const MAP_TO: &str = "->";
const MODEL_NAMESPACE: &str = "model.";
const CONSTANT_TYPES: &[&str] = &[
    "text(",
    "file(",
    "classpath(",
    "int(",
    "long(",
    "float(",
    "double(",
    "boolean(",
];

/// Reserved state-machine keys a data mapping must never overwrite: the
/// read-only metadata seeded into each flow instance, the `model.none` null
/// constant, and the `model.parent` / `model.root` aliases themselves
/// (writing *beneath* parent/root is the shared-state mechanism and allowed).
const RESERVED_MODEL_KEYS: &[&str] = &[
    "model.cid",
    "model.instance",
    "model.flow",
    "model.ttl",
    "model.trace",
    "model.none",
    "model.parent",
    "model.root",
];

/// Java `CompileFlows.reservedModelKeyViolation`: exact matches rejected for
/// all reserved keys; nested writes (`model.cid.x`, `model.cid[0]`) also
/// rejected for the scalar keys. Public so the runtime task executor can
/// re-check dynamically substituted targets (E-4).
pub fn reserved_model_key_violation(rhs: &str) -> Option<&'static str> {
    for reserved in RESERVED_MODEL_KEYS {
        if rhs == *reserved {
            return Some(reserved);
        }
        let shared = *reserved == "model.parent" || *reserved == "model.root";
        if !shared
            && (rhs.starts_with(&format!("{reserved}."))
                || rhs.starts_with(&format!("{reserved}[")))
        {
            return Some(reserved);
        }
    }
    None
}

/// Java `DataMappingHelper.validInput`.
pub fn valid_input(input: &str) -> bool {
    let Some(sep) = input.rfind(MAP_TO) else {
        return false;
    };
    if sep == 0 {
        return false;
    }
    let lhs = input[..sep].trim();
    let rhs = input[sep + 2..].trim();
    if is_pluggable_function(rhs) {
        false
    } else if is_pluggable_function(lhs) {
        is_valid_pluggable_function(lhs)
    } else if valid_model(lhs) && valid_model(rhs) && lhs != rhs {
        valid_input_lhs(lhs)
    } else {
        false
    }
}

/// Java `DataMappingHelper.validOutput(output, isDecision)`.
pub fn valid_output(output: &str, is_decision: bool) -> bool {
    let Some(sep) = output.rfind(MAP_TO) else {
        return false;
    };
    if sep == 0 {
        return false;
    }
    let lhs = output[..sep].trim();
    let rhs = output[sep + 2..].trim();
    if valid_model(lhs) && valid_model(rhs) && lhs != rhs {
        valid_output_lhs(lhs) && valid_output_rhs(rhs, is_decision)
    } else {
        false
    }
}

fn is_pluggable_function(token: &str) -> bool {
    token.starts_with("f:")
}

fn is_valid_pluggable_function(lhs: &str) -> bool {
    if !lhs.ends_with(')') {
        return false;
    }
    match lhs.find('(') {
        Some(bracket) if bracket > 0 => {
            let name = lhs[2..bracket].trim();
            plugins::contains_simple_plugin(name)
        }
        _ => false,
    }
}

/// Java `DataMappingHelper.validModel`: whole-`model` access and the whole
/// `model.parent` / `model.root` namespaces are not allowed.
fn valid_model(key: &str) -> bool {
    let parts = split(key, "!: ()");
    let Some(first) = parts.first() else {
        return false;
    };
    if first == "model" {
        return false;
    }
    if first.starts_with(MODEL_NAMESPACE) {
        let segments = split(first, ".");
        return segments.len() != 1
            && (segments.len() != 2 || (segments[1] != "parent" && segments[1] != "root"));
    }
    true
}

fn valid_input_lhs(lhs: &str) -> bool {
    if lhs == "input"
        || lhs.starts_with("input.")
        || lhs.starts_with('$')
        || lhs.starts_with(MODEL_NAMESPACE)
        || lhs.starts_with("error.")
    {
        true
    } else if lhs.starts_with("map(") && lhs.ends_with(')') {
        valid_key_values(lhs)
    } else {
        CONSTANT_TYPES.iter().any(|t| lhs.starts_with(t)) && lhs.ends_with(')')
    }
}

fn valid_output_lhs(lhs: &str) -> bool {
    if lhs == "input"
        || lhs.starts_with("input.")
        || is_pluggable_function(lhs)
        || lhs.starts_with('$')
        || lhs.starts_with(MODEL_NAMESPACE)
        || lhs == "datatype"
        || lhs == "result"
        || lhs.starts_with("result.")
        || lhs == "status"
        || lhs == "header"
        || lhs.starts_with("header.")
    {
        true
    } else if lhs.starts_with("map(") && lhs.ends_with(')') {
        valid_key_values(lhs)
    } else {
        CONSTANT_TYPES.iter().any(|t| lhs.starts_with(t)) && lhs.ends_with(')')
    }
}

fn valid_output_rhs(rhs: &str, is_decision: bool) -> bool {
    (rhs == "decision" && is_decision)
        || rhs.starts_with("file(")
        || rhs.starts_with("output.")
        || rhs.starts_with(MODEL_NAMESPACE)
        || rhs.starts_with("ext:")
}

/// Java `DataMappingHelper.validKeyValues`: `map(k=v, ...)` — non-empty,
/// unique keys.
fn valid_key_values(text: &str) -> bool {
    let Some(last) = text.rfind(')') else {
        return false;
    };
    let inner = text["map(".len()..last].trim();
    if inner.contains('=') || inner.contains(',') {
        let key_values = split(inner, ",");
        let mut keys = std::collections::HashSet::new();
        for kv in &key_values {
            let key = match kv.find('=') {
                Some(eq) => kv[..eq].trim(),
                None => kv.trim(),
            };
            if key.is_empty() {
                return false;
            }
            keys.insert(key.to_string());
        }
        keys.len() == key_values.len()
    } else {
        !inner.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reserved_keys_are_guarded() {
        assert_eq!(reserved_model_key_violation("model.cid"), Some("model.cid"));
        assert_eq!(
            reserved_model_key_violation("model.cid.x"),
            Some("model.cid")
        );
        assert_eq!(
            reserved_model_key_violation("model.trace[0]"),
            Some("model.trace")
        );
        assert_eq!(
            reserved_model_key_violation("model.parent"),
            Some("model.parent")
        );
        // shared-state writes beneath parent/root stay valid
        assert_eq!(reserved_model_key_violation("model.parent.x"), None);
        assert_eq!(reserved_model_key_violation("model.root.y"), None);
        assert_eq!(reserved_model_key_violation("model.cidx"), None);
    }

    #[test]
    fn input_mapping_rules_match_java() {
        assert!(valid_input("input.body -> data"));
        assert!(valid_input("text(hello) -> greeting"));
        assert!(valid_input("map(k=v) -> m"));
        assert!(valid_input("f:uuid(model.none) -> id"));
        // plugin on RHS is invalid
        assert!(!valid_input("input.body -> f:uuid(x)"));
        // unknown plugin name rejected (registry check)
        assert!(!valid_input("f:noSuchPlugin(model.x) -> y"));
        // whole-model access rejected
        assert!(!valid_input("model -> data"));
        assert!(!valid_input("text(hello) -> model"));
        assert!(!valid_input("input.x -> model.parent"));
        // identical LHS/RHS rejected
        assert!(!valid_input("model.x -> model.x"));
        // duplicate map keys rejected
        assert!(!valid_input("map(k=1, k=2) -> m"));
        assert!(!valid_input("map() -> m"));
        assert!(!valid_input("map(=v) -> m"));
    }

    #[test]
    fn output_mapping_rules_match_java() {
        assert!(valid_output("result -> output.body", false));
        assert!(valid_output("status -> output.status", false));
        assert!(valid_output("header.demo -> output.header.x-demo", false));
        assert!(valid_output("result.code -> decision", true));
        // decision target only valid for decision tasks
        assert!(!valid_output("result.code -> decision", false));
        // output namespace is not a valid output LHS
        assert!(!valid_output("output.something -> model.x", false));
        // RHS must be output/model/file/ext
        assert!(!valid_output("result -> somewhere", false));
        // plugin LHS allowed on output without registry check (Java parity)
        assert!(valid_output("f:whatever(model.x) -> output.body.y", false));
    }
}
