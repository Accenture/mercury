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

import javax.ws.rs.GET;
import javax.ws.rs.Path;
import javax.ws.rs.Produces;
import javax.ws.rs.QueryParam;
import javax.ws.rs.core.MediaType;
import java.io.IOException;
import java.util.Date;
import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.TimeoutException;

@Path("/add")
public class DemoMathEndpoint {

    @GET
    @Produces({MediaType.TEXT_PLAIN, MediaType.APPLICATION_JSON, MediaType.APPLICATION_XML, MediaType.TEXT_HTML})
    public Map<String, Object> adding2numbers(@QueryParam("a") String a, @QueryParam("b") String b) throws IOException, TimeoutException, AppException {

        PostOffice po = PostOffice.getInstance();
        EventEnvelope response = po.request("math.addition", 5000, new Kv("a", a), new Kv("b", b));
        if (response.getBody() instanceof Integer) {
            Map<String, Object> result = new HashMap<>();
            result.put("result", response.getBody());
            result.put("time", new Date());
            result.put("execution_time", response.getExecutionTime());
            result.put("round_trip", response.getRoundTrip());
            return result;
        } else {
            throw new IllegalArgumentException("Sorry math function failed");
        }

    }
}
