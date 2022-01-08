/*

    Copyright 2018-2022 Accenture Technology

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

package org.platformlambda.rest;

import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.serializers.SimpleXmlWriter;

import javax.ws.rs.*;
import javax.ws.rs.core.MediaType;
import java.util.*;

@Path("/hello")
public class HelloWorld {

    private static final SimpleXmlWriter xml = new SimpleXmlWriter();

    @GET
    @Path("/world")
    @Produces({MediaType.TEXT_PLAIN, MediaType.APPLICATION_JSON, MediaType.APPLICATION_XML, MediaType.TEXT_HTML})
    public Object hello(@QueryParam("test") String test) {
        if ("400".equals(test)) {
            throw new IllegalArgumentException("test");
        }
        if ("401".equals(test)) {
            throw new NotAuthorizedException("Unauthorized");
        }
        if ("403".equals(test)) {
            throw new ForbiddenException("Forbidden");
        }
        if ("406".equals(test)) {
            throw new NotAcceptableException("Not acceptable");
        }
        if ("503".equals(test)) {
            throw new ServiceUnavailableException("System temporarily unavailable");
        }
        PoJo result = new PoJo();
        result.name = "Hello World";
        result.address = "100 World Blvd, Earth";
        result.telephone = "123-456-7890";
        result.message = "Congratulations! Hello world example endpoint is working fine";
        result.timestamp = new Date();
        return result;
    }

    @GET
    @Path("/list")
    @Produces({MediaType.APPLICATION_XML})
    public Object helloList() {
        List<String> result = new ArrayList<>();
        result.add("one");
        result.add("two");
        result.add("three");
        return result;
    }

    @GET
    @Path("/json")
    @Produces({MediaType.APPLICATION_JSON})
    public Object helloTextJsonOutput() {
        PoJo result = new PoJo();
        result.name = "Hello World";
        result.address = "100 World Blvd, Earth";
        result.telephone = "123-456-7890";
        result.message = "Congratulations! Hello world example endpoint is working fine";
        result.timestamp = new Date();
        return SimpleMapper.getInstance().getMapper().writeValueAsString(result);
    }


    @GET
    @Path("/xml")
    @Produces({MediaType.APPLICATION_XML})
    public Object helloTextXmlOutput() {
        Map<String, Object> result = new HashMap<>();
        result.put("hello", "xml");
        return xml.write(result);
    }

    @PUT
    @Path("/world")
    @Consumes({MediaType.APPLICATION_JSON, MediaType.APPLICATION_XML})
    @Produces({MediaType.TEXT_PLAIN, MediaType.APPLICATION_JSON, MediaType.APPLICATION_XML, MediaType.TEXT_HTML})
    public Object hello(Map<String, Object> data) {
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