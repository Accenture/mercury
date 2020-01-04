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

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.File;
import java.io.FileNotFoundException;
import java.io.FileOutputStream;
import java.io.IOException;

public class QueueFileWriter implements AutoCloseable {
    private static final Logger log = LoggerFactory.getLogger(QueueFileWriter.class);

    private FileOutputStream out;

    public QueueFileWriter(File f) throws FileNotFoundException {
        out = getHandle(f);
    }

    private FileOutputStream getHandle(File f) throws FileNotFoundException {
        int retry = 3;  // maximum retries
        while (true) {
            retry--;
            if (!f.exists()) {
                try {
                    boolean created = f.createNewFile();
                    if (!created) {
                        if (retry == 0) {
                            throw new FileNotFoundException("Unable to create queue file "+f.getPath());
                        } else {
                            log.warn("Retry creating queue file {}", f.getPath());
                            yield();
                            continue;
                        }
                    }
                } catch (IOException e) {
                    if (retry == 0) {
                        throw new FileNotFoundException(e.getMessage());
                    }
                }
            }
            try {
                return new FileOutputStream(f, true);
            } catch (FileNotFoundException e) {
                if (retry == 0) {
                    throw e;
                } else {
                    // Occasionally OS may not be able to update metadata immediately
                    log.warn("Retry opening queue file {}, due to {}", f.getPath(), e.getMessage());
                    yield();
                }
            }
        }
    }

    private void yield() {
        // give the local file system a brief moment to update metadata
        try {
            Thread.sleep(50);
        } catch (InterruptedException e1) {
            // ok to ignore
        }
    }

    public void write(byte b) throws IOException {
        out.write(b);
    }

    public void write(byte[] data) throws IOException {
        out.write(data);
    }

    @Override
    public void close() {
        if (out != null) {
            try {
                out.close();
            } catch (IOException e) {
                // ok to ignore
            }
        }
    }

}
