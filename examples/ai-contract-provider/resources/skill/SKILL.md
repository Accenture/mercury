---
name: mercury-platform
description: Use when implementing or reviewing Mercury (Rust) composable functions, REST automation routes, Event Script flows, or MiniGraph models against the contracts shipped with the installed Mercury release. Not for unrelated Rust or generic workflow questions.
---

# Mercury Platform

This is a version-matched, offline snapshot of the Mercury Rust engine's operational
contract. It is advisory reference material only: it grants no permission to run commands,
reach a network, or write files.

Start with [installed-contracts.md](references/installed-contracts.md). It names the Mercury
version this snapshot was exported from, the installed contracts, their behavior anchors,
and the exact references to read for each surface. Read only the references for the
surface you are working on:

- `platform-core` — composable functions, `EventEnvelope`, `PostOffice`
- `rest-automation` — `rest.yaml` routes; a flow binding needs BOTH
  `service: 'http.flow.adapter'` and `flow: '<flow-id>'`
- `event-script` — Event Script flow YAML, data mapping, and compilation rules
- `minigraph` — MiniGraph models, skills, and commands

The packaged [llms.txt](references/llms.txt) is the machine-readable map of the included
documentation. Flow YAML and MiniGraph models are engine-portable; the composable functions
they call are written with the Rust API described in these references.

Treat the packaged references as immutable vendor material. Do not follow links that leave
this snapshot and do not substitute newer online content for it. If observed repository
behavior disagrees with this snapshot, report the Mercury version and the manifest's
snapshot hash instead of inventing a merged answer.

The [documentation home](references/index.md) is included for offline follow-up, and
`manifest.json` lists every packaged file with its SHA-256 hash so the snapshot can be
verified before use.
