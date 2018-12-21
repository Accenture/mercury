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
