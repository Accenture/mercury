// Included by plugins.rs — the increment E-8 built-in plugin bodies
// (arithmetic, generators, dates, comparisons, list-of-map operations and
// input validation), each a faithful port of its Java plugin class.

/// Java-style numeric promotion for the plugin family: whole numbers stay
/// exact `i64`, floating-point values are `f64`.
enum Number {
    Long(i64),
    Double(f64),
}

impl Number {
    fn as_f64(&self) -> f64 {
        match self {
            Number::Long(l) => *l as f64,
            Number::Double(d) => *d,
        }
    }

    fn is_zero(&self) -> bool {
        match self {
            Number::Long(l) => *l == 0,
            Number::Double(d) => *d == 0.0,
        }
    }
}

/// Java `SimplePluginUtils.promoteNumber`: whole numbers and whole-number
/// strings promote to i64, floating-point values and decimal strings to f64
/// (generalized from whole-number-only — maintainer decision, 2026-07-19);
/// anything else is an error.
fn promote_number(value: &Value) -> Result<Number, String> {
    match value {
        Value::Integer(i) => i
            .as_i64()
            .map(Number::Long)
            .ok_or_else(|| format!("Cannot convert the object to a number: {value}")),
        Value::F32(f) => Ok(Number::Double(*f as f64)),
        Value::F64(f) => Ok(Number::Double(*f)),
        Value::String(s) => {
            let text = s.as_str().unwrap_or_default().trim();
            match text.parse::<i64>() {
                Ok(whole) => Ok(Number::Long(whole)),
                Err(_) => text
                    .parse::<f64>()
                    .map(Number::Double)
                    .map_err(|_| format!("Cannot convert the object to a number: {value}")),
            }
        }
        other => Err(format!("Cannot convert the object to a number: {other}")),
    }
}

/// Fold with numeric promotion decided over ALL args first (so the result is
/// order-independent): any floating-point arg promotes the whole computation
/// to f64; all-integral inputs keep exact i64 arithmetic — including integer
/// division, exactly as before this generalization.
fn fold_numbers(
    args: &[Value],
    what: &str,
    op_long: impl Fn(i64, i64) -> i64,
    op_double: impl Fn(f64, f64) -> f64,
) -> Result<Value, String> {
    if args.len() < 2 {
        return Err(format!("Expected at least two Numbers to {what}"));
    }
    let numbers: Vec<Number> = args
        .iter()
        .map(promote_number)
        .collect::<Result<_, String>>()?;
    if numbers.iter().any(|n| matches!(n, Number::Double(_))) {
        let mut total = numbers[0].as_f64();
        for n in &numbers[1..] {
            total = op_double(total, n.as_f64());
        }
        Ok(Value::from(total))
    } else {
        let mut total = match numbers[0] {
            Number::Long(l) => l,
            Number::Double(_) => unreachable!("checked above"),
        };
        for n in &numbers[1..] {
            let Number::Long(l) = n else {
                unreachable!("checked above")
            };
            total = op_long(total, *l);
        }
        Ok(Value::from(total))
    }
}

fn plugin_add(args: &[Value]) -> Result<Value, String> {
    fold_numbers(args, "add", |a, b| a + b, |a, b| a + b)
}

fn plugin_subtract(args: &[Value]) -> Result<Value, String> {
    fold_numbers(args, "subtract", |a, b| a - b, |a, b| a - b)
}

fn plugin_multiply(args: &[Value]) -> Result<Value, String> {
    fold_numbers(args, "multiply", |a, b| a * b, |a, b| a * b)
}

fn divide_by_zero_check(divisors: &[Value]) -> Result<(), String> {
    for v in divisors {
        if promote_number(v)?.is_zero() {
            return Err("Dividing the input would cause Division By Zero".to_string());
        }
    }
    Ok(())
}

fn plugin_div(args: &[Value]) -> Result<Value, String> {
    if args.len() < 2 {
        return Err("Expected at least two Numbers to divide".to_string());
    }
    // only divisors can trigger division by zero - a zero dividend is valid
    divide_by_zero_check(&args[1..])?;
    fold_numbers(args, "divide", |a, b| a / b, |a, b| a / b)
}

fn plugin_mod(args: &[Value]) -> Result<Value, String> {
    if args.len() != 2 {
        return Err("Modulus expects only two values".to_string());
    }
    divide_by_zero_check(&args[1..])?;
    fold_numbers(args, "modulus", |a, b| a % b, |a, b| a % b)
}

fn plugin_increment(args: &[Value]) -> Result<Value, String> {
    match args {
        [one] => Ok(match promote_number(one)? {
            Number::Long(l) => Value::from(l + 1),
            Number::Double(d) => Value::from(d + 1.0),
        }),
        _ => Err("Expected exactly one Number to increment".to_string()),
    }
}

fn plugin_decrement(args: &[Value]) -> Result<Value, String> {
    match args {
        [one] => Ok(match promote_number(one)? {
            Number::Long(l) => Value::from(l - 1),
            Number::Double(d) => Value::from(d - 1.0),
        }),
        _ => Err("Expected exactly one Number to decrement".to_string()),
    }
}

/// Half-up decimal rounding (ties away from zero) applied to the value's
/// SHORTEST decimal representation — the Java `BigDecimal.valueOf(double)
/// .setScale(dp, HALF_UP)` analog — so binary representation error does not
/// leak into the rounding decision (1.005 rounds to 1.01 at 2 places, not
/// 1.0 as a naive multiply-round-divide would give).
fn round_half_up(value: f64, dp: usize) -> f64 {
    if !value.is_finite() {
        return value;
    }
    let negative = value.is_sign_negative();
    // Rust's Display prints the shortest round-trip decimal form, without
    // exponent notation (the Double.toString analog)
    let text = format!("{}", value.abs());
    let (int_part, frac_part) = match text.split_once('.') {
        Some((i, f)) => (i, f),
        None => (text.as_str(), ""),
    };
    if frac_part.len() <= dp {
        return value;
    }
    let mut digits: Vec<u8> = format!("{int_part}{}", &frac_part[..dp]).into_bytes();
    if frac_part.as_bytes()[dp] >= b'5' {
        // increment the kept digit string with carry
        let mut i = digits.len();
        loop {
            if i == 0 {
                digits.insert(0, b'1');
                break;
            }
            i -= 1;
            if digits[i] == b'9' {
                digits[i] = b'0';
            } else {
                digits[i] += 1;
                break;
            }
        }
    }
    let digits = String::from_utf8(digits).expect("ascii digits");
    let (new_int, new_frac) = digits.split_at(digits.len() - dp);
    let text = if dp == 0 {
        new_int.to_string()
    } else {
        format!("{new_int}.{new_frac}")
    };
    let rounded: f64 = text.parse().unwrap_or(value);
    if rounded == 0.0 {
        // never return a negative zero
        0.0
    } else if negative {
        -rounded
    } else {
        rounded
    }
}

fn plugin_round(args: &[Value]) -> Result<Value, String> {
    let (number, dp) = match args {
        [one] => (promote_number(one)?, 0usize),
        [one, two] => {
            let dp = match promote_number(two)? {
                Number::Long(l) if l >= 0 => l as usize,
                _ => {
                    return Err(
                        "Decimal places for round must be a whole number >= 0".to_string()
                    )
                }
            };
            (promote_number(one)?, dp)
        }
        _ => return Err("Expected a number and optional decimal places to round".to_string()),
    };
    Ok(match number {
        // a whole number is already exact - rounding never changes it
        Number::Long(l) => Value::from(l),
        Number::Double(d) => Value::from(round_half_up(d, dp)),
    })
}

fn plugin_now(args: &[Value]) -> Result<Value, String> {
    let command = args
        .first()
        .map(get_text_value)
        .unwrap_or_else(|| "iso".to_string());
    match command.to_ascii_lowercase().as_str() {
        "iso" => Ok(Value::from(platform_core::trace::iso8601_utc_now())),
        "local" => Ok(Value::from(
            chrono::Local::now()
                .format("%Y-%m-%d %H:%M:%S%.3f")
                .to_string(),
        )),
        "ms" => Ok(Value::from(chrono::Utc::now().timestamp_millis())),
        _ => Err("Supported types are: iso, local, ms".to_string()),
    }
}

/// Convert a Java `DateTimeFormatter` pattern to a chrono format string
/// (increment 53, parity F5 — was a naive six-token replace that passed
/// everything else through as garbage). A real tokenizer: repeated pattern
/// letters, `'quoted literals'` (with `''` as an escaped quote), literal
/// separators (`%` escaped). Coverage is the practical Java letter set;
/// an unsupported letter fails LOUDLY instead of producing wrong output.
/// Known micro-divergences (closest chrono forms, documented): `X`/`XX`
/// render as `+0530` (no minute-less form), `XXX` renders UTC as `+00:00`
/// where Java prints `Z`; `z` is format-only (chrono cannot parse zone
/// abbreviations).
fn java_pattern_to_chrono(pattern: &str) -> Result<String, String> {
    let chars: Vec<char> = pattern.chars().collect();
    let mut out = String::new();
    let push_literal = |out: &mut String, c: char| {
        if c == '%' {
            out.push_str("%%");
        } else {
            out.push(c);
        }
    };
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c == '\'' {
            // '' outside a quoted section is a literal quote
            if chars.get(i + 1) == Some(&'\'') {
                out.push('\'');
                i += 2;
                continue;
            }
            // quoted literal text, '' inside = one quote
            i += 1;
            while i < chars.len() {
                if chars[i] == '\'' {
                    if chars.get(i + 1) == Some(&'\'') {
                        out.push('\'');
                        i += 2;
                        continue;
                    }
                    i += 1;
                    break;
                }
                push_literal(&mut out, chars[i]);
                i += 1;
            }
            continue;
        }
        if c.is_ascii_alphabetic() {
            let mut n = 1;
            while chars.get(i + n) == Some(&c) {
                n += 1;
            }
            let token = match (c, n) {
                ('y' | 'u', 2) => "%y",
                ('y' | 'u', _) => "%Y",
                ('M', 4..) => "%B",
                ('M', 3) => "%b",
                ('M', 2) => "%m",
                ('M', 1) => "%-m",
                ('d', 2..) => "%d",
                ('d', 1) => "%-d",
                ('E', 4..) => "%A",
                ('E', _) => "%a",
                ('H', 2..) => "%H",
                ('H', 1) => "%-H",
                ('h', 2..) => "%I",
                ('h', 1) => "%-I",
                ('m', 2..) => "%M",
                ('m', 1) => "%-M",
                ('s', 2..) => "%S",
                ('s', 1) => "%-S",
                ('S', 3) => "%3f",
                ('S', _) => {
                    return Err(format!(
                        "Unsupported fractional-second width in '{pattern}' - use SSS (milliseconds)"
                    ))
                }
                ('a', _) => "%p",
                ('X', 3) => "%:z",
                ('X', 1..=2) => "%z",
                ('Z', _) => "%z",
                ('z', _) => "%Z",
                _ => {
                    return Err(format!(
                        "Unsupported date-time pattern letter '{c}' in '{pattern}'"
                    ))
                }
            };
            out.push_str(token);
            i += n;
            continue;
        }
        push_literal(&mut out, c);
        i += 1;
    }
    Ok(out)
}

fn plugin_date_time(args: &[Value]) -> Result<Value, String> {
    // Java DateGenerator: ZonedDateTime.now(zone).format(pattern) — the
    // optional 2nd argument is the zone (ZoneId.of), previously silently
    // dropped (parity F5); no pattern → ISO_DATE_TIME with the [zone-id]
    // suffix; no zone → the system default
    let zone_name = match args.get(1) {
        Some(zone) => get_text_value(zone),
        None => iana_time_zone::get_timezone()
            .map_err(|e| format!("unable to detect the system time zone - {e}"))?,
    };
    let tz: chrono_tz::Tz = zone_name
        .parse()
        .map_err(|_| format!("Unknown time zone '{zone_name}'"))?;
    let now = chrono::Utc::now().with_timezone(&tz);
    match args.first() {
        None => Ok(Value::from(format!(
            "{}[{}]",
            now.format("%Y-%m-%dT%H:%M:%S%.f%:z"),
            zone_name
        ))),
        Some(pattern) => {
            let format = java_pattern_to_chrono(&get_text_value(pattern))?;
            Ok(Value::from(now.format(&format).to_string()))
        }
    }
}

/// Compare with the same promotion rule as the arithmetic: both whole ⇒
/// exact i64 comparison, otherwise f64.
fn compare_numbers(a: &Value, b: &Value) -> Result<std::cmp::Ordering, String> {
    match (promote_number(a)?, promote_number(b)?) {
        (Number::Long(x), Number::Long(y)) => Ok(x.cmp(&y)),
        (x, y) => Ok(x
            .as_f64()
            .partial_cmp(&y.as_f64())
            .unwrap_or(std::cmp::Ordering::Equal)),
    }
}

fn plugin_gt(args: &[Value]) -> Result<Value, String> {
    match args {
        [a, b] => Ok(Value::Boolean(
            compare_numbers(a, b)? == std::cmp::Ordering::Greater,
        )),
        _ => Err("Input is required to compare using 'Greater Than'".to_string()),
    }
}

fn plugin_lt(args: &[Value]) -> Result<Value, String> {
    match args {
        [a, b] => Ok(Value::Boolean(
            compare_numbers(a, b)? == std::cmp::Ordering::Less,
        )),
        _ => Err("Input is required to compare using 'Less Than'".to_string()),
    }
}

fn plugin_ternary(args: &[Value]) -> Result<Value, String> {
    match args {
        [condition, yes, no] => Ok(if convert_boolean(condition)? {
            yes.clone()
        } else {
            no.clone()
        }),
        _ => Err("Three parts are required for Ternary Operation".to_string()),
    }
}

fn plugin_starts_with(args: &[Value]) -> Result<Value, String> {
    match args {
        // case insensitive comparison (Java parity)
        [source, text] => Ok(Value::Boolean(
            get_text_value(source)
                .to_lowercase()
                .starts_with(&get_text_value(text).to_lowercase()),
        )),
        _ => Ok(Value::Boolean(false)),
    }
}

fn plugin_ends_with(args: &[Value]) -> Result<Value, String> {
    match args {
        [source, text] => Ok(Value::Boolean(
            get_text_value(source)
                .to_lowercase()
                .ends_with(&get_text_value(text).to_lowercase()),
        )),
        _ => Ok(Value::Boolean(false)),
    }
}

fn plugin_includes(args: &[Value]) -> Result<Value, String> {
    match args {
        // list membership uses value equality; text search is case-insensitive
        [Value::Array(items), needle] => Ok(Value::Boolean(items.contains(needle))),
        [source, text] => Ok(Value::Boolean(
            get_text_value(source)
                .to_lowercase()
                .contains(&get_text_value(text).to_lowercase()),
        )),
        _ => Ok(Value::Boolean(false)),
    }
}

/// Java `getRules`: split a rule string on ';' and trim each part.
fn get_rules(text: &str) -> Vec<String> {
    text.split(';')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect()
}

fn parsed_date_output(
    local_datetime: chrono::NaiveDateTime,
    rules: &[String],
) -> Result<Value, String> {
    use chrono::TimeZone;
    let zoned = chrono::Local
        .from_local_datetime(&local_datetime)
        .earliest()
        .ok_or_else(|| "invalid local datetime".to_string())?;
    let output = rules.get(1).map(String::as_str).unwrap_or("iso");
    match output.to_ascii_lowercase().as_str() {
        "ms" => Ok(Value::from(zoned.timestamp_millis())),
        "local" => Ok(Value::from(
            zoned.format("%Y-%m-%d %H:%M:%S%.3f").to_string(),
        )),
        "iso" => Ok(Value::from(
            zoned
                .with_timezone(&chrono::Utc)
                .format("%Y-%m-%dT%H:%M:%S%.3fZ")
                .to_string(),
        )),
        _ => Err("Supported types are: iso, local, ms".to_string()),
    }
}

fn plugin_parse_date(args: &[Value]) -> Result<Value, String> {
    match args {
        [value, rule_text] => {
            let rules = get_rules(&get_text_value(rule_text));
            let Some(pattern) = rules.first() else {
                return Err(
                    "Invalid validation rule. Syntax: text(pattern, iso | local | ms)".to_string(),
                );
            };
            let format = java_pattern_to_chrono(pattern)?;
            let date = chrono::NaiveDate::parse_from_str(&get_text_value(value), &format)
                .map_err(|e| format!("unable to parse date - {e}"))?;
            let midnight = date.and_hms_opt(0, 0, 0).ok_or("invalid date")?;
            parsed_date_output(midnight, &rules)
        }
        _ => Err("Invalid validation rule. Syntax: text(pattern, iso | local | ms)".to_string()),
    }
}

fn plugin_parse_date_time(args: &[Value]) -> Result<Value, String> {
    match args {
        [value, rule_text] => {
            let rules = get_rules(&get_text_value(rule_text));
            let Some(pattern) = rules.first() else {
                return Err(
                    "Invalid validation rule. Syntax: text(pattern, iso | local | ms)".to_string(),
                );
            };
            let format = java_pattern_to_chrono(pattern)?;
            let datetime = chrono::NaiveDateTime::parse_from_str(&get_text_value(value), &format)
                .map_err(|e| format!("unable to parse datetime - {e}"))?;
            parsed_date_output(datetime, &rules)
        }
        _ => Err("Invalid validation rule. Syntax: text(pattern, iso | local | ms)".to_string()),
    }
}

// ---- list-of-map operations (Java TypeConversionUtils helpers) ----

fn as_str_key(key: &Value) -> String {
    key.as_str()
        .map(str::to_string)
        .unwrap_or_else(|| key.to_string())
}

/// Java `findMapOfLists`: descend nested maps until one whose values include
/// a list is found.
fn find_map_of_lists(map: &[(Value, Value)]) -> Option<Vec<(Value, Value)>> {
    for (_, v) in map {
        if matches!(v, Value::Array(_)) {
            return Some(map.to_vec());
        }
        if let Value::Map(inner) = v {
            return find_map_of_lists(inner);
        }
    }
    None
}

/// Java `ListOfMap.normalize`: `{k1: [a,b], k2: [x,y]}` →
/// `[{k1: a, k2: x}, {k1: b, k2: y}]`.
fn normalize_map_of_lists(map: &[(Value, Value)]) -> Vec<Value> {
    let max_len = map
        .iter()
        .filter_map(|(_, v)| match v {
            Value::Array(list) => Some(list.len()),
            _ => None,
        })
        .max()
        .unwrap_or(0);
    let mut rows: Vec<Vec<(Value, Value)>> = vec![Vec::new(); max_len];
    for (k, v) in map {
        if let Value::Array(list) = v {
            for (i, item) in list.iter().enumerate() {
                rows[i].push((k.clone(), item.clone()));
            }
        }
    }
    rows.into_iter().map(Value::Map).collect()
}

/// Java `prepareMerge` + `validDataStructure`: subsequent arguments must hold
/// maps of lists whose lengths equal the first list's.
fn prepare_merge(first_len: usize, rest: &[Value]) -> Option<Vec<Vec<(Value, Value)>>> {
    let mut to_merge = Vec::new();
    for arg in rest {
        if let Value::Map(map) = arg {
            if let Some(found) = find_map_of_lists(map) {
                let valid = found.iter().all(|(_, v)| match v {
                    Value::Array(list) => list.len() == first_len,
                    _ => false,
                });
                if !valid {
                    return None;
                }
                to_merge.push(found);
            }
        }
    }
    Some(to_merge)
}

/// Java `merge`: for each row i, add every (key, list[i]) from the
/// additional maps into the row's map (replacing an existing key).
fn merge_lists(first: &[Value], additional: &[Vec<(Value, Value)>]) -> Value {
    let size = first.len();
    let mut copy: Vec<Value> = first.to_vec();
    for (i, row) in copy.iter_mut().enumerate().take(size) {
        if let Value::Map(base) = row {
            for additional_map in additional {
                for (k, v) in additional_map {
                    if let Value::Array(list) = v {
                        if list.len() == size {
                            let key = as_str_key(k);
                            base.retain(|(bk, _)| bk.as_str() != Some(key.as_str()));
                            base.push((Value::from(key.as_str()), list[i].clone()));
                        }
                    }
                }
            }
        }
    }
    Value::Array(copy)
}

fn plugin_list_of_map(args: &[Value]) -> Result<Value, String> {
    let Some(Value::Map(data)) = args.first() else {
        return Ok(Value::Array(Vec::new()));
    };
    let Some(map) = find_map_of_lists(data) else {
        return Ok(Value::Array(Vec::new()));
    };
    let first = normalize_map_of_lists(&map);
    if args.len() > 1 && !first.is_empty() {
        match prepare_merge(first.len(), &args[1..]) {
            Some(to_merge) if !to_merge.is_empty() => Ok(merge_lists(&first, &to_merge)),
            _ => Ok(Value::Array(Vec::new())),
        }
    } else {
        Ok(Value::Array(first))
    }
}

fn plugin_update_list_of_map(args: &[Value]) -> Result<Value, String> {
    if args.len() > 1 {
        if let Some(Value::Array(first)) = args.first() {
            let valid_first = !first.is_empty() && first.iter().all(|v| matches!(v, Value::Map(_)));
            if valid_first {
                if let Some(to_merge) = prepare_merge(first.len(), &args[1..]) {
                    if !to_merge.is_empty() {
                        return Ok(merge_lists(first, &to_merge));
                    }
                }
            }
        }
    }
    Ok(Value::Array(Vec::new()))
}

fn drop_keys(map: &[(Value, Value)], keys: &[Value]) -> Value {
    let dropped: Vec<(Value, Value)> = map
        .iter()
        .filter(|(k, _)| {
            let key = as_str_key(k);
            !keys.iter().any(|drop| get_text_value(drop) == key)
        })
        .cloned()
        .collect();
    Value::Map(dropped)
}

fn plugin_remove_key(args: &[Value]) -> Result<Value, String> {
    if args.len() > 1 {
        match &args[0] {
            Value::Map(map) => return Ok(drop_keys(map, &args[1..])),
            Value::Array(list) => {
                let updated: Vec<Value> = list
                    .iter()
                    .map(|item| match item {
                        Value::Map(map) => drop_keys(map, &args[1..]),
                        other => other.clone(),
                    })
                    .collect();
                return Ok(Value::Array(updated));
            }
            _ => {}
        }
    }
    Ok(Value::Nil)
}

fn plugin_unique_set(args: &[Value]) -> Result<Value, String> {
    match args {
        [Value::Array(items)] => {
            // order-preserving de-duplication (Java LinkedHashSet)
            let mut unique: Vec<Value> = Vec::new();
            for item in items {
                if !unique.contains(item) {
                    unique.push(item.clone());
                }
            }
            Ok(Value::Array(unique))
        }
        _ => Ok(Value::Nil),
    }
}

fn plugin_default_value(args: &[Value]) -> Result<Value, String> {
    if args.len() > 1 && matches!(args[0], Value::Nil) {
        return Ok(args[1].clone());
    }
    Ok(args.first().cloned().unwrap_or(Value::Nil))
}

// ---- input validation (Java InputValidation) ----

fn plugin_validate(args: &[Value]) -> Result<Value, String> {
    let [value, rule_text] = args else {
        return Err("Validation syntax error. Expect 2 arguments where the second one is the validation rule.".to_string());
    };
    let rules = get_rules(&get_text_value(rule_text));
    if rules.len() < 2 {
        return Err("Invalid validation rule. Syntax: text(id, type), text(id, type, required) or text(id, type, range-start, range-end)".to_string());
    }
    let field = &rules[0];
    let type_name = rules[1].as_str();
    let evaluate = rules
        .last()
        .map(|r| r.eq_ignore_ascii_case("evaluate"))
        .unwrap_or(false);
    if matches!(value, Value::Nil) {
        // when 'required' is not configured, the field is optional
        let required = rules.iter().any(|r| r.eq_ignore_ascii_case("required"));
        return if !required {
            Ok(if evaluate { Value::Boolean(true) } else { Value::Nil })
        } else if evaluate {
            Ok(Value::Boolean(false))
        } else {
            Err(format!("{field} is required."))
        };
    }
    let type_ok = match type_name {
        "String" => matches!(value, Value::String(_)),
        "Integer" | "Long" => matches!(value, Value::Integer(_)),
        "Float" | "Double" => matches!(value, Value::F32(_) | Value::F64(_)),
        "Boolean" => matches!(value, Value::Boolean(_)),
        "Map" => matches!(value, Value::Map(_)),
        "List" => matches!(value, Value::Array(_)),
        other => {
            return Err(format!(
                "Validation type '{other}' is not supported. Use String, Integer, Long, Float, Double, Boolean, Map or List."
            ))
        }
    };
    let outcome = if !type_ok {
        Err(format!(
            "Expect {field} as {type_name}, Actual: {}",
            value_type_name(value)
        ))
    } else {
        validate_range(field, value, &filtered_rules(&rules))
    };
    match outcome {
        Ok(v) => Ok(if evaluate { Value::Boolean(true) } else { v }),
        Err(e) => {
            if evaluate {
                Ok(Value::Boolean(false))
            } else {
                Err(e)
            }
        }
    }
}

fn value_type_name(value: &Value) -> &'static str {
    match value {
        Value::String(_) => "String",
        Value::Integer(_) => "Integer",
        Value::F32(_) | Value::F64(_) => "Double",
        Value::Boolean(_) => "Boolean",
        Value::Map(_) => "Map",
        Value::Array(_) => "List",
        _ => "Object",
    }
}

fn filtered_rules(rules: &[String]) -> Vec<String> {
    rules
        .iter()
        .filter(|r| !r.eq_ignore_ascii_case("required") && !r.eq_ignore_ascii_case("evaluate"))
        .cloned()
        .collect()
}

/// Java `rangeCheck`: rules[2]/rules[3] bound the value (string lexicographic,
/// integer, or float comparison per the value's type).
fn validate_range(field: &str, value: &Value, rules: &[String]) -> Result<Value, String> {
    if rules.len() <= 2 {
        return Ok(value.clone());
    }
    let start = rules.get(2);
    let end = rules.get(3);
    match value {
        Value::String(s) => {
            let text = s.as_str().unwrap_or_default();
            if let Some(start) = start {
                if text < start.as_str() {
                    return Err(format!("{field} ({text}) < {start}"));
                }
            }
            if let Some(end) = end {
                if text > end.as_str() {
                    return Err(format!("{field} ({text}) > {end}"));
                }
            }
        }
        Value::Integer(i) => {
            let n = i.as_i64().unwrap_or(-1);
            if let Some(start) = start.and_then(|s| s.parse::<i64>().ok()) {
                if n < start {
                    return Err(format!("{field} ({n}) < {start}"));
                }
            }
            if let Some(end) = end.and_then(|s| s.parse::<i64>().ok()) {
                if n > end {
                    return Err(format!("{field} ({n}) > {end}"));
                }
            }
        }
        Value::F32(_) | Value::F64(_) => {
            let n = crate::conversions::convert_double(value);
            if let Some(start) = start.and_then(|s| s.parse::<f64>().ok()) {
                if n < start {
                    return Err(format!("{field} ({n}) < {start}"));
                }
            }
            if let Some(end) = end.and_then(|s| s.parse::<f64>().ok()) {
                if n > end {
                    return Err(format!("{field} ({n}) > {end}"));
                }
            }
        }
        _ => {}
    }
    Ok(value.clone())
}
