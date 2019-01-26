/*

    Copyright 2018-2019 Accenture Technology

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

package org.platformlambda.core.services;

import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.ElasticQueue;
import org.platformlambda.core.util.Utility;

import java.io.File;
import java.io.IOException;
import java.util.Map;

import static org.platformlambda.core.system.Platform.STREAM_MANAGER;

public class ObjectStreamService implements LambdaFunction {

    public static final String TYPE = "type";
    public static final String READ = "read";
    public static final String PEEK = "peek";
    public static final String WRITE = "write";
    public static final String EOF = "eof";
    public static final String BODY = "body";
    public static final String PENDING = "pending";
    public static final String CLOSE = "close";
    private static final String STREAM = "stream.";
    private static final String NAME = "name";
    private static final String DESTROY = "destroy";
    private static final String QUEUES = "queues";
    private ElasticQueue elasticQueue;
    private String path;
    private boolean writeEOF = false, readEOF = false;

    public ObjectStreamService() {
        this.path = STREAM+Utility.getInstance().getUuid();
        this.elasticQueue = new ElasticQueue(new File(Utility.getInstance().getWorkFolder(), QUEUES), this.path);
    }

    public String getPath() {
        return path;
    }

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {

        if (headers.containsKey(TYPE)) {
            String type = headers.get(TYPE);
            EventEnvelope event = new EventEnvelope();
            event.setHeader(TYPE, type).setBody(body);
            // writing to output stream
            if (type.equals(WRITE)) {
                if (!writeEOF) {
                    write(event);
                }
                return true;
            }
            // write EOF marker
            if (type.equals(EOF)) {
                if (!writeEOF) {
                    write(event);
                    writeEOF = true;
                }
                return true;
            }
            // check if there is content in input stream
            if (type.equals(PEEK)) {
                if (readEOF) {
                    return new EventEnvelope().setHeader(TYPE, EOF);
                } else {
                    EventEnvelope data = elasticQueue.peek();
                    if (data != null) {
                        if (EOF.equals(data.getHeaders().get(TYPE))) {
                            readEOF = true;
                            return new EventEnvelope().setHeader(TYPE, EOF);
                        } else {
                            return new EventEnvelope().setHeader(TYPE, BODY).setBody(data.getBody());
                        }
                    } else {
                        return new EventEnvelope().setHeader(TYPE, PENDING);
                    }
                }
            }
            // read the next block from the input stream
            if (type.equals(READ)) {
                if (readEOF) {
                    return new EventEnvelope().setHeader(TYPE, EOF);
                } else {
                    EventEnvelope data = elasticQueue.read();
                    if (data != null) {
                        if (EOF.equals(data.getHeaders().get(TYPE))) {
                            readEOF = true;
                            return new EventEnvelope().setHeader(TYPE, EOF);
                        } else {
                            return new EventEnvelope().setHeader(TYPE, BODY).setBody(data.getBody());
                        }
                    }
                }
            }
            // close the input stream and release resources
            if (type.equals(CLOSE)) {
                writeEOF = true;
                PostOffice.getInstance().send(STREAM_MANAGER, new Kv(TYPE, DESTROY), new Kv(NAME, path));
            }
        }
        // an empty event envelope => asking the Post Office to skip sending a reply to the caller
        return new EventEnvelope();
    }

    private void write(EventEnvelope event) throws IOException {
        elasticQueue.write(event);
    }

}
