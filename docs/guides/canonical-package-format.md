---
title: Canonical Package Format
summary: The language-neutral, deterministic MsgPack package — sorted keys, a manifest and maps, one canonical profile — that Java and Rust write byte for byte.
layer: reference
audience: [developer, reference, ai-agent]
keywords: [canonical package, msgpack, deterministic, sorted keys, manifest, packager, interoperability, vectors, byte for byte]
---

# Canonical Package Format

*Reference: the deterministic MsgPack package contract shared by every engine that carries the packager.*

> **At a glance**
>
> - **What** — a set of related documents (graphs, flows, rules, tables) packed into **one byte array whose bytes depend
>   only on its content**: map keys sorted at every depth in UTF-8 byte order, one canonical MsgPack profile, a
>   `manifest` plus the `maps`.
> - **Why it matters** — a single artifact has a stable identity that can be recorded, compared and, where the field wants
>   it, hashed or signed with the application's own tools. The Java and Rust engines write identical bytes, proven by a
>   shared vector file.
> - **For** developers who promote related documents as one artifact, and implementers of a new engine. This page is
>   self-contained: the profile below and the [vector file](#vectors) are all an implementation needs.

JSON text is not a stable artifact (key order and whitespace vary), loose files cannot be identified as a set, and MsgPack alone
does not fix it: a map has no guaranteed order and one value has several valid encodings. The packager removes both freedoms.
It is a **packaging** format only. Nothing inside a package refers to a hash, a signature, a key or a timestamp; whether to
protect the exact bytes, with which algorithm and where the proof is kept is the user application's decision.

## The package

A package is one MsgPack map with two entries. `manifest` sorts before `maps`, so a reader can read the metadata before
decoding any map.

```text
{ "manifest": { "format": "mercury-package", "format_version": "1", "graph_id": "quote", ... },
  "maps":     { "quote-fees.json": { ...keys sorted recursively... }, "quote.json": { ... } } }
```

- **`manifest`** is a metadata map of text values. The packager writes `format` (`mercury-package`) and `format_version`
  (`1`); everything else is a caller-defined string field that is recorded and never interpreted. For a graph package the
  convention is that `graph_id` holds the id of the graph the package delivers (the id the deployment manifest lists,
  `POST /api/graph/{graph_id}` serves, and the file `<graph_id>.json` holds), with the other maps as its subgraphs.
- **`maps`** holds the packed maps keyed by **entry name**, normally a file name such as `quote.json`. Entry names are ordered
  by their **UTF-8 bytes**, never by a file system listing or a locale.
- A duplicate entry name, or a caller field named `format` or `format_version`, is an error.

## The canonical profile

Identical in every engine.

| Rule | Detail |
|---|---|
| **Keys** | Text, in ascending order of their **UTF-8 bytes** at every depth, maps inside lists included; list order is kept. This is not the UTF-16 order of a Java `String`, which differs for characters above U+FFFF (U+1F600 sorts after U+FF5E in UTF-8). A non-text key is converted to text; a null key, or two keys that collide after conversion, is an error. |
| **Null values** | Written as nil and never dropped. |
| **Integers** | The smallest encoding (positive values unsigned, negative values signed); only signed 64-bit values are canonical. |
| **Floats** | A finite float64. A 32-bit float, NaN and Infinity are rejected. The integer `1` and the float `1.0` are different content. |
| **Text and bytes** | MsgPack str and bin, each with the shortest header. Strings are written as given: no Unicode normalization. |
| **Exact numbers** | Written as strings: an arbitrary-size integer as its digits, a decimal in plain notation with its scale kept and a zero of any scale as `"0"` (the rule of [`DECIMAL`](knowledge-graph/skills-reference.md#math-decimal)). |
| **Dates** | An ISO-8601 string. |
| **Everything else** | No extension types and no timestamps; any other type is rejected, naming its path. |

The ordering is done by the packager itself, after any JSON parsing and before anything is written, so an input map's own order
never reaches the bytes (a Gson map keeps the order of the text it was parsed from, and a sort by `String` order would be
UTF-16).

## Reading

A read returns the manifest and the maps **in the order the bytes hold them**. The default read is **strict**: it re-encodes
the decoded content canonically and rejects the package if the bytes differ, so an accepted package has exactly one byte form.
A non-strict read decodes anything that is valid in the profile's types. Either way the decoder rejects:

- a key that is not text, and a duplicate key;
- an extension type;
- bytes after the value;
- nesting beyond **64 levels** (sixty-four nested lists decode, sixty-five are refused);
- a top level that is not exactly `manifest` and `maps`, a manifest field that is not text, an entry that is not a map, a
  `format` other than `mercury-package` and a `format_version` other than `1`.

## An example

A package with no caller fields and one map, `only.json`, holding `{"a": 1}`:

```text
82                      map of 2: manifest, maps
  a8 6d616e6966657374     "manifest"
  82                      map of 2
    a6 666f726d6174         "format"
    af 6d6572637572792d7061636b616765   "mercury-package"
    ae 666f726d61745f76657273696f6e     "format_version"
    a1 31                   "1"
  a4 6d617073             "maps"
  81                      map of 1
    a9 6f6e6c792e6a736f6e   "only.json"
    81                      map of 1
      a1 61                   "a"
      01                      1
```

The whole package is `82a86d616e696665737482a6666f726d6174af6d6572637572792d7061636b616765ae666f726d61745f76657273696f6ea131a46d61707381a96f6e6c792e6a736f6e81a16101`
and its SHA-256 is `56ebc3ba08b5181e1afa0fdcbb499f2e82c3958490d9da31d295c0553c926de7`. The same bytes come out of the Java and the Rust engine.

## Vectors {#vectors}

`canonical-package-vectors.json` is the contract between engines and is kept **byte-identical** in the Java and Rust repositories
(`system/platform-core/src/test/resources/` in `mercury-composable` and `crates/platform-core/tests/resources/` here). Its expected bytes come from an
**independent encoder written from this specification**, not from either engine, so an engine that matches the file matches every
other engine that does. It holds:

- **values** — integers at every encoding boundary, float64 including `-0.0`, str and bin header boundaries, Unicode, map and
  list header boundaries, nulls kept, keys sorted at depth, keys whose UTF-8 and UTF-16 orders differ, exact numbers as strings;
- **packages** — a graph set in both entry orders (the same bytes), entry names by UTF-8 bytes, manifest fields sorted, an empty
  package, each with its expected hex and SHA-256;
- **corpus** — 60 packages of random nested documents from a seeded generator, so a divergence in a rarely used corner shows up
  as a byte difference;
- **rejections** — what a build refuses (a 32-bit float, NaN, Infinity, an unsupported type, a duplicate entry name, a reserved
  manifest field) and what a read refuses (a non-canonical integer width, out-of-order keys, trailing bytes, a wrong format or
  version, a duplicate or non-text key, an extension type, a truncated package, nesting of 65 levels), with the cases only a
  strict read rejects marked.

A JSON number in the file without a fraction or exponent is an integer; any other number is a float64. `{"$bytes": hex}` is binary,
`{"$decimal": text}` an exact decimal, `{"$integer": digits}` an arbitrary-size integer. A new engine proves itself by running the
file: every value, package and corpus entry must produce the expected bytes, and every rejection must fail.

## In each engine

| Engine | Where |
|---|---|
| Rust | `platform_core::canonical_packager` — see [the API overview](api-overview.md#deterministic-packaging-canonical_packager) |
| Java | `org.platformlambda.core.serializers.CanonicalPackager` in the `mercury-composable` repository |

The Rust packager takes an `rmpv::Value`, which has no decimal, big-integer or date type, so the caller writes exact numbers and
dates as strings (the profile's rule), and an integer above 2^63-1 is rejected: write it as its digits. The `mercury-python` and
`mercury-nodejs` language packs do not carry it: they serve functions to a Java or Rust application over
[Event over HTTP](event-over-http.md) and never read a graph package.

## Not part of the packager

Trusted timestamps, per-entry hashes in the manifest, hot reload, compression, the folder `pack` and `unpack` tooling, and a
graph-set loader that would register all graphs or none, are deferred until field use asks for them. The decision and its
alternatives are recorded in ADR-0026 in the `mercury-composable` repository.
