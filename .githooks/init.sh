#!/usr/bin/env bash
# agent-memory — first-run init. Run ONCE after a fresh clone (or any time setup looks off).
# Idempotent + advisory. Closes the fresh-clone gap: a clone has the gitignored skill adapters
# ABSENT and the git hook dispatchers UNACTIVATED (git can't auto-run committed hooks on clone — security).
# This single command sets both up. The agent runs it itself on a first session (see memory/PROTOCOL.md);
# this is the human one-command fallback. On GitHub / GitLab.com, CI runs server-side regardless,
# with or without this (a self-managed GitLab needs an admin-registered runner; an Azure DevOps
# pipeline runs once its one-time activation is done).
set -e
cd "$(git rev-parse --show-toplevel 2>/dev/null || echo .)"

echo "agent-memory init: (1/2) regenerating skill adapters…"
if [ -f agent-skills/sync-adapters/scripts/sync-adapters.sh ]; then
  bash    agent-skills/sync-adapters/scripts/sync-adapters.sh \
    || node    agent-skills/sync-adapters/scripts/sync-adapters.mjs \
    || python3 agent-skills/sync-adapters/scripts/sync-adapters.py \
    || echo "  (no bash/node/python runtime for sync-adapters; skipped — adapters can be synced later)"
else
  echo "  (no sync-adapters skill present; skipped)"
fi

echo "agent-memory init: (2/2) activating the git hook dispatchers (pre-commit secret guard + post-commit ritual capture fragments)…"
if [ -d .githooks ]; then
  git config core.hooksPath .githooks
  echo "  core.hooksPath = $(git config core.hooksPath)"
fi

echo "agent-memory init: done — adapters synced + git hook dispatchers active. (On GitHub / GitLab.com, CI runs server-side regardless.)"
