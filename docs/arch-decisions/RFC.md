# Requests for Comments (RFC) — the proposal register

> **For humans.** Work under consideration at the Design altitude: proposals that may become
> an Architecture Decision Record, be reshaped, merge with another, or be withdrawn. It is the
> sibling of `ADR.md` and exists so that **the ledger records decisions only** — an ADR is
> written when a proposal is accepted, never before (`memory/PROTOCOL.md` *Work from intent*,
> `DECAY.md` §12). Read **on demand**, not part of the per-session agent read path (zero
> default token cost, the same footing as `ADR.md`).

## Rules

- **Separate sequences.** `RFC-NNNN` and `ADR-NNNN` never share numbers: a proposal does not
  reserve an ADR number, because proposals and decisions do not map one-to-one — some merge,
  some split, some die.
- **Two exits, both recorded here.** *Promoted:* the human accepted it; the ADR is written in
  `ADR.md` and this entry keeps a pointer (`Promoted → ADR-NNNN`, date). *Withdrawn:* the entry
  stays with the reason. An entry is **never deleted**; a proposal may be revised freely while
  open, and only its final form reaches the ledger.
- **Status:** `Open` (under consideration) · `Parked` (deliberately deferred — reactivate on
  demand) · `Promoted → ADR-NNNN` · `Withdrawn`.
- **Map, don't duplicate.** The live work item stays in memory — an Open Thread in
  `memory/open-threads/` carrying a `→ proposal: RFC-NNNN` pointer — exactly as an accepted ADR
  is pointed to by a `(ADR-NNNN)` tag on its continuity fact. The register holds the proposal's
  reasoning (options, trade-offs, what a decision would commit to); the thread holds its state.
- **The human decides.** The agent raises and revises proposals here; promotion is the
  Design-altitude human gate (`DECAY.md` §12). Newest first.

## Format

```
## RFC-NNNN — <Title>
**Status:** Open · **Raised:** YYYY-MM-DD · **Serves:** <vision-id> · **Thread:** `<thread-id>`
<!-- id: rfc-NNNN | status: open | thread: <thread-id> -->

**Proposal.** What would change, and what a decision would commit the project to.
**Options.** The alternatives on the table, with their trade-offs.
**Resolution.** Empty while open; `Promoted → ADR-NNNN (date)` or `Withdrawn (date): <reason>`.
```

---

*(No proposals yet. The first one goes here, newest first.)*
