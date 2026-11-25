Hot deployment of graph sets
============================
Deploy a certified graph set to this running engine without a restart, and
undeploy it by name (RFC-0017). The Playground's "Graph set packaging" panel
has a Deploy view for it in dev mode; two Node scripts in the starter
template do the same for an engine that has no Playground; and three REST
entries serve both.

A hot-deployable set
--------------------
- Exactly one main graph and zero or more subgraphs, and the set is named
  after its main graph: the set name is the main graph's id, so the pack
  file, the record, the undeploy path and the REST path are one string.
  'graph_id' in the manifest, when present, must equal it.
- The main graph is served at POST /api/graph/{graph-id}; its subgraphs are
  private - reached from the set's own graphs through graph.extension, and
  404 at the graph and OpenAPI endpoints and to another set's extension.
- A graph id is unique across the engine: a set that holds a bundled id, or
  an id another hot-deployed set holds, is refused (409) with the owner
  named. Reuse across sets is by reference to a set of its own, never by
  containment; a composable function, by contrast, is reused by every graph
  that names its route.
- A re-upload of the same set name replaces the set all or none; the same
  bytes again change nothing.

The Deploy view
---------------
Open it from the Tools menu: "Graph set deployment". It is the packaging
panel's third view, so "Deploy…" in that panel and "Deploy to this engine"
in its inspect view (a dropped .pack) reach the same place.

- "Pack and deploy" packs the assembled set on the engine and deploys the
  bytes in one step; a dropped .pack is deployed as it is.
- The package's SHA-256 is computed in the browser and shown, so the digest
  can be written to the field's ledger from the same screen. An expected
  digest pasted from the ledger is sent as X-Graph-Set-SHA256, and the
  engine refuses other bytes. Send it: without the header the engine
  accepts any well-formed package, and bytes altered inside a string value
  still read as one.
- The answer shows the set, its version, its graphs, what was replaced, and
  an advisory, never a refusal: the task routes the set names (known here,
  unknown, or remote through server.url) and the graph.extension targets
  outside the set (known, unknown, or another set's private subgraph).
- "Deployed sets" lists the folder's records with a status per set;
  "Undeploy" asks before it removes one.
- When the engine's switches are off, its entries are not served (404) and
  the view says so, naming graph.hot.deploy and graph.hot.undeploy. A 503
  means a switch is on but the engine refused its folder at startup; its
  log says why.

Without the Playground
----------------------
In the starter template's scripts folder, zero dependencies, Node.js 26 LTS
(minimum 22):

```bash
node scripts/deploy-graph-set.mjs --engine https://gateway.example.com/engine \
     --file settlement.pack --sha256 9f2c... --header "authorization: Bearer $DEPLOY_TOKEN"
node scripts/undeploy-graph-set.mjs --engine https://gateway.example.com/engine --graph settlement
node scripts/undeploy-graph-set.mjs --engine https://gateway.example.com/engine --list
node scripts/deploy-graph-set.mjs --file settlement.pack --digest
```

The answer prints as the engine gives it; the exit code is 0 on success, 1
on a refusal with the reasons on stderr, 2 on a usage error.

The engine side
---------------
- Three properties, the switches off by default: graph.hot.deploy (the
  deploy entry), graph.hot.undeploy (the undeploy entry), either the list
  entry; location.graph.deploy (default file:/tmp/deploy), a file:/ folder
  the application can write, outside the Playground's temp folder and every
  manifest's 'unpack' folder - mount it as a persistent volume to survive a
  restart. graph.hot.deploy.throttle (ms, default 0) is the least time
  between two reads of a set's record.
- The folder holds, per set, the package, one file per graph and the
  engine-written record <id>.deployed.yaml. A package copied into the
  folder by hand is ignored with a warning: the records are the allowlist,
  so the deploy entry is the only door at run time, as the manifest is at
  startup.
- Every instance over the same folder converges on demand, with no poll: a
  lookup for a hot-deployed graph reads its set's record and reconciles the
  set when the record changed or is gone; a lookup for an unknown id checks
  whether a record of that name exists; a restart re-registers every
  recorded set from the bytes its digest vouches for.
- A node that consumes a task route (graph.task, graph.suspend,
  graph.resume) may name its function's provider: server.url, the
  provider's Event-over-HTTP endpoint (/api/event supplied when the URL has
  no path), and server.header[] for a gateway's token - so a hot-deployed
  graph reaches a function the static Event-over-HTTP map has never heard
  of. See 'help graph-task'.

See also 'help package', 'help export' and the guides: the configuration
reference and the AI agent guide's deployment section.
