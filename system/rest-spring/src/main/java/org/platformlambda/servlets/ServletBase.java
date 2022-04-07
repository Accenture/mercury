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

package org.platformlambda.servlets;

import io.vertx.core.Future;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.servlet.AsyncContext;
import javax.servlet.http.HttpServlet;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import javax.ws.rs.core.MediaType;
import java.io.IOException;
import java.util.Map;

public abstract class ServletBase extends HttpServlet {
    private static final Logger log = LoggerFactory.getLogger(ServletBase.class);

    private static final String TYPE = "type";
    private static final String HOST = "host";
    private static final String APP_INSTANCE = "X-App-Instance";
    protected static final AppConfigReader config = AppConfigReader.getInstance();
    protected static final boolean protectEndpoint = "true".equals(
            config.getProperty("protect.info.endpoints", "false"));

    protected void submit(String type, HttpServletRequest request, HttpServletResponse response) throws IOException {
        final String myOrigin = Platform.getInstance().getOrigin();
        final String appOrigin = request.getHeader(APP_INSTANCE);
        if (appOrigin == null) {
            if (protectEndpoint && !isIntranetAddress(request)) {
                response.sendError(404, "Resource not found");
                return;
            }
        }
        final String origin = appOrigin == null? myOrigin : appOrigin;
        PostOffice po = PostOffice.getInstance();
        EventEnvelope event = new EventEnvelope().setHeader(TYPE, type);
        if (origin.equals(myOrigin)) {
            event.setTo(PostOffice.ACTUATOR_SERVICES);
        } else {
            if (!po.exists(origin)) {
                response.sendError(400, origin+" is not reachable");
                return;
            }
            event.setTo(PostOffice.ACTUATOR_SERVICES+"@"+origin);
        }
        AsyncContext context = request.startAsync();
        Future<EventEnvelope> result = po.asyncRequest(event, 10000);
        result.onSuccess(evt -> {
            Utility util = Utility.getInstance();
            HttpServletResponse res = (HttpServletResponse) context.getResponse();
            res.setStatus(evt.getStatus());
            Object data = evt.getBody();
            final byte[] b;
            if (data instanceof String) {
                res.setContentType(MediaType.TEXT_PLAIN);
                b = util.getUTF((String) data);
            } else {
                res.setContentType(MediaType.APPLICATION_JSON);
                b = SimpleMapper.getInstance().getMapper().writeValueAsBytes(evt.getBody());
            }
            res.setContentLength(b.length);
            try {
                res.getOutputStream().write(b);
            } catch (IOException e) {
                log.error("Unable to send HTTP response", e);
            }
            context.complete();
        });
        result.onFailure(ex -> {
            HttpServletResponse res = (HttpServletResponse) context.getResponse();
            res.setContentType(MediaType.APPLICATION_JSON);
            try {
                res.sendError(408, origin+" timeout");
            } catch (IOException e) {
                log.error("Unable to send HTTP response", e);
            }
            context.complete();
        });
    }

    private boolean isIntranetAddress(HttpServletRequest request) {
        return Utility.getInstance().isIntranetAddress(request.getHeader(HOST));
    }

}
