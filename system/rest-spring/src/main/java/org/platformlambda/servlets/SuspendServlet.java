package org.platformlambda.servlets;

import org.platformlambda.core.models.Kv;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.ServiceDiscovery;
import org.platformlambda.core.util.Utility;

import javax.servlet.annotation.WebServlet;
import javax.servlet.http.HttpServlet;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import java.io.IOException;
import java.util.List;

@WebServlet("/suspend/*")
public class SuspendServlet extends HttpServlet {

    private static final String APP_INSTANCE = "X-App-Instance";
    private static final String TYPE = "type";
    private static final String SUSPEND = "suspend";
    private static final String USER = "user";
    private static final String WHEN = "when";
    private static final String NOW = "now";
    private static final String LATER = "later";

    @Override
    protected void doGet(HttpServletRequest request, HttpServletResponse response) throws IOException {
        response.sendError(404, "Not Found");
    }

    @Override
    protected void doPost(HttpServletRequest request, HttpServletResponse response) throws IOException {
        String origin = request.getHeader(APP_INSTANCE);
        if (origin == null) {
            response.sendError(400, "Missing "+ APP_INSTANCE +" in request header");
            return;
        }
        Utility util = Utility.getInstance();
        List<String> path = util.split(request.getPathInfo(), "/");
        String when = !path.isEmpty() && path.get(0).equals(NOW) ? NOW : LATER;
        PostOffice po = PostOffice.getInstance();
        if (origin.equals(Platform.getInstance().getOrigin())) {
            po.send(ServiceDiscovery.SERVICE_REGISTRY, new Kv(TYPE, SUSPEND), new Kv(WHEN, when),
                    new Kv(USER, System.getProperty("user.name")));
        } else {
            if (!po.exists(origin)) {
                response.sendError(400, origin+" is not reachable");
                return;
            }
            po.send(ServiceDiscovery.SERVICE_REGISTRY + "@" + origin, new Kv(TYPE, SUSPEND), new Kv(WHEN, when),
                    new Kv(USER, System.getProperty("user.name")));
        }
        String message = "Suspend request sent to " + origin;
        if (LATER.equals(when)) {
            message += ". It will take effect in one minute.";
        }
        response.sendError(200, message);
    }

}
