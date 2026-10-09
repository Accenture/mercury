Graph schema
------------
Declare the graph's request and response as a 'schema' property on the root
and end nodes, in a closed subset of OpenAPI 3.0. The engine validates every
request at the root, answers an OpenAPI 3.0 document on demand, and the
Schema panel fills the declaration in from discovery and from the last run.
This topic explains the property, the panel and the endpoints; there is no
console command behind it.

The declaration
---------------
- The root node's 'schema' describes the request: 'schema.body' for
  input.body and 'schema.header' for input.header. The end node's 'schema'
  describes the response (output.body, output.header) and is documentary -
  it is never enforced.
- Both parts are ordinary nested properties, written with the composite keys
  of 'create node' and 'update node':

```
update node root
with type Root
with properties
schema.body.type=object
schema.body.required[]=amount
schema.body.properties.amount.type=number
schema.body.properties.amount.minimum=0
schema.body.properties.currency.type=string
schema.body.properties.currency.enum[]=USD
schema.body.properties.currency.enum[]=EUR
schema.header.required[]=X-Tenant
schema.header.properties.X-Tenant.type=string
```

- The vocabulary is closed: type (object, array, string, number, integer,
  boolean), properties, required, items, enum, minimum, maximum,
  exclusiveMinimum, exclusiveMaximum, minLength, maxLength, pattern,
  minItems, maxItems, nullable and additionalProperties; title, description,
  example and format are documentary. The deployment gate - and the
  pre-run check before 'run' - refuses any other keyword, a keyword that
  does not apply to the declared type, and a malformed value, naming the
  location: "node root - schema.body.properties.amount: unknown keyword
  'min' - ...".
- Types are strict JSON types: a numeric string is not a number; money is
  a string with a pattern. A header is text, so an integer header validates
  the parsed text, and header names match case-insensitively.

Validation at the root
----------------------
- When the root carries a 'schema', every run validates input.body and
  input.header against it first - before the root's own skill or anything
  else - through the built-in function graph.schema.validator. The step is
  assumed: nothing is written into the node for it.
- Success prints "Input validated by graph.schema.validator in N ms".
  Failure aborts the run with every violation in one message, at most ten:
  "Graph traversal aborted: Input validation failed - input.body.amount:
  expected number, got string; input.header.X-Tenant: required (node root)".
  A deployed graph answers HTTP 400 with the same message; a root
  'exception=' handler takes over with error.message, error.source and
  error.code.
- A dry run validates exactly as a deployed run does: 'instantiate graph',
  Upload the mock body and the Headers rows, then 'run'.

The panel
---------
Open it from the Tools menu, "Graph schema", or from the root or end node
editor. It takes the console's place; Esc or Cancel gives it back, and
unsaved rows survive a close.

- Two tabs: Input (node root) and Output (node end). Each has a Body
  section, one row per path (items[] for a list), and a Headers section,
  one row per header name: path or name, type, req, description, example,
  and a chip - declared, discovered (the tooltip names the referencing
  nodes), from last run, or new. A second chip summarizes the keywords the
  panel does not edit (enum, minimum, pattern, ...); they are kept as
  declared and edited in the node editor.
- The rows are pre-filled from the engine's view of the draft: what the
  model references, merged with what the nodes declare. The engine's
  declaration-versus-model issues are listed above the rows.
- "Fill from last run" types the rows from the instance's actual input and
  output after a dry run - the only way to type a task's or a fetcher's
  result. "Reload" discards unsaved rows.
- "Save root schema" / "Save end schema" writes the declaration through one
  'update node', so every member of the session sees it; what the gate
  would refuse is flagged in place first.
- "Download YAML" saves the draft's OpenAPI document.

The endpoints
-------------
Dev mode only:

```
GET /api/openapi/{graph-id}                    # a deployed graph, YAML attachment
GET /api/openapi/{graph-id}?format=json        # the same as JSON
GET /api/openapi/{graph-id}?view=contract      # the derived contract with its evidence
GET /api/openapi/session/{sessionId}           # the same views for this session's draft
```

The document holds POST /api/graph/{graph-id} with the request body, the
header parameters, the 200 response with its body and headers, every staged
output.status, and the error shape; it is derived from the model each time
and never stored. Paste the URL into Swagger UI's explore bar to try the
graph from there. 'describe graph {graph-id}' prints the same surface as
text, with [declared] on a path only the declaration knows.

See also 'help describe', 'help upload', 'help run' and the guide
"The graph contract".
