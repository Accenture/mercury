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

//! Null-transport policy for the wire serializers — the Rust mirror of Java's
//! `serializer.null.transport` (Java `SimpleMapper`/Gson for JSON and `MsgPack`

//! A deterministic MsgPack packager (RFC-0002; the Rust twin of the Java `CanonicalPackager`): the same content
//! always gives the same bytes, in every engine - the vector file `canonical-package-vectors.json` is
//! byte-identical in both repositories and its expected bytes come from an independent encoder written from the
//! specification.
//!
//! A package is one MsgPack map with two parts. `manifest` is a metadata map: the packager writes `format` and
//! `format_version`, and everything else is a caller-defined string field that is recorded and never interpreted
//! (for a graph package, by convention `graph_id` holds the id of the graph the package delivers). `maps` holds the
//! packed maps keyed by entry name. Entry names are ordered by their UTF-8 bytes.
//!
//! The canonical profile, identical in every engine:
//!
//! * every map is written with its keys sorted at every depth, maps inside lists included (list order is kept);
//!   keys are text, in ascending order of their UTF-8 bytes; an integer or boolean key is converted to text, and a
//!   null key, any other key type or a collision after conversion is an error;
//! * a null value is written as nil and is never dropped;
//! * integers use the smallest encoding, and only signed 64-bit values are canonical (a larger one is written as
//!   text by the caller);
//! * a floating-point number is a finite float64; an `f32`, NaN and Infinity are rejected;
//! * text and bytes are str and bin, each with the shortest header;
//! * an exact number travels as a string, written by the caller in plain notation with its scale kept and a zero
//!   of any scale as `"0"` (the rule of RFC-0001); a date is an ISO-8601 string;
//! * there are no extension types and no timestamps; any other type is rejected, naming its path.
//!
//! The ordering is done here, by the packager, before anything is written, so the input map's own order never
//! reaches the bytes. The packager is faithful to the type of each value: the integer 1 and the float 1.0 are
//! different content. Strings are written as given: no Unicode normalization is applied.
//!
//! Integrity is not part of a package. Nothing inside it refers to a hash or a signature: the user application
//! decides whether to protect the exact bytes, with which algorithm and where the proof is kept.

use std::collections::BTreeMap;

use rmpv::Value;

pub const FORMAT: &str = "mercury-package";
pub const FORMAT_VERSION: &str = "1";
pub const MANIFEST: &str = "manifest";
pub const MAPS: &str = "maps";
pub const FORMAT_KEY: &str = "format";
pub const FORMAT_VERSION_KEY: &str = "format_version";
// a package is shallow; a bound keeps a hostile byte array from exhausting the stack
const MAX_DEPTH: usize = 64;
// rmpv's own recursion guard for hostile bytes (it allows about half this many levels)
const RMPV_STACK_GUARD: usize = 400;

/// What went wrong: `Invalid` is the Java `IllegalArgumentException` analog (a value the profile rejects, a bad
/// builder call), `Malformed` the `IOException` analog (bytes that are not a canonical package).
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum PackagerError {
    #[error("{0}")]
    Invalid(String),
    #[error("{0}")]
    Malformed(String),
}

impl PackagerError {
    pub fn message(&self) -> &str {
        match self {
            PackagerError::Invalid(m) | PackagerError::Malformed(m) => m,
        }
    }
}

fn invalid(message: impl Into<String>) -> PackagerError {
    PackagerError::Invalid(message.into())
}

fn malformed(message: impl Into<String>) -> PackagerError {
    PackagerError::Malformed(message.into())
}

/// A decoded package: ordered, in the order the bytes hold them.
#[derive(Clone, Debug, PartialEq)]
pub struct Package {
    /// The metadata map, `format` and `format_version` included.
    pub manifest: Vec<(String, String)>,
    /// The packed maps keyed by entry name; each value is a `Value::Map`.
    pub maps: Vec<(String, Value)>,
}

/// Collects a package's manifest fields and maps, and packs them.
#[derive(Debug, Default)]
pub struct Builder {
    fields: BTreeMap<String, String>,
    entries: BTreeMap<String, Value>,
}

impl Builder {
    pub fn new() -> Self {
        Self::default()
    }

    /// A caller-defined manifest field; `format` and `format_version` are reserved.
    pub fn manifest(mut self, key: &str, value: &str) -> Result<Self, PackagerError> {
        if key.is_empty() {
            return Err(invalid("A manifest field needs a name"));
        }
        if key == FORMAT_KEY || key == FORMAT_VERSION_KEY {
            return Err(invalid(format!(
                "The manifest field '{key}' is written by the packager"
            )));
        }
        if self
            .fields
            .insert(key.to_string(), value.to_string())
            .is_some()
        {
            return Err(invalid(format!("Duplicate manifest field '{key}'")));
        }
        Ok(self)
    }

    /// A map to pack under an entry name (a file name such as `quote.json`).
    pub fn add(mut self, name: &str, map: Value) -> Result<Self, PackagerError> {
        if name.is_empty() {
            return Err(invalid("A map needs an entry name"));
        }
        if !matches!(map, Value::Map(_)) {
            return Err(invalid(format!("The map '{name}' is not a map")));
        }
        if self.entries.insert(name.to_string(), map).is_some() {
            return Err(invalid(format!("Duplicate entry name '{name}'")));
        }
        Ok(self)
    }

    /// The package as one deterministic byte array.
    pub fn build(self) -> Result<Vec<u8>, PackagerError> {
        let mut manifest: Vec<(Value, Value)> = self
            .fields
            .iter()
            .map(|(k, v)| (Value::from(k.as_str()), Value::from(v.as_str())))
            .collect();
        manifest.push((Value::from(FORMAT_KEY), Value::from(FORMAT)));
        manifest.push((Value::from(FORMAT_VERSION_KEY), Value::from(FORMAT_VERSION)));
        let maps: Vec<(Value, Value)> = self
            .entries
            .into_iter()
            .map(|(name, map)| (Value::from(name), map))
            .collect();
        encode(&Value::Map(vec![
            (Value::from(MANIFEST), Value::Map(manifest)),
            (Value::from(MAPS), Value::Map(maps)),
        ]))
    }
}

/// Pack any value under the canonical profile.
pub fn encode(value: &Value) -> Result<Vec<u8>, PackagerError> {
    let ordered = canonicalize(value, "$", 0)?;
    let mut out = Vec::new();
    rmpv::encode::write_value(&mut out, &ordered)
        .map_err(|e| invalid(format!("Unable to write the value: {e}")))?;
    Ok(out)
}

/// Read a package with the strict check: the decoded content is re-encoded canonically and the package is
/// rejected when the bytes differ, so an accepted package has exactly one byte form.
pub fn unpack(bytes: &[u8]) -> Result<Package, PackagerError> {
    unpack_with(bytes, true)
}

/// Read a package; `strict` rejects a package whose bytes are not the canonical form of its content.
pub fn unpack_with(bytes: &[u8], strict: bool) -> Result<Package, PackagerError> {
    let content = decode(bytes)?;
    let document = match &content {
        Value::Map(entries)
            if entries.len() == 2 && has_key(entries, MANIFEST) && has_key(entries, MAPS) =>
        {
            entries
        }
        _ => {
            return Err(malformed(format!(
                "A package is one map holding exactly '{MANIFEST}' and '{MAPS}'"
            )))
        }
    };
    let manifest = read_manifest(value_of(document, MANIFEST))?;
    let lookup = |key: &str| {
        manifest
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.as_str())
            .unwrap_or("null")
    };
    if lookup(FORMAT_KEY) != FORMAT {
        return Err(malformed(format!(
            "Not a package: the manifest format is '{}'",
            lookup(FORMAT_KEY)
        )));
    }
    if lookup(FORMAT_VERSION_KEY) != FORMAT_VERSION {
        return Err(malformed(format!(
            "Unsupported package format_version '{}'",
            lookup(FORMAT_VERSION_KEY)
        )));
    }
    let maps = read_maps(value_of(document, MAPS))?;
    if strict {
        let canonical =
            encode(&content).map_err(|e| malformed(format!("Not canonical: {}", e.message())))?;
        if canonical != bytes {
            return Err(malformed(
                "Not canonical: the bytes differ from the canonical form of their content",
            ));
        }
    }
    Ok(Package { manifest, maps })
}

/// Decode MsgPack bytes holding exactly one value into ordered maps, lists and scalars. A non-text key, a
/// duplicate key, an extension type, bytes after the value and nesting beyond the bound are errors.
pub fn decode(bytes: &[u8]) -> Result<Value, PackagerError> {
    let mut cursor = std::io::Cursor::new(bytes);
    // rmpv spends two units of its depth counter per level of nesting, so its own limit is only a stack guard set
    // well above the bound; the bound itself (the same 64 levels as the Java engine) is enforced by check_decoded
    let value =
        rmpv::decode::read_value_with_max_depth(&mut cursor, RMPV_STACK_GUARD).map_err(|e| {
            let text = e.to_string();
            if text.to_lowercase().contains("depth") {
                malformed(format!("Nesting deeper than {MAX_DEPTH}"))
            } else {
                malformed(format!("Malformed MsgPack: {text}"))
            }
        })?;
    if (cursor.position() as usize) != bytes.len() {
        return Err(malformed("Unexpected bytes after the value"));
    }
    check_decoded(&value, "$", 0)?;
    Ok(value)
}

fn check_decoded(value: &Value, path: &str, depth: usize) -> Result<(), PackagerError> {
    if depth > MAX_DEPTH {
        return Err(malformed(format!(
            "Nesting deeper than {MAX_DEPTH} at {path}"
        )));
    }
    match value {
        Value::Array(items) => {
            for (i, item) in items.iter().enumerate() {
                check_decoded(item, &format!("{path}[{i}]"), depth + 1)?;
            }
        }
        Value::Map(entries) => {
            let mut seen = std::collections::HashSet::new();
            for (k, v) in entries {
                let Some(key) = k.as_str() else {
                    return Err(malformed(format!("A key is not text at {path}")));
                };
                if !seen.insert(key.to_string()) {
                    return Err(malformed(format!("Duplicate key '{key}' at {path}")));
                }
                check_decoded(v, &format!("{path}.{key}"), depth + 1)?;
            }
        }
        Value::Ext(..) => return Err(malformed(format!("Unsupported MsgPack type ext at {path}"))),
        Value::String(s) if s.as_str().is_none() => {
            return Err(malformed(format!(
                "Malformed MsgPack: invalid UTF-8 text at {path}"
            )))
        }
        _ => {}
    }
    Ok(())
}

fn has_key(entries: &[(Value, Value)], key: &str) -> bool {
    entries.iter().any(|(k, _)| k.as_str() == Some(key))
}

fn value_of<'a>(entries: &'a [(Value, Value)], key: &str) -> &'a Value {
    entries
        .iter()
        .find(|(k, _)| k.as_str() == Some(key))
        .map(|(_, v)| v)
        .expect("the key was checked")
}

fn read_manifest(value: &Value) -> Result<Vec<(String, String)>, PackagerError> {
    let Value::Map(entries) = value else {
        return Err(malformed("The manifest is not a map"));
    };
    let mut result = Vec::with_capacity(entries.len());
    for (k, v) in entries {
        let key = k.as_str().unwrap_or_default();
        let Some(text) = v.as_str() else {
            return Err(malformed(format!("The manifest field '{key}' is not text")));
        };
        result.push((key.to_string(), text.to_string()));
    }
    Ok(result)
}

fn read_maps(value: &Value) -> Result<Vec<(String, Value)>, PackagerError> {
    let Value::Map(entries) = value else {
        return Err(malformed(format!("'{MAPS}' is not a map")));
    };
    let mut result = Vec::with_capacity(entries.len());
    for (k, v) in entries {
        let name = k.as_str().unwrap_or_default();
        if !matches!(v, Value::Map(_)) {
            return Err(malformed(format!("The entry '{name}' is not a map")));
        }
        result.push((name.to_string(), v.clone()));
    }
    Ok(result)
}

/// A copy of the value with every map's keys converted to text and sorted by UTF-8 bytes, and every scalar
/// checked against the profile.
fn canonicalize(value: &Value, path: &str, depth: usize) -> Result<Value, PackagerError> {
    if depth > MAX_DEPTH {
        return Err(invalid(format!(
            "Nesting deeper than {MAX_DEPTH} at {path}"
        )));
    }
    match value {
        Value::Nil | Value::Boolean(_) | Value::Binary(_) => Ok(value.clone()),
        Value::Integer(n) => {
            if n.as_i64().is_none() {
                return Err(invalid(format!(
                    "An integer above 2^63-1 is not canonical at {path} - write it as text"
                )));
            }
            Ok(value.clone())
        }
        Value::F32(_) => Err(invalid(format!(
            "A Float is not canonical at {path} - use a Double"
        ))),
        Value::F64(d) => {
            if !d.is_finite() {
                return Err(invalid(format!(
                    "A non-finite number is not canonical at {path}: {d}"
                )));
            }
            Ok(value.clone())
        }
        Value::String(s) => {
            if s.as_str().is_none() {
                return Err(invalid(format!("Text that is not valid UTF-8 at {path}")));
            }
            Ok(value.clone())
        }
        Value::Array(items) => {
            let mut out = Vec::with_capacity(items.len());
            for (i, item) in items.iter().enumerate() {
                out.push(canonicalize(item, &format!("{path}[{i}]"), depth + 1)?);
            }
            Ok(Value::Array(out))
        }
        Value::Map(entries) => canonicalize_map(entries, path, depth),
        Value::Ext(..) => Err(invalid(format!("Unsupported type ext at {path}"))),
    }
}

fn canonicalize_map(
    entries: &[(Value, Value)],
    path: &str,
    depth: usize,
) -> Result<Value, PackagerError> {
    let mut keyed: Vec<(String, &Value)> = Vec::with_capacity(entries.len());
    let mut seen = std::collections::HashSet::new();
    for (k, v) in entries {
        let key = match k {
            Value::String(s) => s
                .as_str()
                .ok_or_else(|| invalid(format!("A key that is not valid UTF-8 at {path}")))?
                .to_string(),
            Value::Integer(n) => n.to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::Nil => return Err(invalid(format!("A null key at {path}"))),
            _ => return Err(invalid(format!("A key is not text at {path}"))),
        };
        if !seen.insert(key.clone()) {
            return Err(invalid(format!("Two keys become '{key}' at {path}")));
        }
        keyed.push((key, v));
    }
    // byte order of the UTF-8 text: a &str compares as bytes, so this is not the UTF-16 order of a Java String
    keyed.sort_by(|a, b| a.0.as_bytes().cmp(b.0.as_bytes()));
    let mut out = Vec::with_capacity(keyed.len());
    for (key, v) in keyed {
        let child = canonicalize(v, &format!("{path}.{key}"), depth + 1)?;
        out.push((Value::from(key), child));
    }
    Ok(Value::Map(out))
}
