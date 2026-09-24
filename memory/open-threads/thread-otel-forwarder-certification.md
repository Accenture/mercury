- [x] (certification — OpenTelemetry forwarder, 2026-09-22) **CLOSED 2026-09-22** — the released
  `mercury-opentelemetry-forwarder` 4.12.14 is certified against Dynatrace end to end: the branch build (report
  Scenarios 1–5), the published crate from a registry-only consumer app (Scenario 6: `580cbdc0…`, `5fd1cd3d…`) and one
  trace across both engines (Scenario 7: `72b2e692…`, `47100c7c…`, `f78de6d2…`, `ec6b3fc6…`, `3481c84b…`) — all
  confirmed in Eric's UI screenshots (one trace, two services, spans nested across the Kafka hop). Report on
  `docs/otel-report-acceptance-4.12.14` (PR pending). Lesson: a clean run is evidence only with the negative control
  and the artifact check; a stopped Kafka member is not a gone member (fresh broker per pairing). The Java LeaveGroup
  finding lives in the Java repo's `kafka-consumer-leave-group-on-shutdown`. origin: 2026-09-22-010413.
  <!-- id: otel-forwarder-certification | created: 2026-09-22 | last_used: 2026-09-22 | uses: 1 | tier: archive-candidate | origin: 2026-09-22-010413 -->
