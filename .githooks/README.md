# `.githooks/` — composable, vendor-neutral ritual triggers

These are **committed, vendor-neutral git hook dispatchers and fragments** that reinforce the
after-session ritual for *any* agent (Claude, Copilot, Kiro, …) — because everyone commits,
regardless of which AI did the work. They are **advisory by default** — the pre-commit secret guard
alone **enforces** (findings block the
commit; a deliberate, scoped exception, because secrets carry irreversible after-the-fact cost) —
and the **tool runs nothing itself**: git invokes them in your env at your
opt-in (`no-build-step-agent-run`). See `docs/optional-ritual-hook.md` and `DECAY.md` for the rationale.

## First-run init (one command)

A fresh clone has the gitignored skill **adapters absent** and the dispatchers **unactivated** (git can't
auto-run committed hooks on clone — security). Set both up with one idempotent command:

```sh
bash .githooks/init.sh
```

It regenerates the vendor skill adapters **and** runs `git config core.hooksPath .githooks`. **The agent
runs this itself on a first session** (see `memory/PROTOCOL.md`), so an untrained user does nothing. To
activate the dispatchers alone: `git config core.hooksPath .githooks` (undo:
`git config --unset core.hooksPath`).
**CI is the zero-config floor** — `.github/workflows/agent-memory.yml` on GitHub; `.gitlab-ci.yml` +
`.gitlab/agent-memory-ci.yml` on GitLab (v4.31.0); `.azuredevops/agent-memory-ci.yml` on Azure
DevOps (v4.32.0) — it runs server-side on every push and, on GitHub/GitLab, every pull/merge
request (Azure DevOps PR-time runs need the optional Build Validation policy), so the ritual is
enforced even on a clone where the local hook was never activated. (Honest limits: a self-managed GitLab instance needs an admin-registered runner;
an Azure DevOps pipeline is inert until its one-time `az pipelines create` binding.)

## Dispatcher contract

The `pre-commit` and `post-commit` entrypoints are deliberately small dispatchers. Each runs the
executable regular files in its matching directory — `.githooks/pre-commit.d/` or
`.githooks/post-commit.d/` — in deterministic C-locale filename order. Hidden and non-executable
files are ignored. Hook arguments are forwarded unchanged.

Every fragment runs even when an earlier fragment fails, so independent hook layers all get a
chance to report. The dispatcher returns the **first non-zero status** after the run. That blocks a
pre-commit when any enforcing fragment fails; Git ignores post-commit status, while direct invocation
and tests can still observe failures.

Agent-memory's managed fragments use the `50-` slot:

- `pre-commit.d/50-agent-memory-secret-guard`
- `post-commit.d/50-agent-memory-ritual-capture`

Add another executable fragment instead of replacing either hook entrypoint: use `00-`–`49-` to run
before agent-memory or `51-`–`99-` to run after it. Upgrades refresh unchanged copies of the two
dispatchers and managed `50-` fragments, preserve every differently named fragment, and human-gate
any locally modified managed file instead of silently overwriting it. This gives other hook layers
a stable composition seam and makes each behavior independently runnable in CI.

The dispatcher contract itself is covered by `tests/test_githook_dispatchers.sh` in the tool repo.

## Managed fragments

- **`pre-commit.d/50-agent-memory-secret-guard`** (v4.34.0) — the **`[secret-material]` guard**:
  before the commit exists, scans the
  **staged content** (the index, not the worktree — exactly what this commit would publish; a
  pre-existing finding elsewhere never gates an unrelated commit) of **two surfaces**:
  `memory/**.md` (the full profile — credentials + PII) and **config files** — `.json` / `.yml` /
  `.yaml` / `.properties` / `.toml` / `.ini` / `.env*` anywhere in the repo, credential-class checks
  only (token shapes, key assignments, Authorization headers, private keys; config files
  legitimately carry contact emails and paths). The config surface exists because of a real
  incident: live credentials entered a repo inside a Postman JSON and an OpenShift YAML, then
  contaminated a session log downstream. Findings print with the linter's non-echoing report plus
  redaction/waiver/rotation guidance. **Enforcing by default** — findings **block the commit**
  (secrets are the one category with irreversible after-the-fact cost); opt down to warn-only with
  `AGENT_MEMORY_SECRET_GUARD=advisory` (env, or `git config agent-memory.secretguard advisory`);
  one-off bypass: `git commit --no-verify`.
  Waivers: tag the line `lint:allow-secret-material` where the format has comments (markdown,
  YAML, TOML, INI); JSON has no comments and a `.properties` same-line comment corrupts the value —
  list those files in the committed, human-audited **`.agent/secret-scan-ignore`** (shell-glob per
  line; exempts config files only, never `memory/`). Runs on python3 or node, whichever exists —
  with neither, it skips with a note. Why it exists: the ritual rule covers agents at write time
  and the CI floor covers pushes, but by push time the remote already has the secret and redaction
  is not un-leaking (rotation is) — this is the **one placement that prevents instead of detects**
  (see the `memory/PROTOCOL.md` redaction rule).

- **`post-commit.d/50-agent-memory-ritual-capture`** — after a commit: re-syncs skill adapters if a
  skill changed; and if the commit did
  real work but carried no session log, ensures the session is captured — **once per working session, not
  per commit.** If there is **no** session log within the active-session window (default **30 min**; override
  `AGENT_MEMORY_SESSION_WINDOW_MINUTES`) it **auto-stubs** `memory/sessions/<ts>.md`; if a recent log already
  covers this session — committed *or* a waiting stub, detected by the newest session **filename** (immutable
  and clone-safe, unlike mtime) — it instead **nudges you to enrich that existing log**. The stub guarantees
  the ledger never has a silent gap; the *thoughtful* summary stays the agent's job (capture vs. judgment —
  same split as `memory-lint`).

  > **Splitting code and memory into two commits?** The advisory may fire on the code-only commit (it
  > carries no session log) and point you at the session's existing log — **expected and benign**, not a
  > failure, and it will **not** pile up a second stub (one log per session). To skip the nudge entirely,
  > prefer a **single atomic commit** that includes the work *and* its session log. The hook is advisory
  > and never blocks.

To deactivate: `git config --unset core.hooksPath`.
