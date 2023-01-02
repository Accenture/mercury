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

import io.vertx.core.Future;

import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;

import java.io.IOException;
import java.util.Map;

public class AsyncObjectStreamReader implements AutoCloseable {

    private static final String TYPE = "type";
    private static final String READ = "read";
    private static final String DATA = "data";
    private static final String END_OF_STREAM = "eof";
    private static final String CLOSE = "close";

    private final long timeout;
    private final String streamId;
    private boolean closed = false;
    private boolean eof = false;

    public AsyncObjectStreamReader(String streamId, long timeout) {
        this.streamId = streamId;
        this.timeout = Math.max(1000, timeout);
    }

    public Future<Object> get() {
        final PostOffice po = PostOffice.getInstance();
        return Future.future(p -> {
            if (eof || closed) {
                p.complete(null);
            } else {
                EventEnvelope request = new EventEnvelope().setTo(streamId).setHeader(TYPE, READ);
                try {
                    po.asyncRequest(request, timeout)
                            .onSuccess(event -> {
                                Map<String, String> headers = event.getHeaders();
                                if (DATA.equals(headers.get(TYPE))) {
                                    p.complete(event.getBody());
                                }
                                if (END_OF_STREAM.equals(headers.get(TYPE))) {
                                    eof = true;
                                    p.complete(null);
                                }
                            })
                            .onFailure(p::fail);

                } catch (IOException e) {
                    p.fail(e);
                }
            }
        });
    }

    public boolean isClosed() {
        return closed;
    }

    public boolean isStreamEnd() {
        return eof;
    }

    @Override
    public void close() throws IOException {
        if (!closed) {
            closed = true;
            PostOffice.getInstance().send(streamId, new Kv(TYPE, CLOSE));
        }
    }
}
