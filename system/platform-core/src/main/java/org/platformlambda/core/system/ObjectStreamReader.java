/*

    Copyright 2018-2023 Accenture Technology

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
import java.util.Iterator;
import java.util.Map;
import java.util.concurrent.TimeoutException;

public class ObjectStreamReader implements Iterable<Object>, AutoCloseable {

    private static final String TYPE = "type";
    private static final String READ = "read";
    private static final String DATA = "data";
    private static final String END_OF_STREAM = "eof";
    private static final String CLOSE = "close";

    private final ObjectBlockReader iterator;
    private final long timeout;
    private final String streamId;
    private boolean closed = false;

    public ObjectStreamReader(String streamId, long timeout) {
        this.streamId = streamId;
        this.timeout = Math.max(1000, timeout);
        this.iterator = new ObjectBlockReader();
    }

    @Override
    public Iterator<Object> iterator() {
        return iterator;
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

        @Override
        public boolean hasNext() {
            return !eof;
        }

        @Override
        public Object next() {
            return eof? null : fetch();
        }

        private Object fetch() {
            try {
                EventEnvelope event = PostOffice.getInstance().request(streamId, timeout, new Kv(TYPE, READ));
                if (event.hasError()) {
                    throw new IOException(event.getError());
                }
                Map<String, String> headers = event.getHeaders();
                if (DATA.equals(headers.get(TYPE))) {
                    return event.getBody();
                }
                if (END_OF_STREAM.equals(headers.get(TYPE))) {
                    eof = true;
                }
            } catch (AppException | IOException | TimeoutException e) {
                eof = true;
                throw new RuntimeException(e.getMessage());
            }
            return null;
        }
    }

}
