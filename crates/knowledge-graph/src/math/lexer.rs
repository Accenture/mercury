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

//! Tokenizer (Java `Lexer`/`Token`/`TokenType`). Positions are character
//! indices, matching the Java `charAt` addressing.

use super::{parse_err, MathError};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TokenType {
    // single-char tokens
    LParen,
    RParen,
    Comma,
    Dot,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Question,
    Colon,
    Bang,
    Lt,
    Gt,
    // multi-char tokens
    DoubleStar, // **
    Lte,
    Gte,
    EqEq,
    BangEq,
    AmpAmp,
    BarBar,
    // literals / identifiers
    Number,
    Str,
    Identifier,
    True,
    False,
    Eof,
}

#[derive(Clone, Debug)]
pub(crate) struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub position: usize,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}('{}')@{}",
            self.token_type, self.lexeme, self.position
        )
    }
}

fn token(token_type: TokenType, lexeme: impl Into<String>, position: usize) -> Token {
    Token {
        token_type,
        lexeme: lexeme.into(),
        position,
    }
}

pub(crate) struct Lexer {
    src: Vec<char>,
    idx: usize,
}

impl Lexer {
    pub fn new(src: &str) -> Self {
        Lexer {
            src: src.chars().collect(),
            idx: 0,
        }
    }

    pub fn next(&mut self) -> Result<Token, MathError> {
        self.skip_whitespace();
        if self.eof() {
            return Ok(token(TokenType::Eof, "", self.idx));
        }
        let c = self.src[self.idx];
        let start = self.idx;
        // strings: single or double-quoted
        if c == '\'' || c == '"' {
            return self.string_token();
        }
        // numbers: digit or leading '.' followed by digit
        if c.is_ascii_digit() || (c == '.' && self.peek_is_digit()) {
            return self.number_token();
        }
        // identifiers / keywords: [A-Za-z_][A-Za-z0-9_]*
        if c.is_alphabetic() || c == '_' {
            return Ok(self.identifier_token(start));
        }
        // operators / punctuation (multi-char first)
        self.operator_token(c, start)
    }

    fn string_token(&mut self) -> Result<Token, MathError> {
        let start = self.idx;
        let quote = self.src[self.idx];
        self.idx += 1;
        let mut value = String::new();
        while !self.eof() {
            let ch = self.src[self.idx];
            self.idx += 1;
            if ch == quote {
                return Ok(token(TokenType::Str, value, start));
            }
            if ch == '\\' {
                if self.eof() {
                    return Err(error("Unterminated string literal", start));
                }
                let esc = self.src[self.idx];
                self.idx += 1;
                match esc {
                    'n' => value.push('\n'),
                    'r' => value.push('\r'),
                    't' => value.push('\t'),
                    other => value.push(other), // \\ \' \" and any other char
                }
            } else {
                value.push(ch);
            }
        }
        Err(error("Unterminated string literal", start))
    }

    fn identifier_token(&mut self, start: usize) -> Token {
        self.idx += 1;
        while !self.eof() {
            let d = self.src[self.idx];
            if d.is_alphanumeric() || d == '_' {
                self.idx += 1;
            } else {
                break;
            }
        }
        let id: String = self.src[start..self.idx].iter().collect();
        match id.as_str() {
            "true" => token(TokenType::True, id, start),
            "false" => token(TokenType::False, id, start),
            _ => token(TokenType::Identifier, id, start),
        }
    }

    fn operator_token(&mut self, c: char, start: usize) -> Result<Token, MathError> {
        // multi-char operators first
        let two = match c {
            '*' if self.peek('*') => Some((TokenType::DoubleStar, "**")),
            '!' if self.peek('=') => Some((TokenType::BangEq, "!=")),
            '&' if self.peek('&') => Some((TokenType::AmpAmp, "&&")),
            '|' if self.peek('|') => Some((TokenType::BarBar, "||")),
            '=' if self.peek('=') => Some((TokenType::EqEq, "==")),
            '<' if self.peek('=') => Some((TokenType::Lte, "<=")),
            '>' if self.peek('=') => Some((TokenType::Gte, ">=")),
            _ => None,
        };
        if let Some((t, lex)) = two {
            self.idx += 2;
            return Ok(token(t, lex, start));
        }
        let one = match c {
            '(' => Some((TokenType::LParen, "(")),
            ')' => Some((TokenType::RParen, ")")),
            ',' => Some((TokenType::Comma, ",")),
            '.' => Some((TokenType::Dot, ".")),
            '+' => Some((TokenType::Plus, "+")),
            '-' => Some((TokenType::Minus, "-")),
            '*' => Some((TokenType::Star, "*")),
            '/' => Some((TokenType::Slash, "/")),
            '%' => Some((TokenType::Percent, "%")),
            '?' => Some((TokenType::Question, "?")),
            ':' => Some((TokenType::Colon, ":")),
            '!' => Some((TokenType::Bang, "!")),
            '<' => Some((TokenType::Lt, "<")),
            '>' => Some((TokenType::Gt, ">")),
            _ => None,
        };
        match one {
            Some((t, lex)) => {
                self.idx += 1;
                Ok(token(t, lex, start))
            }
            None => Err(error(&format!("Unexpected character: '{c}'"), start)),
        }
    }

    fn number_token(&mut self) -> Result<Token, MathError> {
        let start = self.idx;
        // integer part
        while !self.eof() && self.src[self.idx].is_ascii_digit() {
            self.idx += 1;
        }
        // fractional part
        if !self.eof() && self.src[self.idx] == '.' {
            self.handle_fractional(start)?;
        }
        // exponent part
        if !self.eof() && (self.src[self.idx] == 'e' || self.src[self.idx] == 'E') {
            self.handle_exponent()?;
        }
        let lex: String = self.src[start..self.idx].iter().collect();
        Ok(token(TokenType::Number, lex, start))
    }

    fn handle_fractional(&mut self, start: usize) -> Result<(), MathError> {
        self.idx += 1;
        if self.eof() || !self.src[self.idx].is_ascii_digit() {
            return Err(error("Malformed number literal", start));
        }
        while !self.eof() && self.src[self.idx].is_ascii_digit() {
            self.idx += 1;
        }
        Ok(())
    }

    fn handle_exponent(&mut self) -> Result<(), MathError> {
        let exp_pos = self.idx;
        self.idx += 1;
        if !self.eof() && (self.src[self.idx] == '+' || self.src[self.idx] == '-') {
            self.idx += 1;
        }
        if self.eof() || !self.src[self.idx].is_ascii_digit() {
            return Err(error("Malformed exponent in number literal", exp_pos));
        }
        while !self.eof() && self.src[self.idx].is_ascii_digit() {
            self.idx += 1;
        }
        Ok(())
    }

    fn skip_whitespace(&mut self) {
        while !self.eof() && self.src[self.idx].is_whitespace() {
            self.idx += 1;
        }
    }

    fn peek(&self, ch: char) -> bool {
        self.idx + 1 < self.src.len() && self.src[self.idx + 1] == ch
    }

    fn peek_is_digit(&self) -> bool {
        self.idx + 1 < self.src.len() && self.src[self.idx + 1].is_ascii_digit()
    }

    fn eof(&self) -> bool {
        self.idx >= self.src.len()
    }
}

fn error(message: &str, position: usize) -> MathError {
    parse_err(format!("{message} at position {position}"))
}
