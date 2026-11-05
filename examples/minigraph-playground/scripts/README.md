# Playground scripts

## The AI companion broker

The AI companion broker, which lets an AI agent **host** a Playground session that humans
subscribe to, lives in one place: [`templates/starter-graph/scripts`](../../../templates/starter-graph/scripts/)
(see its [README](../../../templates/starter-graph/scripts/README.md) for the control API and
the collaboration pattern). Run it from this example's folder against this app's port, `8085`:

```bash
node ../../templates/starter-graph/scripts/ai-companion-broker.mjs --target http://127.0.0.1:8085 --port 8765
```
