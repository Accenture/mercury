Upload mock data
----------------
Print a URL for uploading a JSON payload as the mock 'input.body' of the
current graph instance - convenient when the mock input is too large to seed
line by line.

Syntax
------
```
upload mock data
```

Example
-------
```
> upload mock data
You may upload JSON payload -> POST /api/mock/{name}
```

Notes
-----
- Requires a graph instance (see 'help instantiate').
- An HTTP POST of a JSON payload to the given URL replaces the instance's
  'input.body'; the console confirms with "Mock data loaded into
  'input.body' namespace".
- Only 'input.body' can be uploaded. To mock input headers or model
  variables, seed them with the 'instantiate graph' command before
  uploading.
- In a collaborative session (see 'help session') the uploaded payload
  loads into every member's graph instance - the primary and all its
  subscribers - and each member's console confirms it. In the Playground UI
  the toolbar's Upload button opens the upload form for your own session
  only; uploading is optional (a graph that reads no input runs without it).
