# Copilot Instructions

> **First-run init — do this before anything else (incl. before summarizing).** A fresh clone has the
> gitignored skill adapters **absent** and the hook dispatchers **unactivated** (git can't auto-run committed
> hooks on clone). **If `git config core.hooksPath` is unset, OR vendor adapter dirs like `.github/skills/`
> are empty, run this once first:**
>
> ```sh
> bash .githooks/init.sh
> ```
>
> It regenerates the skill adapters **and** activates the committed pre-commit and post-commit
> dispatchers (idempotent — skip if already set up). Then proceed to load context below.

This project uses the **agent-memory** shared memory system. **Read these now, in order** — don't
defer to a pointer chain (Copilot Ask/Plan modes won't follow it unless the files are attached):

1. **`AGENTS.md`** — the one-line universal discovery shim.
2. **`memory/PROTOCOL.md`** — the memory protocol; follow it before substantive work.
3. **`memory/instructions.md`** — persona, project rules, conventions.
4. **`memory/continuity.md`** — current state, key decisions, open threads, the project's hard rules.
5. **`memory/vision.md`** — the project's north-star target (the VBDI forward layer).
6. **`memory/sessions/`** — scan the latest 2–3 session files for recent context.

**Skills:** project capabilities live in `agent-skills/<name>/SKILL.md` (vendor-neutral, committed —
the source of truth). Copilot adapters are regenerated under `.github/skills/` (gitignored) by the
"sync skill adapters" operation, so Copilot CLI auto-discovers them. See `SKILLS.md` (on demand).

**Session logging follows the lightweight-mode rule** (`memory/PROTOCOL.md` → "Close every session"): a session
that **changed tracked files** (Agent mode) writes a `memory/sessions/YYYY-MM-DD-HHMMSS.md` log and
updates `memory/continuity.md`; a **read-only** session (Ask/Plan modes don't edit tracked files)
correctly writes **no log** — that's the protocol, not a gap. The heavier `REVIEW.md` decay/review pass
is the one step to run by hand when its cadence triggers.

Identify yourself as **GitHub Copilot** in all session logs.
