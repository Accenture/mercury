---
title: Interop Test Report — Canonical Package, Java ⇄ Rust
summary: Permanent record of the byte-for-byte validation of the canonical MsgPack packager between the Java and Rust engines - the 14 tutorial graphs and 50 test graph fixtures packed by each engine, compared with a binary diff and cross-read, and the two graph packager command lines of RFC-0005 compared the same way - kept as the release evidence for the packager.
layer: reference
audience: [developer, architect]
keywords: [interop, canonical package, msgpack, byte for byte, packager, rust, tutorials, test report]
---

# Interop Test Report — Canonical Package, Java ⇄ Rust

*Cross-engine validation between the Java engine
([mercury-composable](https://github.com/Accenture/mercury-composable)) and the official Rust
implementation ([mercury](https://github.com/Accenture/mercury)) of the
[Canonical Package Format](../guides/canonical-package-format.md), conducted 2026-10-01 as the release evidence for
the packager (Java PRs #481, #482, #483 and the v4.12.20 release PR #485; Rust PR #339 and the release PR #340).*

The drive ran on the two `release/4.12.20` branch tips. Both release PRs were then squash-merged, and the merged commits are
content-identical to those tips (`git diff` between each tip and its squash commit is empty outside `memory/`), so the result
describes exactly what shipped.

This report is a permanent record. It answers one question: **do the two engines write identical bytes for the same
documents?** The packager was built so that they do, and the shared vector file already proves it for synthetic
content. This drive proves it for the real artifact the format exists for: the Active Knowledge Graph models the
Playground ships.

## Why this drive exists

A package is useful only if its bytes depend on its content and nothing else, in every engine: that is what lets a
package be recorded, compared and, where the field wants it, hashed or signed with the application's own tools. The
contract is `canonical-package-vectors.json`, kept byte-identical in both repositories, whose expected bytes come from
an independent encoder written from the specification. That file pins the profile on constructed values. It does not
say what happens when each engine loads real graph JSON through its **own** JSON loader first, and the two loaders
differ: Gson hands the Java engine maps in the order of the file's text, while `serde_json` hands the Rust engine its
own order. The packager's sorting is what is supposed to erase that difference, so the honest test starts from the
graphs.

## Method

| | Java | Rust |
|---|---|---|
| Engine under test | `release/4.12.20`, `3c9170de` (merged as `9e515825`, PR #485) | `release/4.12.20`, `63d4c17d` (merged as `b4783c5b`, PR #340) |
| JSON loader | `SimpleMapper` (Gson) into a `Map` | `serde_json` into the engine's `event_script::conversions::from_json` |
| Packager | `org.platformlambda.core.serializers.CanonicalPackager` | `platform_core::canonical_packager` |

1. **Inputs.** The 14 tutorial graphs, `tutorial-1.json` to `tutorial-14.json` (38,453 bytes of JSON), from
   `system/minigraph-playground-engine/src/main/resources/graph/` and `crates/knowledge-graph/resources/graph/`. The
   two copies were compared by SHA-256 first: **byte-identical**, so a difference could only come from an engine.
   A second leg uses the **50 test graph fixtures that exist byte-identically in both repositories** (the
   `unit-test-*` graphs, including the deliberately invalid ones, and the `hello` and `tutorial-11x` graphs).
2. **Pack.** Each engine loads every graph with its own loader and packs them all into **one package**, with the manifest
   field `set` and one entry per graph named after its file. It also packs each graph alone, so a difference could be
   pinned to one graph.
3. **Binary diff.** `cmp` of the two files, and a SHA-256 of each.
4. **Cross-read.** Each engine strict-reads the package the **other** engine wrote, re-packs what it read and compares
   the bytes, and checks that every entry equals its own parse of the original JSON.
5. **Negative control.** The same 14 graphs through each engine's *general* MsgPack serializer (`MsgPack.packMapOrList`
   in Java, `rmpv::encode::write_value` in Rust), without the packager. Zero differences proves little unless the method
   could have shown some.

## Result

| Check | Outcome |
|---|---|
| Leg 1, 14 tutorials in one package | **identical**: 20,236 bytes, `cmp` exit 0, SHA-256 `74fa007d3ae7184cdb37a977adecb5abbb3575f2e4c953d99f9b25e0e09d8d1d` |
| Leg 1, the 14 single-graph packages | **14 of 14 identical** |
| Leg 2, 50 fixtures in one package | **identical**: 53,193 bytes, `cmp` exit 0, SHA-256 `ec1b512590ce9aa580a35c357709eb4cb387d20fb18f2efaa1421e30aef310a1` |
| Leg 2, the 50 single-graph packages | **50 of 50 identical** |
| Java strict-reads the Rust package | accepted, 14 of 14 and 50 of 50 entries; re-pack equals the Rust bytes |
| Rust strict-reads the Java package | accepted, 14 of 14 and 50 of 50 entries; re-pack equals the Java bytes |
| Every entry equals the engine's own parse | 14 of 14 and 50 of 50, in both engines |
| **Negative control**, no packager | **0 of 14 identical** (all 14 differ) |

### Leg 1: the 14 tutorials

Each tutorial packed alone (the manifest is `{format, format_version, set}`); the Java and Rust SHA-256 are identical
for every graph.

| Graph | Package bytes | Java SHA-256 | Rust SHA-256 | |
|---|---|---|---|---|
| tutorial-1 | 385 | `82e9705d8d99babe…` | `82e9705d8d99babe…` | identical |
| tutorial-2 | 366 | `903efb6227dadb04…` | `903efb6227dadb04…` | identical |
| tutorial-3 | 1766 | `3262caa25f5a9789…` | `3262caa25f5a9789…` | identical |
| tutorial-4 | 1044 | `36f2c8ca60963b92…` | `36f2c8ca60963b92…` | identical |
| tutorial-5 | 2329 | `0ae4d892bbe110f7…` | `0ae4d892bbe110f7…` | identical |
| tutorial-6 | 3166 | `9ea84ec8f4113e39…` | `9ea84ec8f4113e39…` | identical |
| tutorial-7 | 751 | `956b0271b10afa57…` | `956b0271b10afa57…` | identical |
| tutorial-8 | 830 | `59336c17d67a1fa1…` | `59336c17d67a1fa1…` | identical |
| tutorial-9 | 878 | `b185ed5d5ad787c2…` | `b185ed5d5ad787c2…` | identical |
| tutorial-10 | 543 | `c786dacbe1750b29…` | `c786dacbe1750b29…` | identical |
| tutorial-11 | 567 | `b05860a18e87f077…` | `b05860a18e87f077…` | identical |
| tutorial-12 | 2628 | `b40ebb389c7bb283…` | `b40ebb389c7bb283…` | identical |
| tutorial-13 | 879 | `d450db713dc51fdf…` | `d450db713dc51fdf…` | identical |
| tutorial-14 | 5157 | `7bc6d15b0b85c05e…` | `7bc6d15b0b85c05e…` | identical |

The head of the shared package, decoded: `82` a map of two, `a8 "manifest"`, `83` a map of three holding `format`
(`mercury-package`), `format_version` (`1`) and `set` (`minigraph-tutorials`) in UTF-8 byte order, then `a4 "maps"` and
`8e`, a map of fourteen entries in the order `tutorial-1.json`, `tutorial-10.json`, `tutorial-11.json` ... (sorted by
UTF-8 bytes, so `tutorial-10` precedes `tutorial-2`).

### Leg 2: the 50 fixtures

The fixtures add volume (1,122 maps, 704 lists, 1,822 strings across 50 graphs) and the cases the tutorials lack: graphs
that are rejected by the deployment gate on purpose, and the only booleans in the corpus. The 53,193-byte package is
identical in both engines, and so is every one of the 50 single-graph packages.

### The negative control

Without the packager, the engines disagree on **every** graph. The two general serializers write the same number of bytes
for a graph (`tutorial-3`: 1,669 bytes in each) and differ **from the second byte**, which is the first map header's
first key. That is the order of the keys: the Java loader keeps the order of the file, the Rust loader its own, and
neither serializer sorts. The canonical packager is the step that makes them agree, and this drive shows it doing so on
real graphs: 0 of 14 identical without it, 14 of 14 with it.

## What the graphs do and do not exercise

The graphs are **models, not data**: the 14 tutorials hold 385 maps, 252 lists and 658 strings, with 814 keys, nesting up
to five levels and strings up to 349 bytes (so the str16 header is exercised), and the 50 fixtures add six booleans. They
contain **no numbers, no nulls and no non-ASCII text**. This drive therefore proves what real graph content needs: key
sorting at depth, entry ordering, the str, list and map headers, and the manifest. The typed corners (every integer
encoding boundary, float64 and the `Float` widening, nulls kept, Unicode and the UTF-8 versus UTF-16 key order, exact numbers
as strings) are pinned by the shared vector file, whose 73 values, 6 packages, 60-document random corpus and 25 rejections
both engines pass; the two proofs are complementary.

## Reproduce

The harnesses are throwaway test classes, deliberately not committed (a drive, not a regression test: the regression
test is the vector file). Build each engine from the commits above in a clean worktree, then:

```java
// Java: a JUnit test in platform-core, the engine's own JSON mapper
var mapper = SimpleMapper.getInstance().getMapper();
var builder = CanonicalPackager.builder().manifest("set", "minigraph-tutorials");
for (int i = 1; i <= 14; i++) {
    var name = "tutorial-" + i + ".json";
    builder.add(name, mapper.readValue(Files.readString(graphs.resolve(name)), Map.class));
}
Files.write(out.resolve("java.pkg"), builder.build());
```

```rust
// Rust: an integration test in event-script, the engine's own JSON conversion
let mut builder = Builder::new().manifest("set", "minigraph-tutorials").unwrap();
for i in 1..=14 {
    let name = format!("tutorial-{i}.json");
    let json: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(graphs.join(&name)).unwrap()).unwrap();
    builder = builder.add(&name, from_json(&json)).unwrap();
}
std::fs::write(out.join("rust.pkg"), builder.build().unwrap()).unwrap();
```

Then `cmp java.pkg rust.pkg` (exit 0) and `shasum -a 256` on both. For the cross-read, strict-read each file with the
other engine's `unpack` and re-pack the entries; for the negative control, replace the packager with
`MsgPack.packMapOrList` and `rmpv::encode::write_value`.

## Addendum: the graph packager command lines (RFC-0005)

*Conducted 2026-10-05 on the `feature/graph-packager-cli` branch of each repository, which adds the graph packager: Java
`helpers/graph-packager` (an executable jar) and Rust `tools/graph-packager` (a binary).* Each command line reads graph
files with its engine's JSON reader, checks every graph with its engine's deployment gate and writes with its engine's
packager, so the question widens: **do the two command lines produce the same `.pack` file, accept the same graphs and read
each other's files?** Both ran the same command on byte-identical inputs:

```bash
graph-packager pack --set tutorials --manifest version=4.12.20 --manifest description="The tutorial graphs" --out <dir> <graphs>
```

| Check | Outcome |
|---|---|
| The 14 tutorials, packed by each command line | **identical**: 20,274 bytes, SHA-256 `4715598820261ca6c98617fd124cdfb15f3c6cd138b0e9dffabde3adac7f051f` |
| `inspect` and `inspect --json` of that file, run by each command line | **identical output**, text and JSON |
| Each command line unpacks the file the other wrote | accepted; the 14 unpacked JSON files are **identical** across the engines |
| Each command line packs the folder the other unpacked | **identical** to the original bytes |
| The gate on the 52 fixtures that are byte-identical in both repositories | both refuse the **same 18 graphs**; 17 reasons are word for word the same, and the eighteenth differs only because the Java message for a misplaced `ttl` also names `graph.js`, which the Rust engine does not register |
| The 34 fixtures both gates accept, packed by each command line | **identical**: 44,776 bytes, SHA-256 `7acbad4d862f209205468a1ef6cbd10af6eb59bdcade285a5f5120cbabe712f2` |
| A graph holding `"key": null` (a root property and a relation property) and `"key": ""`, packed by each command line | **identical**: 336 bytes, SHA-256 `f2517ea7c0402067ab26afc2a02bcf24c8401012b76f782b40b625ab8b7330a2`; the null properties are filtered out, the empty string is kept, and the unpacked files are identical |
| **Negative control**: one character changed in one tutorial (`hello world` to `hello World`) before the Rust pack | the files **differ** (`cmp` exit 1) |

The tutorial package is 38 bytes longer than Leg 1's because its manifest holds `set`, `version` and `description`
instead of `set` alone: the manifest is content, so it is part of the bytes.

**One difference between the engines, found by this drive and closed in the same change.** When it normalizes a graph, the
Java configuration reader drops a key whose value is null, as the engine's serializer does by default, while the Rust reader
kept it and the Rust graph import refused a null property: a graph holding `"key": null` deployed on Java and was refused on
Rust, by the startup gate and by the packager alike. The rule is now one rule in both engines: **a graph holds no null
property**. A map entry whose value is null is filtered out, at every depth, when a graph is deployed, packed or read; an
empty string (`"key": ""`) is a value and is kept, and a list keeps its elements in place. The null graph's row above is the
proof. The rule does not follow `serializer.null.transport`, the switch with which platform-core's serializers keep nulls on
the event transport: both packagers write through the canonical packager, never through those serializers, and both engines'
tests pack the same bytes with the switch on (Java in a second JVM, where `SimpleMapper` is shown keeping a null first).
The read at deployment now agrees beyond nulls too: the Rust engine reproduces the Java configuration reader's normalization of
a graph (an empty map or list is dropped, and inside a list such an element is null when a value follows it and dropped at the
end), pinned by the vector file `graph-read-normalization-vectors.json`, byte-identical in both repositories.

To reproduce, build each command line from its branch and run the command above on
`system/minigraph-playground-engine/src/main/resources/graph` (Java,
`java -jar helpers/graph-packager/target/graph-packager-<version>.jar`) and on `crates/knowledge-graph/resources/graph`
(Rust, `cargo run -p mercury-graph-packager --`), then `cmp` the two files.

## Conclusion

The Java and Rust engines write **identical bytes** for the real graph models the Playground ships: 14 of 14 tutorials and
50 of 50 test fixtures, one package or one graph at a time, each engine accepting the other's package under the strict
read. The same comparison fails on all 14 graphs without the packager, so the agreement is the packager's doing and the
method can see a divergence. Together with the vector file, which holds the typed corners, the contract of the
[Canonical Package Format](../guides/canonical-package-format.md) is verified across both engines from synthetic and from real
content. Since RFC-0005 the same holds for the graph packager command lines: they pack the tutorials and the fixtures both
gates accept to identical files, refuse the same graphs and read each other's packages.
