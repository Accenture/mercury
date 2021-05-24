/*

    Copyright 2018-2021 Accenture Technology

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

package org.platformlambda.servlets;

import com.google.api.client.http.*;
import com.google.api.client.http.javanet.NetHttpTransport;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.util.Utility;

import javax.servlet.annotation.WebServlet;
import javax.servlet.http.HttpServlet;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import java.io.IOException;
import java.util.List;

/**
 * This is a convenient class for making a HTTP GET to another application instance
 * in the same subnet, usually in a Kubernetes cluster.
 *
 * This allows DevOps admin easy access to the "/info", "/info/lib", "/info/routes" and "/health" endpoints
 * of other application instances through the presence monitor.
 */
@WebServlet("/http/*")
public class SimpleHttpGet extends HttpServlet {

    private static final HttpRequestFactory factory = new NetHttpTransport().createRequestFactory();
    private static final String APP_INSTANCE = "X-App-Instance";
    private static final int READ_TIMEOUT = 30000;

    @Override
    protected void doGet(HttpServletRequest request, HttpServletResponse response) throws IOException {
        String myOrigin = Platform.getInstance().getOrigin();
        String origin = request.getHeader(APP_INSTANCE);
        boolean secure = "true".equalsIgnoreCase(request.getParameter("secure"));
        if (!myOrigin.equals(origin)) {
            response.sendError(400, "Please provide correct x-app-instance header for this presence monitor");
            return;
        }
        Utility util = Utility.getInstance();
        List<String> uriSegments = util.split(request.getPathInfo(), "/");
        if (uriSegments.isEmpty()) {
            response.sendError(400, "Simple HTTP GET path should be /http/{host:port}/{uri_path}");
            return;
        }
        String protocol = secure? "https://" : "http://";
        GenericUrl target = new GenericUrl(protocol+getPath(uriSegments));
        HttpRequest req = factory.buildGetRequest(target).setReadTimeout(READ_TIMEOUT);
        HttpResponse res = null;
        try {
            res = req.execute();
            byte[] data = util.stream2bytes(res.getContent());
            HttpHeaders headers = res.getHeaders();
            String contentType = headers.getContentType();
            if (contentType != null) {
                response.setContentType(contentType);
            }
            response.getOutputStream().write(data);

        } catch (HttpResponseException e) {
            response.setStatus(e.getStatusCode());
            response.getWriter().write(e.getContent());
        } finally {
            if (res != null) {
                res.disconnect();
            }
        }
    }

    private String getPath(List<String> paths) {
        StringBuilder sb = new StringBuilder();
        for (String p: paths) {
            sb.append(p);
            sb.append('/');
        }
        return sb.substring(0, sb.length()-1);
    }

}
