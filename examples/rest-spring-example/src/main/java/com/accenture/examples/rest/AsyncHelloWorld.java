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

package com.accenture.examples.rest;

import io.vertx.core.Future;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.Utility;

import javax.servlet.http.HttpServletRequest;
import javax.ws.rs.GET;
import javax.ws.rs.Path;
import javax.ws.rs.Produces;
import javax.ws.rs.container.AsyncResponse;
import javax.ws.rs.container.Suspended;
import javax.ws.rs.core.Context;
import javax.ws.rs.core.MediaType;
import java.io.IOException;
import java.util.Enumeration;
import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.atomic.AtomicInteger;

@Path("/hello")
public class AsyncHelloWorld {

    private static final AtomicInteger seq = new AtomicInteger(0);

    @GET
    @Path("/world")
    @Produces({MediaType.TEXT_PLAIN, MediaType.APPLICATION_JSON, MediaType.APPLICATION_XML, MediaType.TEXT_HTML})
    public void hello(@Context HttpServletRequest request, @Suspended AsyncResponse response) throws IOException {

        String traceId = Utility.getInstance().getUuid();
        PostOffice po = new PostOffice("hello.world.endpoint", traceId, "GET /api/hello/world");
        Map<String, Object> forward = new HashMap<>();

        Enumeration<String> headers = request.getHeaderNames();
        while (headers.hasMoreElements()) {
            String key = headers.nextElement();
            forward.put(key, request.getHeader(key));
        }
        // As a demo, just put the incoming HTTP headers as a payload and a parameter showing the sequence counter.
        // The echo service will return both.
        int n = seq.incrementAndGet();
        EventEnvelope req = new EventEnvelope();
        req.setTo("hello.world").setBody(forward).setHeader("seq", n);
        Future<EventEnvelope> res = po.asyncRequest(req, 3000);
        res.onSuccess(event -> {
            Map<String, Object> result = new HashMap<>();
            result.put("status", event.getStatus());
            result.put("headers", event.getHeaders());
            result.put("body", event.getBody());
            result.put("execution_time", event.getExecutionTime());
            result.put("round_trip", event.getRoundTrip());
            response.resume(result);
        });
        res.onFailure(ex -> response.resume(new AppException(408, ex.getMessage())));
    }
}
