/*

    Copyright 2018-2021 Accenture Technology

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

package org.platformlambda.automation.models;

import org.platformlambda.core.system.ObjectStreamIO;
import org.platformlambda.core.system.ObjectStreamWriter;

import java.io.IOException;

public class ObjectStreamWrapper {

    private final int expirySeconds;
    private ObjectStreamIO stream = null;
    private ObjectStreamWriter out = null;

    public ObjectStreamWrapper(int expirySeconds) {
        this.expirySeconds = expirySeconds;
    }

    public void write(byte[] b, int start, int end) throws IOException {
        if (out == null) {
            stream = new ObjectStreamIO(expirySeconds);
            out = stream.getOutputStream();
        }
        out.write(b, start, end);
    }

    public void write(byte[] b) throws IOException {
        if (out == null) {
            stream = new ObjectStreamIO(expirySeconds);
            out = stream.getOutputStream();
        }
        out.write(b);
    }

    public String getId() {
        return stream == null? null : stream.getRoute();
    }

    public void close() throws IOException {
        if (out != null) {
            out.close();
        }
    }

}
