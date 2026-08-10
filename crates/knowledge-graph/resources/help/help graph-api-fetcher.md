Skill: Graph API Fetcher
------------------------
Calls an external HTTP API declaratively. The node never holds a URL itself:
it names one or more Dictionary nodes (data attributes), and each Dictionary
names the Provider node (endpoint definition) that supplies it. When
traversal reaches the node, the fetcher resolves the provider through the
dictionary, makes the call(s), and collects the result set into the node's
"result" property.

Authoring the Dictionary and Provider configuration nodes is covered in
'help data-dictionary' - read that first.

Route name
----------
"graph.api.fetcher"

Properties
----------
```
skill=graph.api.fetcher
dictionary[]={dictionary-node-name}
input[]={source} -> {dictionary-parameter}
output[]={source} -> {target}
```

- dictionary[] (required) - one or more Dictionary node names configured in
  the same graph model. This is the only hard-required property.
- input[] - required whenever the dictionaries declare parameters (the usual
  case). Each entry's TARGET must match a dictionary parameter name exactly,
  or execution fails.
- output[] (optional) - maps the result set onward (e.g. to output.* or
  model.*). Optional because the result set always lands at {node}.result,
  where a later data mapper can pick it up.

Optional:

```
for_each[]={array-source} -> model.{var}   (iterative fetching - see below)
concurrency={1-30}                         (parallel fan-out, default 3)
exception={error-handler-node}             (jump on failure instead of abort)
```

Result set
----------
On success the result set - the values the Dictionary's output[] mappings
produced as result.{key} - is stored at {node}.result. In this node's own
output[] mappings, result.{key} reads from that set; later nodes read
{node}.result.{key}.

Example
-------
```
create node fetcher
with type Fetcher
with properties
skill=graph.api.fetcher
dictionary[]=person-profile
input[]=input.body.person_id -> person_id
output[]=result.name -> output.body.name
output[]=result.address -> output.body.address
```

Iterative fetching (for_each)
-----------------------------
A fetcher can execute once per element of a runtime array - the mechanism
for "fetch details for each item in a list obtained from a previous call":

```
create node accounts-fetcher
with type Fetcher
with properties
skill=graph.api.fetcher
dictionary[]=account-detail
for_each[]=profile-fetcher.result.accounts -> model.account_id
concurrency=3
input[]=input.body.person_id -> person_id
input[]=model.account_id -> account_id
output[]=result.detail -> model.account_details
```

- The for_each source MUST resolve to a list - typically a prior fetcher's
  result ({fetcher}.result.{key}) or a model.* array. Multiple for_each[]
  lines iterate multiple parameters in lock-step.
- Wire the current element into each call with an ordinary input mapping:
  input[]=model.{var} -> {dictionary-parameter}. Non-iterated inputs (like
  person_id above) pass unchanged to every call.
- concurrency bounds the parallel fan-out (1-30, default 3); calls run in
  batches of that size to avoid overwhelming the target service.
- Aggregation is GUARANTEED and ordered: each iteration's result.{key}
  values are appended into a single array on this node's result set - after
  N iterations, result.detail above is an array of N - and the aggregated
  array preserves the source list's order regardless of concurrency.

Failure routing (exception)
---------------------------
On a failed call (HTTP status >= 400):

- {node}.status and {node}.error are set (the engine's error record; {node}.stack is
  added when the failure carries a stack trace)
- the output[] mappings are SKIPPED
- with exception={handler-node}, traversal JUMPS to the handler; without
  it, the run ABORTS and the error is returned to the caller.

When traversal jumps to the handler, the engine also stages a generic exception context that
does not name the failing node:

- error.source  - the failing node's alias
- error.code    - the status code
- error.message - the error message
- error.stack   - the stack trace, when the failure carries one

so ONE handler node can serve the "exception" route of every node in the graph. A generic
handler reads the context in its data mapping without naming any failing node:

```
mapping[]=error.source -> output.body.failed_at
mapping[]=error.code -> output.body.status
mapping[]=error.message -> output.body.message
```

Anchor a shared handler from an island (root -> island -> handler): the handler is reached by
jumping, so the island keeps it non-orphan while plain traversal stops at the island. A handler
node may also connect onward to more nodes for sophisticated recovery (e.g. a graph.task
invoking a composable function). Note that a node is visited at most once per run unless RESET,
so if two parallel branches fail together, only the first jump enters a shared handler. The
alias 'error' is reserved for this namespace - probe it in a dry-run session with
"inspect error".

A retry handler is typically a graph.math decision node that inspects the
fetcher's status/error, counts attempts, and retries with a bound:

```
create node error-handler
with type Decision
with properties
skill=graph.math
statement[]=RESET: {error.source}, error-handler
statement[]=MAPPING: f:defaultValue(model.attempts, int(0)) -> model.attempts
statement[]=MAPPING: f:add(model.attempts, int(1)) -> model.attempts
statement[]='''
IF: {model.attempts} >= 3
THEN: recovery-node
ELSE: next
'''
statement[]=NEXT: {error.source}
statement[]=DELAY: 50
```

The handler is fully GENERIC: every statement command resolves {dynamic variables}, so
RESET: {error.source} and NEXT: {error.source} retry whichever node routed here - one handler
serves every fetcher and task in the graph. See tutorial 12 for the full walkthrough.

RESET comes first among the action statements so it runs on every path (a
taken IF jump ends the list) - the attempt counters live in the "model"
namespace, which RESET never touches. If the handler also carries a defensive
check on the failed node's status, that check must come BEFORE the RESET (it
reads state the reset wipes).

Wire the handler back explicitly (connect error-handler to fetcher with
retry) - no node left unconnected. See 'help graph-math' for the statement
grammar and the engine's loop guard.

Notes
-----
- One Provider call is exactly one HTTP request - redirects are never
  followed. A 3xx answer is a non-failure: its status and body are captured
  and traversal proceeds (only >= 400 triggers failure routing). Point the
  Provider url at the redirect target to land on it.
- {node}.status always carries the HTTP status of the fetch, success
  included (a 200 or a 301 is readable there, not just failures). The
  response.* namespace in a Dictionary output[] addresses the BODY only;
  the bare root (response -> result.page) captures a whole non-JSON body
  such as an HTML page.
- Deduplication: identical requests (same provider + same input values)
  within one graph instance are deduplicated into a single HTTP call. Only
  SUCCESSFUL responses are cached - a failed call is never cached, so a
  retry after RESET makes a real call, while an identical successful call
  reuses the cached response.
- Provider feature[] flags declare capabilities this fetcher must support.
  Built-ins: log-request-headers and log-response-headers - the fetcher
  logs request/response headers into the "header" section of its
  properties. An unsupported feature produces a warning (a custom fetcher
  may enforce it).
- Keep chains minimalist: fetchers can be chained to make multiple API
  calls, but an overly complex chain means slow performance. Take only the
  minimal set of data your application requires - don't abuse the
  flexibility of the API fetcher.
- Wire the Dictionary and Provider nodes into the island knowledge layer so
  no node is left unconnected - see 'help graph-island'.
