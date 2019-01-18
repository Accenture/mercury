package org.platformlambda.rest.spring.servlet;

import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.servlet.annotation.WebServlet;
import javax.servlet.http.HttpServlet;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import java.io.IOException;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;
import java.util.concurrent.TimeoutException;

@WebServlet("/health")
public class HealthServlet extends HttpServlet {
	private static final long serialVersionUID = 3238645611193082445L;
	private static final Logger log = LoggerFactory.getLogger(HealthServlet.class);

    private static final ConcurrentMap<String, Map<String, Object>> metadata = new ConcurrentHashMap<>();
    private static final String TYPE = "type";
    private static final String QUERY = "query";
    private static final String INFO = "info";
    private static final String HEALTH = "health";
    private static final String NODE_INFO = "node.info";
    private static final String REQUIRED_SERVICES = "mandatory.health.dependencies";
    private static final String OPTIONAL_SERVICES = "optional.health.dependencies";
    private static final String ROUTE = "route";
    private static final String MESSAGE = "message";
    private static final String STATUS = "status";
    private static final String STATUS_CODE = "statusCode";
    private static final String REQUIRED = "required";
    private static final String UPSTREAM = "upstream";
    private static final Map<String, Object> basicInfo = new HashMap<>();
    private static List<String> requiredServices = new ArrayList<>();
    private static List<String> optionalServices = new ArrayList<>();
    private static boolean loadInfo = false, listInfo = false;

    @SuppressWarnings("unchecked")
    public static Map<String, Object> getBasicInfo() {
        if (!loadInfo) {
            // get platform specific node information
            AppConfigReader reader = AppConfigReader.getInstance();
            String nodeInfo = reader.getProperty(NODE_INFO, "node.info");
            if (Platform.getInstance().hasRoute(nodeInfo)) {
                // do only once
                loadInfo = true;
                try {
                    EventEnvelope response = PostOffice.getInstance().request(nodeInfo, 5000, new Kv(TYPE, QUERY));
                    if (response.getBody() instanceof Map) {
                        basicInfo.putAll((Map<String, Object>) response.getBody());
                    }
                } catch (IOException | TimeoutException | AppException e) {
                    log.warn("Unable to obtain node info - {}", e.getMessage());
                }
            }
        }
        return basicInfo;
    }

    private void loadConfig() {
        HealthServlet.getBasicInfo();
        if (!listInfo) {
            listInfo = true;
            AppConfigReader reader = AppConfigReader.getInstance();
            requiredServices = Utility.getInstance().split(reader.getProperty(REQUIRED_SERVICES, ""), ", ");
            if (requiredServices.isEmpty()) {
                log.info("Mandatory service dependencies - {}", requiredServices);
            }
            optionalServices = Utility.getInstance().split(reader.getProperty(OPTIONAL_SERVICES, ""), ", ");
            if (!optionalServices.isEmpty()) {
                log.info("Optional services dependencies - {}", optionalServices);
            }
        }
    }

    @Override
    protected void doGet(HttpServletRequest request, HttpServletResponse response) throws IOException {
        loadConfig();
        boolean up = true;
        Map<String, Object> result = new HashMap<>(basicInfo);
        List<Map<String, Object>> upstream = new ArrayList<>();
        result.put(UPSTREAM, upstream);
        /*
         * Checking dependencies
         */
        checkServices(upstream, optionalServices, false);
        if (!checkServices(upstream, requiredServices, true)) {
            up = false;
        }
        result.put(STATUS, up? "UP" : "DOWN");
        response.setContentType("application/json");
        response.setCharacterEncoding("utf-8");
        response.getWriter().write(SimpleMapper.getInstance().getMapper().writeValueAsString(result));
    }

    @SuppressWarnings("unchecked")
    private boolean checkServices(List<Map<String, Object>> upstream, List<String> healthServices, boolean required) {
        PostOffice po = PostOffice.getInstance();
        boolean up = true;
        for (String route: healthServices) {
            Map<String, Object> m = new HashMap<>();
            m.put(ROUTE, route);
            m.put(REQUIRED, required);
            try {
                if (!metadata.containsKey(route)) {
                    EventEnvelope infoRes = po.request(route, 3000, new Kv(TYPE, INFO));
                    if (infoRes.getBody() instanceof Map) {
                        metadata.put(route, (Map<String, Object>) infoRes.getBody());
                    }
                }
                Map<String, Object> info = metadata.get(route);
                if (info != null) {
                    for (String k : info.keySet()) {
                        m.put(k, info.get(k));
                    }
                }
                EventEnvelope res = po.request(route, 10000, new Kv(TYPE, HEALTH));
                if (res.getBody() instanceof String) {
                    m.put(STATUS_CODE, res.getStatus());
                    m.put(MESSAGE, res.getBody());
                    if (res.getStatus() != 200) {
                        up = false;
                    }
                }
            } catch (IOException e) {
                m.put(STATUS_CODE, 500);
                m.put(MESSAGE, e.getMessage());
                up = false;
            } catch (TimeoutException e) {
                m.put(STATUS_CODE, 408);
                m.put(MESSAGE, e.getMessage());
                up = false;
            } catch (AppException e) {
                m.put(STATUS_CODE, e.getStatus());
                m.put(MESSAGE, e.getMessage());
                up = false;
            }
            upstream.add(m);
        }
        return up;
    }

}
