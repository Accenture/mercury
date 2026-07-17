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

//! Non-recursive expression parser (Java `Parser`): shunting-yard to AST
//! with postfix (call/member) support and the strict JS rule that a unary
//! expression cannot be the left operand of `**`.

use super::lexer::{Lexer, Token, TokenType};
use super::{parse_err, Expr, MathError};

// precedence (higher binds tighter)
const BP_EXP: i32 = 70;
const BP_UNARY: i32 = 60; // prefix + - !
const BP_MUL: i32 = 50;
const BP_ADD: i32 = 40;
const BP_REL: i32 = 30; // < <= > >=
const BP_EQ: i32 = 20; // == !=
const BP_AND: i32 = 12; // &&
const BP_OR: i32 = 10; // ||
const BP_TERNARY: i32 = 5; // ?:

#[derive(Clone, Copy, PartialEq, Eq)]
enum Assoc {
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Binary,
    Unary,
    TernaryQ,
    LParen,
}

struct Op {
    kind: Kind,
    symbol: &'static str,
    prec: i32,
    assoc: Assoc,
    seen_colon: bool, // only for TernaryQ
}

impl Op {
    fn new(kind: Kind, symbol: &'static str, prec: i32, assoc: Assoc) -> Self {
        Op {
            kind,
            symbol,
            prec,
            assoc,
            seen_colon: false,
        }
    }
}

/// Parse an expression with the strict JS `**` rule (the only mode the
/// engine uses).
pub(crate) fn parse(src: &str) -> Result<Expr, MathError> {
    let mut parser = Parser::new(src, true)?;
    let expr = parser.parse_expression_until(&[TokenType::Eof])?;
    parser.expect(TokenType::Eof)?;
    Ok(expr)
}

struct Parser {
    lexer: Lexer,
    lookahead: Token,
    strict_js_exponentiation_rule: bool,
}

impl Parser {
    fn new(src: &str, strict_js_exponentiation_rule: bool) -> Result<Self, MathError> {
        let mut lexer = Lexer::new(src);
        let lookahead = lexer.next()?;
        Ok(Parser {
            lexer,
            lookahead,
            strict_js_exponentiation_rule,
        })
    }

    fn parse_expression_until(&mut self, stoppers: &[TokenType]) -> Result<Expr, MathError> {
        let mut values: Vec<Expr> = Vec::new();
        let mut ops: Vec<Op> = Vec::new();
        let mut expect_operand = true;

        loop {
            let t = self.lookahead.clone();
            if stoppers.contains(&t.token_type) && not_contains_lparen(&ops) {
                break;
            }
            match t.token_type {
                // ----- primaries -----
                TokenType::Number => {
                    self.consume()?;
                    values.push(Expr::NumberLiteral(parse_double(&t.lexeme, t.position)?));
                    expect_operand = false;
                    self.parse_postfix_chain_on_top(&mut values)?;
                }
                TokenType::Str => {
                    self.consume()?;
                    values.push(Expr::StringLiteral(t.lexeme.clone()));
                    expect_operand = false;
                    self.parse_postfix_chain_on_top(&mut values)?;
                }
                TokenType::True | TokenType::False => {
                    self.consume()?;
                    values.push(Expr::BooleanLiteral(t.token_type == TokenType::True));
                    expect_operand = false;
                    self.parse_postfix_chain_on_top(&mut values)?;
                }
                TokenType::Identifier => {
                    self.consume()?;
                    values.push(Expr::Variable(t.lexeme.clone()));
                    expect_operand = false;
                    self.parse_postfix_chain_on_top(&mut values)?;
                }
                TokenType::LParen => {
                    self.consume()?;
                    ops.push(Op::new(Kind::LParen, "(", -1, Assoc::Left));
                    expect_operand = true;
                }
                TokenType::RParen => {
                    self.consume()?;
                    while ops.last().is_some_and(|op| op.kind != Kind::LParen) {
                        let op = ops.pop().expect("checked non-empty");
                        apply_op(&mut values, op)?;
                    }
                    if ops.last().is_none_or(|op| op.kind != Kind::LParen) {
                        return Err(parse_err(format!(
                            "Unmatched ')' at position {}",
                            t.position
                        )));
                    }
                    ops.pop();
                    expect_operand = false;
                    self.parse_postfix_chain_on_top(&mut values)?;
                }
                // ----- separators -----
                // a comma that is a live stopper broke at the top of the
                // loop, so reaching here is always an error (Java parity)
                TokenType::Comma => {
                    return Err(parse_err(format!(
                        "Unexpected ',' at position {}",
                        t.position
                    )));
                }
                TokenType::Colon => {
                    self.consume()?;
                    while ops.last().is_some_and(|op| op.kind != Kind::TernaryQ) {
                        let op = ops.pop().expect("checked non-empty");
                        apply_op(&mut values, op)?;
                    }
                    match ops.last_mut() {
                        Some(op) if op.kind == Kind::TernaryQ => op.seen_colon = true,
                        _ => {
                            return Err(parse_err(format!(
                                "':' without matching '?' at position {}",
                                t.position
                            )));
                        }
                    }
                    expect_operand = true;
                }
                // ----- operators -----
                TokenType::Question => {
                    // reduce higher-precedence ops so ternary sees the
                    // completed test on the left: (A || B) ? X : Y
                    while !ops.is_empty() {
                        if break_on_precedence(&mut ops, &mut values)? {
                            break;
                        }
                    }
                    self.consume()?;
                    ops.push(Op::new(Kind::TernaryQ, "?:", BP_TERNARY, Assoc::Right));
                    expect_operand = true;
                }
                TokenType::Plus | TokenType::Minus | TokenType::Bang => {
                    if expect_operand {
                        self.consume()?;
                        let symbol = match t.token_type {
                            TokenType::Plus => "+",
                            TokenType::Minus => "-",
                            _ => "!",
                        };
                        ops.push(Op::new(Kind::Unary, symbol, BP_UNARY, Assoc::Right));
                    } else {
                        if t.token_type == TokenType::Bang {
                            return Err(parse_err(format!(
                                "Unexpected '!' at position {}",
                                t.position
                            )));
                        }
                        self.push_binary_and_reduce(&mut values, &mut ops, &t)?;
                        expect_operand = true;
                    }
                }
                TokenType::Star
                | TokenType::Slash
                | TokenType::Percent
                | TokenType::DoubleStar
                | TokenType::Lt
                | TokenType::Lte
                | TokenType::Gt
                | TokenType::Gte
                | TokenType::EqEq
                | TokenType::BangEq
                | TokenType::AmpAmp
                | TokenType::BarBar => {
                    if expect_operand {
                        return Err(parse_err(format!(
                            "Missing left operand before '{}' at position {}",
                            t.lexeme, t.position
                        )));
                    }
                    // JS rule: UnaryExpression cannot be left operand of '**'
                    if t.token_type == TokenType::DoubleStar
                        && self.strict_js_exponentiation_rule
                        && ops.last().is_some_and(|op| op.kind == Kind::Unary)
                    {
                        return Err(parse_err(
                            "Unary expression cannot be left operand of '**'. \
                             Use parentheses, e.g., -(2**2).",
                        ));
                    }
                    self.push_binary_and_reduce(&mut values, &mut ops, &t)?;
                    expect_operand = true;
                }
                _ => return Err(parse_err(format!("Unexpected token: {t}"))),
            }
        }

        while let Some(op) = ops.pop() {
            if op.kind == Kind::LParen {
                return Err(parse_err("Unmatched '('"));
            }
            apply_op(&mut values, op)?;
        }
        if values.len() != 1 {
            return Err(parse_err("Incomplete expression"));
        }
        Ok(values.pop().expect("length checked"))
    }

    fn push_binary_and_reduce(
        &mut self,
        values: &mut Vec<Expr>,
        ops: &mut Vec<Op>,
        t: &Token,
    ) -> Result<(), MathError> {
        let curr = match t.token_type {
            TokenType::DoubleStar => Op::new(Kind::Binary, "**", BP_EXP, Assoc::Right),
            TokenType::Star => Op::new(Kind::Binary, "*", BP_MUL, Assoc::Left),
            TokenType::Slash => Op::new(Kind::Binary, "/", BP_MUL, Assoc::Left),
            TokenType::Percent => Op::new(Kind::Binary, "%", BP_MUL, Assoc::Left),
            TokenType::Plus => Op::new(Kind::Binary, "+", BP_ADD, Assoc::Left),
            TokenType::Minus => Op::new(Kind::Binary, "-", BP_ADD, Assoc::Left),
            TokenType::Lt => Op::new(Kind::Binary, "<", BP_REL, Assoc::Left),
            TokenType::Lte => Op::new(Kind::Binary, "<=", BP_REL, Assoc::Left),
            TokenType::Gt => Op::new(Kind::Binary, ">", BP_REL, Assoc::Left),
            TokenType::Gte => Op::new(Kind::Binary, ">=", BP_REL, Assoc::Left),
            TokenType::EqEq => Op::new(Kind::Binary, "==", BP_EQ, Assoc::Left),
            TokenType::BangEq => Op::new(Kind::Binary, "!=", BP_EQ, Assoc::Left),
            TokenType::AmpAmp => Op::new(Kind::Binary, "&&", BP_AND, Assoc::Left),
            TokenType::BarBar => Op::new(Kind::Binary, "||", BP_OR, Assoc::Left),
            _ => return Err(parse_err(format!("Not a binary operator: {t}"))),
        };
        while !ops.is_empty() {
            if break_on_paren_ternary(&curr, ops, values)? {
                break;
            }
        }
        ops.push(curr);
        self.consume()
    }

    fn parse_postfix_chain_on_top(&mut self, values: &mut Vec<Expr>) -> Result<(), MathError> {
        loop {
            if self.lookahead.token_type == TokenType::Dot {
                self.consume()?;
                let id = self.expect(TokenType::Identifier)?;
                let target = values
                    .pop()
                    .ok_or_else(|| parse_err("Incomplete expression"))?;
                values.push(Expr::MemberAccess {
                    target: Box::new(target),
                    property: id.lexeme,
                });
            } else if self.lookahead.token_type == TokenType::LParen {
                self.consume()?;
                let mut args = Vec::new();
                if self.lookahead.token_type != TokenType::RParen {
                    loop {
                        let arg =
                            self.parse_expression_until(&[TokenType::Comma, TokenType::RParen])?;
                        args.push(arg);
                        if self.lookahead.token_type == TokenType::Comma {
                            self.consume()?;
                        } else {
                            break;
                        }
                    }
                }
                self.expect(TokenType::RParen)?;
                let callee = values
                    .pop()
                    .ok_or_else(|| parse_err("Incomplete expression"))?;
                values.push(Expr::Call {
                    callee: Box::new(callee),
                    args,
                });
            } else {
                break;
            }
        }
        Ok(())
    }

    fn expect(&mut self, token_type: TokenType) -> Result<Token, MathError> {
        if self.lookahead.token_type != token_type {
            return Err(parse_err(format!(
                "Expected {token_type:?} but found {}",
                self.lookahead
            )));
        }
        let t = self.lookahead.clone();
        self.lookahead = self.lexer.next()?;
        Ok(t)
    }

    fn consume(&mut self) -> Result<(), MathError> {
        self.lookahead = self.lexer.next()?;
        Ok(())
    }
}

/// Ternary reduction: apply unaries and higher-precedence binaries; stop at
/// `(`, another `?` or lower precedence.
fn break_on_precedence(ops: &mut Vec<Op>, values: &mut Vec<Expr>) -> Result<bool, MathError> {
    let Some(top) = ops.last() else {
        return Ok(true);
    };
    match top.kind {
        Kind::LParen => Ok(true),
        Kind::Unary => {
            let op = ops.pop().expect("checked non-empty");
            apply_op(values, op)?;
            Ok(false)
        }
        Kind::TernaryQ => Ok(true), // chain ternaries normally
        Kind::Binary => {
            if top.prec > BP_TERNARY {
                let op = ops.pop().expect("checked non-empty");
                apply_op(values, op)?;
                Ok(false)
            } else {
                Ok(true)
            }
        }
    }
}

fn break_on_paren_ternary(
    curr: &Op,
    ops: &mut Vec<Op>,
    values: &mut Vec<Expr>,
) -> Result<bool, MathError> {
    let Some(top) = ops.last() else {
        return Ok(true);
    };
    match top.kind {
        Kind::LParen => Ok(true),
        Kind::TernaryQ => {
            if curr.prec <= BP_TERNARY {
                let op = ops.pop().expect("checked non-empty");
                apply_op(values, op)?;
                Ok(false)
            } else {
                Ok(true)
            }
        }
        Kind::Unary => {
            let op = ops.pop().expect("checked non-empty");
            apply_op(values, op)?;
            Ok(false)
        }
        Kind::Binary => {
            let take_top = top.prec > curr.prec
                || (top.prec == curr.prec && top.assoc == Assoc::Left && curr.assoc == Assoc::Left);
            if take_top {
                let op = ops.pop().expect("checked non-empty");
                apply_op(values, op)?;
                Ok(false)
            } else {
                Ok(true)
            }
        }
    }
}

fn apply_op(values: &mut Vec<Expr>, op: Op) -> Result<(), MathError> {
    match op.kind {
        Kind::Unary => {
            let right = values
                .pop()
                .ok_or_else(|| parse_err(format!("Missing operand for unary '{}'", op.symbol)))?;
            values.push(Expr::Unary {
                op: op.symbol,
                right: Box::new(right),
            });
            Ok(())
        }
        Kind::Binary => {
            let right = values.pop();
            let left = values.pop();
            let (Some(left), Some(right)) = (left, right) else {
                return Err(parse_err(format!(
                    "Missing operands for binary '{}'",
                    op.symbol
                )));
            };
            values.push(Expr::Binary {
                op: op.symbol,
                left: Box::new(left),
                right: Box::new(right),
            });
            Ok(())
        }
        Kind::TernaryQ => {
            if !op.seen_colon {
                return Err(parse_err("Missing ':' for ternary operator"));
            }
            let alternate = values.pop();
            let consequent = values.pop();
            let test = values.pop();
            let (Some(test), Some(consequent), Some(alternate)) = (test, consequent, alternate)
            else {
                return Err(parse_err("Malformed ternary expression"));
            };
            values.push(Expr::Conditional {
                test: Box::new(test),
                consequent: Box::new(consequent),
                alternate: Box::new(alternate),
            });
            Ok(())
        }
        Kind::LParen => Err(parse_err("Unmatched '('")),
    }
}

fn not_contains_lparen(ops: &[Op]) -> bool {
    !ops.iter().any(|op| op.kind == Kind::LParen)
}

fn parse_double(s: &str, position: usize) -> Result<f64, MathError> {
    s.parse::<f64>()
        .map_err(|_| parse_err(format!("Invalid number '{s}' at position {position}")))
}
