---
name: memory-lint
description: Deterministic integrity check for the agent-memory layer. Use after a memory review, before committing memory/ changes, or in CI to catch decay miscounts — facts archived while still referenced, an id in both continuity and the archive, tier or supersession drift. The agent judges meaning; the script does the counting.
provenance: agent-memory-builtin
---

# memory-lint

> ⚠️ **Tool-managed skill provided by agent-memory** (`provenance: agent-memory-builtin`). Don't edit it
> in place — it is overwritten on upgrade. To change behavior: **fork** it under a new skill name, or
> **upstream** a genuine fix to the agent-memory project (file an issue in its repo, or bring it to the
> tool maintainer) for back-port + validation. See `SKILLS.md` → "Tool-provided (system) skills".

A deterministic safety net for the evolving-memory layer. Decay is integer-counting by design, but
an agent counting session files **by hand** can miscompute `sessions_since_last_used` and archive a
fact that's still in use — this actually happened (a fresh-agent review over-archived recent facts,
and a hand re-check still missed one). This skill moves that counting from agent judgment to a
script, so the riskiest operation is verified against observable evidence.

## When to use

- **After** running the memory **review** ritual (`REVIEW.md`) — verify its archival decisions.
- **Before** committing `memory/` changes.
- In **CI** or a **pre-commit hook** (the optional reinforcement `AGENTS.md` mentions) — a non-zero
  exit fails the gate.

## What to do

1. Run the bundled checker from the repo root. **Two interchangeable runtimes** — use whichever the
   machine has (stdlib/built-ins only, no install). Same flags, same output, same exit codes:
   ```bash
   python3 agent-skills/memory-lint/scripts/memory-lint.py    # Python 3 (>= 3.8)
   node    agent-skills/memory-lint/scripts/memory-lint.mjs    # Node (>= 18)
   ```
   Flags: `--strict` (also fail on warnings), `--root PATH` (point at a specific repo).
   *Run the test suite (the cross-runtime contract — both implementations pass the same fixtures):*
   ```bash
   python3 -m unittest agent-skills/memory-lint/scripts/test_memory_lint.py
   node --test          agent-skills/memory-lint/scripts/test_memory_lint.mjs
   ```
2. It checks, deterministically:
   - **no id lives in both `continuity.md` and the archive** (a fact exists in exactly one place);
   - **no archived-as-faded fact was referenced within `archive_window` sessions** — the decay-miscount
     guard: if it was, the count was wrong, so **reactivate it**;
   - *advisory* — continuity facts overdue for archival (`sslu > archive_window`), excluding `core`,
     `superseded`, and pinned `- [ ]` open threads (which never decay);
   - supersession links resolve (`supersedes` / `superseded-by` point at real footers);
   - **`.agent/version.md`, if present, carries a parseable `- **version:** X.Y.Z` line** — an
     empty/malformed manifest breaks Mode B upgrade detection (this was a real bug: a truncating stamp
     one-liner emptied it). A *missing* file is the valid pre-versioning baseline and is not flagged.
   - **no leftover merge-conflict markers** (`<<<<<<<` / `>>>>>>>` / diff3 `|||||||`) in the **live
     top-level `memory/*.md`** files (`continuity.md`, `instructions.md`, `vision.md`, `decay-policy.md`,
     `smoke-test.md`) — an unresolved conflict there silently corrupts shared memory the agent reads as
     truth. `sessions/` and `archive/` are **excluded** (immutable/append narrative that legitimately
     *quotes* markers — e.g. a session log pasting a diff). A bare `=======` line is *not* flagged
     (it's a valid Markdown setext heading underline).
   - *advisory* — **`[review-overdue]`**: `sessions_since_last_review ≥ review_every` (read from the
     `last_review` Project-State stamp), so a lapsed review ritual surfaces on every lint run + CI,
     not just when someone remembers to check;
   - *advisory* — **`[continuity-bloat]`**: more than `continuity_max_facts` decaying facts/threads
     (the primary lean signal — a count, immune to verbosity & session velocity), or more than
     `continuity_max_lines` lines (a coarse backstop). Both say "a review is due to lean it down."
   - *advisory* — **`[stale-metadata]`**: a fact's stored `tier` disagrees with the tier recomputed from
     the reference log (review steps 2–3 — apply events / re-tier — were skipped), excluding `core`,
     `superseded`, never-referenced facts, and **pinned `- [ ]` open threads** (their tier label isn't
     enforced — pinned-ness protects them; v4.26.1). Clear it with the **`refresh-metadata`** skill (or a review).
3. **ERROR** (exit 1) → fix per `DECAY.md` / `REVIEW.md`: reactivate an over-archived fact (move it
   back into `continuity.md`), de-duplicate, or repair a link. **WARN** is advisory — the next review
   may act on it.

## Notes — scope and philosophy

- **Optional; the tool never runs it.** The memory layer works without it (markdown + agent is the
  runtime — `no-build-step-agent-run`). This is a *verifier*, invoked by the agent / human / CI at
  your direction.
- It lints the **arithmetic and integrity**; it does **not** judge *meaning* (what's worth recording,
  supersession truth-state, contradictions) — that stays human/agent.
- **Two runtimes so the deterministic check survives a missing one.** The verifier ships in both
  Python (`memory-lint.py`) and Node (`memory-lint.mjs`) — kept equivalent by a shared test contract —
  so a machine with only one of them still gets the script, not a hand count.
- **Neither Python nor Node? Install one — do not hand-count.** Decay arithmetic is *only ever*
  script-verified, never counted by hand: an LLM hand-counting `sessions_since_last_used` has already
  over-archived a still-referenced fact, which is the whole reason this skill exists. So when neither
  runtime is present, **do not estimate the decay by hand** — surface this to the human and pause any
  archival decision:

  > ⚠️ **memory-lint needs a runtime to verify decay deterministically — neither Python nor Node was
  > found. Please install *either one* (both are free, one-time, and need no extra packages — the
  > verifier is standard-library only): https://www.python.org/downloads/ or https://nodejs.org . Then
  > re-run the check. Until then, archival/decay decisions are on hold.**

  The memory layer itself still works without the verifier (it's optional — see above); reading,
  writing, and recall need no runtime. What waits for a runtime is *committing a decay decision* — if
  this machine genuinely can't install one, defer that decision to a machine that can, rather than
  hand-counting it.
- **Why fail-loud-and-halt instead of a fallback?** Missing *both* runtimes is rare: a developer
  machine almost always has at least one of Python or Node. The realistic trigger is a **thin or
  locked-down CI/container image** that hasn't provisioned tooling yet — and CI is exactly where this
  skill runs as a gate. There, a hard stop ("add a runtime to the image") is the *correct* outcome, not
  a degradation to paper over; and on the rare bare machine you defer rather than guess. So a one-time
  install nudge is the proportionate response — engineering a runtime-free fallback would spend effort
  reintroducing the very hand count this skill exists to remove.
