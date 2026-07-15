# SKILLS — Portable Capabilities (cross-vendor)

> How skills work in an agent-memory repo: **author once in `agent-skills/`, run on any vendor.**
> **Read this on demand** — only when authoring, syncing, adopting, or sanity-checking a skill.
> It is deliberately **not** part of the per-session read (that's `AGENTS.md`). Like `DECAY.md`
> / `REVIEW.md`, it ships into every enabled repo so a target's own agent — any vendor — can
> act on its own.
>
> **Why on-demand:** skill creation is a *conscious, occasional* developer action. The
> per-session path does not police skills; alignment is explicit (the operations below).

---

## What a skill is

A committed, vendor-neutral **`agent-skills/<name>/SKILL.md`**: frontmatter `name` + a
`description` (the *when-to-use* trigger), then the procedure, plus optional helper scripts
under `agent-skills/<name>/scripts/`.

The **runtime is the agent itself** — when a task matches a skill's `description`, it reads and
follows that `SKILL.md`. That baseline lives in `AGENTS.md` and works on any vendor with no
engine. `agent-skills/` is **committed** (it travels with the repo); per-vendor **adapters**
are **thin, gitignored, regenerated pointers** for native auto-trigger — the neutral skill is
always the source of truth.

## Authoring a skill

Create **`agent-skills/<name>/SKILL.md`** — **never** a vendor folder (those are gitignored,
regenerated pointers, and won't be shared). Keep the `description` a **single line, quote-free,
and concise**: a compact, trigger-phrase-rich summary (~1–2 sentences; it's matched within a
small discovery budget, so avoid long abstract paragraphs). Put helper scripts under
`agent-skills/<name>/scripts/`.

**Authoring is not finished when you save the file — `sync` and reload are part of it. Do all three:**
1. Write `agent-skills/<name>/SKILL.md` (the neutral source of truth).
2. **Run `sync skill adapters`** — the runnable `sync-adapters` script (`node …/sync-adapters.mjs` or
   `python3 …/sync-adapters.py`; see Operations below). Until you do, *no vendor sees the skill* — it
   lives only in the neutral layer. Do this in the **same turn** as authoring; don't wait to be asked.
3. **Reload your runtime if it loads adapters at startup.** GitHub Copilot CLI parses `.github/skills/`
   at init, so the new `/<name>` won't be live until you **`/restart`** (or trigger a skills rescan).
   Claude / Cursor / Kiro pick up a new description-matched skill without a restart.

## Tool-provided (system) skills

Some skills are **provided by agent-memory itself** — the built-ins installed into every enabled repo
(`memory-lint`, `second-opinion`, `apply-critique`, `sync-adapters`). Each is marked in its
`agent-skills/<name>/SKILL.md` frontmatter:

```
provenance: agent-memory-builtin
```

These are **tool-managed copies**: an enable/upgrade re-copies (overwrites) them, so a local edit is
**not durable** — the next upgrade discards it, and the maintainer never learns of it.

**Before editing a skill, check its frontmatter.** If it carries `provenance: agent-memory-builtin`, do
**not** edit it in place — advise the human and pick one:

- **Local variant** → **fork** it under a *new* skill name (`agent-skills/<your-name>/`), which the tool
  never overwrites. Edit the fork freely.
- **Genuine fix or improvement** → **upstream it** so everyone gets it and it survives upgrades. Give the
  human a clear advisory: *this is an agent-memory-provided skill; the change should go back to the
  agent-memory project for back-port + validation.* **In production**, file an issue in the agent-memory
  repository (describe the change; attach the diff). **Pre-release**, bring it to the tool maintainer —
  an advisory message is the best-effort hand-off until there's an official release / issue tracker.
  (Keep the pointer generic — "the agent-memory project / its maintainer" — not a hard-coded URL, which
  would go stale.)

Why it matters: a target's AI that silently edits a built-in **strands** the change (overwritten on the
next upgrade) — exactly how a real `memory-lint` fix nearly got lost. The marker lets any vendor's agent
recognize a system skill and route the change correctly. At **upgrade** time, `ENABLE.md` §5i's
warn-before-overwrite is the backstop: it diffs the installed built-in and surfaces the same advisory.

## Adapter recipe

The adapter `description` **mirrors the neutral skill's verbatim** (never abbreviate — that
drifts). For each `agent-skills/<name>/SKILL.md` (using its `name` + `description`):

- **Claude Code** → `.claude/skills/<name>/SKILL.md`:
  ```
  ---
  name: <name>
  description: <description>
  ---
  Maintained vendor-neutrally. Read and follow `agent-skills/<name>/SKILL.md` (repo root)
  and any scripts it references.
  ```
- **Gemini CLI** → `.gemini/commands/<name>.toml` (a **slash command** — invoked explicitly as
  `/<name>`; Gemini does **not** auto-match commands against natural language, so a phrase like
  "run <name>" routes through the `AGENTS.md` baseline instead — which reads the *same* neutral
  skill, so the result is identical):
  ```
  description = "<description>"
  prompt = "Read and follow the skill at agent-skills/<name>/SKILL.md (repo root), including any scripts it references, then carry out: {{args}}"
  ```
- **Cursor** → `.cursor/rules/<name>.mdc` (the "agent-requested" type — description-matched,
  so `globs` is empty and `alwaysApply` is false):
  ```
  ---
  description: <description>
  globs:
  alwaysApply: false
  ---
  When this applies, read and follow `agent-skills/<name>/SKILL.md` (repo root) and any
  scripts it references.
  ```
- **Kiro** → `.kiro/skills/<name>/SKILL.md` (Kiro follows the open Agent Skills standard, so
  this is the **same shape as the Claude adapter** — workspace skills live under `.kiro/skills/`):
  ```
  ---
  name: <name>
  description: <description>
  ---
  Maintained vendor-neutrally. Read and follow `agent-skills/<name>/SKILL.md` (repo root)
  and any scripts it references.
  ```
- **GitHub Copilot CLI** → `.github/skills/<name>/SKILL.md` (Copilot CLI follows the open Agent
  Skills standard, so this is the **same shape as the Claude / Kiro adapter**; `.github/skills/`
  is Copilot's canonical project-skill location. Copilot CLI *also* reads `.claude/skills/`, but a
  dedicated `.github/skills/` adapter is unambiguous and survives even if that compatibility path
  changes — note `name` must be lowercase-with-hyphens, which our skill names already are):
  ```
  ---
  name: <name>
  description: <description>
  ---
  Maintained vendor-neutrally. Read and follow `agent-skills/<name>/SKILL.md` (repo root)
  and any scripts it references.
  ```
- **Google Antigravity (`agy`)** → `.agents/skills/<name>/SKILL.md` (Antigravity is the **Gemini CLI
  successor** and merged custom commands into the open Agent Skills standard, so this is the **same shape
  as the Claude / Kiro / Copilot adapter**. It reads workspace skills from `.agents/skills/` and **does
  not** read the old `.gemini/commands/*.toml` — without this adapter, `agy` reports `/<name>` as *not
  found* even though the Gemini TOML adapter exists):
  ```
  ---
  name: <name>
  description: <description>
  ---
  Maintained vendor-neutrally. Read and follow `agent-skills/<name>/SKILL.md` (repo root)
  and any scripts it references.
  ```

**Trigger semantics differ per vendor — set expectations accordingly.** Claude / Cursor / Kiro /
**Copilot** / **Antigravity** adapters are *description-matched* — they auto-fire when a natural-language
request matches the `description` (Copilot CLI **also** accepts an explicit `/<name>`, so it's both). The
**Gemini** adapter is *slash-only* — it fires only on an explicit
`/<name>`, never on a natural-language phrase. This is **not** drift or a missing adapter: every
adapter is a thin pointer back to the **same** `agent-skills/<name>/SKILL.md`, and the
`AGENTS.md` baseline runs that neutral skill on any vendor regardless. So "checks `agent-skills/`
first" for a natural-language request on Gemini is *correct* — the baseline and the slash command
land on the identical file. Don't expect Gemini to auto-trigger a command from prose.

Keep descriptions single-line and free of `"` so they embed safely into TOML / `.mdc` / YAML
frontmatter; if a `"` is unavoidable, escape it for the target format (TOML: a single-quoted
literal string; `.mdc`/YAML: quote the whole value). YAML `>`/`|` folded/literal blocks work
*only* in YAML frontmatter — the value also lands in a TOML adapter, so the canonical value is
**one logical line** (a clean `>` folds to that anyway).

## Operations

### `sync skill adapters`
**This operation is a runnable script (v4.18.0) — run it; do not hand-write the adapter files or hunt
for an npm/MCP command.** From the repo root (output is byte-identical across all three; **bash needs no
runtime install — prefer it**, especially in a non-Node project):

```
bash agent-skills/sync-adapters/scripts/sync-adapters.sh
# no bash? (e.g. native Windows) — use whichever runtime you have:
node agent-skills/sync-adapters/scripts/sync-adapters.mjs
python3 agent-skills/sync-adapters/scripts/sync-adapters.py
```

For **each** `agent-skills/<name>/SKILL.md` it (re)writes the six adapters defined in the recipe above
and **prunes** orphaned adapters it previously generated — *signature-guarded*, so it never deletes a
hand-authored vendor file (only its own pointers). Idempotent; writes only the gitignored adapter dirs,
never `agent-skills/` or a committed file; not a version change. The script ships as the built-in
**`sync-adapters`** skill, so an agent can also trigger it by description. **Enable and every Mode B
re-enable run it automatically** (v4.12.0). Still run it by hand after authoring/editing a skill, or
after a clone/pull. (Before v4.18.0 this was a prose recipe the agent acted out by hand — which led
agents to hunt for a non-existent command; the script removes that ambiguity.)

**Hot-reload caveat (vendor-specific).** Some runtimes read skill adapters **only at startup** — e.g.
**GitHub Copilot CLI** parses `.github/skills/` on init, so a freshly-synced skill (and its `/<name>`
slash command) isn't available mid-session until you **restart the session or trigger a skills rescan**
(in Copilot CLI: `/restart`, or its `/skills` rescan). Claude / Cursor / Kiro pick up a new
description-matched skill without a restart. If a just-synced skill doesn't fire, restart the runtime.
(The same applies to a newly-added `.github/hooks/` config — Copilot loads hooks at CLI start too.)

> **Never commit the adapters — and never tell the user to.** The vendor adapter dirs
> (`.claude/`, `.gemini/`, `.cursor/`, `.kiro/`, and Copilot's `.github/skills/`) are gitignored,
> per-machine, and regenerated; the **only** committed skills artifact is the neutral
> `agent-skills/`. (Copilot's adapter is path-scoped — only `.github/skills/` is ignored; the rest
> of `.github/`, e.g. `copilot-instructions.md` and `workflows/`, stays tracked.) After a sync, do
> **not** `git add` an adapter dir or suggest the user commit one. Report it like: *"regenerated N
> local adapters (gitignored — do not commit; only `agent-skills/` is shared)."*

### `adopt skill` (vendor → neutral)
If a skill was authored natively in a vendor folder (e.g. a vendor's built-in skill creator
wrote to `.claude/skills/<name>/`), it's **stranded** — gitignored, not the source of truth.
Promote it:
1. Copy its content into `agent-skills/<name>/SKILL.md` — preserve the procedure, neutralize
   vendor-specific phrasing; normalize frontmatter to `name` + `description`; move bundled
   scripts to `agent-skills/<name>/scripts/`.
2. Run **`sync skill adapters`** — regenerates the vendor file as a *pointer*.
3. Stage `agent-skills/<name>/` for the normal commit (on demand you may commit directly; the
   agent doesn't self-commit mid-ritual).

### `delete a skill` (remove / deprecate)
To remove or deprecate a skill, delete the **neutral source** and let the next sync prune its adapters —
don't hand-delete each vendor dir:
1. **Remove the source:** `rm -rf agent-skills/<name>/` (or `git rm -r agent-skills/<name>`). This stays a
   deliberate, human-visible step — the `sync-adapters` script **never** deletes `agent-skills/` itself
   (it only writes the gitignored adapter dirs).
2. **Run `sync skill adapters`** (the `sync-adapters` script). It **prunes** the now-orphaned vendor
   adapters automatically — *signature-guarded*, so it removes only adapters it generated, never a
   hand-authored vendor file.
3. **If the skill's `id` was referenced in `memory/continuity.md`,** apply the supersession rule
   (`DECAY.md` §9): replacing it → add the successor + mark the old fact `tier: superseded`; pure removal
   → note it. Same truth-state edit as any retired fact.

Then reload your runtime if it loads adapters at startup (e.g. Copilot CLI `/restart`) so the dropped
`/<name>` disappears from the session.

### `skill sanity check` (heavyweight — run deliberately)
The full alignment, for when you've been authoring/editing skills or suspect drift. Reads file
contents (heavier than the upgrade-time filename check), so run it **on demand**, not every
session:
1. **Adopt** any vendor-folder skill with no matching `agent-skills/<name>/`.
2. **Content check** each adapter against its neutral skill (description mirrors verbatim;
   pointer path correct); regenerate any that drifted.
3. **Prune** orphaned generated adapters.
4. Report adopted / regenerated / pruned.

---

> **Lightweight by design.** Enable and **every** Mode B re-enable (upgrade or already-up-to-date)
> **run** `sync skill adapters` as their closing skills step (v4.12.0) — idempotent and
> gitignored-only, so it self-heals a missing adapter (a fresh clone/pull, or a skill that predates
> a new adapter target) with no committed change and no version bump. The deliberate
> `skill sanity check` additionally realigns *content* drift (a description that no longer mirrors
> its skill) when you choose to run it. None of this is in the **per-session** path — skill work
> there is still conscious and on-demand.
