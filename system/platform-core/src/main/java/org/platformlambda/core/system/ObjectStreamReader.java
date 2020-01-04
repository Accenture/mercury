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
import org.platformlambda.core.services.ObjectStreamService;

import java.io.Closeable;
import java.io.IOException;
import java.util.Iterator;
import java.util.Map;
import java.util.concurrent.TimeoutException;

public class ObjectStreamReader implements Iterable<Object>, Closeable {

    private static final String TYPE = ObjectStreamService.TYPE;
    private static final String READ = ObjectStreamService.READ;
    private static final String PEEK = ObjectStreamService.PEEK;
    private static final String BODY = ObjectStreamService.BODY;
    private static final String EOF = ObjectStreamService.EOF;
    private static final String PENDING = ObjectStreamService.PENDING;
    private static final String CLOSE = ObjectStreamService.CLOSE;

    private ObjectBlockReader iterator;
    private long timeout;
    private String streamId;
    private boolean closed = false;

    public ObjectStreamReader(String streamId, long timeout) {
        this.streamId = streamId;
        // the minimum timeout for an event stream is one second
        this.timeout = timeout < 1000? 1000 : timeout;
        this.iterator = new ObjectBlockReader();
    }

    @Override
    public Iterator<Object> iterator() {
        return iterator;
    }

    public boolean isPending() {
        EventEnvelope event = iterator.peek();
        String type = event.getHeaders().get(TYPE);
        return PENDING.equals(type);
    }

    public boolean isEof() {
        if (iterator.isEof()) {
            return true;
        } else {
            EventEnvelope event = iterator.peek();
            String type = event.getHeaders().get(TYPE);
            return EOF.equals(type);
        }
    }

    @Override
    public void close() throws IOException {
        if (!closed) {
            closed = true;
            PostOffice.getInstance().send(streamId, new Kv(TYPE, CLOSE));
        }
    }

    private class ObjectBlockReader implements Iterator<Object> {

        private boolean eof = false;
        private Object block = null;
        private EventEnvelope peeked = null;

        @Override
        public boolean hasNext() {
            if (eof) {
                return false;
            }
            fetch();
            return block != null;
        }

        @Override
        public Object next() {
            if (eof) {
                return null;
            }
            if (block == null) {
                fetch();
            }
            Object result = block;
            block = null;
            return result;
        }

        private boolean isEof() {
            return eof;
        }

        /**
         * Non-blocking peek
         *
         * @return event envelope indicates EOF, BODY or PENDING
         */
        private EventEnvelope peek() {
            if (peeked != null) {
                return peeked;
            }
            try {
                EventEnvelope event = PostOffice.getInstance().request(streamId, timeout, new Kv(TYPE, PEEK));
                if (event.hasError()) {
                    throw new AppException(event.getStatus(), event.getError());
                }
                peeked = event;
                return event;
            } catch (Exception e) {
                throw new IllegalArgumentException(e.getMessage());
            }
        }

        private void fetch() {
            // clear peeked item
            peeked = null;
            // read next item
            try {
                EventEnvelope event = PostOffice.getInstance().request(streamId, timeout, new Kv(TYPE, READ));
                if (event.hasError()) {
                    throw new AppException(event.getStatus(), event.getError());
                }
                Map<String, String> headers = event.getHeaders();
                if (headers.containsKey(TYPE)) {
                    String type = headers.get(TYPE);
                    if (type.equals(EOF)) {
                        eof = true;
                    }
                    if (type.equals(BODY)) {
                        block = event.getBody();
                    }
                }
            } catch (AppException | IOException | TimeoutException e) {
                // translate to RunTimeException because this method is used by an iterator
                throw new RuntimeException(e.getMessage());
            }
        }
    }

}
