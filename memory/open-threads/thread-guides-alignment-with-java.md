- [ ] **The guides are being aligned with the Java pages, step by step, after a review of every page (2026-11-24).**
  The review compared the 38 paired guide pages at `61a4237f` with the Java guides: none identical, 1 close, 23 with
  findings, 14 different documents; 20 Java pages have no page here, 5 of them about features this port has. Its
  report is not in the repository. It proposed fifteen small pull requests, cheapest first.
  - **Done:** steps 1 and 2, stale statements and broken anchors (Increment 212; a pull request from branch
    `docs/guides-stale-statements-and-anchors`, whose merge is the maintainer's).
  - **Left, in the proposed order:** 3 format-only passages (rewraps in the skills reference, the graph contract, the
    suspension guide, the developer guide, `event-over-http.md`; contractions in `minimalist-kafka.md`). 4
    `event-script/syntax.md`: the Java paragraphs that hold here ("Null sources", "Null arguments are
    plugin-specific", the compile-time conversion sentence, the bad-decision paragraph, two figures) and a *Rust port*
    note for `datatype`. 5 `flow-grammar.md` and the flow agent guide (the default error body; "both keys are
    required"). 6 `knowledge-graph/ai-agent-guide.md` and `minigraph-commands.json` (the Import row and three routes,
    `output.status`). 7 the two `graph.extension` bullets of the skills reference. 8 `configuration-reference.md`: seven
    `sync.*` keys this engine reads, and `yaml.flow.automation` as a list. 9 `claims-registry.json`: two Java quotes,
    and three claims that each need a Rust test first. 10 `ai-developer-guide.md`. 11 pages close to Java with a few
    items each. 12 the paraphrased pages (`polyglot-functions.md`, `methodology.md`, `composing-the-layers.md`). 13
    `rest-automation`. 14 the fourteen different documents. 15 missing pages (a `sync-over-async.md` guide, the
    property graph API summary, the `EventScriptMock` and `FlowExecutor` testing sections).
  - **The maintainer's decisions the review marked:** `transaction.journal.recorder` (remove the row, or keep it with
    a note that journaling is not ported); one `rest-automation` page or Java's folder of four; whether to re-base
    each of the fourteen different documents on the Java text; whether Java's `documentation-conventions.md` (front
    matter, At a glance) binds this repository, where 25 of 37 pages have no front matter; a link to the Java site or
    pages here for the language packs' agent tools.
  - **Must not be copied from Java:** on a `rest.yaml` entry with `flow:` and no `service`, Java skips the entry and
    this engine fails the whole load; Java's "local time" statement in the observability guide (this engine's log
    time is UTC); the garbled See also entry of Java's `event-over-http.md`.
  - **How to apply:** check a Java statement against this engine's source before taking it
    ([[conv-guides-follow-java-rust]]); a different document needs the maintainer's decision first.
  - origin: 2026-11-24-061701
  → serves: vision-mercury
  <!-- id: guides-alignment-with-java | created: 2026-11-24 | last_used: 2026-11-24 | uses: 1 | tier: working | origin: 2026-11-24-061701 -->
