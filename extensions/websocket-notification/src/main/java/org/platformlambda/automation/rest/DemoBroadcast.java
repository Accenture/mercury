/*

    Copyright 2018-2020 Accenture Technology

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

package org.platformlambda.automation.rest;

import org.platformlambda.automation.MainApp;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.system.PostOffice;

import javax.ws.rs.*;
import javax.ws.rs.core.MediaType;
import java.io.IOException;
import java.util.Date;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.TimeoutException;

@Path("/ws")
public class DemoBroadcast {

    private static final String TYPE = "type";
    private static final String GET_PATHS = "get_paths";
    private static final String USER_ID = "user_id";

    @GET
    @Path("/notify/{user}")
    @SuppressWarnings("unchecked")
    @Produces({MediaType.TEXT_PLAIN, MediaType.APPLICATION_XML, MediaType.APPLICATION_JSON, MediaType.TEXT_HTML})
    public Object tellUser(@PathParam("user") String user, @QueryParam("message") String message)
            throws IOException, AppException, TimeoutException {
        if (message == null) {
            throw new IllegalArgumentException("Missing message in query");
        }
        PostOffice po = PostOffice.getInstance();
        EventEnvelope list = po.request(MainApp.WS_NOTIFICATION_SERVICE, 5000,
                                            new Kv(TYPE, GET_PATHS), new Kv(USER_ID, user));
        if (list.getBody() instanceof List) {
            List<String> paths = (List<String>) list.getBody();
            if (paths.isEmpty()) {
                throw new AppException(404, user+" not found");
            } else {
                /*
                 * we recommend putting an event type in the message so that
                 * the browser can make sense of the notification message.
                 */
                Map<String, Object> event = new HashMap<>();
                event.put("type", "notification");
                event.put("message", message);
                event.put("time", new Date());
                for (String p: paths) {
                    po.send(p, event);
                }
                Map<String, Object> result = new HashMap<>();
                result.put("message", message);
                result.put("status", 200);
                result.put("result", "message delivered to "+paths.size()+" connection"+(paths.size() == 1? "":"s"));
                result.put("time", new Date());
                return result;
            }

        } else {
            throw new IllegalArgumentException("Invalid response from "+MainApp.WS_NOTIFICATION_SERVICE);
        }
    }

}
