/*

    Copyright 2018-2023 Accenture Technology

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

import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.system.EventEmitter;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.util.Utility;

import java.io.IOException;
import java.util.List;

public class SuspendResume {
    private static final String APP_INSTANCE = "X-App-Instance";
    private static final String TYPE = "type";
    private static final String SUSPEND = "suspend";
    private static final String RESUME = "resume";
    private static final String USER = "user";
    private static final String WHEN = "when";
    private static final String NOW = "now";
    private static final String LATER = "later";

    public static void handle(boolean resume, HttpServletRequest request, HttpServletResponse response)
            throws IOException {
        String type = resume? RESUME : SUSPEND;
        String origin = request.getHeader(APP_INSTANCE);
        if (origin == null) {
            response.sendError(400, "Missing "+ APP_INSTANCE +" in request header");
            return;
        }
        EventEmitter po = EventEmitter.getInstance();
        EventEnvelope event = new EventEnvelope().setHeader(TYPE, type)
                .setHeader(USER, System.getProperty("user.name"));
        if (origin.equals(Platform.getInstance().getOrigin())) {
            event.setTo(EventEmitter.ACTUATOR_SERVICES);
        } else {
            if (!po.exists(origin)) {
                response.sendError(404, origin+" is not reachable");
                return;
            }
            event.setTo(EventEmitter.ACTUATOR_SERVICES+"@"+origin);
        }
        List<String> path = Utility.getInstance().split(request.getPathInfo(), "/");
        String when = !path.isEmpty() && NOW.equals(path.get(0)) ? NOW : LATER;
        event.setHeader(WHEN, when);
        po.send(event);
        String message = type+" request sent to " + origin;
        if (LATER.equals(when)) {
            message += ". It will take effect in one minute.";
        }
        response.sendError(200, message);
    }
}
