- [x] **`json` simple plugin twin — land the PR (lock-step with the Java engine).** DONE
  2026-09-02: PR #225 squash `b049dc67`, CI green before merge; Java pair = mercury-composable
  PR #311 squash `1519dc0f`. Increment 98: parse_json + `#[simple_plugin("json")]`,
  BUILTIN_PLUGIN_COUNT 48, fixture/flow twins, doc twins. Durable lesson: engine
  difference documented rather than papered over — serde strict vs Gson lenient;
  portable flows use strict JSON (same pattern as the length UTF-16/scalar note).
  origin: 2026-09-02-163225
  <!-- id: ot-json-plugin-twin | created: 2026-09-02 | last_used: 2026-09-02 | uses: 3 | tier: active | origin: 2026-09-02-163225 -->
