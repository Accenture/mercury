- [x] **A flow reads the request's cookies and session values, on `main` as Increment 211 — DONE 2026-11-24.** Pull
  request #421 (commit `7ed34591`, merge `e9341da3`): the flow adapter copied seven of the eleven keys the Java
  `HttpToFlow` passes and now passes `cookie` and `session` as well, each always a map; pinned through the real HTTP
  edge by the fixture `input-cookie-session`. Still open, with no thread: `stream` and `filename` wait for upload
  streams and multipart upload at the HTTP edge, and `export` always writes the model file where the Java engine skips
  an identical one. Lesson: a module note that calls something deferred goes stale when the deferral lands elsewhere;
  check the note against the code before writing it into a guide. origin: 2026-11-24-050812.
  → serves: vision-mercury
  <!-- id: flow-input-cookie-session | created: 2026-11-24 | last_used: 2026-11-24 | uses: 1 | tier: working | origin: 2026-11-24-050812 -->
