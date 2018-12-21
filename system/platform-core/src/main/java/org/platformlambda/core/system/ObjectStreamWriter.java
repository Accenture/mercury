/*

    Copyright 2018 Accenture Technology

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

import org.platformlambda.core.models.Kv;
import org.platformlambda.core.services.ObjectStreamService;

import java.io.IOException;

public class ObjectStreamWriter {

    private static final String TYPE = ObjectStreamService.TYPE;
    private static final String WRITE = ObjectStreamService.WRITE;
    private static final String EOF = ObjectStreamService.EOF;

    private String streamId;
    private boolean closed = false;

    public ObjectStreamWriter(String streamId) {
        this.streamId = streamId;
    }

    public void write(Object payload) throws IOException {
        if (payload == null) {
            // writing a null payload also indicates EOF
            close();
        } else {
            PostOffice.getInstance().send(streamId, payload, new Kv(TYPE, WRITE));
        }
    }

    public void close() throws IOException {
        if (!closed) {
            closed = true;
            PostOffice.getInstance().send(streamId, new Kv(TYPE, EOF));
        }
    }

}


