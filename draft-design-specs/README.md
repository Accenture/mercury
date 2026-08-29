# Design specs — a work-in-progress journal

The documents in this folder are **working design specs**: they are written and refined
during the design and implementation of a feature, capturing the problem, the options
considered, the decisions taken (and revised), and the implementation status as the work
unfolds.

**After a feature ships, a spec is a journal record for reference only.** It is kept as
history — transparent, in the open-source spirit — so a human reader can understand how
the project evolved and why decisions were made at the time.

**Source code is the source of truth.** A shipped feature's authoritative description
lives in the code, its tests, and the published guides (`docs/guides/`); durable
architecture decisions live in the ADR ledger (`docs/arch-decisions/ADR.md`), and the
increment-by-increment history in `docs/INCREMENTS.md`. The port's foundational design
documents (the layer-by-layer port designs) live in this folder alongside the working
specs. Where a design spec and the code disagree, the code is right — the spec records what was planned and learned, not what necessarily holds
today.

This folder mirrors the Java repository's `draft-design-specs/` — feature specs shared
across the two engines are drafted there (Java is the reference implementation) and
port-specific working specs are drafted here.
