---
name: sync-adapters
description: Regenerate the per-vendor skill adapters from the committed agent-skills/ layer. Run after authoring or editing a skill, or after a clone or pull, so Claude, Gemini, Cursor, Kiro, GitHub Copilot, and Google Antigravity each get a current native adapter.
provenance: agent-memory-builtin
---

> ⚠️ **Tool-managed skill provided by agent-memory** (`provenance: agent-memory-builtin`). Don't edit it
> in place — fork under a new name, or upstream a fix to the agent-memory project (see `SKILLS.md`).

This skill **is** the `sync skill adapters` operation. It is a **runnable script** — do **not** hunt for an
npm command, MCP tool, or hand-write the adapter files. Run one of these from the repo root (output is
byte-identical across all three; **bash needs no runtime install — prefer it**, especially in a non-Node
project):

```bash
bash agent-skills/sync-adapters/scripts/sync-adapters.sh
# no bash? (e.g. native Windows) — use whichever runtime you have:
node agent-skills/sync-adapters/scripts/sync-adapters.mjs
python3 agent-skills/sync-adapters/scripts/sync-adapters.py
```

For each `agent-skills/<name>/SKILL.md` it (re)writes all **six** vendor adapters —
`.claude/skills/<name>/SKILL.md`, `.gemini/commands/<name>.toml`, `.cursor/rules/<name>.mdc`,
`.kiro/skills/<name>/SKILL.md`, `.github/skills/<name>/SKILL.md`, `.agents/skills/<name>/SKILL.md` — and
**prunes** orphaned adapters it previously generated (identified by their pointer signature; never touches
hand-authored vendor files). The `.agents/skills/` adapter is the Agent Skills standard dir read by Google
Antigravity (`agy`, the Gemini CLI successor) — Antigravity ignores `.gemini/commands`, so this is what
makes `/<name>` work there.

It is **idempotent** and writes **only the gitignored adapter dirs** — never `agent-skills/`, never a
committed file. Pass `--dry-run` to preview. The neutral `agent-skills/<name>/SKILL.md` is always the
source of truth; the adapters are thin regenerated pointers (do not commit them).

**After syncing, reload your runtime if it loads adapters at startup** (e.g. GitHub Copilot CLI:
`/restart` or a skills rescan) for a new `/<name>` to become available this session.
