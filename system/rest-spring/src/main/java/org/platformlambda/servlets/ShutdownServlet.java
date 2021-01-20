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

import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;

import javax.servlet.annotation.WebServlet;
import javax.servlet.http.HttpServlet;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import java.io.IOException;
import java.util.Date;

@WebServlet("/shutdown")
public class ShutdownServlet extends HttpServlet {
    private static final long serialVersionUID = 37489647906664051L;

    private static final int GRACE_PERIOD = 5000;
    private static final String ORIGIN = "origin";
    private static final String KEY = "key";
    private static final String SHUTDOWN_KEY = "app.shutdown.key";
    private static String secret;

    @Override
    protected void doGet(HttpServletRequest request, HttpServletResponse response) throws IOException {
        // send HTTP-404 "not found" instead of HTTP-405 "method not allowed"
        response.sendError(404, "Not Found");
    }

    @Override
    protected void doPost(HttpServletRequest request, HttpServletResponse response) throws IOException {
        String origin = request.getParameter(ORIGIN);
        if (origin == null) {
            response.sendError(400, "Missing "+ORIGIN+" in request body");
            return;
        }
        String key = request.getParameter(KEY);
        if (key == null) {
            response.sendError(400, "Missing "+KEY+" in request body");
            return;
        }
        if (!key.equals(getSecret())) {
            response.sendError(400, "key does not match "+SHUTDOWN_KEY+" in application.properties");
            return;
        }
        if (origin.equals(Platform.getInstance().getOrigin())) {
            EventEnvelope event = new EventEnvelope();
            event.setTo(PostOffice.SHUTDOWN_SERVICE).setBody(PostOffice.SHUTDOWN_SERVICE);
            PostOffice.getInstance().sendLater(event, new Date(System.currentTimeMillis() + GRACE_PERIOD));
            response.sendError(200, "This application will be shutdown in "+GRACE_PERIOD+" ms");
        } else {
            PostOffice po = PostOffice.getInstance();
            if (!po.exists(origin)) {
                response.sendError(400, origin+" is not reachable");
                return;
            }
            EventEnvelope event = new EventEnvelope();
            event.setTo(PostOffice.SHUTDOWN_SERVICE+"@"+origin).setBody(PostOffice.SHUTDOWN_SERVICE);
            PostOffice.getInstance().sendLater(event, new Date(System.currentTimeMillis() + GRACE_PERIOD));
            response.sendError(200, origin+" will be shutdown in "+GRACE_PERIOD+" ms");
        }
    }

    private String getSecret() {
        if (secret == null) {
            AppConfigReader config = AppConfigReader.getInstance();
            secret = config.getProperty(SHUTDOWN_KEY, Utility.getInstance().getUuid());
        }
        return secret;
    }

}
