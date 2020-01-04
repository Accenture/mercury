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

package org.platformlambda.rest.spring.servlet;

import org.platformlambda.core.annotations.OptionalService;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.Utility;

import javax.servlet.annotation.WebServlet;
import javax.servlet.http.HttpServlet;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import java.io.IOException;
import java.util.Date;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

@WebServlet("/route/substitution/*")
@OptionalService("application.feature.route.substitution")
public class RouteSubstitution extends HttpServlet {

    @Override
    protected void doPut(HttpServletRequest request, HttpServletResponse response) throws IOException {
        // PUT /route/substitution/{route}/{replacement}
        Utility util = Utility.getInstance();
        List<String> pathElements = util.split(request.getPathInfo(), "/");
        if (pathElements.size() == 2) {
            String original = pathElements.get(0);
            String replacement = pathElements.get(1);
            if (util.validServiceName(original) && util.validServiceName(replacement)
                    && original.contains(".") && replacement.contains(".")) {
                if (original.equals(replacement)) {
                    response.sendError(400, "route and replacement cannot be the same");
                } else {
                    response.setContentType("application/json");
                    response.setCharacterEncoding("utf-8");
                    Platform platform = Platform.getInstance();
                    PostOffice po = PostOffice.getInstance();
                    try {
                        po.addRouteSubstitution(original, replacement);
                        // just broadcast the new route substitution
                        po.broadcast(platform.getRouteManagerName(),
                                new Kv("subsystem", "route_substitution"),
                                new Kv("origin", platform.getOrigin()),
                                new Kv("route", original),
                                new Kv("replacement", replacement),
                                new Kv("type", "add"));
                        // return completion status
                        Map<String, Object> result = new HashMap<>();
                        result.put("status", 200);
                        result.put("original", original);
                        result.put("substitution", replacement);
                        result.put("message", "Route substitution added");
                        result.put("time", new Date());
                        response.getWriter().println(SimpleMapper.getInstance().getMapper().writeValueAsString(result));
                    } catch (Exception e) {
                        response.sendError(400, e.getMessage());
                    }
                }
            } else {
                response.sendError(400, "Invalid route names");
            }
        } else {
            response.sendError(400, "The PUT request URL should be /info/route_substitution/{route}/{replacement}");
        }
    }

    @Override
    protected void doDelete(HttpServletRequest request, HttpServletResponse response) throws IOException {
        // DELETE /route/substitution/{route}
        Utility util = Utility.getInstance();
        List<String> pathElements = util.split(request.getPathInfo(), "/");
        if (pathElements.size() == 1) {
            String original = pathElements.get(0);
            if (util.validServiceName(original) && original.contains(".")) {
                response.setContentType("application/json");
                response.setCharacterEncoding("utf-8");
                Platform platform = Platform.getInstance();
                PostOffice po = PostOffice.getInstance();
                try {
                    po.removeRouteSubstitution(original);
                    // just broadcast the new route substitution
                    po.broadcast(platform.getRouteManagerName(),
                            new Kv("subsystem", "route_substitution"),
                            new Kv("origin", platform.getOrigin()),
                            new Kv("route", original),
                            new Kv("type", "remove"));
                    // return completion status
                    Map<String, Object> result = new HashMap<>();
                    result.put("status", 200);
                    result.put("route", original);
                    result.put("message", "Route substitution removed");
                    result.put("time", new Date());
                    response.getWriter().println(SimpleMapper.getInstance().getMapper().writeValueAsString(result));
                } catch (Exception e) {
                    response.sendError(400, e.getMessage());
                }
            } else {
                response.sendError(400, "Invalid service names");
            }
        } else {
            response.sendError(400, "The PUT request URL should be /info/route_substitution/{route}/{replacement}");
        }
    }

    @Override
    protected void doPost(HttpServletRequest request, HttpServletResponse response) throws IOException {
        // POST /route/substitution/sync
        Utility util = Utility.getInstance();
        List<String> pathElements = util.split(request.getPathInfo(), "/");
        if (pathElements.size() == 1 && pathElements.get(0).equals("sync")) {
            response.setContentType("application/json");
            response.setCharacterEncoding("utf-8");
            Platform platform = Platform.getInstance();
            PostOffice po = PostOffice.getInstance();
            try {
                po.broadcast(platform.getRouteManagerName(),
                        new Kv("subsystem", "route_substitution"),
                        new Kv("origin", platform.getOrigin()),
                        new Kv("type", "sync"));
                Map<String, Object> result = new HashMap<>();
                result.put("status", 200);
                result.put("message", "Merging route substitution table with other application instances of this module");
                result.put("time", new Date());
                response.getWriter().println(SimpleMapper.getInstance().getMapper().writeValueAsString(result));
            } catch (Exception e) {
                response.sendError(400, e.getMessage());
            }
        } else {
            response.sendError(400, "The POST request URL should be /info/route_substitution/sync");
        }
    }

}
