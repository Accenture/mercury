# API playground

A standalone Mercury application that renders OpenAPI 3.0 documents with Swagger UI, wrapped in a
Mercury-branded page. It is a convenient tool for API design and tests, and the natural companion
of a MiniGraph application: a graph that declares its contract answers an OpenAPI document on
demand, and this page loads it in one dialog.

This is the Rust twin of the Java `extensions/api-playground`: the Reactive HTTP server of
`platform-core` serves the page as static content from `resources/public` and the three REST
endpoints below through REST automation, on one port. The page files and the example document are
mirrored byte-for-byte from the Java repository, the reference.

## Before you start

Swagger UI is third-party code that goes stale quickly, so it is not committed here: the
`resources/public/swagger-ui` folder is in `.gitignore`, and a script fetches the latest
release into it. Run one of the two (they do the same thing; Python 3.8+ or Node.js 18+, no
dependencies):

```shell
python3 scripts/fetch-swagger-ui.py
# or
node scripts/fetch-swagger-ui.js
```

Then run the application from the workspace root:

```shell
cargo run -p api-playground
```

The script resolves the latest release of https://github.com/swagger-api/swagger-ui, downloads the
tag's source tarball and extracts the assets of its `dist` folder into `resources/public/swagger-ui`
- all but upstream's own page (`index.html`, `index.css`, `swagger-initializer.js`), which this
application replaces with its own - and records what it fetched in `swagger-ui-version.txt`.
`--version v5.33.1` pins a release and `--target DIR` names another folder. Run it again to
upgrade: the script empties the target folder before it writes, so a fetch never leaves files of
two releases side by side (a non-empty folder that holds no previous fetch is refused rather than
emptied, in case `--target` was mistyped). The application refuses to start with the same
instruction when the folder has not been fetched. A release tarball carries the `dist` files and not the
`.map` source maps (the upstream project marks them `export-ignore`); the page does not need them.

## Running this application

```shell
cargo run -p api-playground
```

Then visit http://127.0.0.1:8200 - the page opens on the sample document `demo.yaml`. The binary
resolves `resources/` against its crate folder and the working directory, so `cargo run` from the
workspace root works as is.

## Loading a document

The header offers four sources; the status line under it says what is loaded.

1. **URL** - enter the URL of an OpenAPI document and press Load. `http://127.0.0.1:8200/?url=...`
   opens the page on that document.
2. **MiniGraph** - a dialog for a MiniGraph application running in dev mode: host (default
   `127.0.0.1`), port (default `8085`) and either a graph ID (a deployed graph,
   `GET /api/openapi/{graph-id}`) or a session ID (a Playground session's draft,
   `GET /api/openapi/session/{session-id}`). The dialog shows the URL it will load and remembers
   the last values in this browser. The engine's dev routes carry the wildcard CORS entry, so the
   browser may read the document, and its `servers` entry points at the engine, so **Try it out**
   posts to the graph.
3. **Specs** - the specs documents this application serves itself: the files of the specs folder below
   and the sample document `demo.yaml`.
4. **Browse**, or **drag and drop** a `.yaml`, `.yml` or `.json` file anywhere on the page. A
   file is a point-in-time copy; a URL is always current.

**Deploy** saves whatever is loaded - a MiniGraph document, a URL's document, a dropped file - into
the specs folder under a name it suggests from the source (`tutorial-3.yaml` for the graph
`tutorial-3`), so the document is served by this application from then on. An existing name is
replaced only after you confirm.

## The specs folder

`api.playground.specs` in `resources/application.yml` names the folder (default `/tmp/api-playground`;
created on the first Deploy). Its yaml, yml and json files appear in the **Specs** list and at
`GET /api/specs/{filename}`, and you may copy files into it by hand as well. A `demo.yaml` in the
folder replaces the sample document. A file name must be a plain name - letters, digits, dot, dash
and underscore.

## Sample REST endpoints

The sample document `demo.yaml` documents the application's own endpoints, declared in `resources/rest.yaml`
and served by the three functions in `src/main.rs`:

```text
GET http://127.0.0.1:8200/api/specs

# the specs documents this application offers: the specs folder's files and the sample document

GET http://127.0.0.1:8200/api/specs/demo.yaml

# one document, as application/yaml or application/json

PUT http://127.0.0.1:8200/api/specs/demo.yaml
Content-Type: text/plain

# deploy a document into the specs folder; 409 when the name exists, unless ?overwrite=true
```

Because the page and the endpoints share one origin, **Try it out** on the sample document works as is.

## Acknowledgements

This application uses the following open source software:
1. Mercury (the Rust port of Mercury Composable) under the Apache 2.0 license - https://github.com/Accenture/mercury/blob/main/LICENSE
2. Swagger UI under the Apache 2.0 license, fetched at build time and not redistributed here - https://swagger.io/license/
3. The "atom" icon of the page header, from Material Design Icons under the Apache 2.0 license - https://pictogrammers.com/library/mdi/
