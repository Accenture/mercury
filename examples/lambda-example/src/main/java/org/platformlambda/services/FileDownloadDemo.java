/*

    Copyright 2018-2024 Accenture Technology

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

import org.platformlambda.core.annotations.PreLoad;
import org.platformlambda.core.models.AsyncHttpRequest;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.TypedLambdaFunction;
import org.platformlambda.core.system.ObjectStreamIO;
import org.platformlambda.core.system.ObjectStreamWriter;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.File;
import java.io.InputStream;
import java.nio.file.Files;
import java.util.Map;

/**
 * This demo function assumes the request comes from REST automation as an HTTP request event.
 * <p>
 * It demonstrates the use of EventEnvelope as return value so that you can set
 * headers.
 * <p>
 * For result that is delivered as HTTP response event, the system will map
 * the event envelope's headers as HTTP response headers.
 * <p>
 * You can return a file by writing it as an output stream.
 * The system HTTP response handler will pick up the stream-ID and
 * open the input stream to read data accordingly.
 */
@PreLoad(route="hello.download", instances=10)
public class FileDownloadDemo implements TypedLambdaFunction<AsyncHttpRequest, EventEnvelope> {
    private static final Logger log = LoggerFactory.getLogger(FileDownloadDemo.class);

    private static final String TEMP_DEMO_FOLDER = "/tmp/upload-download-demo";
    private static final String FILENAME = "filename";
    private static final String SAMPLE_FILE = "helloworld.txt";

    @Override
    public EventEnvelope handleEvent(Map<String, String> headers, AsyncHttpRequest input, int instance)
            throws Exception {
        if (input.getUrl() == null) {
            throw new IllegalArgumentException("The input does not appear to be an HTTP request. " +
                                                "Please route the request through REST automation");
        }
        // demonstrates reading common HTTP attributes from the request
        String filename = input.getPathParameter(FILENAME);
        String uri = input.getUrl();
        Map<String, Object> queries = input.getQueryParameters();
        Map<String, String> cookies = input.getCookies();
        String ip = input.getRemoteIp();
        log.info("Got download request - URI={}, filename={}, ip={}, queries={}, cookies={}",
                    uri, filename, ip, queries, cookies);
        final String filePath;
        final InputStream in;
        if (filename == null) {
            in = FileDownloadDemo.class.getResourceAsStream("/"+SAMPLE_FILE);
            filename = SAMPLE_FILE;
            filePath = "classpath:/"+SAMPLE_FILE;
        } else {
            File dir = new File(TEMP_DEMO_FOLDER);
            File file = new File(dir, filename);
            if (!file.exists()) {
                throw new IllegalArgumentException("File not found");
            } else {
                in = Files.newInputStream(file.toPath());
                filePath = file.getPath();
            }
        }
        if (in == null) {
            throw new IllegalArgumentException("Unable to download "+filePath);
        }
        try {
            int len;
            int total = 0;
            ObjectStreamIO stream = new ObjectStreamIO(60);
            try (ObjectStreamWriter out = new ObjectStreamWriter(stream.getOutputStreamId())) {
                // send file as an event stream
                byte[] buffer = new byte[1024];
                while ((len = in.read(buffer, 0, buffer.length)) != -1) {
                    total += len;
                    out.write(buffer, 0, len);
                }
            }
            log.info("Sending {} with {} bytes to {}", filePath, total, stream.getOutputStreamId());
            return new EventEnvelope().setHeader("stream", stream.getInputStreamId())
                    .setHeader("Content-Type", "application/octet-stream")
                    .setHeader("Content-Disposition", "attachment; filename="+filename);
        } finally {
            in.close();
        }
    }
}
