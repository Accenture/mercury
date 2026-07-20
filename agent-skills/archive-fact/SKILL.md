---
name: archive-fact
description: Safely move a decayed fact or completed thread from continuity.md into the archive — the mechanical half of a memory review, done deterministically. Use during the REVIEW.md ritual (step 4) or whenever a specific fact must be archived, instead of hand-editing continuity.md (which has truncated the file twice). The agent decides what to archive; this performs the move safely.
provenance: agent-memory-builtin
---

> ⚠️ **Tool-managed skill provided by agent-memory** (`provenance: agent-memory-builtin`). Don't edit it
> in place — fork under a new name, or upstream a fix to the agent-memory project (see `SKILLS.md`).

This skill **is** the safe archive-move. It performs `REVIEW.md` step 4 — moving a faded/superseded fact's
block out of `continuity.md` and into the quarter archive — as a **runnable script**, so the move can't be
botched. **Don't hand-edit `continuity.md` to do this** (the read-modify-write `open(f,"w").write(open(f).read()+…)`
truncates the file *before* the read — it has wiped a `version.md` stamp and then this repo's archive,
50 facts → 6, once each). This script reads the whole file into memory first and writes once, so truncation
is structurally impossible.

## What it does NOT do

It does **not** decide *what* to archive — that judgment (which facts have faded, contradiction scans,
supersession) stays with the agent and the `REVIEW.md` ritual (`never-pick-a-winner`). This is the
mechanical executor only: you name the id(s); it moves them. Same split as `memory-lint` (the script counts;
the agent judges).

## How to run

From the repo root, use whichever runtime the machine has (output is byte-identical):

```sh
python3 agent-skills/archive-fact/scripts/archive-fact.py --id <fact-id> [--id <fact-id> …]
# or
node    agent-skills/archive-fact/scripts/archive-fact.mjs --id <fact-id> [--id <fact-id> …]
```

Flags:
- `--id ID` — the fact/thread id to archive (repeatable; the move is **all-or-nothing** — if any id is
  missing it refuses and changes nothing).
- `--reason` — `faded` (default) or e.g. `"superseded by <new-id>"`.
- `--quarter YYYY-Qn` — target `memory/archive/<quarter>.md` (default: derived from today, UTC).
- `--note TEXT` — the `INDEX.md` one-liner (single `--id` only; otherwise derived from the block).
- `--dry-run` — print what *would* move; change nothing. **Preview first when unsure.**

It (1) extracts each id's block (the column-0 `- ` bullet down through its `<!-- id: … -->` footer, whole),
(2) **appends** it to the quarter archive under a dated heading and adds an `INDEX.md` line (append mode —
never truncates), and (3) rewrites `continuity.md` without those blocks (read-into-variable, single write).

**Always run `memory-lint` afterward** (it confirms no id ended up in both places and no link dangles). The
helper is safe by construction; the lint is the deterministic proof.

## Guards (it refuses, exit 1)
- an id with no footer in `continuity.md` (typo / already moved);
- an id already present in the archive (no double-archive);
- a move that would leave `continuity.md` empty.

## Notes
- **No runtime? Don't hand-roll the truncating write.** Fall back to the manual safe method `REVIEW.md`
  documents (append-mode / read-into-variable), then run `memory-lint`. (Same stance as `memory-lint`:
  install Python or Node rather than improvise the dangerous step.)
- Two runtimes at output parity, with mirror tests (`test_archive_fact.py` / `.mjs`) — the deterministic
  guarantee shouldn't depend on which runtime the machine happens to have.
- Superseded facts: pass `--reason "superseded by <new-id>"`; mark the supersession links in the footers
  (`DECAY.md` §9) before or after — the helper moves the block as-is.
