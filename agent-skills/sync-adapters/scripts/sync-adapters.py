#!/usr/bin/env python3
"""sync-adapters.py — regenerate per-vendor skill adapters from the neutral agent-skills/ layer.

Part of agent-memory (provenance: agent-memory-builtin). Python 3 stdlib only, no install.

Idempotent. Writes ONLY the gitignored vendor adapter dirs (.claude/skills, .gemini/commands,
.cursor/rules, .kiro/skills, .github/skills, .agents/skills); never edits agent-skills/ or any
committed file. (.agents/skills is the Agent Skills standard dir read by Google Antigravity 'agy',
the Gemini CLI successor — it ignores .gemini/commands.)
Prunes only adapters THIS tool generated (identified by their pointer signature), so a
hand-authored vendor skill is never deleted. Run from the repo root. `--dry-run` previews.

Kept byte-for-byte equivalent to sync-adapters.mjs (shared behavior is the cross-runtime contract).
"""
import os
import re
import shutil
import sys

ROOT = os.getcwd()
SRC = os.path.join(ROOT, "agent-skills")
DRY = "--dry-run" in sys.argv[1:]

SIG = "Maintained vendor-neutrally."  # pointer signature for claude/kiro/copilot adapters
GEM_SIG = "Read and follow the skill at agent-skills/"
CUR_SIG = "read and follow `agent-skills/"


def frontmatter(path):
    with open(path, encoding="utf-8") as fh:
        text = fh.read()
    m = re.match(r"^---\n(.*?)\n---", text, re.S)
    fm = m.group(1) if m else ""

    def field(k):
        mm = re.search(r"^" + k + r":[ \t]*(.*)$", fm, re.M)
        return mm.group(1).strip() if mm else ""

    return field("name"), field("description")


def pointer(name):
    return ("Maintained vendor-neutrally. Read and follow `agent-skills/" + name +
            "/SKILL.md` (repo root)\nand any scripts it references.\n")


def claude_like(n, d):
    return "---\nname: %s\ndescription: %s\n---\n" % (n, d) + pointer(n)


def gemini_toml(n, d):
    return ('description = "%s"\nprompt = "Read and follow the skill at agent-skills/%s/SKILL.md '
            '(repo root), including any scripts it references, then carry out: {{args}}"\n' % (d, n))


def cursor_mdc(n, d):
    return ("---\ndescription: %s\nglobs:\nalwaysApply: false\n---\n" % d +
            "When this applies, read and follow `agent-skills/" + n + "/SKILL.md` (repo root) and any\n" +
            "scripts it references.\n")


# --- discover neutral skills ---
skills = []
if os.path.isdir(SRC):
    for entry in sorted(os.listdir(SRC)):
        md = os.path.join(SRC, entry, "SKILL.md")
        if os.path.isfile(md):
            name, description = frontmatter(md)
            skills.append({"dir": entry, "name": name or entry, "description": description})


def W(p, c):
    if DRY:
        print("would write " + p)
    else:
        with open(p, "w", encoding="utf-8") as fh:
            fh.write(c)


def M(p):
    if not DRY:
        os.makedirs(p, exist_ok=True)


adapters = 0
for s in skills:
    n, d = s["name"], s["description"]
    M(os.path.join(ROOT, ".claude/skills/" + n));  W(os.path.join(ROOT, ".claude/skills/" + n + "/SKILL.md"), claude_like(n, d))
    M(os.path.join(ROOT, ".gemini/commands"));      W(os.path.join(ROOT, ".gemini/commands/" + n + ".toml"), gemini_toml(n, d))
    M(os.path.join(ROOT, ".cursor/rules"));         W(os.path.join(ROOT, ".cursor/rules/" + n + ".mdc"), cursor_mdc(n, d))
    M(os.path.join(ROOT, ".kiro/skills/" + n));     W(os.path.join(ROOT, ".kiro/skills/" + n + "/SKILL.md"), claude_like(n, d))
    M(os.path.join(ROOT, ".github/skills/" + n));   W(os.path.join(ROOT, ".github/skills/" + n + "/SKILL.md"), claude_like(n, d))
    M(os.path.join(ROOT, ".agents/skills/" + n));   W(os.path.join(ROOT, ".agents/skills/" + n + "/SKILL.md"), claude_like(n, d))
    adapters += 6

# --- prune orphaned adapters we generated (signature-guarded) ---
live = set()
for s in skills:
    live.add(s["name"])
    live.add(s["dir"])
pruned = 0


def _contains(path, sig):
    try:
        with open(path, encoding="utf-8") as fh:
            return sig in fh.read()
    except OSError:
        return False


def prune_dirs(base, signature):
    global pruned
    p = os.path.join(ROOT, base)
    if not os.path.isdir(p):
        return
    for e in os.listdir(p):
        if e in live:
            continue
        md = os.path.join(p, e, "SKILL.md")
        if os.path.isfile(md) and _contains(md, signature):
            if not DRY:
                shutil.rmtree(os.path.join(p, e), ignore_errors=True)
            pruned += 1


def prune_files(base, ext, signature):
    global pruned
    p = os.path.join(ROOT, base)
    if not os.path.isdir(p):
        return
    for e in os.listdir(p):
        if not e.endswith(ext):
            continue
        n = e[: -len(ext)]
        if n in live:
            continue
        if _contains(os.path.join(p, e), signature):
            if not DRY:
                os.remove(os.path.join(p, e))
            pruned += 1


prune_dirs(".claude/skills", SIG)
prune_dirs(".kiro/skills", SIG)
prune_dirs(".github/skills", SIG)
prune_dirs(".agents/skills", SIG)
prune_files(".gemini/commands", ".toml", GEM_SIG)
prune_files(".cursor/rules", ".mdc", CUR_SIG)

print(
    "sync-adapters: synced %d skill(s) → %d adapters "
    "(gitignored — do not commit; only agent-skills/ is shared); pruned %d orphan(s)"
    % (len(skills), adapters, pruned)
)
