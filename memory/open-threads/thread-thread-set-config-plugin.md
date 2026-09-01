- [x] (feature — lock-step with the Java engine, implemented and **MERGED 2026-08-21 as
  mercury PR #208, merge `9a7b3a47` carrying `338fc895` (tree verified identical), CI green
  (test 2m20s + recheck), branches deleted both ends; the Java half merged the same day as
  [PR #289](https://github.com/Accenture/mercury-composable/pull/289) squash `b5aeaf56`,
  tree verified. Both ride the next release via CHANGELOG Unreleased.**) **Event Script
  `f:setConfig` simple plugin — set/override a config parameter at run-time via the process
  override registry (`overrides::set`, the System.setProperty analog).** Key = non-empty
  string; value = any object → `get_text_value` (Java String.valueOf); invalid input →
  false without side effect. BUILTIN_PLUGIN_COUNT 46→47; the loaded-flow-set parity pin
  gained `set-config-parameter`; flow fixture BYTE-IDENTICAL to Java's set-config.yml
  (set in task one, `map(key)` read-back in task two, runtime-asserted); unit twin of
  SetConfigParameterTest. Docs: syntax.md catalog row + configuration override detail
  (Rust wording: override registry / -D args); CHANGELOG Unreleased; INCREMENTS 87.
  Gate: 58 suites / 306 tests, clippy 0, fmt clean. Java half: mercury-composable branch
  `feature/config-plugin` (same day; Eric's plugin, reviewed + ruled there).
  <!-- id: thread-set-config-plugin | created: 2026-08-21 | last_used: 2026-08-22 | uses: 2 | tier: archive-candidate | origin: 2026-08-21-234417 -->
