Upload mock data
----------------
Print the URLs for uploading mock input to the current graph instance - a
JSON payload as 'input.body', optional request headers as 'input.header' and
optional model variables - convenient when the mock input is too large to
seed line by line, or when the graph reads a header or a model variable that
a request's edge would set.

Syntax
------
```
upload mock data
```

Example
-------
```
> upload mock data
You may upload JSON payload -> POST /api/mock/{name} (mock headers, a JSON object of text values -> POST /api/mock/{name}?namespace=header; model variables, a JSON object -> POST /api/mock/{name}?namespace=model)
```

Notes
-----
- Requires a graph instance (see 'help instantiate').
- An HTTP POST of a JSON payload (a map or a list) to the first URL replaces
  the instance's 'input.body'; the console confirms with "Mock data loaded
  into 'input.body' namespace".
- An HTTP POST of a JSON object of text values to the same URL with
  '?namespace=header' replaces the instance's 'input.header'; the console
  confirms with "Mock data loaded into 'input.header' namespace". The headers
  are processed as a real request's are: every name is stored in lower case
  (two names that differ only by case are refused), CR and LF are removed
  from a value, the business correlation-ID header (the property
  'http.correlation.id.header', X-Correlation-Id by default) sets
  'model.cid' - without one, the header carries the instance's 'model.cid',
  as a request always does - and an 'x-iteration-index' header sets
  'model.iteration_index', as the deployed walker lifts it for a subgraph's
  for_each iteration. The console names each value it derives, for example
  "model.cid = order-1001 (from input.header.x-correlation-id)".
- An HTTP POST of a JSON object to the same URL with '?namespace=model' sets
  each key as a model variable ('tier' becomes 'model.tier'; a dotted or
  indexed key such as 'limits.daily' or 'items[0]' sets a nested value) and
  leaves the other model variables in place; the console confirms with "Mock
  data loaded into 'model' namespace". A request does not set model
  variables - the edge and the graph do - so a dry run mocks them here. A
  key with an empty path segment, or a null value, is refused.
- Any other namespace, or a payload of the wrong shape, is refused (HTTP
  400).
- The 'instantiate graph' command (see 'help instantiate') seeds the same
  namespaces line by line: its '{value} -> input.body.{key}',
  '{value} -> input.header.{name}' and '{value} -> model.{key}' lines write
  the state machine directly, without the request processing above.
- In a collaborative session (see 'help session') an uploaded payload loads
  into every member's graph instance - the primary and all its subscribers -
  and each member's console confirms it. In the Playground UI the toolbar's
  Upload button opens the upload form for your own session only, with the
  JSON body, optional input header rows and optional model variable rows
  (each with a type: text, number, boolean or JSON); the form posts the body,
  then the headers, then the model variables, so a model variable it names
  wins over one a header derives. Uploading is optional (a graph that reads
  no input runs without it).
