# Mercury (Rust) — consumer starting point (scoped guide)

> **Contributors working in this repository:** follow the root [AGENTS.md](../AGENTS.md)
> (the agent-memory protocol) first — this file does not replace it. This scoped guide
> serves AI tools that **consume the Mercury Rust engine as a dependency** and need the
> fastest correct starting point. It lives at the same path as the Java repo's consumer
> guide (`system/AGENTS.md`), so one tool convention finds both engines.

## Starting point for consumer AI tools

Mercury's version-matched operational contract is served by a dedicated composable app:
[`system/ai-contract-provider`](ai-contract-provider/README.md).

- **Live discovery:** run the app (`cargo run -p ai-contract-provider`, port 8999) and
  start with `GET /api/discovery` — it names the Mercury version, the installed contracts,
  and every other endpoint (`/api/contracts`, `/api/contracts/{id}`, `/api/skill`,
  `/api/references?path=...`, `/api/manifest`).
- **Offline:** `cargo run -p ai-contract-provider -- --export <dir>` writes the
  self-contained `mercury-platform` Agent Skill: `SKILL.md` entrypoint, the packaged
  documentation closure (including `references/llms.txt`, the machine-readable map), the
  installed contract inventory, and a per-file SHA-256 manifest for integrity verification.

Both surfaces report `mercury_version` from the installed release — verify claims against
it rather than assuming the docs match your runtime. Contract ids are identical to the
Java engine's (`platform-core`, `rest-automation`, `event-script`, `minigraph`); flow YAML
and MiniGraph models port between the two engines unchanged, while the composable
functions they call use this port's Rust API.

## What lives in this workspace

| Crate | Role |
| --- | --- |
| `crates/platform-core` | core engine: functions, `EventEnvelope`, `PostOffice`, REST automation |
| `crates/event-script` | Event Script: YAML flows replacing orchestration code |
| `crates/knowledge-graph` | Active Knowledge Graph engine + playground |
| `crates/*-macros` | the `#[preload]` / `#[main_application]` / plugin attribute macros |
| `extensions/minigraph-state-redis` | Redis store for graph workflow suspension |
| `system/ai-contract-provider` | AI discovery app serving this operational contract |
| `examples/` | runnable apps: hello-world, hello-flow, minigraph-playground |

## Key references (repo-relative)

- Read first: `docs/llms.txt` — the machine-readable documentation map
- Machine catalogs: `docs/guides/event-script/event-script-flow.json`,
  `docs/guides/knowledge-graph/minigraph-commands.json`
- REST automation: `docs/guides/rest-automation.md` — a flow binding needs BOTH
  `service: 'http.flow.adapter'` and `flow: '<flow-id>'`
- Function authoring: `docs/guides/event-driven/ai-agent-guide.md` (the Rust contract)

## Efficient lookup

**For "how do I configure / use X" questions, start with the guide — not the source.**
The guides served by `ai-contract-provider` (and exported into the `mercury-platform` skill)
are the version-matched, token-efficient answer surface. A typical configuration question
costs 3–5× more tokens when answered from source discovery instead of the guide.

Lookup order:
1. `docs/llms.txt` → find the matching guide page by keyword.
2. Read the relevant guide section.
3. Fall back to source only when the guide is genuinely silent on the specific behavior or you
   need to verify a subtle invariant (exact constant name, safety contract, test-proven edge case).

When source reveals something the guide missed, note the gap and consider raising an issue
or PR against the upstream OSS project (github.com/Accenture/mercury) — that is how the
guides improve.
