# DECAY — Evolving Memory Reference

> The rules of the evolving-memory layer: the metadata each fact carries, the tier
> lifecycle, and the **deterministic** rules that move facts between tiers.
> `REVIEW.md` is the *ritual* that applies these rules; this file is the *reference*.
>
> **This doc is generic and ships into every enabled repo** (installed at the repo
> root by `ENABLE.md`), because the review ritual runs *inside* the repo during
> normal sessions — an agent there needs the rules locally. It is the same in every
> repo; it carries no project-specific content.
>
> **Design principle — no floating-point math.** Every decision below reduces to
> counting items in a list or comparing integers, so any agent (Claude, Gemini,
> Cursor, …) reaches the *same* result. There is deliberately no `strength` score.

---

## 1. Fact metadata

Every fact in `memory/continuity.md` carries an HTML-comment footer. Invisible
when rendered, readable and editable by any agent or human, diff-friendly.

```markdown
- POST-only for mutations, no PUT/PATCH (legacy decision, do not change)
  <!-- id: post-only-mutations | created: 2026-06-08 | last_used: 2026-06-12 | uses: 14 | tier: active -->
```

| Field | Set | Recomputed at review? |
|---|---|---|
| `id` | once at creation; kebab-case, unique within the file; never changes | no |
| `created` | once at creation (date the fact entered memory) | no |
| `last_used` | date of the most recent session that referenced the id | **yes** |
| `uses` | count of sessions that referenced the id | **yes** |
| `tier` | lifecycle bucket (see §3) | **yes** |
| `superseded-by` | *(optional)* id of the fact that replaced this one; set when it is superseded (§9) | no |
| `supersedes` | *(optional)* id of the fact this one replaced; set on the successor (§9) | no |
| `origin` | *(optional)* the session-log file where the fact was `Created` — its provenance (§11) | repairable |

There is no `strength` field. Importance is expressed structurally: `tier: core`,
or membership in the `## Architectural Invariants` section.

### What the agent sets at creation vs. what the review owns
At creation the agent writes only: `id`, `created: <today>`, `tier: working`, and
seeds `last_used: <today> | uses: 1` — the creating session names the fact under
`Created` in its `## Memory References`, so `1` is the honest first count. After
creation the agent **never hand-edits `uses`, `last_used`, or `tier`**; it records
references in session logs (§2) and the review recomputes those three (the "yes"
rows above). `id` and `created` are immutable. Ordinary facts are born `working`
(§3); only `## Architectural Invariants` facts are born `core`.

### Assigning an id
Lowercase, hyphenated, derived from the fact's gist (`webhook-fire-forget`,
`drizzle-over-prisma`). Unique within `continuity.md`. Once assigned it is
permanent — it is the handle that session logs use to reference the fact.

---

## 2. The event log — how usage is recorded

Usage is **not** hand-maintained on each fact. It is *derived* from session logs.
Every session log carries a `## Memory References` section:

```markdown
## Memory References
- Referenced: post-only-mutations, webhook-fire-forget
- Created: graphql-gateway-added (tier: working)
- Reactivated: drizzle-over-prisma
```

- **Referenced** — ids the session relied on or reinforced.
- **Created** — new facts added this session (born `tier: working`).
- **Reactivated** — ids pulled back from the archive.

So, for any id:
- `uses` = number of session logs whose `## Memory References` name it.
- `last_used` = the date of the latest such session log.

Both are recomputed during review (`REVIEW.md`), never typed by hand mid-session.

> **Session logs are immutable.** They are the source of truth for this projection.
> Never edit, renumber, or archive a past session log. `continuity.md` metadata is
> the *derived view*; the logs are the *ledger*.

---

## 3. Tiers

```
core              permanent. Human-set (§5). Never decays.
active            referenced within active_window sessions.
working           created within working_window sessions, not yet re-referenced. Probationary.
archive-candidate not referenced for > active_window but ≤ archive_window. Flagged, not yet moved.
archived          not referenced for > archive_window (faded from disuse). Moved to memory/archive/YYYY-QN.md.
superseded        no longer true — a decision reversed or a fact invalidated (§9). Terminal,
                  agent/human-set; archived flagged "superseded" (not "faded"); never reactivated.
```

Movement is **bidirectional**: an archived id named in a session's `Referenced` or
`Reactivated` list is pulled back to `active`. Nothing is ever deleted. The one
exception is `superseded` (§9) — it is *terminal*: being referenced never revives a
false fact; only a human can reverse a supersession.

---

## 4. The only arithmetic: counting session files

`sessions_since_last_used` = **the number of session files chronologically after
the one in `last_used`.** Session files are named `YYYY-MM-DD-HHMMSS.md` and sort
lexicographically = chronologically, so this is the length of a list, not a
formula: list `memory/sessions/`, find the file matching `last_used`, count what
comes after it. Every agent gets the same integer.

(If several sessions share a `last_used` date, count by file, not by date.)

---

## 5. Tier decision rules — apply in order, first match wins

Windows come from `memory/decay-policy.md` (integers, in sessions).

1. `tier: superseded` (or `superseded-by` is set) → **stays superseded** (§9). The
   review archives it flagged "superseded"; it never decays back, never reactivates.
2. `tier: core` → **stays core.** Never auto-demoted. (Human override.)
3. Under `## Architectural Invariants` → **pinned**, treated as core.
4. Unchecked Open Thread (`- [ ]`) → **pinned**, never decays (incomplete work). Its **pinned-ness**
   (being unchecked) is what protects it — **not** the tier label, which the tooling therefore leaves
   as-is (`memory-lint` won't flag a pinned thread's tier, `refresh-metadata` won't rewrite it; v4.26.1).
5. `created` ≤ `working_window` sessions ago AND `uses ≤ 1` → **working**.
6. `sessions_since_last_used ≤ active_window` → **active**.
7. `active_window < sessions_since_last_used ≤ archive_window` → **archive-candidate**.
8. `sessions_since_last_used > archive_window` → **archived** → move to archive/.

---

## 6. Never-decay set

Exempt from the decay steps (§5 rules 5–8) regardless of counts:
- `tier: core`
- everything under `## Architectural Invariants`
- unchecked Open Threads (`- [ ]`)
- `tier: superseded` — exempt from *decay*, but the review archives it promptly
  flagged "superseded" (§9): it leaves because it is false, not because it faded.

A *checked* Open Thread (`- [x]`) becomes eligible to be swept to the archive once
its completion is older than `archive_window` sessions (see `REVIEW.md`).

> **Never-decay ≠ never-checked.** `core` facts and Architectural Invariants can quietly
> become *wrong* when circumstances change. The review periodically prompts a human to
> re-confirm them (or supersede the false ones, §9) — see `verify_invariants_every` in
> `decay-policy.md` and `REVIEW.md` routine step 6.

---

## 7. Auto-core (default OFF)

`core` is **human-set only** by default — the system never silently makes a fact
permanent. If `auto-core` is enabled in `decay-policy.md`, a fact may be promoted
to `core` only when `uses ≥ core_min_uses` **and** it has stayed `active` across
`core_min_reviews` consecutive reviews — and even then, surface it for the user to
confirm rather than promoting silently.

---

## 8. Manual override always wins

Any field a human edits by hand — especially `tier:` — is authoritative. Review
must not overwrite a hand-set `tier: core`, `tier: superseded`, or a hand-set `id`.
Prefer archiving over deleting, but if a human deletes a fact outright, respect it.

---

## 9. Supersession — when a fact becomes *false*

Decay handles facts that fall out of *use*. Supersession handles facts that become
*untrue* — a decision reversed, a convention changed, a dependency dropped. A false
fact is worse than a stale one, so it is handled **immediately**, not by waiting for
a window.

When a fact is reversed or invalidated, in the same session:

1. **Record the event** in the session log's `## Memory References`:
   `- Superseded: <old-id> → <new-id>` (replacement) or
   `- Superseded: <old-id> (invalidated)` (no replacement). This is the ledger entry.
2. **Add the successor** (if any) as a normal new fact, born `tier: working`, with
   `supersedes: <old-id>` in its footer.
3. **Mark the old fact immediately.** A truth-state change is a manual edit the agent
   *owns* (unlike `uses`/`last_used`/decay-`tier`, which the review owns): set its
   footer to `tier: superseded` and add `superseded-by: <new-id>` (omit for pure
   invalidation). Leave it in place, visibly marked, until the next review — a reader
   sees "X (superseded by Y)" in the meantime.

At the next review the superseded fact is **archived flagged "superseded"** (distinct
from "faded"), its `superseded-by`/`supersedes` links preserved in the archive and
`INDEX.md`, and a `Superseded: N` line added to the review summary. A superseded fact
is **terminal**: it never decays back and is never reactivated by a reference (it is
false, not dormant). Only a human can reverse a supersession by hand-editing it back.

> This is the markdown-native analogue of bi-temporal `valid_at`/`expired_at`/
> `invalid_at`: `created` is "valid from"; supersession is "invalid from now,
> replaced by <id>" — recorded as an event and a terminal tier, with no scoring.

---

## 10. Write-time contradiction check

A fact is cheapest to validate the moment it's written. When you add or materially
rewrite a fact, first scan the facts it might conflict with — `core` / Architectural
Invariants and the active Key Decisions / Conventions in the same area — and act on any
contradiction instead of silently letting both coexist:

- **Clear replacement** → **supersede** (§9): the new fact wins, the old is marked
  `superseded`. (E.g. "versioning now via the `Accept` header" replaces "version in the
  URI path".)
- **Genuine conflict, unclear winner** → **don't pick a winner.** Keep the existing
  fact and raise an Open Thread for a human:
  `- [ ] Contradiction: <new fact> conflicts with <id> — resolve (supersede one, or reconcile)`.
- **Contradicts a `core` invariant** → surface it prominently and stop; a new fact must
  not silently override an invariant. Either the invariant is being deliberately
  superseded (a human-confirmed act, §9) or the new fact is wrong.
- **Drifts from a higher altitude (VBDI, §12)** → the same check runs *up the chain*: an
  Implementation that doesn't serve its Design, a Design that doesn't serve its Blueprint
  gap, or a Blueprint gap that doesn't serve the Vision is drift — surface it as a
  `- [ ] Drift: <item> doesn't serve <id>` Open Thread.

This is "pre-consolidation validation," scaled to markdown — a cheap read-and-compare at
write time, with the review as a periodic backstop (`REVIEW.md`). It is the same
"surface contradictions, never pick a winner" rule the migration path already uses
(`MIGRATE.md`), now applied to normal sessions too.

---

## 11. Provenance & retrieval

**Provenance.** The event log already records where each fact came from — its `Created`
entry in some session's `## Memory References`. Surface that on the fact with an optional
`origin: <session-file>` footer field, set at creation (the agent knows which session it's
writing). It makes provenance one hop: read a fact → open its `origin` session → get the
full "why." A review can repair/backfill `origin` deterministically — the earliest session
whose `## Memory References` names the id under `Created`. Like `created`, it never changes
once set. (This is also the cheap defence against memory poisoning: every fact is traceable
to a session in the immutable ledger.)

**Retrieval at scale.** Retrieval here is deliberately **lexical + indexed**, not
vector/semantic — that keeps the layer no-code and human-auditable. As memory grows, find
things by: (1) grep `continuity.md`; (2) grep `archive/INDEX.md` (every archived fact, one
greppable line); (3) follow a fact's `origin` (or an id's `Created` event) to its session;
(4) optionally keep `sessions/INDEX.md` for one-line-per-session orientation. This is
bounded by project scale — one project's memory stays grep-sized. Full vector/semantic
retrieval is intentionally out of scope: if a repo outgrows grep, that's a signal to split
or archive, not to bolt on an index server.

---

## 12. The forward layer (VBDI) — Current State → Vision → Blueprint → Design → Implementation → Feedback

Everything above is *backward*-looking — it keeps memory faithful to what happened. The
**VBDI cognitive loop** is the *forward* complement: it keeps delivery faithful to what was
*intended*. Full design: `docs/DESIGN-vbdi-lifecycle.md`. The rule-level essentials the
memory layer enforces:

- **The primitives.** *Current State* = `continuity.md` (read at session start). *Vision* =
  `memory/vision.md` (the target; `core`, invariant-verified). *Blueprint* = typed
  `(blueprint)` Open Threads = the Vision↔Current-State gap. *Design* = Key Decisions /
  Architectural Invariants (and, **optionally**, a human-facing `docs/arch-decisions/ADR.md` decision log
  — Architecture Decision Records, read on demand, never in the per-session path; its
  supersede/deprecate-never-delete lifecycle mirrors §9, and — once the log exists — is **kept in
  sync** with fact supersession: superseding an `(ADR-NNNN)`-tagged fact, or making a new durable
  decision, prompts a human-gated ledger update). *Implementation* = code/commits,
  traced in sessions. *Feedback* = the review ritual. **Only Vision + Blueprint are new; the
  rest is the existing layer, named.**
- **The trace is the determinism.** Implementation → Design → Blueprint (`serves: <gap>`)
  → Vision (`serves: <vision-id>`), linked by `id`. A missing or broken link is drift
  (§10, grep-detectable). The *trace and the gates* are deterministic; the *content* — the
  vision, the design ideas — is the human–AI partnership. No scoring.
- **Human gates.** Each altitude transition (confirm the Vision; open/close a Blueprint
  gap) is a human gate — an Open Thread the human checks off, not a phase review. The agent
  proposes; the human approves.
- **The loop is the cadence, not new ceremony.** It rides the existing session → review
  rhythm. Greenfield → brownfield: each delivered increment becomes the next Current State.
- **Never fabricate the Vision** — it is the human's to set (like User Preferences).
  Enable/upgrade bootstrap a ⚠️ DRAFT stub and gate it; until it's confirmed,
  drift-detection stays advisory.
- **Process-neutral — survives whatever the target chooses.** This loop is the
  *lightweight default*; it neither requires nor forbids heavier process. A target repo's
  owner may layer SDLC / scrum on top (sprints, standups, roles) — that is their call.
  Keep that ceremony, and any **scoring** (velocity, story points, estimates), in the
  *target's own space* (its tracker/docs), **never in `memory/`** — the substrate stays
  determinism-pure (no floating-point, see the design principle up top). The loop is
  cadence-agnostic: a "sprint boundary" is just "run a review," and extra tags (e.g.
  `(sprint)`) on Blueprint threads coexist with the primitives. The tool stays lightweight
  regardless of how heavy a process the target chooses to run on top.
