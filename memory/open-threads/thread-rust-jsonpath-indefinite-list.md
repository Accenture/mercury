- [x] (parity) **CLOSED 2026-10-04 06:08Z — a `$.` JSONPath result takes Jayway's shape on both engines.** PR #352 (merge `bdc7b5be`, Increment 157):
  `MultiLevelMap::json_path_query` shapes by the kind of path, not the count - a definite path yields the value or nothing, an indefinite one always a list
  (`[x]`, `[]`), a missing member name before the first indefinite step yields nothing - pinned by the shared fixture `unit-test-jsonpath-1` (Java pin
  mercury-composable #504); the rule is [[jsonpath-jayway-result-shape]]. Lesson: probe the reference library before porting its rule - the task text had
  missed Jayway's missing-name exception. origin: 2026-10-04-054444; close 2026-10-04-060541.
  <!-- id: rust-jsonpath-indefinite-list | created: 2026-10-03 | last_used: 2026-10-03 | uses: 1 | tier: working | origin: 2026-10-04-054444 -->
