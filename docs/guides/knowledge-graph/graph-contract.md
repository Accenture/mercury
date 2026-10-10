---
title: The graph contract
summary: Declare a graph's request and response as a `schema` on the root and end nodes, a
  closed subset of OpenAPI 3.0, and put it to three uses - the engine validates every request at
  the root, answers an OpenAPI 3.0 document on demand, and the Playground's Schema panel fills the
  declaration in from discovery and from the last run.
layer: knowledge-graph
audience: [developer, architect, ai-agent]
keywords: [graph contract, schema, openapi, input validation, schema panel, describe graph, swagger, api playground]
related:
  - guides/knowledge-graph/command-reference.md
  - guides/knowledge-graph/ai-agent-guide.md
  - guides/knowledge-graph/playground-and-companion.md
  - guides/configuration-reference.md
---

# The graph contract

> **At a glance**
>
> - A graph declares its contract as an optional **`schema` property on the root node** (the
>   request: `schema.body` for `input.body`, `schema.header` for `input.header`) and **on the end
>   node** (the response: `output.body`, `output.header`). Each part is a schema object in a
>   **closed subset of OpenAPI 3.0**; the deployment gate refuses anything outside it.
> - **Three uses.** When the root carries a `schema`, every run validates the request at the root
>   before anything else runs. `GET /api/openapi/{graph-id}` answers a minimal **OpenAPI 3.0
>   document** derived from the model on demand. The Playground's **Schema panel** starts from what
>   the engine discovers in the model and from the last dry run, so a contract is confirmed rather
>   than typed.
> - The declaration is **data on the graph**: it travels with export, import, pack and
>   `graph.extension`, needs no code, and the engine assumes the validation step - nothing is
>   written into the node for it.

## Declare the contract {#declare}

The root node describes the request and the end node the response - the node says which side it is.
Each `schema` has two optional parts that mirror the state machine's namespaces:

| Part | Describes | Shape |
|---|---|---|
| `schema.body` | `input.body` (root) or `output.body` (end) | any schema object of the vocabulary below |
| `schema.header` | `input.header` (root) or `output.header` (end) | an object schema whose `properties` are header names; a header is text |

The parts are ordinary nested properties, written through the node grammar's composite keys - so
`create node`, `update node`, `edit node`, the node editor and the Schema panel all carry them with
no new syntax:

```
update node root
with type Root
with properties
name=payment
purpose=A root schema validates the request before anything runs
schema.body.type=object
schema.body.required[]=amount
schema.body.required[]=currency
schema.body.properties.amount.type=number
schema.body.properties.amount.minimum=0
schema.body.properties.amount.exclusiveMinimum=true
schema.body.properties.amount.description=the amount to charge
schema.body.properties.currency.type=string
schema.body.properties.currency.enum[]=USD
schema.body.properties.currency.enum[]=EUR
schema.body.properties.note.type=string
schema.body.properties.note.maxLength=40
schema.body.properties.note.nullable=true
schema.header.required[]=X-Tenant
schema.header.properties.X-Tenant.type=string
schema.header.properties.X-Tenant.minLength=4
schema.header.properties.X-Retry.type=integer
schema.header.properties.X-Retry.minimum=0
```

A value the grammar stores as text (`minimum=0`, `nullable=true`) is read as the keyword expects. In
the exported JSON the same declaration is the nested object you would write by hand. The end node's
`schema` is **documentary**: it describes the response in the OpenAPI document and is never enforced,
headers included.

## The vocabulary {#vocabulary}

The vocabulary is a closed subset of OpenAPI 3.0 keywords, and the gate refuses any other keyword:

| Keywords | Applies to |
|---|---|
| `type` - `object`, `array`, `string`, `number`, `integer`, `boolean` | every schema; a header is `string`, `number`, `integer` or `boolean` |
| `properties`, `required`, `additionalProperties` | `object` |
| `items`, `minItems`, `maxItems` | `array` |
| `enum`, `nullable` | any scalar (`nullable` not on a header) |
| `minimum`, `maximum`, `exclusiveMinimum`, `exclusiveMaximum` | `number`, `integer` |
| `minLength`, `maxLength`, `pattern` | `string` |
| `title`, `description`, `example`, `format` | documentary - carried, never validated |

The rules that follow from it:

- **Strict JSON types.** A numeric string is not a number and `true` is not `1`; an integral float
  (`2.0`) counts as an integer. Money is `type: string` with a `pattern`, as the
  [`DECIMAL` statement](skills-reference.md#math-decimal) already advises - a JSON number is a double
  on the wire.
- **A header is text.** `type: integer`, `number` or `boolean` validates the parsed text (`"12"`
  passes `integer`, `"many"` fails). Header names match **case-insensitively**, the engine's rule for
  `input.header.*`; the declared spelling is kept in the document. `properties`, `required`, `items`,
  `minItems`, `maxItems`, `additionalProperties` and `nullable` do not apply to a header.
- **`pattern`** runs after `maxLength`, in the regular-expression subset common to the Java and the
  Rust engine: lookaround, atomic groups, backreferences and possessive quantifiers are refused at
  the gate, and character classes are Unicode-aware on both engines. A pattern comes from the author,
  never from the caller.
- **`additionalProperties`** keeps OpenAPI's default: extra fields pass unless the author declares
  `false` (or a schema for them). Lengths count code points.
- **At most ten violations** are reported per message, with the rest counted (`; and 3 more`).

The deployment gate (`CompileGraph`, the graph packager, the graph-set loader and the Playground's
pre-run check) compiles both declarations and refuses the model when one is malformed, naming the
location:

```
Rejected graph payment - node root - schema.body.properties.amount: unknown keyword 'min' - the vocabulary is type, properties, required, items, enum, minimum, maximum, exclusiveMinimum, exclusiveMaximum, minLength, maxLength, pattern, minItems, maxItems, nullable and additionalProperties (title, description, example and format are documentary)
Rejected graph payment - node root - schema.header.properties.X-Count: 'pattern' does not apply to type integer
```

A keyword that cannot apply to the declared type is refused like an unknown one, because a constraint
the validator would silently ignore teaches that *unflagged means safe*. A **mismatch** between the
declaration and the model's data surface - a declared path the model never reads, or a read path the
declaration lacks - is a WARN at deploy and pack time (`Graph payment - input.body.note is declared but
never referenced by the model`), never a refusal: a declaration may describe what a whole-body
passthrough forwards.

## Discovery and `describe graph` {#discovery}

The engine derives the contract from the model in three tiers and merges the declaration over it:

1. **The path scan** - every `input.*` and `output.*` reference in mappings, statements, `for_each`
   sources and JSONPath `$.input.body...` expressions, with nesting and the array markers `[]`, `[*]`
   and `[0]`.
2. **Direct evidence** - typed constants and wrappers (`int(…)`, `boolean(…)`, `text(…)`), plugins with
   a known result (`f:now` is a string, `f:listOfMap` a list), `for_each` sources (lists), `graph.math`
   results and operands (arithmetic and ordered comparisons are numbers; a boolean is never a number).
3. **One hop** through a typed `model.*` or `{node}.result.*` variable, and a `graph.extension`
   target's declared `schema.body` as its `result`.

The declaration wins: a discovered path the declaration lacks is appended untyped and flagged, and a
declared path the model never references is kept and flagged. `describe graph {graph-id}` prints the
merged surface with its types, `[declared]` on a path only the declaration knows, every staged
`output.status`, and the document's URL:

```
Input surface:
  input.body.amount (number)
  input.body.currency (string)
  input.body.note (string) [declared]
  input.header.X-Tenant (string)
  input.header.X-Retry (integer)
Output surface:
  output.body.charged (number)
  output.body.currency (string)
  output.body.status (string)
Declared schema: input
OpenAPI document: GET /api/openapi/payment
```

The same derivation is what the Schema panel and an agent read as JSON through
`?view=contract` (below): `input.body.paths[]` with `path`, `type`, `origin` (`declared`,
`discovered` or `both`), `required` and `usedBy`, and `issues[]` with the mismatches. Query
parameters are not part of a graph's API - `/api/graph/{graph-id}` is one endpoint URI for every
graph - so they are neither declared nor derived.

## The OpenAPI document on demand {#openapi}

Dev-mode routes (`app.env=dev`, in the four `rest.yaml` copies the starter and the examples carry):

| Route | Answers |
|---|---|
| `GET /api/openapi/{graph-id}` | the document of a **deployed** graph, as a YAML attachment named `{graph-id}.yaml` |
| `GET /api/openapi/{graph-id}?format=json` | the same document as JSON inline |
| `GET /api/openapi/{graph-id}?view=contract` | the derived contract with its evidence, as JSON |
| `GET /api/openapi/session/{sessionId}` | the same three views for a session's **draft**, named after its root node's `name` (else `draft`) |

The document is derived from the model each time it is requested and never stored. It holds the graph's
one endpoint, `POST /api/graph/{graph-id}`, with the request body schema, the declared or discovered
`input.header.*` as header parameters (a declared header carries its `required` and its schema), the
`200` response with its body schema and `output.header.*` as response headers, every status the model
stages with `int(N) -> output.status` as a response code, the engine's error shape
(`{type, status, message}`) under `default`, `info` from the root node's `purpose` and the deployed
set's version (else the application's `info.app.version`), and `servers` from the request's Host
header, so a downloaded file points back at the engine that generated it:

```bash
curl -sS -o payment.yaml -D - http://127.0.0.1:8085/api/openapi/payment
#   Content-Type: application/yaml; charset=utf-8
#   Content-Disposition: attachment; filename="payment.yaml"
curl -sS 'http://127.0.0.1:8085/api/openapi/payment?view=contract' | jq '.issues'
```

An unknown graph or session answers 404; an unknown `format` or `view` answers 400.

## Input validation at the root {#validation}

When the root node carries a `schema`, `input.body` and `input.header` are validated against the root
node's schema as the first thing at the root on every run, and a failed validation never reaches the
root's own skill. The step is **assumed** by the engine - the walker sends one request to the
built-in function `graph.schema.validator` with `{body, header, schema}` and nothing is written into
the node, so there is no validator line to mistype and no second source of truth. A resumed
traversal (`graph.resume`) never validates again.

- **Success** is one line in the traversal log and on the Playground console:
  `Input validated by graph.schema.validator in 2 ms`.
- **Failure** is the standard task error path. The deployed endpoint answers **400** with every
  violation in one message:
  ```json
  {"type": "error", "status": 400,
   "message": "Input validation failed - input.body.amount: expected number, got string; input.body.currency: must be one of USD, EUR; input.body.note: must be at most 40 characters; input.header.X-Tenant: required"}
  ```
  The Playground prints `Graph traversal aborted: Input validation failed - ... (node root)`.
- **A root `exception=` handler** takes over a failed validation with the generic exception context,
  `error.message`, `error.source` (`root`) and `error.code` (`400`), so a graph can answer in its own
  shape - the shipped test graph answers 422 with a `reason`.
- The application property **`graph.schema.validator`** names a substitute function with the same
  contract ([configuration reference](../configuration-reference.md#graph-schema-validator); set it
  for one run with the `-D` program argument); a
  direct caller gets `{valid: true}` or a 400 whose error is the message above, and a refused
  schema `Invalid schema - schema.body: unknown keyword 'min' - ...`.

A dry run in the Playground validates the input exactly as a deployed run does: `instantiate graph`,
Upload the mock body and headers, `run`, and the console shows the same line or the same refusal. The
Upload step's **Headers** rows (`POST /api/mock/{sessionId}?namespace=header`) are how a dry run
satisfies a header requirement.

Validation is a **contract check, not business logic**: it says what a well-formed request is. A rule
such as "a refund may not exceed the charge" belongs to a `graph.math` node that stages
`int(400) -> output.status`, as before.

## The Schema panel {#panel}

**Tools → Graph schema** - or the "Open the Schema panel…" link in the root or end node editor -
opens the panel in the console's slot. It has an **Input** tab (the root's `schema`) and an **Output**
tab (the end's), each with a body section (one row per path) and a header section (one row per
header name): the path or name, the type (an array also has an item type), **req**, the description,
an example, and a chip saying where the row came from:

| Chip | Meaning |
|---|---|
| `declared` | on the node; the tooltip names the nodes that also reference the path |
| `discovered` | referenced by the model, not declared yet |
| `from last run` | typed from the instance's actual value |
| `new` | added in the panel, saved with the declaration |

A second chip summarizes the keywords the panel does not edit (`enum (2) · minimum 0 · pattern`);
they are carried through a Save unchanged and edited in the node editor. The rows are pre-filled from
the contract view; **Fill from last run** reads the instance's `input.body`, `input.header`,
`output.body` and `output.header` after a dry run and fills the gaps - the only way to type a
`graph.task`'s or a fetcher's result. **Save** writes the side's declaration through one `update node`
command, so every member of a collaborative session sees it and an agent does the same by command;
**Download YAML** saves the draft's document. What the gate or the grammar would refuse is flagged in
place before anything is sent, and the engine's declaration-versus-model issues are listed above the
rows. See `help schema` in the Playground.

## Round trip through the API playground {#api-playground}

The document is written for the tooling the field already uses - Swagger UI and OpenAPI 3.0
generators. With the [API playground](https://github.com/Accenture/mercury-composable/tree/main/extensions/api-playground)
of the Java repository (Swagger UI served by a Mercury app; any Swagger UI works the same) and a
MiniGraph application in dev mode (`cargo run -p minigraph-playground`):

1. **Load the engine's URL directly.** Paste `http://127.0.0.1:8085/api/openapi/payment` into the
   Swagger UI explore bar. The dev routes carry the wildcard CORS entry, so the browser may read it,
   and `servers` points at the engine, so **Try it out** posts the request body and the header
   parameters to `POST /api/graph/payment` on the engine - a bad request shows the 400 above.
2. **Or keep the file.** Download `payment.yaml` (the attachment, or the Schema panel's **Download
   YAML**), copy it into the API playground's `resources/sample/yaml` folder (or the externalized
   folder its `static-locations` names), and `GET /api/specs` lists it beside `demo.yaml`;
   `http://127.0.0.1:8200/yaml/payment.yaml` loads it. A file kept this way is a point-in-time
   export; the engine's URL is always current.
3. **Round trip.** Correct the graph in the Playground - a type, a `required`, a new path - Save in the
   Schema panel, and reload the explore bar: the document follows the draft at
   `/api/openapi/session/{sessionId}`, and the deployed graph's at `/api/openapi/{graph-id}` after
   the next deploy.

## Design rules {#design-rules}

- **Confirm, don't type.** Start from `describe graph` or the panel's discovery, then declare the
  types the model does not reveal - a task's or a fetcher's result - from a dry run.
- **Declare what you validate.** A declared path the model never reads is a warning for a reason:
  a contract wider than the model is a promise the graph does not keep.
- **Keep the response documentary.** The end node's `schema` describes; the engine never refuses its
  own response.
- **Money is text.** `type: string` with a `pattern`, validated at the root and computed with
  `DECIMAL:`.
- **Validation is not rules.** A business rule stays a `graph.math` statement with a staged status;
  the schema says what a request *is*, not whether it *may* proceed.

## See also {#see-also}

- [MiniGraph command grammar](command-reference.md#describe) - `describe graph` and the node grammar
  the declaration is written in.
- [AI agent guide](ai-agent-guide.md#contract-schema) - the recipe an agent follows to declare a
  contract through the companion endpoint.
- [Playground & AI companion](playground-and-companion.md) - the Schema panel among the Tools.
- [Configuration reference](../configuration-reference.md#graph-schema-validator) -
  `graph.schema.validator`.
- [Built-in skills reference](skills-reference.md#math-decimal) - the `DECIMAL` statement for exact
  money arithmetic.

---

*Adapted from the mercury-composable guide `knowledge-graph/graph-contract.md`; behavior verified
against this engine's copies of the shared vector files `graph-contract-vectors.json` and
`graph-schema-vectors.json` (Increments 171 and 172) and the Playground bundle it serves (Increment 173).*
