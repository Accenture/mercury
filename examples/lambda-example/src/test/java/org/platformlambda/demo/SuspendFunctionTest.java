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

package org.platformlambda.demo;

import org.junit.Assert;
import org.junit.Test;
import org.platformlambda.core.models.AsyncHttpRequest;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.system.ObjectStreamIO;
import org.platformlambda.core.system.ObjectStreamWriter;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.Utility;
import org.platformlambda.demo.common.TestBase;

import java.io.ByteArrayOutputStream;
import java.io.File;
import java.io.IOException;
import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.BlockingQueue;
import java.util.concurrent.TimeUnit;

public class SuspendFunctionTest extends TestBase {

    @SuppressWarnings("unchecked")
    @Test
    public void uploadTest() throws IOException, InterruptedException {
        String FILENAME = "unit-test-data.txt";
        BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        Utility util = Utility.getInstance();
        String traceId = Utility.getInstance().getUuid();
        PostOffice po = new PostOffice("unit.test", traceId, "/stream/upload/test");
        int len = 0;
        ByteArrayOutputStream bytes = new ByteArrayOutputStream();
        ObjectStreamIO stream = new ObjectStreamIO();
        ObjectStreamWriter out = new ObjectStreamWriter(stream.getOutputStreamId());
        for (int i=0; i < 10; i++) {
            String line = "hello world "+i+"\n";
            byte[] d = util.getUTF(line);
            out.write(d);
            bytes.write(d);
            len += d.length;
        }
        out.close();
        // emulate a multi-part file upload
        AsyncHttpRequest req = new AsyncHttpRequest();
        req.setMethod("POST");
        req.setUrl("/api/upload/demo");
        req.setTargetHost("http://127.0.0.1:8080");
        req.setHeader("accept", "application/json");
        req.setHeader("content-type", "multipart/form-data");
        req.setContentLength(len);
        req.setFileName(FILENAME);
        req.setStreamRoute(stream.getInputStreamId());
        // send the HTTP request event to the "hello.upload" function
        EventEnvelope request = new EventEnvelope().setTo("hello.upload")
                .setBody(req).setTrace("12345", "/api/upload/demo").setFrom("unit.test");
        po.asyncRequest(request, 8000).onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertEquals(HashMap.class, response.getBody().getClass());
        Map<String, Object> map = (Map<String, Object>) response.getBody();
        System.out.println(response.getBody());
        Assert.assertEquals(len, map.get("expected_size"));
        Assert.assertEquals(len, map.get("actual_size"));
        Assert.assertEquals(FILENAME, map.get("filename"));
        Assert.assertEquals("Upload completed", map.get("message"));
        // finally check that "hello.upload" has saved the test file
        File dir = new File("/tmp/upload-download-demo");
        File file = new File(dir, FILENAME);
        Assert.assertTrue(file.exists());
        Assert.assertEquals(len, file.length());
        // compare file content
        byte[] b = Utility.getInstance().file2bytes(file);
        Assert.assertArrayEquals(bytes.toByteArray(), b);
    }
}
