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

//! The math expression engine — Rust port of the Java
//! `com.accenture.minigraph.math` package (lexer → non-recursive
//! shunting-yard parser → evaluator). The `graph.math` skill's fast inline
//! compute/branching path: JS-like expressions over numbers, booleans and
//! strings with short-circuit logic, ternaries, string concatenation and a
//! `Math.*` function namespace. Deliberately typed and bounded — this is the
//! sanctioned alternative to the retired `graph.js`.

mod context;
mod evaluator;
mod lexer;
mod parser;

pub use context::EvalContext;

/// Errors split the same way Java does: `Parse` is the `ParseException`
/// analog (lexer/parser), `Eval` the `IllegalArgumentException` analog
/// (evaluation-time type and lookup failures).
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum MathError {
    #[error("{0}")]
    Parse(String),
    #[error("{0}")]
    Eval(String),
}

impl MathError {
    pub fn message(&self) -> &str {
        match self {
            MathError::Parse(m) | MathError::Eval(m) => m,
        }
    }
}

pub(crate) fn parse_err(message: impl Into<String>) -> MathError {
    MathError::Parse(message.into())
}

pub(crate) fn eval_err(message: impl Into<String>) -> MathError {
    MathError::Eval(message.into())
}

/// Runtime values: number, boolean, string (the Java sealed `Value` union).
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(f64),
    Bool(bool),
    Str(String),
}

impl Value {
    /// Numeric view. Strings never coerce (Java parity).
    pub fn as_double(&self) -> Result<f64, MathError> {
        match self {
            Value::Number(n) => Ok(*n),
            Value::Bool(b) => Ok(if *b { 1.0 } else { 0.0 }),
            Value::Str(s) => Err(eval_err(format!("Cannot coerce string to number: \"{s}\""))),
        }
    }

    /// JS-like truthiness: 0 and NaN are falsy; the empty string is falsy.
    pub fn as_boolean(&self) -> bool {
        match self {
            Value::Number(n) => *n != 0.0 && !n.is_nan(),
            Value::Bool(b) => *b,
            Value::Str(s) => !s.is_empty(),
        }
    }

    /// Display view (Java `Value.asString()`): numbers render like Java's
    /// `Double.toString` — integral doubles keep a trailing `.0` ("3.0").
    /// String *concatenation* uses the plainer JS-like form instead (see
    /// [`evaluator`]).
    pub fn as_string(&self) -> String {
        match self {
            Value::Number(n) => java_double_to_string(*n),
            Value::Bool(b) => b.to_string(),
            Value::Str(s) => s.clone(),
        }
    }
}

/// Java `Double.toString` rendering for the tested/practical range: shortest
/// round-trip form with a forced decimal point (`3.0`, `0.56`), Java
/// spellings for the non-finite values.
pub(crate) fn java_double_to_string(d: f64) -> String {
    if d.is_nan() {
        "NaN".to_string()
    } else if d.is_infinite() {
        if d > 0.0 { "Infinity" } else { "-Infinity" }.to_string()
    } else {
        format!("{d:?}")
    }
}

/// The expression AST (the Java sealed `Expr` interface).
#[derive(Clone, Debug)]
pub(crate) enum Expr {
    NumberLiteral(f64),
    StringLiteral(String),
    BooleanLiteral(bool),
    Variable(String),
    Unary {
        op: &'static str,
        right: Box<Expr>,
    },
    Binary {
        op: &'static str,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    MemberAccess {
        target: Box<Expr>,
        property: String,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    Conditional {
        test: Box<Expr>,
        consequent: Box<Expr>,
        alternate: Box<Expr>,
    },
}

/// Convenience facade for parsing/evaluating expressions (Java
/// `ExpressionEngine`).
pub struct ExpressionEngine {
    ctx: EvalContext,
}

impl Default for ExpressionEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl ExpressionEngine {
    pub fn new() -> Self {
        Self::with_context(EvalContext::with_defaults())
    }

    pub fn with_context(ctx: EvalContext) -> Self {
        ExpressionEngine { ctx }
    }

    /// Evaluate to a number; booleans coerce to 0/1.
    pub fn eval_number(&self, expr: &str) -> Result<f64, MathError> {
        self.evaluate_value(expr)?.as_double()
    }

    /// Evaluate to a boolean with JS-like truthiness.
    pub fn eval_boolean(&self, expr: &str) -> Result<bool, MathError> {
        Ok(self.evaluate_value(expr)?.as_boolean())
    }

    pub fn evaluate_value(&self, expr: &str) -> Result<Value, MathError> {
        let ast = parser::parse(expr)?;
        evaluator::eval(&ast, &self.ctx)
    }

    pub fn context(&self) -> &EvalContext {
        &self.ctx
    }
}
