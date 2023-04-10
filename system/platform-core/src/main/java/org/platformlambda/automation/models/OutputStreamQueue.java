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

package org.platformlambda.automation.models;

import io.vertx.core.AsyncResult;
import io.vertx.core.Future;
import io.vertx.core.Handler;
import io.vertx.core.buffer.Buffer;
import io.vertx.core.streams.WriteStream;
import org.platformlambda.core.util.ElasticQueue;
import org.platformlambda.core.util.Utility;

public class OutputStreamQueue implements WriteStream<Buffer> {
    private final ElasticQueue queue = new ElasticQueue("http."+Utility.getInstance().getUuid());
    private final AcknowledgeResult acknowledgement = new AcknowledgeResult();

    public byte[] read() {
        return queue.read();
    }

    public void close() {
        queue.close();
    }

    @Override
    public WriteStream<Buffer> exceptionHandler(Handler<Throwable> handler) {
        // no need to implement
        return null;
    }

    @Override
    public Future<Void> write(Buffer data) {
        // no need to implement
        return null;
    }

    @Override
    public void write(Buffer block, Handler<AsyncResult<Void>> handler) {
        queue.write(block.getBytes());
        handler.handle(acknowledgement);
    }

    @Override
    public void end(Handler<AsyncResult<Void>> handler) {
        handler.handle(acknowledgement);
    }

    @Override
    public WriteStream<Buffer> setWriteQueueMaxSize(int maxSize) {
        // no need to implement
        return null;
    }

    @Override
    public boolean writeQueueFull() {
        return false;
    }

    @Override
    public WriteStream<Buffer> drainHandler(Handler<Void> handler) {
        // no need to implement
        return null;
    }

}
