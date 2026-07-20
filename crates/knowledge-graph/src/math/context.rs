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

//! The evaluation context (Java `EvalContext`): variables, functions and the
//! `Math.*` namespace mirror. Functions and constants registered with
//! [`EvalContext::define_constant`] / [`define_function`] appear both
//! top-level and under `Math.`; plain variables/strings stay top-level only.

use std::collections::HashMap;
use std::sync::Arc;

use super::{eval_err, MathError};

/// The Java `MathFunction` analog; a Java function may throw, so the Rust
/// closure returns a `Result`.
pub type MathFunction = Arc<dyn Fn(&[f64]) -> Result<f64, MathError> + Send + Sync>;

/// What a context name can hold (the Java `Map<String, Object>` payloads).
#[derive(Clone)]
pub(crate) enum ContextValue {
    Number(f64),
    Str(String),
    Function(MathFunction),
    Namespace(HashMap<String, ContextValue>),
}

pub struct EvalContext {
    root: HashMap<String, ContextValue>,
}

impl Default for EvalContext {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl EvalContext {
    /// Constants (`PI`, `E`) and the standard function set, each mirrored
    /// into the `Math.*` namespace (Java `withDefaults`).
    pub fn with_defaults() -> Self {
        let mut ctx = EvalContext {
            root: HashMap::new(),
        };
        ctx.root
            .insert("Math".to_string(), ContextValue::Namespace(HashMap::new()));
        ctx = ctx
            .define_constant("PI", std::f64::consts::PI)
            .define_constant("E", std::f64::consts::E)
            .define_function("sin", |a| Ok(req1(a)?.sin()))
            .define_function("cos", |a| Ok(req1(a)?.cos()))
            .define_function("tan", |a| Ok(req1(a)?.tan()))
            .define_function("asin", |a| Ok(req1(a)?.asin()))
            .define_function("acos", |a| Ok(req1(a)?.acos()))
            .define_function("atan", |a| Ok(req1(a)?.atan()))
            .define_function("sqrt", |a| Ok(req1(a)?.sqrt()))
            .define_function("abs", |a| Ok(req1(a)?.abs()))
            .define_function("floor", |a| Ok(req1(a)?.floor()))
            .define_function("ceil", |a| Ok(req1(a)?.ceil()))
            .define_function("round", |a| Ok(java_round(req1(a)?)))
            .define_function("log", |a| Ok(req1(a)?.ln()))
            .define_function("log10", |a| Ok(req1(a)?.log10()))
            .define_function("exp", |a| Ok(req1(a)?.exp()))
            .define_function("min", |a| {
                Ok(a.iter().copied().fold(f64::INFINITY, f64::min))
            })
            .define_function("max", |a| {
                Ok(a.iter().copied().fold(f64::NEG_INFINITY, f64::max))
            })
            .define_function("pow", |a| {
                req_n(a, 2, "pow")?;
                Ok(a[0].powf(a[1]))
            })
            .define_function("random", |a| {
                req_n(a, 0, "random")?;
                secure_random()
            });
        ctx
    }

    /// Define a constant, mirrored into `Math.*`.
    pub fn define_constant(mut self, name: &str, value: f64) -> Self {
        self.root
            .insert(name.to_string(), ContextValue::Number(value));
        self.mirror_into_math(name, ContextValue::Number(value));
        self
    }

    /// Define a function, mirrored into `Math.*`.
    pub fn define_function(
        mut self,
        name: &str,
        function: impl Fn(&[f64]) -> Result<f64, MathError> + Send + Sync + 'static,
    ) -> Self {
        let f: MathFunction = Arc::new(function);
        self.root
            .insert(name.to_string(), ContextValue::Function(f.clone()));
        self.mirror_into_math(name, ContextValue::Function(f));
        self
    }

    /// Define a numeric variable (top-level only; for strings use
    /// [`define_string`](Self::define_string)).
    pub fn define_variable(mut self, name: &str, value: f64) -> Self {
        self.root
            .insert(name.to_string(), ContextValue::Number(value));
        self
    }

    /// Define a string variable (top-level only).
    pub fn define_string(mut self, name: &str, value: &str) -> Self {
        self.root
            .insert(name.to_string(), ContextValue::Str(value.to_string()));
        self
    }

    pub(crate) fn lookup(&self, name: &str) -> Option<&ContextValue> {
        self.root.get(name)
    }

    /// Member lookup: only namespaces have members (Java `member`).
    pub(crate) fn member<'a>(
        &self,
        target: Option<&'a ContextValue>,
        property: &str,
    ) -> Option<&'a ContextValue> {
        match target {
            Some(ContextValue::Namespace(map)) => map.get(property),
            _ => None,
        }
    }

    fn mirror_into_math(&mut self, name: &str, value: ContextValue) {
        if let Some(ContextValue::Namespace(map)) = self.root.get_mut("Math") {
            map.insert(name.to_string(), value);
        }
    }
}

// Java Math.round: floor(x + 0.5) clamped to the long range, NaN -> 0 —
// differs from Rust's f64::round (half away from zero) for negative halves
// like -2.5
fn java_round(x: f64) -> f64 {
    if x.is_nan() {
        return 0.0;
    }
    (x + 0.5).floor().clamp(i64::MIN as f64, i64::MAX as f64)
}

/// OS entropy → uniform double in [0, 1) — the `SecureRandom.nextDouble`
/// analog (53 random bits over 2^53).
fn secure_random() -> Result<f64, MathError> {
    let mut buf = [0u8; 8];
    getrandom::fill(&mut buf).map_err(|e| eval_err(format!("random() failed: {e}")))?;
    let bits = u64::from_le_bytes(buf) >> 11;
    Ok(bits as f64 / (1u64 << 53) as f64)
}

fn req1(a: &[f64]) -> Result<f64, MathError> {
    if a.len() != 1 {
        return Err(eval_err("Expected 1 argument"));
    }
    Ok(a[0])
}

fn req_n(a: &[f64], n: usize, function: &str) -> Result<(), MathError> {
    if a.len() != n {
        return Err(eval_err(format!(
            "Function {function} expects {n} args, got {}",
            a.len()
        )));
    }
    Ok(())
}
