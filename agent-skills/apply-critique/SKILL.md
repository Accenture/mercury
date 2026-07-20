---
name: apply-critique
description: Apply a fresh-context reviewer's critique to the current work through a bounded, validated, human-gated loop. Use after a second-opinion review returns a critique — parse it, plan a few scoped fixes, run the build/tests and memory-lint, then summarize what was applied versus rejected and why.
provenance: agent-memory-builtin
---

# apply-critique

> ⚠️ **Tool-managed skill provided by agent-memory** (`provenance: agent-memory-builtin`). Don't edit it
> in place — it is overwritten on upgrade. To change behavior: **fork** it under a new skill name, or
> **upstream** a genuine fix to the agent-memory project (file an issue in its repo, or bring it to the
> tool maintainer) for back-port + validation. See `SKILLS.md` → "Tool-provided (system) skills".

Consumes the critique a clean-memory reviewer hands back from a **`second-opinion`** review
and turns it into a **bounded, validated** set of changes — never an open autonomous loop.
The critique is **advisory**: it is one input, gated by deterministic checks and a human
decision.

## When to use

After a fresh-context reviewer (a clean session or a different vendor) returns a critique of
a snapshot you generated with `second-opinion`. The human points you at the critique (a
pasted block or a `review-scratch/critique-<UTC>.md` file).

## What to do

1. **Parse the critique** into: critical issues, gaps, design feedback, execution guidance,
   risks, confidence. (The expected shape is below; tolerate a free-form critique by mapping
   it onto these buckets.)
2. **Plan a small, focused set of actions** — only a handful of scoped changes that address
   the critical issues and execution guidance. A bounded repair, not a rewrite. State the
   plan before editing.
3. **Apply** the scoped changes.
4. **Validate deterministically:**
   - the target's **build / tests** for code changes;
   - **`memory-lint`** (`python3 agent-skills/memory-lint/scripts/memory-lint.py`) if any
     `memory/` file changed.
   - **If neither gate applies** (e.g. a docs-only change in a repo with no test suite and no
     `memory/` edit), **say so explicitly** in the summary — the change then rests on judgment
     alone, which the human must know. This is exactly the failure mode the deterministic gate
     exists to prevent, so don't let "no gate fired" pass silently.
   A failing check blocks the change — fix or revert; do not paper over it.
5. **Summarize applied vs. rejected** recommendations *and why.* A recommendation you
   disagree with is fine to reject — say so explicitly. A genuine conflict you can't resolve
   becomes a `- [ ] Contradiction:` Open Thread (`never-pick-a-winner`), not a silent
   overwrite. **The human gates the result.**
6. **Record it.** A critique→repair cycle is a memory-relevant event → write a normal
   session log with a `## Memory References` section (it stays inside the existing ritual,
   not beside it).

## Critique template (what the reviewer should return)

```md
# Fresh-Context Critique

## Summary
## Critical Issues
## Gaps
## Design Feedback
## Execution Guidance
1.
2.
## Validation Strategy
## Risks
## Confidence
```

## Notes

- **Bounded, not autonomous** — a small number of scoped actions per cycle; re-snapshot and
  re-review if more is needed, rather than looping unattended.
- **Deterministic gate over LLM judgment** — the reviewer (and you) can miscount or
  misjudge; build/tests and `memory-lint` are the objective backstop. This is the lesson the
  memory layer already learned the hard way.
- **No-code, agent-run** — you apply changes and run checks at the human's direction; the
  tool itself runs nothing (`no-build-step-agent-run`).
