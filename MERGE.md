# MERGE — Resolving conflicts in the shared memory layer

> How to resolve a git **merge/rebase conflict** inside `memory/` — especially
> `continuity.md`, the one file every teammate edits. **Read this on demand**, when git
> reports a conflict in `memory/`; it is **not** part of the per-session read path.

## Why this exists

`memory/` is committed and shared, so concurrent teammates — on any vendor — will sometimes
collide. Session logs almost never conflict (timestamped filenames); `continuity.md` can.
The hard rule comes straight from the **`never-pick-a-winner`** invariant:

> **An AI must never silently choose a winner between two conflicting memory facts.**

Free-editing the raw conflict markers to "just resolve it" *is* that failure — the model
quietly drops one side and the loss is invisible. So resolution here is **structured,
validated, and human-gated**, not free-text surgery:

```
git conflict in memory/
   ↓  (1) classify each conflicted hunk
   ↓  (2) resolve by tier
   ↓  (3) memory-lint  — deterministic gate: no markers left, file parses, decay intact
   ↓  (4) human approves the merge commit
done
```

Never skip (3) or (4). (You still delete the `<<<<<<<`/`=======`/`>>>>>>>` lines — the rule
isn't "don't touch the text," it's "don't silently pick a winner.")

## (1) Classify the hunk

Look at what actually diverged between the two sides:

- **Additive** — both sides *added* different facts/bullets to an append-only section
  (`## Open Threads`, `## Key Decisions`, `## Conventions`, `## What's Been Built`). The
  commonest case.
- **Scalar** — both sides bumped the same single-value field (`last_session`, `last_review`,
  `last_invariant_check`, the `status` version token).
- **Semantic clash** — both sides changed the **same** fact's substance; or one checked a
  thread `[ ]`→`[x]` while the other edited it; or a fact was superseded on one side and
  edited on the other. The rare case — and the **only** one needing judgment.

## (2) Resolve by tier

### Tier 1 — mechanical (deterministic; no AI judgment)

- **Additive → UNION. Keep BOTH sides' additions.** They are independent facts; dropping
  either loses information. Order doesn't matter — the review re-sorts and decays. Each fact
  keeps its own `id` + footer.
- **Scalar → take the LATER value** (later date / higher semver). `last_session` is also
  derivable from the newest `sessions/` filename, so never block on it.

No "propose a repair" step — apply the rule.

### Tier 2 — semantic clash (NEVER pick a winner — structure it, hand to the human)

You may **not** decide which version is "right." Preserve intent as structured facts instead:

- **Both edited the same fact's substance** → keep **both** versions as distinct facts (give
  the incoming one a fresh `id`) and raise a Contradiction (the same write-time mechanism,
  `DECAY.md` §10 / `.agent/schema.md`):
  `- [ ] Contradiction: <id-A> vs <id-B> — <one line on what diverged>; human to reconcile`
  Both survive until the human reconciles.
- **Supersession is the human's call, never yours.** Do **not** judge that one side "genuinely
  supersedes" the other — that judgment is exactly where a disguised winner-pick hides (you talk
  yourself into your preferred side being the "real" one). A semantic clash is **always** resolved
  as **keep-both + a Contradiction thread**, *unless the human explicitly instructs* a supersession —
  in which case record it: successor born `tier: working` with `supersedes: <old>`; mark the old
  `superseded` + `superseded-by: <new>` (`DECAY.md` §9). Absent that explicit instruction, keep both.
- **`[ ]`→`[x]` race** → this is a *mechanical* state transition, not a substance clash: keep it
  **checked `[x]`** if either side completed it (completion wins over incomplete; the review later
  sweeps it). If the bodies *also* diverged, that's a substance clash — handle as Contradiction above.
- Preserve **provenance** — keep each fact's `origin`; don't rewrite footers to erase where a
  fact came from.

When in doubt, prefer **keep-both + a Contradiction thread** over choosing. Choosing is the one
thing you may not do.

## (3) Validate — `memory-lint` is the gate

Run `memory-lint` (any runtime). It deterministically confirms:

- **no leftover conflict marker survived** (check 7 — `<<<<<<<` / `>>>>>>>` / diff3 `|||||||`);
- no `id` lives in both `continuity.md` and the archive; nothing over-archived; supersession
  links resolve; the file still parses (footers, version manifest).

Fix every ERROR before proceeding. `memory-lint` does the counting; you did the meaning.

## (4) Human approves

Conflict resolution is a **human-gated** act — like supersession and the review's archival. Show
the human what you **kept**, what you **unioned**, and every **Contradiction / supersession** you
raised, then let them approve the merge commit. **Never auto-commit a conflict resolution.**

---

See `.agent/schema.md` → **"Concurrency & merge-friendliness"** for the authoring conventions that
keep conflicts rare and mechanical in the first place (one fact per line, append-only sections);
`DECAY.md` §9–10 for supersession / contradiction; `REVIEW.md` for the periodic sweep that tidies up
afterward.
