# Test fixtures — reused verbatim from the Java original

`flows/`, `parser-flows/`, `flows.yaml` and `more-flows.yaml` are copied **unchanged**
from `mercury-composable` (Java, v4.8.6) `system/event-script-engine/src/test/resources/`
per design decision **E2** (flow YAML syntax verbatim): a flow written for the Java
engine must compile on the Rust engine with identical results, so the canonical fixtures
double as behavior-parity tests. Do not edit them here — fixture changes come from the
Java repo.

`application.yml` is Rust-side test configuration (not a Java copy).
