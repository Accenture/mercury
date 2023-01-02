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

import org.platformlambda.core.models.Kv;

import java.io.IOException;
import java.util.Arrays;

public class ObjectStreamWriter implements AutoCloseable {
    private static final String TYPE = "type";
    private static final String DATA = "data";
    private static final String END_OF_STREAM = "eof";

    private final String streamId;
    private boolean eof = false;

    public ObjectStreamWriter(String streamId) {
        this.streamId = streamId;
    }

    public void write(Object payload) throws IOException {
        if (payload == null) {
            // null payload means EOF
            close();
        } else {
            if (payload instanceof byte[]) {
                byte[] b = (byte[]) payload;
                write(b, 0, b.length);
            } else {
                PostOffice.getInstance().send(streamId, payload, new Kv(TYPE, DATA));
            }
        }
    }

    public void write(byte[] payload, int start, int end) throws IOException {
        if (payload == null) {
            // null payload means EOF
            close();
        } else {
            if (start > end) {
                throw new IOException("Invalid byte range. Actual: start/end=" + start + "/" + end);
            }
            if (end > payload.length) {
                throw new IOException("end pointer must not be larger than payload buffer size");
            }
            // always create a new byte array
            byte[] b = start == end? new byte[0] : Arrays.copyOfRange(payload, start, end);
            PostOffice.getInstance().send(streamId, b, new Kv(TYPE, DATA));
        }
    }

    @Override
    public void close() throws IOException {
        if (!eof) {
            eof = true;
            PostOffice.getInstance().send(streamId, new Kv(TYPE, END_OF_STREAM));
        }
    }

}


