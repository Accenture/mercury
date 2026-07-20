---
name: second-opinion
description: Generate a fresh-context review snapshot of the current task for a clean-memory AI reviewer (any vendor or a clean session). Use at a milestone that looks complete, or when blocked, uncertain, or facing a risky change, to get an independent second opinion before proceeding; shows a security advisory before exporting any state.
provenance: agent-memory-builtin
---

# second-opinion

> ⚠️ **Tool-managed skill provided by agent-memory** (`provenance: agent-memory-builtin`). Don't edit it
> in place — it is overwritten on upgrade. To change behavior: **fork** it under a new skill name, or
> **upstream** a genuine fix to the agent-memory project (file an issue in its repo, or bring it to the
> tool maintainer) for back-port + validation. See `SKILLS.md` → "Tool-provided (system) skills".

A deliberate **fresh-context review** ritual. A long session accumulates assumptions and
self-trust — the agent that built a solution over-trusts its own trajectory. This skill
packages a compact snapshot of the current task so a reviewer with **clean memory** (a fresh
session, or a different vendor) can challenge it. The reviewer's value is precisely that it
did **not** live the session.

Pairs with **`apply-critique`**, which consumes the critique the reviewer hands back.

## When to use

- **Milestone mode** — the work *looks* complete and you're about to close it. This is the
  highest-value case: the in-session agent is least likely to challenge itself exactly when
  it feels done.
- **Reactive mode** — you're blocked, uncertain, the logic seems inconsistent, several
  designs compete, or the change is high-stakes.

Invoked by the human, on demand. It never runs during ordinary work.

## What to do

1. **Determine the trigger** from how the human invoked you — `milestone` vs. `reactive`.
   Note it in the snapshot's Exchange Intent.

2. **Show the security advisory and wait for explicit acknowledgment.** Packaging task
   state for *another* AI system is a trust-boundary event, even when the reviewer is
   internal. Print this verbatim and **stop until the human types `acknowledge` (or
   cancels)** — do not generate the snapshot first:

   ```
   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   Fresh-Context Review — Snapshot Advisory
   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   You are about to generate a work-in-progress snapshot for review by another
   AI system (a clean session or a different vendor). Before you share it, ensure:

   - no client secrets, credentials, or tokens are included;
   - no personally identifiable information (PII) is exposed;
   - the content is appropriate to share beyond this project's perimeter.

   You may prefer a reviewer inside your organization's security boundary. Even
   then, review before sharing — think of it as handing context to a trusted
   collaborator. Power comes with responsibility.

   Type "acknowledge" to proceed, or cancel.
   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   ```

3. **Build the snapshot — derived, not duplicated.** Read `memory/continuity.md` and the
   last 2–3 session logs (reach further back if the milestone spans more), then distill
   **only the decision-relevant deltas** into the
   template below: objective, recent actions, state changes, current state, assumptions,
   open questions, next actions, and verification evidence. **It is not a memory dump, and
   it is never a parallel committed state file** — `continuity.md` + the session logs remain
   the single source of truth. For `milestone` mode, fill the Milestone Check; for
   `reactive`, state where you're stuck.

   **Make it stand alone.** A reviewer with no repo access (e.g. a web-based chat AI — the
   common case) sees *only* what's in the snapshot. So put the decision-relevant substance
   *in the snapshot itself* — paraphrase the key change, don't merely cite a file path that
   won't resolve for them. For anything the reviewer truly needs to see in full (a tricky
   file, a diff), list it under **Attach to reviewer** (in the template) so the human knows
   exactly what to bring.

4. **Write it to gitignored scratch.** Save to `review-scratch/snapshot-<UTC>.md` (timestamp
   from `date -u +%Y-%m-%d-%H%M%S`). If `review-scratch/` is absent, create it and write the
   personal-files README (text in *Notes* below). This folder is gitignored and per-machine
   — the files are the human's, and sharing one is the human's conscious decision.

5. **Hand off — match the reviewer's access.** Tell the human to give the snapshot to a
   clean-memory reviewer and ask it to **challenge** the work (not rubber-stamp it):
   - milestone → "Is this actually complete? What's the highest-risk area? What's missing
     or untested?"
   - reactive → "What are the critical issues, and what's the smallest path forward?"

   There are two reviewer types — deliver the context accordingly:
   - **Repo-access reviewer** (a CLI agent / another vendor pointed at this repo): the
     snapshot's cited paths resolve, so it can read them directly.
   - **Repo-less reviewer** (a web-based chat AI — the common case): cited paths **won't**
     resolve. The human pastes the snapshot **and attaches** whatever you listed under
     *Attach to reviewer*. This is why the snapshot must stand on its own (step 3) and name
     its attachments — otherwise the human has to guess what to bring.

   The reviewer returns a critique (the shape `apply-critique` expects). Then run
   **`apply-critique`** on it.

## Snapshot template

```md
# Fresh-Context Review Snapshot

## Exchange Intent
- Trigger: milestone | blocked | uncertain | risk
- Request to reviewer: <what you want challenged>

## Milestone Check   (milestone mode only)
- What's considered complete:
- Why complete:
- Highest-risk areas:
- Confidence before review:

## Objective
- Goal:

## Recent Actions (deltas)
- 

## State Changes
- Files / decisions changed:

## Current State
- Status:

## Assumptions
- 

## Open Questions
- 

## Next Actions
- 

## Verification
- Build / tests:
- Memory integrity (memory-lint, if memory touched):
- Notes:

## Attach to reviewer (if it can't read the repo)
- <files / excerpts / diffs a repo-less (e.g. web) reviewer must be given — leave empty if the
  snapshot above is fully self-contained, or if the reviewer can read the repo directly>
```

## Notes — scope and philosophy

- **The reviewer is a hypothesis generator, not an authority.** A fresh reviewer can be
  confidently wrong — it lacks the session context (this repo lived exactly that: a
  clean-context reviewer once over-archived still-referenced facts). So critique is
  **advisory**; `apply-critique` gates it with deterministic checks (build/tests,
  `memory-lint`) and a human decision before anything is applied.
- **Same-vendor clean session vs. a different vendor — know which one you have.** A fresh
  session (or a spawned clean-context subagent) of the *same* model tests the **mechanism**:
  does the snapshot stand alone, does `apply-critique` parse and apply cleanly, does the gate
  fire. That is genuinely useful and the common case. But it shares the author's model priors,
  training data, and stylistic blind spots, so it is **not** a full test of *epistemic
  diversity*. For a high-stakes milestone, prefer a **different vendor** (the strongest "didn't
  live the session" signal) — the snapshot is built to stand alone precisely so you can hand it
  to one.
- **No-code, agent-run** — the snapshot is markdown you write; the tool runs nothing on its
  own (`no-build-step-agent-run`).
- **No automatic agent-to-agent transport** — *the human carries the snapshot.* This is
  low-friction, human-mediated review by design, not an orchestration runtime.
- **Personal-files README** to write into `review-scratch/README.md` when creating the folder:

  ```md
  # review-scratch — personal, uncommitted

  Snapshots and critiques from the `second-opinion` / `apply-critique` skills land here.
  This folder is **gitignored** — these are *your* personal working files, not shared
  project memory. Sharing a snapshot or critique with another AI system is **your conscious
  decision**: review it for secrets, credentials, tokens, and PII first (the `second-opinion`
  advisory prompts you). Delete files here whenever you like — nothing depends on them.
  ```
