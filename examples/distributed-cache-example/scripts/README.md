# Playground scripts

## The AI companion broker

The AI companion broker, which lets an AI agent **host** a Playground session that humans
subscribe to, lives in one place: [`templates/starter-graph/scripts`](../../../templates/starter-graph/scripts/)
(see its [README](../../../templates/starter-graph/scripts/README.md) for the control API and
the collaboration pattern). Run it from this example's folder against this app's port, `8305`
(`rest.server.port`; the broker's built-in default, `8085`, is the minigraph-playground example's):

```bash
node ../../templates/starter-graph/scripts/ai-companion-broker.mjs --target http://127.0.0.1:8305 --port 8765
```

The graph to co-author here is **`profile-cache`** — bring it into a session with
`import graph from profile-cache`, edit it, dry-run it with `run`, then `export graph as
profile-cache` and restart the app to redeploy it through the CompileGraph gate.
