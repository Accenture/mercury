package org.platformlambda.servlets;

import org.platformlambda.MainApp;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.ServiceDiscovery;
import org.platformlambda.services.MonitorService;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.servlet.annotation.WebServlet;
import javax.servlet.http.HttpServlet;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import java.io.IOException;
import java.util.*;
import java.util.concurrent.TimeoutException;

@WebServlet("/route/sync")
public class RouteSync extends HttpServlet {
    private static final Logger log = LoggerFactory.getLogger(RouteSync.class);

    private static final String MANAGER = MainApp.MANAGER;
    private static final String TYPE = "type";
    private static final String LIST = "list";
    private static final String PING = "ping";

    @Override
    @SuppressWarnings("unchecked")
    protected void doGet(HttpServletRequest request, HttpServletResponse response) throws IOException {
        PostOffice po = PostOffice.getInstance();
        try {
            EventEnvelope res = po.request(MANAGER, 30000, new Kv(TYPE, LIST));
            List<String> topics = res.getBody() instanceof List? (List<String>) res.getBody() : new ArrayList<>();
            Map<String, Object> connections = MonitorService.getConnections();
            List<String> origins = new ArrayList<>(connections.keySet());
            List<String> targets = new ArrayList<>();
            for (String p : origins) {
                if (topics.contains(p)) {
                    try {
                        po.send(ServiceDiscovery.SERVICE_REGISTRY + "@" + p, new Kv(TYPE, PING));
                        targets.add(p);
                    } catch (IOException e) {
                        log.error("Unable to ping {} - {}", p, e.getMessage());
                    }
                } else {
                    // this should not happen
                    log.error("{} is not a valid topic", p);
                }
            }
            Map<String, Object> result = new HashMap<>();
            if (targets.isEmpty()) {
                response.sendError(404, "There are no connected applications to ping");
            } else {
                result.put("ping", targets);
                result.put("time", new Date());
                response.setContentType("application/json");
                response.getOutputStream().write(SimpleMapper.getInstance().getMapper().writeValueAsBytes(result));
            }

        } catch (IOException e) {
            response.sendError(500, e.getMessage());
        } catch (TimeoutException e) {
            response.sendError(408, e.getMessage());
        } catch (AppException e) {
            response.sendError(e.getStatus(), e.getMessage());
        }

    }

}
