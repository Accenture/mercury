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

package org.platformlambda.services

import io.vertx.kotlin.coroutines.awaitBlocking
import org.platformlambda.core.annotations.PreLoad
import org.platformlambda.core.models.AsyncHttpRequest
import org.platformlambda.core.models.EventEnvelope
import org.platformlambda.core.models.KotlinLambdaFunction
import org.platformlambda.core.models.Kv
import org.platformlambda.core.system.EventEmitter
import org.platformlambda.core.system.FastRPC
import org.slf4j.LoggerFactory
import java.io.File

@PreLoad(route="hello.upload", instances=10)
class FileUploadDemo: KotlinLambdaFunction<AsyncHttpRequest, Any> {
    override suspend fun handleEvent(headers: Map<String, String>, input: AsyncHttpRequest, instance: Int): Any {
        val fastRPC = FastRPC(headers)
        val streamId: String? = input.streamRoute
        val filename: String? = input.fileName
        val size: Int = input.contentLength
        val uri = input.url
        val queries = input.queryParameters
        val cookies = input.cookies
        val ip = input.remoteIp
        log.info("Got upload request - URI={}, filename={}, ip={}, queries={}, cookies={}",
                    uri, filename, ip, queries, cookies)
        // if streamId and filename are available, the request contains an input stream
        if (streamId != null && filename != null) {
            if (size > 0) {
                log.info("Receiving {} of {} bytes", filename, size)
                val dir = File(TEMP_DEMO_FOLDER)
                if (!dir.exists()) {
                    dir.mkdirs()
                }
                var total = 0
                val file = File(dir, filename)
                val out = file.outputStream()
                val req = EventEnvelope().setTo(streamId).setHeader(TYPE, READ)
                /*
                 * looping inside a suspend function is fine if you include one of the following:
                 * 1. fastRPC.awaitRequest() method to make RPC calls to another function or external REST endpoint
                 * 2. await APIs
                 * 3. delay(milliseconds) API
                 * 4. yield() API
                 *
                 * awaitRequest is used to make an RPC call to another function
                 * awaitBlocking is used when you cannot avoid blocking code. e.g. File I/O.
                 * delay(ms) is used to let the function sleep
                 * yield() would yield control to the event loop.
                 * e.g. when you are writing many data blocks to a stream, you can insert the yield() statement
                 *      to release control after writing a block to the stream. In this way, your function
                 *      will not block other functions even when it goes into loop sending data continuously.
                 *
                 * IMPORTANT: You must not use blocking code in a suspend function. Otherwise, it will block
                 *            the main event loop, causing the whole application to slow down.
                 *            i.e. never use Thread.sleep(ms), BlockingQueue, the "synchronous" keyword, etc.
                 */
                var n = 0;
                while (true) {
                    val event = fastRPC.awaitRequest(req, 5000)
                    if (event.status == 408) {
                        log.error("File upload timeout with incomplete contents")
                        awaitBlocking {
                            out.close()
                        }
                        break
                    }
                    if (EOF == event.headers[TYPE]) {
                        log.info("Saved {}, total {} bytes", file, total)
                        awaitBlocking {
                            out.close()
                        }
                        // use EventEmitter instead of PostOffice because tracing is not required for object stream I/O
                        EventEmitter.getInstance().send(streamId, Kv(TYPE, CLOSE))
                        break
                    }
                    if (DATA == event.headers[TYPE]) {
                        val block = event.body
                        if (block is ByteArray) {
                            n++;
                            total += block.size
                            log.info("Saving {} - block#{} - {} bytes", filename, n, block.size)
                            awaitBlocking {
                                out.write(block)
                            }
                        }
                    }
                }
                val result = HashMap<String, Any>()
                result["filename"] = filename
                result["expected_size"] = size
                result["actual_size"] = total
                result["message"] = "Upload completed"
                return result
            } else {
                throw IllegalArgumentException("Input file is empty")
            }

        } else {
            throw IllegalArgumentException("Please check if input file is available")
        }
    }

    companion object {
        private val log = LoggerFactory.getLogger(FileUploadDemo::class.java)
        private const val TYPE = "type"
        private const val READ = "read"
        private const val DATA = "data"
        private const val EOF = "eof"
        private const val CLOSE = "close"
        private const val TEMP_DEMO_FOLDER = "/tmp/upload-download-demo"
    }
}