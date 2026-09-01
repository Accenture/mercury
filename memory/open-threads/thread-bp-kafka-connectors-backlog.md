- [ ] **(backlog) Port the lightweight cloud-native connectors + sync-over-async.** Maintainer
  scope refinement 2026-07-20 (stated while reviewing the docs site): `minimalist-kafka` and
  `twin-kafka` are **lightweight, cloud-native connectors** — distinct from the Kafka service
  mesh (service discovery + sync-over-Kafka, which stays permanently out of scope) — and will
  be ported in future iterations, along with **`sync-over-async`** (the request/response
  bridge over async transports). This is also why HTTP config keys keep their `http.` prefix
  (connector counterparts arrive later). Vision non-goals + instructions + the public
  `docs/background/port-scope.md` all updated to the refined wording. → serves: vision-mercury
  <!-- id: bp-kafka-connectors-backlog | created: 2026-07-20 | last_used: 2026-08-14 | uses: 5 | tier: working | origin: 2026-07-20-030615.md -->
