package org.platformlambda.rest;

import org.platformlambda.services.MonitorService;
import org.platformlambda.services.PingScheduler;

import javax.servlet.http.HttpServletRequest;
import javax.ws.rs.GET;
import javax.ws.rs.Path;
import javax.ws.rs.Produces;
import javax.ws.rs.core.Context;
import javax.ws.rs.core.MediaType;
import java.util.Date;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

@Path("/ping")
public class PingEndpoint {

    @GET
    @Path("/now")
    @Produces({MediaType.TEXT_PLAIN, MediaType.APPLICATION_JSON, MediaType.APPLICATION_XML, MediaType.TEXT_HTML})
    public Map<String, Object> pingNow(@Context HttpServletRequest request) {
        PingScheduler.ping();
        List<String> origins = MonitorService.getOrigins();
        Map<String, Object> result = new HashMap<>();
        result.put("ping", true);
        result.put("app_instances", origins);
        result.put("total_instances", origins.size());
        result.put("time", new Date());
        result.put("message", "Broadcasting ping request to connected applications");
        return result;
    }
    
}
