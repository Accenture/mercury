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

//! Small ports of the Java `Utility` methods the compiler relies on, kept
//! behavior-identical (`split` drops empty tokens; `getDurationInSeconds`
//! takes s/m/h/d suffixes, plain numbers are seconds).

/// Java `Utility.split(text, chars)`: tokenize on ANY character in
/// `separators`, dropping empty tokens.
pub fn split(text: &str, separators: &str) -> Vec<String> {
    text.split(|c| separators.contains(c))
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

/// Java `Utility.isNumeric`: all digits, with one optional leading minus
/// (a lone "-" is not numeric).
pub fn is_numeric(text: &str) -> bool {
    let digits = text
        .strip_prefix('-')
        .filter(|_| text.len() > 1)
        .unwrap_or(text);
    !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit())
}

/// Java `Utility.str2long` semantics for our use: parse or 0.
pub fn str2long(text: &str) -> i64 {
    text.trim().parse::<i64>().unwrap_or(0)
}

/// Java `Utility.getDurationInSeconds`: `s`/`m`/`h`/`d` suffix, otherwise the
/// number is already seconds; non-numeric input yields 0 (Java `str2int`).
pub fn duration_in_seconds(duration: &str) -> i64 {
    let d = duration.trim();
    let (number, multiplier) = match d.as_bytes().last() {
        Some(b's') => (&d[..d.len() - 1], 1),
        Some(b'm') => (&d[..d.len() - 1], 60),
        Some(b'h') => (&d[..d.len() - 1], 60 * 60),
        Some(b'd') => (&d[..d.len() - 1], 24 * 60 * 60),
        _ => (d, 1),
    };
    str2long(number) * multiplier
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_drops_empty_tokens() {
        assert_eq!(split("a, b,, c", ", "), vec!["a", "b", "c"]);
        assert_eq!(
            split("(model.n = 0; model.n < 3; model.n++)", "(;)").len(),
            3
        );
    }

    #[test]
    fn numeric_check_matches_java() {
        assert!(is_numeric("123"));
        assert!(is_numeric("-5"));
        assert!(!is_numeric("-"));
        assert!(!is_numeric("12.5"));
        assert!(!is_numeric(""));
    }

    #[test]
    fn durations_match_java() {
        assert_eq!(duration_in_seconds("10s"), 10);
        assert_eq!(duration_in_seconds("5m"), 300);
        assert_eq!(duration_in_seconds("2h"), 7200);
        assert_eq!(duration_in_seconds("1d"), 86400);
        assert_eq!(duration_in_seconds("30"), 30);
    }
}
