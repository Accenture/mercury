# Graph packager

A command line that packs graph models into a **graph set**: one deterministic `.pack` file that a build pipeline can
sign, store and deploy as one certified unit. It is a thin front over the engine's own packager - the canonical
MsgPack package of ADR-0026, `platform_core::canonical_packager` - and over the deployment gate's own checks
(`knowledge_graph::model_gate`), so a set the packager accepts holds only graphs that the compiler would accept at
startup. It never starts the platform. It is the twin of the Java engine's `helpers/graph-packager`: the same commands,
messages and exit codes, and the same bytes for the same graphs (see the interop report,
[`docs/test-reports/canonical-package-java-rust-interop.md`](../../docs/test-reports/canonical-package-java-rust-interop.md)).

The design is RFC-0005 in the Java repository's `docs/arch-decisions/RFC.md`; the package format is described in the
guide [Canonical package format](../../docs/guides/canonical-package-format.md).

## Build and run

```bash
cargo build --release -p mercury-graph-packager
target/release/graph-packager --help
```

`cargo run -p mercury-graph-packager -- <command> …` runs it from the workspace. The crate is `publish = false`: it
is built with the workspace and is not published to crates.io.

## Commands

```text
graph-packager pack    --set <name> [--manifest key=value]... [--out <dir>] <graph.json>... | <folder>
graph-packager pack    --set <name> --from-manifest <graphs.yaml> [--manifest key=value]... [--out <dir>]
graph-packager unpack  <file.pack> --out <dir>
graph-packager inspect <file.pack> [--json]
```

- **`pack`** reads graph JSON files, or every `*.json` file in a folder, and writes `<out>/<name>.pack` (the current
  folder by default). The graph id is the file name without `.json`. `--from-manifest` packs exactly the graphs a
  deployment manifest lists, read from its `location`: a `file:` folder, or a plain folder path resolved against the
  manifest's own folder. A `classpath:` location is inside an application, so the packager asks for the folder
  instead. `pack` prints the file's SHA-256, a convenience for a signer.
- **`unpack`** writes each graph as `<out>/<graph-id>.json`: readable JSON in the canonical key order with a two-space
  indent, so two versions of a set diff cleanly, and an unpacked folder packs to the same bytes again.
- **`inspect`** prints the manifest, the graphs (nodes and connections) and the size and SHA-256 of the file;
  `--json` prints the same as one JSON object for a pipeline. Standard output carries only the result; log lines go
  to standard error (warnings and errors; `RUST_LOG` sets another level).

## The rules a set follows

`pack` checks every rule before it writes anything and refuses the set with every reason it finds:

- the set name and every graph id use letters, digits, `_` and `-` only (the engine's file-name rule);
- when a graph's root node declares a `name`, it equals the graph id (the rule of `export graph as`);
- every graph passes the deployment gate's checks: the structure, the root node's `purpose`, an `end` node, the data
  mapping syntax and the rules of the model validator (the same function the compiler calls);
- the manifest field `set` is written from `--set`, and `format` and `format_version` by the packager;
- the optional manifest field `graph_id`, the set's entry-point graph, names a graph of the set.

A graph holds no null property: a `"key": null` is filtered out when a set is packed or read, as the engine's
serializer does by default and as an application's configuration reader does when it loads a deployed graph. An empty
string (`"key": ""`) is a value and is kept, and a list keeps its elements in place. This holds whatever
`serializer.null.transport` says: that switch governs what platform-core's serializers keep on the event transport
(SimpleMapper and MsgPack), and the packager writes a package through the canonical packager, never through them.
When an application loads a deployed graph it also drops an empty map or list; a package keeps them, as the files hold them,
and both engines read them the same way at deployment.

`unpack` and `inspect` read a package strictly (a byte form that is not canonical is refused) and check the entry
names, the root names and `graph_id` before any name becomes a path.

**The same graphs and fields always give the same bytes**, in this engine and in the Java one. Nothing is taken from
the clock or the environment - pass a build time with `--manifest` when a pipeline wants one. A `${...}` reference in a
graph is packed as written, because it belongs to the environment the set is deployed to; the gate's check reads the
graph with its references resolved, as an application does at startup.

## Exit codes

| Code | Meaning |
| --- | --- |
| 0 | success |
| 1 | a refused input: a rule the set breaks, or a usage error |
| 2 | an I/O or format error: a file that cannot be read or written, a graph file that is not a JSON object, a package that fails the strict read |

## Signing

A package carries no hash or signature: integrity is the application's choice (ADR-0026). The convention for a
separate signing tool is a detached `<set>.pack.sig` beside the package; the SHA-256 that `pack` and `inspect` print
is the digest of the exact bytes such a tool protects.
