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

package com.accenture.examples.rest;

import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.system.PostOffice;

import javax.servlet.http.HttpServletRequest;
import javax.ws.rs.GET;
import javax.ws.rs.Path;
import javax.ws.rs.Produces;
import javax.ws.rs.core.Context;
import javax.ws.rs.core.MediaType;
import java.io.IOException;
import java.util.Date;
import java.util.Enumeration;
import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.TimeoutException;
import java.util.concurrent.atomic.AtomicInteger;

@Path("/hello")
public class HelloWorld {

    private static AtomicInteger seq = new AtomicInteger(0);

    @GET
    @Path("/world")
    @Produces({MediaType.TEXT_PLAIN, MediaType.APPLICATION_JSON, MediaType.APPLICATION_XML, MediaType.TEXT_HTML})
    public Map<String, Object> hello(@Context HttpServletRequest request) throws IOException, TimeoutException, AppException {

        PostOffice po = PostOffice.getInstance();
        Map<String, Object> forward = new HashMap<>();
        forward.put("time", new Date());

        Enumeration<String> headers = request.getHeaderNames();
        while (headers.hasMoreElements()) {
            String key = headers.nextElement();
            forward.put(key, request.getHeader(key));
        }
        // As a demo, just put the incoming HTTP headers as a payload and a parameter showing the sequence counter.
        // The echo service will return both.
        int n = seq.incrementAndGet();
        EventEnvelope response = po.request("hello.world", 3000, forward, new Kv("seq", n));

        Map<String, Object> result = new HashMap<>();
        result.put("status", response.getStatus());
        result.put("headers", response.getHeaders());
        result.put("body", response.getBody());
        result.put("execution_time", response.getExecutionTime());
        result.put("round_trip", response.getRoundTrip());
        return result;
    }

}
