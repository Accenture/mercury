/*

    Copyright 2018-2020 Accenture Technology

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.

 */

package org.platformlambda.core.system;

import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;

import java.io.IOException;
import java.util.Collections;
import java.util.Map;
import java.util.concurrent.TimeoutException;

/**
 * This is a convenient class for creating an event stream.
 * It does not implements "Closeable" because it encapsulates both input stream and output stream.
 * Both input stream and output stream are closeable.
 * The close method for output stream will send EOF to the stream.
 * The close method for input stream will close the event stream and release resources.
 * The event stream would spread across the network where sender and recipient are mostly in different machine.
 *
 * The typical use case is to create a new event stream using ObjectStreamIO() in the receiving side.
 * The recipient can obtain the fully qualified route name using the getRoute() and transmit the route to the sender.
 * The sender will create the stream object using ObjectStreamIO(route).
 * The sender can then sender PoJo or Java primitives as object thru the output stream.
 * The recipient can read the input stream to retrieve the object.
 *
 * Event stream is designed for use by a single sender and a single recipient to guarantee message sequencing.
 * If you use multiple senders and recipients, the event stream will mix the events.
 *
 * The sender can signal EOF by closing the output stream.
 * The recipient can release the stream by closing the input stream.
 */
public class ObjectStreamIO {

    private static final String STREAM_MANAGER = Platform.STREAM_MANAGER;
    private static final String TYPE = "type";
    private static final String CREATE = "create";
    private static final String QUERY = "query";
    private static final String STREAM_PREFIX = "stream.";
    private static final String EXPIRY_SEC = "expiry_seconds";
    private String route;
    private ObjectStreamWriter writer;
    private ObjectStreamReader reader;
    private int expirySeconds = 1800;
    private boolean inputClosed = false;

    public ObjectStreamIO() throws IOException {
        this.createStream();
    }

    public ObjectStreamIO(int expirySeconds) throws IOException {
        this.expirySeconds = expirySeconds;
        this.createStream();
    }

    public int getExpirySeconds() {
        return expirySeconds;
    }

    /**
     * Open an existing stream
     *
     * @param streamId for an existing stream
     * @throws IOException in case the stream ID is invalid
     */
    public ObjectStreamIO(String streamId) throws IOException {
        if (streamId.startsWith(STREAM_PREFIX) && streamId.contains("@")) {
            this.route = streamId;
        } else {
            throw new IOException("Invalid stream route");
        }
    }

    private void createStream() throws IOException {
        try {
            EventEnvelope response = PostOffice.getInstance().request(STREAM_MANAGER, 5000,
                    new Kv(TYPE, CREATE), new Kv(EXPIRY_SEC, expirySeconds));
            if (response.getBody() instanceof String) {
                String name = (String) response.getBody();
                if (name.startsWith(STREAM_PREFIX) && name.contains("@")) {
                    route = name;
                }
            }
        } catch (AppException | TimeoutException e) {
            throw new IOException(e.getMessage());
        }
        if (route == null) {
            throw new IOException("Stream manager is not responding correctly");
        }
    }

    public String getRoute() {
        return route;
    }

    public ObjectStreamReader getInputStream(long timeoutMs) {
        if (reader == null) {
            reader = new ObjectStreamReader(route, timeoutMs);
        }
        return reader;
    }

    public ObjectStreamWriter getOutputStream() {
        if (writer == null) {
            writer = new ObjectStreamWriter(route);
        }
        return writer;
    }

    @SuppressWarnings("unchecked")
    public Map<String, Object> getLocalStreams() throws IOException {
        try {
            EventEnvelope query = PostOffice.getInstance().request(STREAM_MANAGER, 5000, new Kv(TYPE, QUERY));
            return query.getBody() instanceof Map? (Map<String, Object>) query.getBody() : Collections.emptyMap();
        } catch (TimeoutException | AppException e) {
            throw new IOException(e.getMessage());
        }
    }

    public void close() throws IOException {
        if (!inputClosed) {
            inputClosed = true;
            ObjectStreamReader stream = getInputStream(5000);
            stream.close();
        }
    }

}
