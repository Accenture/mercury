package org.platformlambda.rest;

import javax.ws.rs.*;
import javax.ws.rs.core.MediaType;
import java.util.Date;
import java.util.Map;

@Path("/hello")
public class HelloWorld {

    @GET
    @Path("/world")
    @Produces({MediaType.TEXT_PLAIN, MediaType.APPLICATION_JSON, MediaType.APPLICATION_XML, MediaType.TEXT_HTML})
    public Object hello() {
        // demonstrate automatic serialization
        PoJo result = new PoJo();
        result.name = "Hello World";
        result.address = "100 World Blvd, Earth";
        result.telephone = "123-456-7890";
        result.message = "Congratulations! Hello world example endpoint is working fine";
        result.timestamp = new Date();
        return result;
    }

    @PUT
    @Path("/world")
    @Consumes({MediaType.APPLICATION_JSON, MediaType.APPLICATION_XML})
    @Produces({MediaType.TEXT_PLAIN, MediaType.APPLICATION_JSON, MediaType.APPLICATION_XML, MediaType.TEXT_HTML})
    public Object hello(Map<String, Object> data) {
        // demonstrate automatic serialization
        PoJo result = new PoJo();
        result.name = "Hello World";
        result.address = "100 World Blvd, Earth";
        result.telephone = "123-456-7890";
        result.message = "Congratulations! Hello world example endpoint is working fine";
        result.timestamp = new Date();
        result.data = data;
        return result;
    }

    private class PoJo {
        public String name, address, telephone, message;
        public Date timestamp;
        public Map<String, Object> data;
    }

}