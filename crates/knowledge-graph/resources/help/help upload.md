Upload mock data
----------------
Print the URLs for uploading mock input to the current graph instance - a
JSON payload as 'input.body', and optional request headers as 'input.header'
- convenient when the mock input is too large to seed line by line, or when
the graph reads a header.

Syntax
------
```
upload mock data
```

Example
-------
```
> upload mock data
You may upload JSON payload -> POST /api/mock/{name} (mock headers, a JSON object of text values -> POST /api/mock/{name}?namespace=header)
```

Notes
-----
- Requires a graph instance (see 'help instantiate').
- An HTTP POST of a JSON payload (a map or a list) to the first URL replaces
  the instance's 'input.body'; the console confirms with "Mock data loaded
  into 'input.body' namespace".
- An HTTP POST of a JSON object of text values to the same URL with
  '?namespace=header' replaces the instance's 'input.header'; the console
  confirms with "Mock data loaded into 'input.header' namespace". Header
  names are kept as given and read case-insensitively by the graph, exactly
  as a real request's headers are. Any other namespace, or a header payload
  that is not an object of text values, is refused (HTTP 400).
- To seed model variables, or small inputs line by line, use the
  'instantiate graph' command (see 'help instantiate'): its
  '{value} -> input.body.{key}' and '{value} -> input.header.{name}' lines
  do the same by hand.
- In a collaborative session (see 'help session') an uploaded payload loads
  into every member's graph instance - the primary and all its subscribers -
  and each member's console confirms it. In the Playground UI the toolbar's
  Upload button opens the upload form for your own session only, with the
  JSON body and optional header rows; uploading is optional (a graph that
  reads no input runs without it).
