/*

    Copyright 2018-2019 Accenture Technology

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

import com.accenture.examples.circuit.breaker.CircuitBreakerCommand;
import org.platformlambda.core.models.Kv;

import javax.ws.rs.GET;
import javax.ws.rs.Path;
import javax.ws.rs.Produces;
import javax.ws.rs.core.MediaType;
import java.util.concurrent.atomic.AtomicInteger;

@Path("/circuit")
public class JaxCircuitBreaker {

    private static AtomicInteger seq = new AtomicInteger(0);

    @GET
    @Path("/breaker")
    @Produces({MediaType.TEXT_PLAIN, MediaType.APPLICATION_JSON, MediaType.APPLICATION_XML, MediaType.TEXT_HTML})
    public Object hello() {
        CircuitBreakerCommand demo = new CircuitBreakerCommand("no.such.service", "hello.world",
                3000, null, new Kv("hello", "world"), new Kv("seq", seq.incrementAndGet()));
        // this will automatically fall back to use "hello.world"
        return demo.execute();
    }

}
