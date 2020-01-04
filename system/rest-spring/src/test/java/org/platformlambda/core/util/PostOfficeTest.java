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

import org.junit.BeforeClass;
import org.junit.Test;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;

import java.io.IOException;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.TimeoutException;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;

public class PostOfficeTest {

    @BeforeClass
    public static void setup() throws IOException {
        Platform platform = Platform.getInstance();

        LambdaFunction echo = (headers, body, instance) -> {
            int c = body instanceof Integer? (int) body : 2;
            if (c % 2 == 0) {
                Thread.sleep(1000);
                // timeout the incoming request
            }
            Map<String, Object> result = new HashMap<>();
            result.put("headers", headers);
            result.put("body", body);
            result.put("instance", instance);
            result.put("counter", c);
            result.put("origin", platform.getOrigin());
            return result;
        };
        platform.register("hello.world", echo, 10);
    }

    @Test(expected = TimeoutException.class)
    public void singleRequestWithException() throws TimeoutException, IOException, AppException {
        PostOffice po = PostOffice.getInstance();
        po.request("hello.world", 500, 0);
    }

    @Test
    @SuppressWarnings("unchecked")
    public void singleRequest() throws TimeoutException, IOException, AppException {
        int input = 111;
        PostOffice po = PostOffice.getInstance();
        EventEnvelope response = po.request("hello.world", 500, input);
        assertEquals(HashMap.class, response.getBody().getClass());
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        assertEquals(input, result.get("body"));
    }

    @Test
    @SuppressWarnings("unchecked")
    public void parallelRequests() throws IOException {
        PostOffice po = PostOffice.getInstance();
        List<EventEnvelope> parallelEvents = new ArrayList<>();
        for (int i=0; i < 4; i++) {
            EventEnvelope event = new EventEnvelope();
            event.setTo("hello.world");
            event.setBody(i);
            event.setHeader("request", "#"+(i+1));
            parallelEvents.add(event);
        }
        List<EventEnvelope> results = po.request(parallelEvents, 500);
        // expect partial results of 2 items because the other two will timeout
        assertEquals(2, results.size());
        // check partial results
        for (EventEnvelope evt: results) {
            assertTrue(evt.getBody() instanceof Map);
            Map<String, Object> values = (Map<String, Object>) evt.getBody();
            assertTrue(values.containsKey("body"));
            Object v = values.get("body");
            assertTrue(v instanceof Integer);
            int val = (int) v;
            // expect body as odd number because even number will timeout
            assertTrue(val % 2 != 0);
        }
    }

    @Test
    @SuppressWarnings("unchecked")
    public void routeSubstitution() throws TimeoutException, IOException, AppException {
        int input = 111;
        PostOffice po = PostOffice.getInstance();
        // with route substitution in the application.properties, hello.test will route to hello.world
        EventEnvelope response = po.request("hello.test", 500, input);
        assertEquals(HashMap.class, response.getBody().getClass());
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        assertEquals(input, result.get("body"));
    }

}
