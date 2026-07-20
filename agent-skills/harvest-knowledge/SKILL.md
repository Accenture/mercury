---
name: harvest-knowledge
description: Re-scan the repo's human-authored documentation (docs trees, ADRs, decision logs, design specs, roadmaps, kanban) and fold newly-durable facts into the shared memory/ layer — additively, on demand. Use when docs have grown or changed since enable, or to backfill a repo enabled before the curious harvest existed. Distills into neutral memory; never overwrites curated facts.
provenance: agent-memory-builtin
---

# harvest-knowledge

> ⚠️ **Tool-managed skill provided by agent-memory** (`provenance: agent-memory-builtin`). Don't edit it
> in place — it is overwritten on upgrade. To change behavior: **fork** it under a new skill name, or
> **upstream** a genuine fix to the agent-memory project (file an issue in its repo, or bring it to the
> tool maintainer) for back-port + validation. See `SKILLS.md` → "Tool-provided (system) skills".

The **on-demand, recurring** counterpart to the enable-time knowledge harvest (`ENABLE.md` Step 4b).
Enable seeds memory **once** from the team's docs; this skill **keeps it in sync** as those docs evolve.

## When to use

- The repo's docs have **grown or changed** since it was enabled — new ADRs, a design spec, decision-log
  entries, a roadmap/kanban update.
- To **backfill** a repo that was enabled *before* the curious harvest existed (the post-upgrade catch-up).
- A human or agent asks to *"harvest knowledge"* / *"refresh memory from the docs."*

**On demand only** — never part of the per-session ritual, and not tied to enable/upgrade mode.

## It is NOT a vendor `/init`

A vendor `/init` does a deep **code** analysis and (re)writes a **vendor** steering file (`CLAUDE.md`,
`.cursorrules`, …), usually **overwriting** it. `harvest-knowledge` instead:

- reads the team's **human-authored knowledge** docs (prose — not code),
- distills **durable facts** into the **neutral, shared, committed** `memory/` layer (every vendor sees it),
- is **additive + incremental** — it never overwrites curated facts; a conflict becomes a `Contradiction`
  thread, not a silent rewrite,
- is **repeatable** — run it whenever docs change, not a one-shot bootstrap.

You may *borrow* `/init`'s analysis muscle, but the output goes to `memory/`, **never** a vendor file.

## What to do

1. **Enumerate, recursively** (same net as `ENABLE.md` Step 4b). Recurse every documentation tree —
   `docs/`, `doc/`, `documentation/`, `wiki/`, `rfcs/`, `adr/`, `design/`, `notes/` (**all subfolders**) —
   and sweep the repo + module roots for human-authored knowledge markdown: decision logs / `DECISIONS*`,
   `ADR*`, `ROADMAP*` / `TODO*` / `BACKLOG*`, kanban/board files, `ARCHITECTURE*` / `DESIGN*`,
   `CONTRIBUTING*`, `RFC*`, `GLOSSARY*`, onboarding / runbook / postmortem notes. Match `.md/.markdown/.mdx/.rst/.txt/.adoc`.
   **Exclude** (not team prose): `node_modules/`, `vendor/`, `.venv`/`venv/`, `target/`, `dist/`, `build/`,
   `.git/`, generated API reference, minified/vendored files, anything already `.gitignore`d.
2. **Scope to what's NEW or CHANGED since the last harvest.** Read **`last_harvest`** from
   `continuity.md` → Project State (the marker this skill stamps in step 7). If present, look only at docs
   changed since then — `git log --since=<date>` / `git diff --name-only <since>..HEAD` over the doc paths.
   If it's **absent** (never harvested — or the repo predates this field), treat it as a **full first pass**.
   Either way, **budget with disclosure:** cap the read (prioritize roots → `docs/` → recently-modified);
   if the budget is hit, record a `- [ ] (knowledge-harvest)` Open Thread listing what's left, so nothing
   vanishes silently. (`last_harvest` only *scopes* the read — the step-4 check-existing-first guard is
   what actually prevents duplicates, so a re-scan is always safe.)
3. **Distill — don't transcribe — into memory, additively:**
   - conventions / architecture decisions / hard constraints → `memory/instructions.md` (and seed
     **Architectural Invariants** from explicit "must / never" rules);
   - current goals / roadmap / in-flight work / decision-log open items → **Open Threads**; the aspiration →
     the **Current-state context** of `memory/vision.md` (never *fabricate* the target);
   - newcomer-facing knowledge → candidate **smoke-test** questions.
   - **Map, don't mirror:** link the canonical doc and capture only the enduring fact — never duplicate a
     living doc into memory.
4. **Check each candidate against existing facts first** (`DECAY.md` §10) so a re-run doesn't duplicate.
   A harvested fact that **contradicts** an existing one → raise a `- [ ] Contradiction: …` Open Thread
   (`never-pick-a-winner`); a genuine **replacement** → supersede the old fact (`DECAY.md` §9). Never
   silently overwrite curated memory.
5. **Human-gated.** Summarize what you folded in, what you superseded/flagged, and what you skipped for
   budget — let the human review before it's treated as settled.
6. **Record it.** A harvest is a memory-relevant event → write a session log with a `## Memory References`
   section listing the fact ids you created / changed (it stays inside the normal after-session ritual).
7. **Stamp `last_harvest`** in `continuity.md` → Project State: `last_harvest: <today> | through <this
   session-file>` (create the field if absent — it sits with `last_review` / `last_invariant_check`). This
   is what step 2 reads next time, so each run scopes incrementally. The harvest **owns** `last_harvest`,
   the way the review owns `last_review`. Even a **no-op** run (nothing new to fold) stamps it — that
   records "docs were checked through here." Write it safely (`REVIEW.md` → Safety: never truncate-before-read).

## Notes

- **No-code, agent-run** — there is no script; the agent reads the docs and writes memory
  (`no-build-step-agent-run`). The runtime is the agent, on any vendor.
- **Additive + safe-to-repeat** — running it again should enrich, not duplicate (step 4) and never destroy
  curated memory. When scripting any memory edit, follow `REVIEW.md` → Safety (never truncate-before-read).
- **Pairs with the enable-time harvest** (`ENABLE.md` Step 4b): identical rules, different trigger —
  one-shot at enable vs. recurring on demand here.
