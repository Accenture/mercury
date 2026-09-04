# Memory Smoke Test — mercury

> A cheap, manual check that the memory layer can actually orient a newcomer. **A fresh
> agent answers these from `memory/` alone** — no source code, no asking the user — then
> marks each ✅ (answerable from memory) or ❌ (gap). An ❌ is a *memory* gap: fix it by
> adding the missing fact, never by softening the question. App-level memory evaluation
> is an unsolved, bespoke problem industry-wide; this is the no-code, markdown version.

## How to run

1. Read **only** `memory/instructions.md`, `memory/continuity.md`, `memory/vision.md`, the
   unchecked (`- [ ]`) files in `memory/open-threads/`, the latest 2–3 `memory/sessions/`,
   and `memory/archive/INDEX.md`. Do not read source or ask the user.
   *(`open-threads/` + `vision.md` joined the read set 2026-09-04: threads moved one-per-file
   in agent-memory v4.39.0, so "what is in progress" became unanswerable without them —
   found by the smoke test in both repos.)*
2. Answer each question from those alone; mark ✅ or ❌ (with a one-line note on misses).
3. Append a row to the **Result log**. For each ❌, add the missing fact to memory (or
   open a thread to capture it) — then the next run should pass.

Run it **on demand** ("run the memory smoke test"), after a large change, or alongside a
review. Don't edit the questions to make them pass.

## Orientation questions (generic — apply to any repo)

1. What does this project do, and what type is it? *(→ instructions "What This Project Is")*
2. What is the stack — language, key dependencies, versions? *(→ continuity "Stack & Tools")*
3. What are the architectural invariants — things that must never change? *(→ continuity "Architectural Invariants")*
4. What were the last 2–3 key decisions, and **why**? *(→ continuity "Key Decisions" / recent sessions)*
5. What is in progress right now? *(→ continuity "Open Threads")*
6. What conventions should new code follow? *(→ instructions / continuity "Conventions")*
7. Any recorded user preferences or team / agent assignments? *(→ continuity — explicit only)*
8. Has any past decision been reversed or **superseded** — and by what? *(→ continuity superseded facts / `archive/INDEX.md`)*

## Project-specific questions (seeded at enable; grow these as the project does)

1. What is mercury's **Vision** — the target state the project is being built toward? *(→ `memory/vision.md`)*
2. What **stage** is mercury at — is there code yet, and what (if anything) is in the stack? *(→ continuity "Project State" / "Stack & Tools")*
3. **Why** was mercury AI-enabled *before* any code existed, and what does that imply for how work should proceed? *(→ continuity "Key Decisions" / the enable session log)*
4. Which **layers** are being ported (and in what order), and what is explicitly **out of scope**? *(→ `memory/vision.md` / continuity "Key Decisions" / Blueprint)*
5. Where does the **authoritative spec** for the behavior being ported live, and why isn't it copied into this repo? *(→ `memory/vision.md` "map, don't mirror" / the knowledge-harvest thread)*

## Result log

| Date | Through session | Score (✅/total) | Gaps found → action |
|---|---|---|---|
| 2026-07-15 | (enable) | — | baseline — run the test to populate |
| 2026-08-14 | 2026-08-14-005444 | 13/13 | none |
| 2026-08-22 | 2026-08-22-032041 | 13/13 | All questions pass (fresh-context agent, memory files only), plus 3 staleness findings fixed: (1) status line said v4.11.4 while v4.11.10 shipped 2026-08-21 → corrected; (2) instructions "Conventions Observed" still said "None yet — no Rust code" (stale since increment 1) → now points at the live conventions-rust-baseline fact; (3) last_invariant_check said "all five confirmed" without naming its set (a fresh reader counting the 2 listed invariants misreads it as data loss) → the five never-decay facts are now named in the stamp. |
| 2026-09-04 | 2026-09-04-043850 | 10/13 | Fresh-context agent (cross-repo, so `vision.md` was NOT auto-imported — project Q1 scored provisionally). **Regression from 13/13**, and the headline was real: `instructions.md` still described a greenfield repo with no code, no tests and no CI, five unqualified present-tense claims, while v4.12.2 ships from seven crates.io crates — a newcomer asking "what's the layout / how do I run tests" got a wrong answer. Rewritten to the real workspace, test harness and CI. Also fixed: the canonical Java version was pinned at v4.8.6 in three places (now "released lock-step"); `last_invariant_check` had an unclosed paren and said "four" while enumerating five ids; **no `## Blueprint` section existed at all** (threads moved to files in v4.39.0, taking the Vision link with them) — restored. Two facts added for answers lost to archival or never recorded: `why-ai-enabled-before-code` (Q3's rationale had faded with its carriers, and the INDEX one-liners hold the *what*, not the *why*) and `conv-llms-txt-curated-map` (today's ruling lived only in a session log about to leave the window — with a live risk that a parity sweep "fixes" the deliberate divergence). Read set fixed as in the Java repo. **Raised for Eric, not inferred:** User Preferences and Team/Members are both "(none recorded yet)" while the release-gate rhythm is visibly in force — this section carries an explicit never-infer rule, so it needs his word. Deferred: truncated `archive/INDEX.md` one-liners; the toolchain line still anchored to "increments 1–2"; the ~10-line `status:` carrying version history against its own rule. |
