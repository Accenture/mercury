# ai-contract-provider

Mercury's version-matched operational contract for AI discovery — the Rust lock-step twin
of the Java `system/ai-contract-provider`. A standalone composable app: every REST endpoint
is wired `rest.yaml` → Event Script flow → function (the flow YAML files are byte-identical
to the Java app's), so the app is itself a reference implementation of the composable
pattern.

## Run the discovery server

```bash
cargo run -p ai-contract-provider
curl 'http://127.0.0.1:8999/api/discovery'
```

| Endpoint | Serves |
| --- | --- |
| `GET /api/discovery` | Mercury version, contract ids, endpoint map |
| `GET /api/contracts` | the installed contract list |
| `GET /api/contracts/{id}` | one contract: behavior anchors + references |
| `GET /api/skill` | the `mercury-platform` Agent Skill entrypoint (SKILL.md) |
| `GET /api/references?path=...` | one packaged reference file by inventory path |
| `GET /api/manifest` | per-file SHA-256 manifest + whole-snapshot hash |

## Export the offline Agent Skill

```bash
cargo run -p ai-contract-provider -- --export /path/to/existing/directory
```

Writes `mercury-platform/` with `SKILL.md`, the packaged documentation closure
(`references/…`, including `references/llms.txt`), the generated installed-contracts
inventory, and `manifest.json` written LAST as the completion marker. Two exports of the
same build are byte-identical; an existing `mercury-platform/` target is never overwritten.

## Design notes

- The snapshot is embedded at COMPILE TIME by `build.rs` from
  `resources/skill/files.list` — a missing documentation file fails the build, and the
  binary is self-contained under every deployment.
- Contract behavior anchors in `resources/contracts.yaml` are fully-qualified Rust paths,
  resolved at compile time by the anchor test (the Java `Class.forName` analog) — a renamed
  behavior item fails the workspace build. The knowledge-graph crate is a dev-only
  dependency: a runtime dependency would preload the playground into this app.
- `mercury_version` is the workspace-pinned version: one Cargo lockfile makes the mixed
  platform-core/event-script assembly the Java app refuses at startup impossible to build.
