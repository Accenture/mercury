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
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;
import org.platformlambda.models.SamplePoJo;

import javax.ws.rs.GET;
import javax.ws.rs.Path;
import javax.ws.rs.PathParam;
import javax.ws.rs.Produces;
import javax.ws.rs.container.AsyncResponse;
import javax.ws.rs.container.Suspended;
import javax.ws.rs.core.MediaType;
import java.io.IOException;
import java.util.Collections;
import java.util.HashMap;

@Path("/pojo")
public class HelloPoJoEventOverHttp {

    @GET
    @Path("/http/{id}")
    @Produces({MediaType.TEXT_PLAIN, MediaType.APPLICATION_XML, MediaType.APPLICATION_JSON, MediaType.TEXT_HTML})
    public void getPoJo(@PathParam("id") Integer id, @Suspended AsyncResponse response) throws IOException {
        AppConfigReader config = AppConfigReader.getInstance();
        String remotePort = config.getProperty("lambda.example.port", "8085");
        String remoteEndpoint = "http://127.0.0.1:"+remotePort+"/api/event";
        String traceId = Utility.getInstance().getUuid();
        PostOffice po = new PostOffice("hello.pojo.endpoint", traceId, "GET /api/pojo/http");
        EventEnvelope req = new EventEnvelope().setTo("hello.pojo").setHeader("id", id);
        // to add security header(s) such as "Authorization", replace the empty map with some key-values
        Future<EventEnvelope> res = po.asyncRequest(req, 5000, Collections.emptyMap(), remoteEndpoint, true);
        res.onSuccess(event -> {
            // confirm that the PoJo object is transported correctly over the event stream system
            if (event.getBody() instanceof SamplePoJo) {
                response.resume(event.getBody());
            } else {
                response.resume(new AppException(event.getStatus(), event.getError()));
            }
        });
        res.onFailure(response::resume);
    }
}
