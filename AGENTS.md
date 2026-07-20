# Agent Instructions

This repository uses the **agent-memory** shared memory system.
It is configured for AI-assisted development with any major agent runtime.

## Two Memory Layers

This repo's `memory/` holds **project state, shared across all agents and committed
to git.** It is separate from any **personal, user-scoped memory your runtime keeps
outside the repo** (e.g. Claude Code's `~/.claude/`, which holds your individual
preferences). Project facts, decisions, and session logs go in this repo's
`memory/`; personal preferences stay in your runtime's own store.

## Before Every Session

Read these files before responding to anything:

1. `memory/instructions.md` — project context, rules, and conventions
2. `memory/continuity.md`   — current state, open threads, key decisions (+ Blueprint gaps)
3. `memory/vision.md`       — the target the work serves (the VBDI north star)
4. `memory/sessions/`       — scan the most recent 2–3 session logs

If a topic seems unfamiliar, grep `memory/archive/INDEX.md` (and follow a fact's
`origin` to its session) before saying you have no context — retrieval here is lexical
+ indexed by design (`DECAY.md` §11); facts fade to the archive but are never deleted.

## The cognitive loop (VBDI)

This repo runs a forward loop on top of the memory layer (`DECAY.md` §12):
**Current State (`continuity.md`) → Vision (`memory/vision.md`) → Blueprint (gap) →
Design → Implementation → Feedback (review) → repeat.** When you propose significant
work, tie it to a Blueprint gap (a `(blueprint)` Open Thread that `serves:` the Vision)
and to the Design it realizes — so intent is traceable and drift is detectable. Each
altitude transition (confirming the Vision, opening or closing a gap) is a **human
gate**: propose, then let the human approve. Never fabricate the Vision.

The Design altitude *may* keep an **optional** Architecture Decision Record log,
`docs/arch-decisions/ADR.md` — a human-facing governance ledger of durable architecture decisions
(see `.agent/schema.md`). It is read **on demand**, **not** part of the per-session read;
any `(ADR-NNNN)` tag on an invariant is a human pointer, not a cue to open it.
**If the log exists, keep it alive:** when you make a new durable architecture decision, or
supersede/invalidate a continuity fact carrying an `(ADR-NNNN)` tag, **propose** a matching
ledger update — add a newer ADR, mark the old one `Superseded`/`Deprecated` (never delete),
keep `formalizes:` ↔ `(ADR-NNNN)` in sync — and let the human approve (a Design-altitude change
is a human gate; `DECAY.md` §9, §12). That supersession is the one time you open the ledger.

## Skills

If an `agent-skills/` directory exists, it holds the project's **capabilities** — committed,
vendor-neutral `agent-skills/<name>/SKILL.md` files. **This is the runtime:** when a task
matches a skill's `description`, read and follow that `SKILL.md` (and any scripts it
references). The agent is the runtime — works on any vendor, no engine.

Per-vendor adapters (`.claude/skills/`, `.gemini/commands/`, `.cursor/rules/`, `.kiro/skills/`,
`.github/skills/`, `.agents/skills/`) are thin, gitignored, regenerated pointers — **never commit them** (only
`agent-skills/` is shared); the source of truth is always `agent-skills/<name>/SKILL.md`.

**Authoring, syncing, adopting, sanity-checking, or editing a tool-provided skill?** See **`SKILLS.md`**
(read on demand — it is *not* part of this per-session read). **Authoring a skill is a 3-step action —
write `agent-skills/<name>/SKILL.md`, run `sync skill adapters`, then reload your runtime if it loads
adapters at startup (e.g. GitHub Copilot CLI `/restart`); it is not done after step 1.** Skill work is
a deliberate, occasional action, never part of the session ritual. A skill whose frontmatter says `provenance: agent-memory-builtin`
is **tool-managed** (overwritten on upgrade) — don't edit it in place; fork it under a new name, or
upstream a genuine fix to the agent-memory project (`SKILLS.md` → "Tool-provided (system) skills").

## During the Session

- Treat `memory/continuity.md` as your working memory.
- Reference prior decisions before suggesting changes that might contradict them.
- Note any new facts, preferences, or decisions for post-session write.
- Track which fact **ids** you rely on, create, or pull back from the archive — you
  will list them in the session log's `## Memory References`. Do **not** edit fact
  metadata mid-session; the review ritual does the counting.

## After Every Session

A "session" is **one log-write** — the work since the last log, not necessarily a
whole conversation. A long, multi-task conversation may produce several logs; that's
expected (the decay math counts log files — `DECAY.md` §4).

1. **Create** `memory/sessions/YYYY-MM-DD-HHMMSS.md` using the UTC timestamp at
   **persist time** (when you write the file). **Always run `date -u +%Y-%m-%d-%H%M%S`
   to get the filename** — the `currentDate` injected into your context is date-only
   and produces a non-conforming `YYYY-MM-DD.md` name if used directly. Omit colons
   for cross-platform compatibility. Title line:
   `# Session (endZ)` — the persist-time UTC stamp (full ISO 8601 ms) is required; a
   start time is optional/best-effort, so don't fabricate one. Never append to
   another contributor's session file.
   Include a `## Memory References` section listing the fact ids you referenced,
   created (born `tier: working`), or reactivated. This is the event log the review
   ritual reads — see `DECAY.md`.
2. **Update** `memory/continuity.md`:
   - Set `last_session` to today's date and your agent name.
   - Keep **`status` a short current-state line — never a changelog.** Don't accrete
     per-version history onto it (that line is shared by every teammate; a long mutable
     line is a merge-conflict bomb). History belongs in the session logs / `CHANGELOG`,
     not `status`. One fact per line; see `.agent/schema.md` → "Concurrency &
     merge-friendliness" for the keep-both / take-later merge conventions.
   - Mark completed Open Threads `- [x]` and **leave them in place** — the review
     sweeps them once older than `archive_window`; don't archive them by hand.
   - Add new Open Threads surfaced during the session.
   - **Before recording a new fact, check it against existing ones** (`DECAY.md` §10):
     if it clearly replaces one, supersede that one (see below); if it genuinely
     conflicts, raise a `- [ ] Contradiction: …` Open Thread rather than keeping both.
   - Give any new fact a kebab `id` + footer: set `id`, `created`, `tier: working`
     (or `core` for an Architectural Invariant), `origin: <this session's file>`, and
     seed `last_used: today | uses: 1`. Don't hand-edit `uses`/`last_used`/`tier`
     afterward — the review owns them.
   - Update the substance of any fact that changed (not its usage metadata).
   - **Reversed a decision / a fact became false?** Add the successor (born
     `tier: working`, `supersedes: <old>`), mark the old fact `tier: superseded` +
     `superseded-by: <new>` (omit the link for pure invalidation), and record
     `Superseded: <old> → <new>` in `## Memory References`. This is a truth-state edit
     you own; the review archives it flagged "superseded" (`DECAY.md` §9).
3. **Review cadence.** If `sessions_since_last_review ≥ review_every`
   (`memory/decay-policy.md`), or `continuity.md` has grown past `continuity_max_facts`
   (decaying facts/threads) or `continuity_max_lines`, run the review ritual now — see
   `REVIEW.md`. `memory-lint` flags all three (`[review-overdue]` / `[continuity-bloat]`),
   so they don't depend on you remembering. (Also run on demand if the user says "review memory".)
4. Remind the user: `git add memory/ && git commit -m "session YYYY-MM-DD [agent]"`.
   **Commits are deliberate and human-initiated.** When you commit at the human's direction,
   **identify yourself** the same way you do in session logs — with a `Co-Authored-By:` trailer. But
   you **co-author the commit message *with* your harness**: many runtimes inject their *own*
   `Co-Authored-By` (often a **model-version** name like `Claude Opus 4.8`). So treat the message the
   harness will commit as the **base and reconcile it** — don't blindly append, or you get two lines
   for one collaborator. **Invariant: at most one `Co-Authored-By` per collaborator, matched on
   email** — `Claude Code`, `Claude Opus 4.8`, `Gemini CLI`, `Gemini 2.5` … are **one** collaborator
   at **one** address (`noreply@…`); the *name* varies, the email doesn't. Resolve deterministically:
   - **Harness injects a trailer you can't suppress** → that **is** your one trailer; add nothing
     more (accept its model-version name).
   - **You control the message** → emit **exactly one** trailer using your **stable agent name**
     (e.g. `Claude Code`, `Gemini CLI` — not a model version) and remove any harness-injected
     duplicate at the same email.
   - **Never emit both.** One line per collaborator, keyed on email.
   On a **squash-merge** the forge compounds this — each squashed commit's inline trailers pile up
   *and* the forge appends its own consolidated one after the `---------` line. Keep the canonical
   trailer **once** (in the PR-description footer; omit it from per-commit bodies), then trim inline
   repeats so exactly one line per collaborator survives. Stable identity keeps attribution as one
   collaborator across releases and vendors.
   - **Opening a pull request?** Lead the description with two short sections — **What** (the
     change) and **Why** (the intent it serves — the Blueprint gap, decision, or problem behind
     it; substantive intent, *not* a restatement of What) — each 1–2 short paragraphs, drawn from
     the session log(s) in the PR. Close it with the **same self-identifying `Co-Authored-By:`
     footer** you use on commits and session logs, so PR authorship is traceable across vendors
     too. A `.github/pull_request_template.md` seeds all of this; keep it advisory, never a gate.
     (The *why* is a first-class artifact throughout this protocol, so a PR is no exception.)

**After-session checklist** (the ritual is convention — run it each time):
- [ ] session log written — ran `date -u +%Y-%m-%d-%H%M%S` for the filename (not `currentDate`); includes `## Memory References`
- [ ] `continuity.md`: `last_session` set, threads checked, new facts have footers
- [ ] review run if cadence/size triggered (`REVIEW.md`)
- [ ] reminded the user to commit `memory/` (deliberate, human-initiated, with a self-identifying co-author trailer)
- [ ] if a PR was opened, its description leads with **What** / **Why** (drawn from the session log)

> **Lightweight mode — key the write to whether a *tracked* file changed (the *objective* test is the
> git diff, not any filesystem write — and never a "trivial" judgment; both AI and human misjudge "trivial").**
> - **Read-only session** (no tracked file changed — orientation, Q&A, exploration, **or a run whose
>   only writes are gitignored, regenerated artifacts**: `sync skill adapters`, `review-scratch/`
>   snapshots, the compiled lint artifact): **no session log** — nothing entered the repo, nothing to
>   commit, no event to record.
> - **A tracked file changed but produced no memory-relevant event** (no new/changed fact, no decision
>   worth recording, no Open Thread touched, no project-state change — e.g. a one-line fix, a typo):
>   write a **one-line "lite" session log** (persist-time filename + `**Agent:**` + a *lightweight*-marked
>   summary + `## Memory References` → `(none)`) and skip the rest (full template, fact-footers,
>   continuity edits; `last_session` is derivable from the newest session file). **Don't skip the log
>   just because it felt "trivial"** — a misjudged change that actually mattered must still be logged.
>   **One log per working *session*, not per commit:** if you already wrote a session log earlier in
>   *this* working session, a later **memory-neutral** commit should **enrich that existing log** (a
>   one-line "also: …" note) rather than spawn another near-duplicate lite log — a burst of commits in
>   one sitting is *one* session. This keeps the decay session-count honest (it counts log files) and
>   mirrors the post-commit hook's per-session windowing (v4.22.1).
> - **A memory-relevant event** (fact / decision / Open Thread / project-state change, or anything
>   touching Vision / Blueprint / invariant / supersession): the **full** ritual. (Distinct
>   memory-relevant work still gets its **own** log, so a multi-task conversation may still yield several —
>   the rule above only stops *trivial* follow-on commits from each minting a near-duplicate lite log.)
> The ledger stays continuous for anything that touched a *tracked* file; the review treats a lite log
> as a normal reference-free session, so usage is unaffected.

> **Reinforced, not just documented (v4.19.0).** This repo ships vendor-neutral triggers: a committed
> **`.githooks/post-commit`** (auto-stubs a session log when a commit does real work without one;
> activated via `git config core.hooksPath .githooks`) + a **CI floor**
> (`.github/workflows/agent-memory.yml`, zero per-user setup). Treat the session log as part of **done** —
> *a task that changed tracked files isn't finished until its log exists.* The triggers are **advisory**
> (never block) and **no-code** (git/CI run them in your env; the tool runs nothing). See
> **`.githooks/README.md`** for activation + per-vendor end-of-turn hook extras.
>
> **First session in a fresh clone? Self-initialize.** A clone has the gitignored skill adapters
> **absent** and the hook **unactivated** (git can't auto-run committed hooks on clone). If you see
> empty vendor adapter dirs or `git config core.hooksPath` is unset, run **`bash .githooks/init.sh`**
> once (regenerates adapters + activates the post-commit hook) — do this proactively, before other
> work. (CI runs server-side regardless.)

> **Long session? Keep state externalized so compaction is safe (v4.23.2).** Compaction (your tool's
> `/compact`, an auto-compact at a context-usage threshold, or a fresh session) summarizes the
> conversation and drops verbatim detail. The **objective** health signal is **context-window utilization —
> tokens used vs. the model's limit** — which your harness tracks and may auto-act on; wall-clock time and
> a "replies feel vague" sense are only proxies (the model can't reliably self-measure its own context, so
> don't gate on a felt "fog"). You usually **can't compact yourself**, but you control the one thing that
> makes compaction lossless:
> - **Write the session log + any `continuity.md` update at each natural seam** — a milestone landed, a
>   phase shift (explore → implement → verify), or before pivoting to an unrelated task — **before**
>   compaction, not after. Externalized state reloads as context next turn; whatever lives **only** in the
>   buffer is what a summary can lose (summaries keep the narrative, not the verbatim texture).
> - **At a seam with high utilization, suggest compacting** (or rely on the harness's auto-compact) rather
>   than carrying a full buffer into the next phase. **Never mid-task**, with hot, unwritten state.
> - **After any compaction, re-verify specifics against live files** rather than trusting the paraphrase.
> The session log *is* the seam marker: "when do I write the log?" and "when is it safe to compact?" share
> the same beat. This is *why* the memory layer lives in **files**, not the chat buffer.

## Multi-Agent Continuity

Check `last_session` in `continuity.md` and note the agent name recorded there.
If it is **not your own agent family** (e.g. Claude, Gemini, Copilot, Cursor),
read that day's session log in full before proceeding — the memory files are the
shared ground truth across all agents.

## Memory File Locations

```
memory/
  instructions.md     ← project context + agent rules    (edit rarely)
  continuity.md       ← live project state               (update every session)
  decay-policy.md     ← evolving-memory windows/triggers (tune as needed)
  sessions/           ← dated session logs (event log)   (append; never edit past logs)
  archive/            ← faded facts + swept threads       (cold storage; never deleted)
    INDEX.md          ← greppable index of archived facts
agent-skills/               ← cross-vendor capabilities          (committed; vendor-neutral)
  <name>/SKILL.md     ← one skill: name + when-to-use + procedure (the source of truth)
.agent/
  schema.md           ← file format reference
  version.md          ← which agent-memory version this repo is on
DECAY.md              ← evolving-memory rules (metadata, tiers, deterministic decay)
REVIEW.md             ← the review ritual (when/how to recompute + archive)
SKILLS.md             ← skills reference: authoring, sync, adopt, sanity (read on demand)
```
