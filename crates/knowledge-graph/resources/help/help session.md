Session commands
----------------
The session commands are used for user collaboration.

1. Display current session 
2. Subscribe to a session for collaboration with another user
3. Reset the current session
4. Unsubscribe from another session

Syntax
------

Display current session
-----------------------
```
session
```

For example, when your session is subscribed by another user.
```
> session
Session ws-178443-2 started since 2026-06-02 10:20:32.054
subscribed by [ws-485844-4]
```

Subscribe to another session
----------------------------
```
session subscribe {session-id}
```

e.g.
```
> session subscribe ws-178443-2
Subscribed to ws-178443-2
```

When you subscribe to a session, input commands from you and the other user
will be executed in both the sessions, thus syncing the action and content
of the graph sessions.

If the target session is not a primary session, you will see this error.

```
> session subscribe ws-485844-4
ws-485844-4 is not a primary session
```

The system will also reject your subscription request if you try to subscribe
to yourself.

Reset as a new session
----------------------
```
session reset
```

e.g.
```
> session reset
Session restarted
```

When you reset a session and you are the primary session, all subscribers will be disconnected.
Your session will be cleared but the previous subscribers would retain their own graphs so they can
continue updating them.

Unsubscribe
-----------
```
session unsubscribe
```

e.g.
```
> session unsubscribe
Session unsubscribed from ws-287159-4
```

If you have subscribed to another session, the "unsubscribe" command decouples your session from it.
The graph in your session is retained so that you can continue editing.

If you are the primary session, the system will reject your "unsubscribe" command with an error message
"Nothing to unsubscribe".
