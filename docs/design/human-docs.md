# Design — Human Documentation Site (MkDocs)

> **Status:** DRAFT v1 for maintainer review · **Realizes:** `ot-human-guides-backlog` ·
> **Serves:** `vision-mercury` · **Author:** Claude Code · **Date:** 2026-07-20
> **Content references:** the Java repo's human guides (`~/sandbox/mercury-composable/docs/guides/`,
> ~8,900 lines), this repo's source code + engine-verified AI docs, and the in-app help pages.
> **Presentation reference:** the `agent-memory` MkDocs site (`~/sandbox/agent-memory`) — the
> maintainer-preferred layout.
> This is a *Design-altitude* artifact (VBDI): implementation waits on the gate.

## 1. Goal & audiences

Deliver the deferred **human developer documentation** for the Rust port, as a browsable site:

- **Developers** — install → hello world → write functions, flows, graphs (task-oriented guides).
- **Architects** — the three-layer architecture, the composability methodology, port scope.
- **Operators** — configuration, observability, actuators, deployment.

The **AI docs stay agent-optimized and separate** (they are engine-verified and self-sufficient —
the sweep's core deliverable). The human pages **link** to them rather than restate rules, so
there is exactly one source of truth per fact. The site surfaces the AI set under Reference.

## 2. Toolchain (decision D-H1)

**MkDocs + Material**, pinned (`mkdocs-material==9.7.6`), reusing the proven `agent-memory`
recipe verbatim where it applies:

- `mkdocs.yml` at the repo root; **`docs/` is already the docs tree** (MkDocs default
  `docs_dir`) — the human pages join it; internal material is excluded (see §5).
- Theme features: `navigation.tabs` + `navigation.sections` + `navigation.indexes` +
  `navigation.top/footer`, `toc.follow`, `search.suggest/highlight`, `content.code.copy`,
  `content.tabs.link`.
- Markdown extensions: `admonition`, `def_list`, `attr_list`, `footnotes`, `tables`, `toc`
  (permalinks), `pymdownx.details/superfences (mermaid)/tabbed/highlight/inlinehilite/snippets`.
- `docs-requirements.txt` + a CI **build-only** check (`mkdocs build --strict`) on push;
  GitHub Pages deployment is **deferred** until the repo graduates/goes public.
- Palette: Material default unless the maintainer wants a brand color (open question Q3).

## 3. The presentation rule (decision D-H2 — the reason this design exists)

The Java site's reference pages use **wide multi-column tables** (e.g.
`configuration-reference.md`: Key | Type | Default | Description) that overflow the content
column and force horizontal scrolling — the maintainer-identified defect.

**Rule: no reference table wider than 3 columns; reference entries are entry-per-heading.**
Each configuration key / annotation / API item becomes:

```markdown
#### `rest.server.port`

| Type | Default |
|---|---|
| int | `8085` |

Port of the REST automation server. `0` binds an ephemeral port (tests).
```

or a **definition list** (`def_list`) for short entries. Long defaults, examples, and multi-line
values go in fenced code blocks *under* the entry — never inside a table cell. Benefits: full
content width for descriptions, deep-linkable per-key anchors (`toc` permalinks), searchable
headings, and mobile-safe rendering.

Supporting rules:
- **Mermaid** (`superfences`) for diagrams — no static images to go stale.
- **Admonitions** for port truths: a `!!! note "Rust port"` box wherever behavior deliberately
  differs from the Java original (the no-silent-divergence convention, applied to docs).
- **Content tabs** (`pymdownx.tabbed`) where Java/Rust comparison genuinely helps
  (e.g. `@PreLoad` vs `#[preload]` in the migration notes) — sparingly.

## 4. Information architecture (nav)

Tabs modeled on `agent-memory` (Home / Getting Started / Concepts / Guides / Reference /
Background):

```yaml
nav:
  - Home: index.md                          # what mercury is; the three layers; quick links
  - Getting Started: getting-started.md     # toolchain, clone, build, run hello-world,
                                            # first flow, first graph (cargo, not maven)
  - Concepts:
      - concepts/index.md
      - Architecture: concepts/architecture.md          # actor model, EventEnvelope, bus,
                                                        # Platform/PostOffice, three layers
      - Composability Methodology: concepts/methodology.md
      - Observability Model: concepts/observability.md  # traces, spans, telemetry, W3C
  - Guides:
      - guides/index.md
      - Write Your First Function: guides/write-your-first-function.md
      - Function Execution: guides/function-execution.md   # instances, RPC, callback,
                                                                 # fork-join, interceptors
      - Write Your First Flow: guides/write-your-first-flow.md
      - REST Automation: guides/rest-automation.md          # rest.yaml IS the router
      - Build Your First Graph: guides/build-your-first-graph.md
      - Playground & AI Companion: guides/playground-and-companion.md
      - Composing the Layers: guides/composing-the-layers.md  # closes the dangling
                                                                    # link tut-5 flagged
      - Build, Test, Deploy: guides/build-test-deploy.md
  - Reference:
      - reference/index.md
      - Configuration Keys: reference/configuration.md      # entry-per-heading (D-H2)
      - Macros & Annotations: reference/macros.md           # #[preload], #[optional_service],
                                                            # #[main_application], features…
      - EventEnvelope: reference/event-envelope.md
      - Platform & PostOffice API: reference/api-overview.md
      - Reserved Names & Headers: reference/reserved-names.md
      - Actuators & HTTP Client: reference/actuators-and-http-client.md
      - Flow Schema: reference/flow-schema.md
      - AI Agent Docs:                                      # the EXISTING engine-verified set,
          - guides/knowledge-graph/ai-agent-guide.md        # included as-is (no duplication)
          - guides/knowledge-graph/command-reference.md
          - guides/knowledge-graph/skills-reference.md
          - guides/event-script/ai-agent-guide.md
          - guides/event-script/flow-grammar.md
          - guides/event-script/syntax.md
          - guides/event-driven/ai-agent-guide.md
  - Background:
      - Port Scope & Fidelity: background/port-scope.md     # map-don't-mirror; what is out
                                                            # (Kafka mesh, Spring, graph.js)
      - Architecture Decisions: arch-decisions/ADR.md
```

Naming note *(superseded by Q1)*: human pages live **inside `docs/guides/`** alongside the
AI set, mirroring the Java repo — one tree for both audiences; existing AI-doc paths are
untouched because only new files are added.

## 5. What is excluded from the site

`exclude_docs` (internal / working material): `design/`, `INCREMENTS.md`,
`AI-companion-test.md`. `not_in_nav` (linked, not navigated): `llms.txt`,
`guides/knowledge-graph/minigraph-commands.json`, `guides/event-script/event-script-flow.json`.

## 6. Source mapping (map, don't mirror)

| New page | Java source | Adaptation notes |
|---|---|---|
| index, getting-started | `getting-started.md`, `README` | cargo workspace, `auto_start_main!`, port 8100 examples |
| concepts/architecture | `architecture.md`, `api-overview.md` (intro) | tokio (not virtual threads); compile-time registration (not classgraph) |
| concepts/methodology | `methodology.md` | unchanged philosophy; Rust examples |
| concepts/observability | `observability.md` | Rust telemetry format (verified against `logging.rs`/`trace`) |
| write-your-first-function | `event-driven/write-your-first-function.md` | `#[preload]` + `ComposableFunction` (real code from `examples/hello-world`) |
| function-execution | `event-driven/function-execution.md` | instances/workers, RPC, fork-join over tokio |
| write-your-first-flow | `event-script/index.md` + flow guides | flow YAML identical; Rust functions |
| rest-automation | `rest-automation/*` | hyper boundary; the exact Java content-type dispatch (increment 36) |
| build-your-first-graph | `knowledge-graph/build-your-first-graph.md` | Playground on 8100; `graph.js` retired; discovery commands (increment 42) |
| playground-and-companion | `knowledge-graph/playground-and-companion.md` | `/sync` endpoint; read-only session rule |
| composing-the-layers | `knowledge-graph/composing-the-layers.md` | **closes the dangling link** the sweep found (rollup #9) |
| build-test-deploy | `build-test-deploy.md` | cargo build/test/clippy/fmt; standalone example crates |
| reference/configuration | `configuration-reference.md` | **entry-per-heading (D-H2)**; `APP_PROFILES_ACTIVE` + `application.name` (increment 37) — keys verified against `AppConfigReader` usage |
| reference/macros | `annotations-reference.md` | the macro set from `platform-macros` (source-verified) |
| reference/event-envelope | `event-envelope-reference.md` | rmpv body; serializer null-omission (increment 33) |
| reference/api-overview | `api-overview.md` | `Platform`/`PostOffice` Rust signatures |
| reference/reserved-names | `reserved-names-and-headers.md` | verified against source constants |
| reference/actuators-and-http-client | `actuators-and-http-client.md` | `/info/lib` + `/info/routes` deferred — say so |
| reference/flow-schema | `flow-schema-reference.md` | flow YAML schema (shared with Java) |
| background/port-scope | `memory/vision.md` + instructions | public-facing scope statement |

**Skipped Java pages** (out of port scope — listed in `background/port-scope.md` instead):
`minimalist-kafka.md`, `twin-kafka.md`, `service-mesh.md`, `spring-boot.md`,
`schema-registry-mock.md`, `sync-over-async.md`, `event-over-http.md` (not yet ported),
`ai-developer-guide.md` (superseded by `llms.txt` + the AI set).

## 7. Verification protocol (same bar as the help-page rewrite)

- Every code sample is **taken from or verified against** this repo's source/examples — no
  pseudo-Rust; each page footer names the Java origin page for side-by-side review.
- Configuration keys checked against actual `AppConfigReader` reads; endpoint/port examples
  against `rest.yaml`s; behavior claims against the engine-verified AI docs (link, don't restate).
- `mkdocs build --strict` (broken links fail the build) locally + in CI.

## 8. Increments (each lands green; gate at phase 1)

1. **Scaffold**: `mkdocs.yml` + `docs-requirements.txt` + CI build check + Home +
   Getting Started (buildable site from day one).
2. **Concepts + event-driven/event-script guides.**
3. **REST automation + knowledge-graph guides + the D-H2 reference conversions**
   (configuration/macros/envelope/api-overview/reserved-names/actuators/flow-schema).
4. **Background + AI-docs nav integration + full `--strict` link pass.**

## 9. Open questions — ANSWERED (maintainer, 2026-07-20)

1. **Q1 — page tree:** keep **`docs/guides/`** for BOTH human and AI docs (Java-repo
   consistency — humans can read the AI docs in the same tree). Human pages join the existing
   `guides/` tree (`guides/getting-started.md`, `guides/{layer}/…`); no `guides/`.
2. **Q2 — site identity:** **`https://github.com/Accenture/mercury`** — the repo becomes the
   official Rust home after this documentation update (the Vision's graduation milestone);
   thereafter the regular PR process applies.
3. **Q3 — palette:** Material fine; **purple** is an Accenture corporate design theme →
   `primary: deep purple`.
4. **Q4 — in-app help pages:** unchanged; the UI rewrite is a separate future project.

### Original questions (for the record)

1. **Q1 — page tree name:** `docs/guides/` as proposed, or fold human pages directly
   under `docs/` top level (agent-memory style: `concepts/`, `guides/`… — would require the AI
   set to move or the human guides tab to use a different word)? *(Default: as proposed —
   zero disturbance to AI-doc paths.)*
2. **Q2 — site identity:** `site_url`/`repo_url` point at the private repo for now, or
   placeholders until graduation? *(Default: real private-repo URLs; trivially edited at
   graduation.)*
3. **Q3 — palette:** Material default, or a brand primary (agent-memory uses deep purple)?
4. **Q4 — in-app help pages:** unchanged (they serve the Playground console); the site links
   to the Playground rather than duplicating them. Confirm.
