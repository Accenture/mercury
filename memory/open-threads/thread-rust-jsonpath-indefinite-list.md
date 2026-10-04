- [ ] (parity) **A JSONPath filter with one match yields a scalar here and a one-element list in Java (found 2026-10-03).** `MultiLevelMap::json_path_query`
  (`crates/event-script/src/mlm.rs`, serde_json_path) shapes results by COUNT - none is `None`, one is the scalar, more is an array (its unit test says
  "single match -> scalar") - while Jayway, behind the Java `MultiLevelMap`, shapes by PATH KIND: a definite path yields the value, an indefinite one (a filter,
  a wildcard, a deep scan, a union or a slice) always yields a list, `[]` for no match. So `$.input.body.people[?(@.name == 'Peter')].age` is `[42]` in Java
  and `42` here, and a mapping or a `for_each` that expects a list breaks in Event Script flows and graphs alike. Next: decide definite vs indefinite from the
  parsed path, return an array for an indefinite path whatever the count (check how the Java mapping layer treats an empty list so the null-source rule stays
  the same), pin it with a one-match case in a fixture shared byte-identical with the Java repo, and add a READ note. The `unit-test-mapping-1` fixture
  (Increment 156) sidesteps it with a two-row filter. Relates [[mapping-source-verbatim-substitution]].
  **In review (2026-10-04):** PR #352 (Increment 157) implements it - `path_shape` classifies the parsed path string, `misses_a_member_name` applies Jayway's one
  exception (a missing member name before the first indefinite step, or a name on a non-object, is not found; a missing index there only empties the list), probed
  against Jayway 3.0.0; pinned by `json_path_result_shape_follows_jayway` and the shared fixture `unit-test-jsonpath-1`, whose Java pin is mercury-composable #504.
  Close on merge.
  <!-- id: rust-jsonpath-indefinite-list | created: 2026-10-03 | last_used: 2026-10-03 | uses: 1 | tier: working | origin: 2026-10-04-054444 -->
