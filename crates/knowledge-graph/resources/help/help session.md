Session commands
----------------
Manage your Playground session and collaborate with other users by
subscribing to a primary session, so both users see and drive the same graph.

Syntax
------
```
session                    show this session's id and subscriptions
session subscribe {id}     mirror a primary session into yours
session unsubscribe        detach from the session you subscribed to
session reset              restart your session
```

Example
-------
```
> session
Session ws-178443-2 started since 2026-06-02 10:20:32.054
subscribed by [ws-485844-4]
```

Notes
-----
- 'session' shows the session id and start time, the session you subscribed
  to (if any), and the sessions subscribed to yours.
- Subscribing mirrors commands both ways: input commands from either user
  run in both sessions, keeping the graphs in sync. On subscribe the graphs
  are aligned - if the primary session is empty, your draft is pushed to it;
  otherwise its graph replaces your draft.
- You can subscribe only to a primary session (one that has not itself
  subscribed to another), and never to yourself. If you are already
  subscribed, do 'session reset' before subscribing to another session.
- 'session unsubscribe' decouples your session from the one you subscribed
  to; your graph is retained so you can continue editing. A primary session
  gets "Nothing to unsubscribe".
- 'session reset' restarts your session. As a primary session it disconnects
  all subscribers (they keep their own graphs); as a subscriber it
  unsubscribes first. It resets subscriptions but does NOT clear your draft
  graph - the UI restores the draft when it reconnects. To start clean,
  delete the nodes explicitly (see 'help delete').
- The companion REST endpoints reject 'session subscribe', 'session
  unsubscribe' and 'session reset': a companion is an assistant to a
  session, not a session of its own. Only the read-only 'session' status
  query works there - session topology is managed from a
  WebSocket-connected session (the browser console) only.
