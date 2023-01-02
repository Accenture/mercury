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

package org.platformlambda.services;

import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.AsyncHttpRequest;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.ObjectStreamIO;
import org.platformlambda.core.system.ObjectStreamWriter;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.InputStream;
import java.util.Map;

/**
 * This service demonstrates file download.
 *
 * IMPORTANT - for LambdaFunction, the handleEvent method is the event handler.
 * Please do not use any global scope variables. All variables must be in functional scope.
 * If you must use global scope variables, you may use Java Concurrent collections.
 *
 * It assumes the download request comes from the REST automation app.
 */
public class FileDownload implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(FileDownload.class);

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
        AsyncHttpRequest request = new AsyncHttpRequest(body);
        if (request.getUrl() == null) {
            throw new IllegalArgumentException("The input does not appear to be a HTTP request. " +
                    "Please route the request through REST automation");
        }
        // demonstrates how to get common HTTP attributes
        String uri = request.getUrl();
        Map<String, Object> queries = request.getQueryParameters();
        Map<String, String> cookies = request.getCookies();
        String ip = request.getRemoteIp();
        log.info("URI={}, ip={}, queries={}, cookies={}", uri, ip, queries, cookies);

        try (InputStream in = FileDownload.class.getResourceAsStream("/helloworld.txt")) {
            if (in == null) {
                throw new AppException(404, "helloworld.txt not found");
            }
            ObjectStreamIO stream = new ObjectStreamIO(60);
            ObjectStreamWriter out = new ObjectStreamWriter(stream.getOutputStreamId());
            // send file as an event stream
            int len, total = 0;
            byte[] buffer = new byte[1024];
            while ((len = in.read(buffer, 0, buffer.length)) != -1) {
                total += len;
                out.write(buffer, 0, len);
            }
            // indicate EOF
            out.close();
            log.info("Written {} bytes to {}", total, stream.getOutputStreamId());
            return new EventEnvelope().setHeader("stream", stream.getInputStreamId()).setHeader("timeout", 10)
                    .setHeader("Content-Type", "application/octet-stream")
                    .setHeader("Content-Disposition", "attachment; filename=\"helloworld.txt\"");
        }

    }
}
