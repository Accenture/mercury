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

package org.platformlambda.core.util;

import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.system.QueueFileWriter;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.File;
import java.io.FileInputStream;
import java.io.IOException;
import java.util.concurrent.ConcurrentLinkedQueue;

public class ElasticQueue {
    private static final Logger log = LoggerFactory.getLogger(ElasticQueue.class);

    private static final Utility util = Utility.getInstance();

    public static final int MEMORY_BUFFER = 10;
    private static final String QUEUE = "data-";
    private static final int MAX_FILE_SIZE = 10 * 1024 * 1024;
    private static final byte DATA = 0x01;
    private static final byte EOF = 0x00;
    private int writeFileNumber, readFileNumber;
    private long readCounter, writeCounter;
    private File dir;
    private String id;
    private FileInputStream in;
    private boolean empty = false, createDir = false;
    private ConcurrentLinkedQueue<EventEnvelope> memory = new ConcurrentLinkedQueue<>();
    private EventEnvelope peeked = null;

    /**
     * Two-stage elastic queue using memory and disk
     *
     * @param dir parent queue directory
     * @param id service route path
     */
    public ElasticQueue(File dir, String id) {
        // guarantee valid filename
        this.id = util.validServiceName(id)? id : util.filteredServiceName(id);
        this.dir = new File(dir, this.id);
        initialize();
    }

    public String getId() {
        return id;
    }

    public void close() {
        if (in != null) {
            try {
                in.close();
            } catch (IOException e) {
                // ok to ignore
            }
            in = null;
        }
        initialize();
    }

    public boolean isClosed() {
        return in == null && writeCounter == 0;
    }

    public void destroy() {
        // guarantee that it is closed
        close();
        if (isClosed()) {
            util.cleanupDir(dir);
        }
    }

    private void initialize() {
        if (!empty) {
            empty = true;
            if (dir.exists()) {
                util.cleanupDir(dir, true);
            } else {
                createDir = true;
            }
            readFileNumber = writeFileNumber = 1;
            readCounter = writeCounter = 0;
            memory.clear();
        }
    }

    public void write(EventEnvelope event) {
        if (writeCounter < MEMORY_BUFFER) {
            // for highest performance, save to memory for the first few blocks
            memory.offer(event);
            writeCounter++;
            empty = false;
        } else {
            if (createDir) {
                createDir = false;
                dir.mkdirs();
            }
            // otherwise, save to disk
            File f = new File(dir, QUEUE + writeFileNumber);
            long len = f.exists() ? f.length() : 0;
            try (QueueFileWriter out = new QueueFileWriter(f)) {
                // serialize to bytes
                byte[] b = event.toBytes();
                out.write(DATA);
                out.write(util.int2bytes(b.length));
                out.write(b);
                len += b.length;
                if (len >= MAX_FILE_SIZE) {
                    // write EOF indicator and increment file sequence
                    out.write(EOF);
                    writeFileNumber++;
                }
                writeCounter++;
                empty = false;
            } catch (IOException e) {
                // this should not happen
                log.error("Event lost. Unable to persist {} to {} - {}", id, f.getPath(), e.getMessage());
            }
        }

    }

    public EventEnvelope peek() throws IOException {
        if (peeked != null) {
            return peeked;
        }
        peeked = read();
        return peeked;
    }

    public EventEnvelope read() throws IOException {
        if (peeked != null) {
            EventEnvelope result = peeked;
            peeked = null;
            return result;
        }
        if (readCounter >= writeCounter) {
            // catch up with writes and thus nothing to read
            close();
            return null;
        }
        if (readCounter < MEMORY_BUFFER) {
            EventEnvelope event = memory.poll();
            if (event != null) {
                readCounter++;
            }
            return event;
        }
        if (in == null) {
            File f = new File(dir, QUEUE+ readFileNumber);
            if (f.exists()) {
                in = new FileInputStream(f);
            } else {
                return null;
            }
        }
        byte[] control = new byte[1];
        int count = in.read(control);
        if (count == -1) {
            return null;
        }
        if (control[0] == EOF) {
            // EOF - drop file and increment read sequence
            in.close();
            in = null;
            File f = new File(dir, QUEUE+ readFileNumber);
            if (f.exists()) {
                f.delete();
                readFileNumber++;
                return read();
            } else {
                throw new IOException("Corrupted queue for "+ id);
            }
        }
        if (control[0] != DATA) {
            throw new IOException("Corrupted queue for "+ id);
        }
        byte[] size = new byte[4];
        count = in.read(size);
        if (count != 4) {
            throw new IOException("Corrupted queue for "+ id);
        }
        int blockSize = util.bytes2int(size);
        byte[] result = new byte[blockSize];
        count = in.read(result);
        if (count != blockSize) {
            throw new IOException("Corrupted queue for "+ id);
        }
        readCounter++;
        return new EventEnvelope(result);
    }

}
