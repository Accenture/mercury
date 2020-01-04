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
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.Utility;

import javax.servlet.http.HttpServletRequest;
import javax.ws.rs.GET;
import javax.ws.rs.Path;
import javax.ws.rs.Produces;
import javax.ws.rs.core.Context;
import javax.ws.rs.core.MediaType;
import java.io.IOException;
import java.util.*;

@Path("/hello")
public class HelloConcurrent {

    @GET
    @Path("/concurrent")
    @Produces({MediaType.TEXT_PLAIN, MediaType.APPLICATION_XML, MediaType.APPLICATION_JSON, MediaType.TEXT_HTML})
    public Map<String, Object> hello(@Context HttpServletRequest request) throws IOException, AppException {

        Utility util = Utility.getInstance();
        PostOffice po = PostOffice.getInstance();

        Map<String, Object> forwardHeaders = new HashMap<>();
        forwardHeaders.put("time", new Date());

        Enumeration<String> headers = request.getHeaderNames();
        while (headers.hasMoreElements()) {
            String key = headers.nextElement();
            forwardHeaders.put(key, request.getHeader(key));
        }
        int TOTAL = 10;
        List<EventEnvelope> parallelEvents = new ArrayList<>();
        for (int i=0; i < TOTAL; i++) {
            EventEnvelope event = new EventEnvelope();
            event.setTo("hello.world");
            event.setBody(forwardHeaders);
            event.setHeader("request", "#"+(i+1));
            parallelEvents.add(event);
        }

        Map<String, Object> results = new HashMap<>();
        List<EventEnvelope> responses = po.request(parallelEvents, 3000);
        /*
         * Note that for parallel requests, you may received partial results if some target com.accenture.examples.services did not respond.
         * Therefore, you must check the total number of responses in the list.
         */
        if (responses.size() < TOTAL) {
            throw new AppException(408, "Only "+responses.size()+" of "+TOTAL+" responded");
        }
        int n = 0;
        for (EventEnvelope evt: responses) {
            Map<String, Object> singleResult = new HashMap<>();
            singleResult.put("status", evt.getStatus());
            singleResult.put("headers", evt.getHeaders());
            singleResult.put("body", evt.getBody());
            singleResult.put("seq", evt.getCorrelationId());
            singleResult.put("execution_time", evt.getExecutionTime());
            singleResult.put("round_trip", evt.getRoundTrip());
            n++;
            results.put("result_"+util.zeroFill(n, 999), singleResult);
        }
        return results;
    }

}
