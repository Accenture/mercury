# WebSocket notificaton use case and sample application

WebSocket is usually employed as a notification channel to the browser so that your service can detect "presence" of 
the user and asynchronously send notification events to the browser.

The REST automation helper application supports this websocket notification use case. The sample rest.yaml 
configuration file contains a websocket routing entry to the sample.ws.auth and ws.notification services.

The sample.ws.auth is the authentication service so that your backend application
can validate if the incoming websocket is associated with an authenticated user
session.

The ws.notification is the websocket service that receives incoming requests and save a mapping of the 
user's websocket outgoing paths that your backend services can send notification to.

This supports multi-device user sessions. When a user makes a request to change something, the backend services can 
send notification events to all connected devices of the same user.

To test websocket notification, 
1. Load the event node (platform as a box), 
2. Start the rest-automation application
3. Run this websocket-notification example application
4. Visit the rest-automation app at http://127.0.0.1:8100/ws.html and use "notification:demo-token" or "notification:admin-token" as input.
5. Go to http://127.0.0.1:8083/api/ws/notify/{user}?message={your test message} where "user" is "demo" or "admin"
6. The test message will be broadcast to user's websocket connections

## Sending notification events

The following code asks the notification for a list of outgoing websocket paths for a given user. 
It then sends notification event to the user using the outgoing paths.

If your notification event is a Map, it will be send as JSON to the browser.
Alternatively, you may send text or byte messages directly.

```java
    PostOffice po = PostOffice.getInstance();
    EventEnvelope list = po.request("ws.notification", 5000,
                                     new Kv("type", "get_path"), new Kv("user_id", user));
    if (list.getBody() instanceof List) {
        List<String> paths = (List<String>) list.getBody();
        // the list contains the outgoing paths to the websocket connections of the user
        Map<String, Object> event = new HashMap<>();
        event.put("message", message);
        event.put("time", new Date());
        for (String p: paths) {
            po.send(p, event);
        }
    }
```

## Using this template

HelloNotification, InitialLoad, UserChannels and UserSession are designed to be reusable for generic websocket notification.
You should be able to use them out-of-the-box without modification.

HelloAuthentication is a place-holder. Please change the "getUser" method accordingly.

DemoBroadcast is a REST endpoint to demonstrate how to send notification events. Please remove this in production.
