- [ ] **A flow reads the request's cookies and session values (Increment 211); the pull request from branch
  `fix/flow-input-cookie-session` waits for the maintainer's merge.** The flow adapter copied seven of the eleven keys
  the Java `HttpToFlow` passes; it now passes `cookie` and `session` as well, each always a map. Opened on the
  maintainer's word of 2026-11-24, "Please create a new one for the additional work".
  - **Next:** the merge is the maintainer's; record the pull request's number and the merge here when it lands.
  - **Still open after it:** `stream` and `filename` cannot be passed until the HTTP edge has upload streams and
    multipart upload (a platform-core deferral, not started). `export` always writes the model file where the Java
    engine skips an identical one; no ruling yet on whether to match it.
  - origin: 2026-11-24-050812
  → serves: vision-mercury
  <!-- id: flow-input-cookie-session | created: 2026-11-24 | last_used: 2026-11-24 | uses: 1 | tier: working | origin: 2026-11-24-050812 -->
