/*

    Copyright 2018-2022 Accenture Technology

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
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;

public class StreamHolder {
    private static final Logger log = LoggerFactory.getLogger(StreamHolder.class);

    private ObjectStreamIO stream = null;
    private ObjectStreamWriter out = null;
    private final int timeoutSeconds;

    public StreamHolder(int timeoutSeconds) {
        this.timeoutSeconds = timeoutSeconds;
    }

    public ObjectStreamWriter getOutputStream() {
        if (out == null) {
            try {
                stream = new ObjectStreamIO(timeoutSeconds);
                out = new ObjectStreamWriter(stream.getOutputStreamId());
            } catch (IOException e) {
                log.error("Unable to create stream - {}", e.getMessage());
            }
        }
        return out;
    }

    public String getInputStreamId() {
        return stream == null? null : stream.getInputStreamId();
    }

    public void close() {
        if (out != null) {
            try {
                out.close();
                log.info("{} closed", getInputStreamId());
            } catch (IOException e) {
                log.error("Unable to close stream - {}", e.getMessage());
            }
        }
    }

}
