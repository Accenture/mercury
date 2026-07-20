/**
 * URL helpers
 *
 * Single source of truth for constructing backend URLs.  All other files
 * import from here instead of inlining the same `import.meta.env.DEV` logic.
 *
 * WebSocket URLs
 * ──────────────
 * In dev the Vite dev-server proxies  /ws  →  ws://localhost:8085
 * but the browser must target the proxy on port 3000, so we use:
 *   ws://localhost:3000<wsPath>
 * In production the server is on the same origin, so:
 *   ws://<host><wsPath>
 *
 * HTTP/API URLs
 * ─────────────
 * Always relative (empty string prefix).  The Vite proxy forwards /api,
 * /info, /health, and /env to http://localhost:8085 in dev; in production
 * the same-origin server serves them directly.
 */

/**
 * Returns the full WebSocket URL for a given path.
 *
 * @param wsPath  e.g. "/ws/json/path" or "/ws/graph/playground"
 */
export function makeWsUrl(wsPath: string): string {
  return import.meta.env.DEV
    ? `ws://localhost:3000${wsPath}`
    : `ws://${window.location.host}${wsPath}`;
}
