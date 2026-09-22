- [ ] (certification — OpenTelemetry forwarder, 2026-09-22) **Close the forwarder's live certification and its
  field acceptance** — Scenario 5 of `docs/test-reports/otel-dynatrace-certification.md`: Eric confirms in the
  Dynatrace non-prod UI that the traces are queryable — under service `mercury-otel-cert` (the Java
  certification's name; the drive repeated 01:18Z on Eric's question) `f658763c71844a998d02521c033c5918` and
  `b0ee5e2087ff495a9b8f13977620c758`, and under service `hello-flow` `0ad577c2bef646cba174147ebf923c01`
  (00:52:04Z) and `092b2a1947e54f298d5ab945aa901331` (00:52:26Z) — five spans each, scope
  `mercury-opentelemetry-forwarder` 4.12.12 — are queryable with the engine's nesting, and that leg B's
  `eedf60cd9bbe47618859a1a7b5967837` and `ababd406bfcf43c181ded9446d1c0198` are absent; record the outcome in the report (Scenario 5) and here. Then,
  after the v4.12.14 release, field acceptance on the released crate (scope version 4.12.14 — the artifact check
  the Java certification used to close its own). Parked in the report's *What remains*: the Splunk `X-SF-Token:`
  header form not run live; `compression=gzip` a declared delta (implement only on a field need). Relates
  [[otel-forwarder-no-sdk]].
  origin: 2026-09-22-010413.
  <!-- id: otel-forwarder-certification | created: 2026-09-22 | last_used: 2026-09-22 | uses: 1 | tier: working | origin: 2026-09-22-010413 -->
