package org.platformlambda.servlets;

import org.platformlambda.core.models.Kv;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.ServiceDiscovery;

import javax.servlet.annotation.WebServlet;
import javax.servlet.http.HttpServlet;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import java.io.IOException;

@WebServlet("/resume")
public class ResumeServlet extends HttpServlet {

    private static final String APP_INSTANCE = "X-App-Instance";
    private static final String TYPE = "type";
    private static final String RESUME = "resume";
    private static final String USER = "user";

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
        PostOffice po = PostOffice.getInstance();
        if (origin.equals(Platform.getInstance().getOrigin())) {
            po.send(ServiceDiscovery.SERVICE_REGISTRY, new Kv(TYPE, RESUME),
                    new Kv(USER, System.getProperty("user.name")));
        } else {
            if (!po.exists(origin)) {
                response.sendError(400, origin+" is not reachable");
                return;
            }
            po.send(ServiceDiscovery.SERVICE_REGISTRY + "@" + origin, new Kv(TYPE, RESUME),
                    new Kv(USER, System.getProperty("user.name")));
        }
        response.sendError(200, "Resume request sent to "+origin+". It will take effect within one minute.");
    }

}
