---
name: refresh-metadata
description: Recompute continuity.md fact footers (last_used, uses, tier) from the session reference log — REVIEW.md steps 2–3, done deterministically. Use during a review, or whenever memory-lint reports [stale-metadata], instead of updating footers by hand. Pure arithmetic from the references; it never decides what to archive.
provenance: agent-memory-builtin
---

> ⚠️ **Tool-managed skill provided by agent-memory** (`provenance: agent-memory-builtin`). Don't edit it
> in place — fork under a new name, or upstream a fix to the agent-memory project (see `SKILLS.md`).

This skill performs **REVIEW.md steps 2–3 (apply events + re-tier)** as a **runnable script**. For every
fact in `continuity.md` it recomputes `last_used`, `uses`, and `tier` from the `## Memory References` across
`memory/sessions/`, and writes the footers back. This is the **"full rebuild" path** `REVIEW.md` already
calls *"deterministic and reproducible by any agent"* — pure arithmetic, no judgment — so it's safe to
mechanize. **Agents routinely skip this pass** (they archive faded facts but don't re-tier the ones that
stay), leaving stale footers; a cross-vendor review proved it. This closes that gap.

## What it does NOT do

- It does **not** decide what to **archive** — that's `archive-fact` plus the agent's judgment
  (`never-pick-a-winner`). It clamps tier at `archive-candidate`; a fact still in continuity is never
  marked `archived` (that tier means *moved*).
- It does **not** touch `core` or `superseded` facts (human-set / terminal), or facts with **no** reference
  in any session log (legacy — can't recompute, so preserved), and it never **adds** missing fields.
- For a **pinned `- [ ]` open thread** it refreshes the factual fields (`uses` / `last_used`) but **leaves
  the tier label as-is** (v4.26.1) — pinned-ness protects an open thread, not its tier, so the tool doesn't
  opine on it (a `working`-tagged open thread is fine).

So the division of labor across the three memory tools mirrors the meaning/mechanics split:
`memory-lint` **verifies** (read-only) · `refresh-metadata` **re-tiers** (arithmetic) · `archive-fact`
**moves** (the decided archival). Only *which facts to archive* needs the agent.

## How to run

From the repo root, whichever runtime the machine has (output is byte-identical):

```sh
python3 agent-skills/refresh-metadata/scripts/refresh-metadata.py [--dry-run]
# or
node    agent-skills/refresh-metadata/scripts/refresh-metadata.mjs [--dry-run]
```

- `--dry-run` — print the footers that *would* change (tier / uses), change nothing. **Preview first.**

It reads `continuity.md` into memory and writes once (truncate-before-read is impossible). For each fact:
`uses` = number of sessions that reference it; `last_used` = the latest such session's date; `tier` =
DECAY.md §5 applied to `sessions_since_last_used` (clamped at `archive-candidate`). Idempotent — a second
run reports "nothing to refresh."

**Run `memory-lint` afterward** — the `[stale-metadata]` advisories should clear. Facts that come out
`archive-candidate` and are past `archive_window` will still show `[overdue]`; archive those with
`archive-fact` (the move it can't make for you).

## Notes
- **No runtime? Don't hand-edit 30 footers** (error-prone, and the truncation trap lurks). Install Python or
  Node, or do the review's metadata pass carefully by hand per `REVIEW.md` (read-into-var, never truncate-first).
- Two runtimes at output parity, with mirror tests (`test_refresh_metadata.py` / `.mjs`).
