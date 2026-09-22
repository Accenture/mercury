- [ ] (certification — OpenTelemetry forwarder, 2026-09-22) **Close the forwarder's live certification and its
  field acceptance** — Scenario 5 of `docs/test-reports/otel-dynatrace-certification.md`: Eric confirms in the
  Dynatrace non-prod UI that the traces are queryable — under service `mercury-otel-cert` (the Java
  certification's name; the drive repeated 01:18Z on Eric's question) `f658763c71844a998d02521c033c5918` and
  `b0ee5e2087ff495a9b8f13977620c758`, and under service `hello-flow` `0ad577c2bef646cba174147ebf923c01`
  (00:52:04Z) and `092b2a1947e54f298d5ab945aa901331` (00:52:26Z) — five spans each, scope
  `mercury-opentelemetry-forwarder` 4.12.12 — are queryable with the engine's nesting, and that leg B's
  `eedf60cd9bbe47618859a1a7b5967837` and `ababd406bfcf43c181ded9446d1c0198` are absent — **DONE 2026-09-22:** Eric's
  screenshots of both `mercury-otel-cert` traces show the nesting as recorded, `Internal` kinds under the `server` root,
  status OK, scope `mercury-opentelemetry-forwarder` 4.12.12 and the attributes as mapped; report Scenario 5 closed on
  `release/v4.12.14`. **Field-acceptance drives DONE 2026-09-22 (report Scenarios 6–7, branch `docs/otel-report-acceptance-4.12.14`):**
  (6) a standalone app on the crates.io artifacts only — traces `580cbdc0f96441aa9afd9c6ed51bc4ce`, `5fd1cd3db9ba41a0a8b8b5de8e603b1a`
  (0/6 failures; negative control 3/3 401), scope version to read **4.12.14**; (7) Eric's two-engine drive — the sync-over-async
  interop with the forwarder on both engines, services `mercury-otel-cert-java` / `mercury-otel-cert-rust`: Java facade + Rust
  backend `f78de6d2d9a649d425acaec09a6bba53`, `72b2e692bac8478e1e2a9c148e3d1606`; Rust facade + Java backend
  `ec6b3fc64b769c9f79c1f80d50371a2e`, `3481c84b80e6804849a2df2ea6967237` — 48 spans, 0 failures, cross-engine lineage both
  ways. **Remaining — Eric's UI confirmation of those six traces (one trace, two services; scope 4.12.14), then close.**
  Superseded item:
  after the v4.12.14 release, field acceptance on the released crate (scope version 4.12.14 — the artifact check
  the Java certification used to close its own). Parked in the report's *What remains*: the Splunk `X-SF-Token:`
  header form not run live; `compression=gzip` a declared delta (implement only on a field need). Relates
  [[otel-forwarder-no-sdk]].
  origin: 2026-09-22-010413.
  <!-- id: otel-forwarder-certification | created: 2026-09-22 | last_used: 2026-09-22 | uses: 1 | tier: working | origin: 2026-09-22-010413 -->
