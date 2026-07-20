#!/usr/bin/env bash
# sync-adapters.sh — regenerate per-vendor skill adapters from the neutral agent-skills/ layer.
# Part of agent-memory (provenance: agent-memory-builtin). Pure bash + coreutils — no runtime to install.
#
# Idempotent. Writes ONLY the gitignored vendor adapter dirs (.claude/skills, .gemini/commands,
# .cursor/rules, .kiro/skills, .github/skills, .agents/skills); never edits agent-skills/ or any
# committed file. (.agents/skills is the Agent Skills standard dir read by Google Antigravity 'agy',
# the Gemini CLI successor — it ignores .gemini/commands.)
# Prunes only adapters THIS tool generated (signature-guarded). Run from the repo root. --dry-run previews.
#
# Byte-for-byte equivalent to sync-adapters.mjs / sync-adapters.py (shared behavior = the contract).
# Bash 3.2-compatible (works with the macOS system bash) — no associative arrays, no `set -u` traps.

ROOT="$(pwd)"
SRC="$ROOT/agent-skills"
DRY=0
[ "${1:-}" = "--dry-run" ] && DRY=1

SIG="Maintained vendor-neutrally."
GEM_SIG="Read and follow the skill at agent-skills/"
CUR_SIG='read and follow `agent-skills/'
LIVE=" "

frontfield() { # $1=file $2=key — value from the first frontmatter block, trimmed
  awk -v key="$2" '
    /^---$/ { n++; next }
    n==1 && index($0, key ":")==1 {
      v=$0; sub("^" key ":[ \t]*", "", v); sub(/[ \t]+$/, "", v); print v; exit
    }' "$1"
}

# claude/kiro/github adapters share one shape:
w_claude() { # $1=path $2=name $3=desc
  if [ "$DRY" -eq 1 ]; then echo "would write $1"; return; fi
  printf -- '---\nname: %s\ndescription: %s\n---\nMaintained vendor-neutrally. Read and follow `agent-skills/%s/SKILL.md` (repo root)\nand any scripts it references.\n' "$2" "$3" "$2" > "$1"
}
w_gemini() { # $1=path $2=name $3=desc
  if [ "$DRY" -eq 1 ]; then echo "would write $1"; return; fi
  printf -- 'description = "%s"\nprompt = "Read and follow the skill at agent-skills/%s/SKILL.md (repo root), including any scripts it references, then carry out: {{args}}"\n' "$3" "$2" > "$1"
}
w_cursor() { # $1=path $2=name $3=desc
  if [ "$DRY" -eq 1 ]; then echo "would write $1"; return; fi
  printf -- '---\ndescription: %s\nglobs:\nalwaysApply: false\n---\nWhen this applies, read and follow `agent-skills/%s/SKILL.md` (repo root) and any\nscripts it references.\n' "$3" "$2" > "$1"
}

skills=0
adapters=0
if [ -d "$SRC" ]; then
  for d in "$SRC"/*/; do
    [ -f "${d}SKILL.md" ] || continue
    dir="$(basename "$d")"
    name="$(frontfield "${d}SKILL.md" name)"; [ -n "$name" ] || name="$dir"
    desc="$(frontfield "${d}SKILL.md" description)"
    LIVE="$LIVE$name $dir "
    if [ "$DRY" -eq 0 ]; then
      mkdir -p "$ROOT/.claude/skills/$name" "$ROOT/.gemini/commands" "$ROOT/.cursor/rules" \
               "$ROOT/.kiro/skills/$name" "$ROOT/.github/skills/$name" "$ROOT/.agents/skills/$name"
    fi
    w_claude "$ROOT/.claude/skills/$name/SKILL.md" "$name" "$desc"
    w_gemini "$ROOT/.gemini/commands/$name.toml" "$name" "$desc"
    w_cursor "$ROOT/.cursor/rules/$name.mdc" "$name" "$desc"
    w_claude "$ROOT/.kiro/skills/$name/SKILL.md" "$name" "$desc"
    w_claude "$ROOT/.github/skills/$name/SKILL.md" "$name" "$desc"
    w_claude "$ROOT/.agents/skills/$name/SKILL.md" "$name" "$desc"
    skills=$((skills + 1))
    adapters=$((adapters + 6))
  done
fi

is_live() { case "$LIVE" in *" $1 "*) return 0 ;; *) return 1 ;; esac; }
pruned=0

prune_dirs() { # $1=base $2=signature
  local p="$ROOT/$1" e b
  [ -d "$p" ] || return 0
  for e in "$p"/*/; do
    [ -d "$e" ] || continue
    b="$(basename "$e")"
    is_live "$b" && continue
    if [ -f "${e}SKILL.md" ] && grep -qF "$2" "${e}SKILL.md"; then
      [ "$DRY" -eq 1 ] || rm -rf "$e"
      pruned=$((pruned + 1))
    fi
  done
}
prune_files() { # $1=base $2=ext $3=signature
  local p="$ROOT/$1" f b
  [ -d "$p" ] || return 0
  for f in "$p"/*"$2"; do
    [ -e "$f" ] || continue
    b="$(basename "$f" "$2")"
    is_live "$b" && continue
    if grep -qF "$3" "$f"; then
      [ "$DRY" -eq 1 ] || rm -f "$f"
      pruned=$((pruned + 1))
    fi
  done
}

prune_dirs ".claude/skills" "$SIG"
prune_dirs ".kiro/skills" "$SIG"
prune_dirs ".github/skills" "$SIG"
prune_dirs ".agents/skills" "$SIG"
prune_files ".gemini/commands" ".toml" "$GEM_SIG"
prune_files ".cursor/rules" ".mdc" "$CUR_SIG"

printf 'sync-adapters: synced %d skill(s) → %d adapters (gitignored — do not commit; only agent-skills/ is shared); pruned %d orphan(s)\n' "$skills" "$adapters" "$pruned"
