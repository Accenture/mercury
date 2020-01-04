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

package org.platformlambda.core.services;

import org.platformlambda.core.annotations.ZeroTracing;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.ServiceDiscovery;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.Map;

@ZeroTracing
public class RouteSubstitutionManager implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(RouteSubstitutionManager.class);

    private static final String SUBSYSTEM = "subsystem";
    private static final String ROUTE_SUBSTITUTION = "route_substitution";
    private static final String ORIGIN = "origin";
    private static final String SYNC = "sync";
    private static final String MERGE = "merge";
    private static final String TYPE = ServiceDiscovery.TYPE;
    private static final String ADD = ServiceDiscovery.ADD;
    private static final String REMOVE = ServiceDiscovery.REMOVE;
    private static final String ROUTE = "route";
    private static final String REPLACEMENT = "replacement";

    @Override
    @SuppressWarnings("unchecked")
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws IOException {
        Platform platform = Platform.getInstance();
        PostOffice po = PostOffice.getInstance();
        if (headers.containsKey(SUBSYSTEM) && headers.containsKey(ORIGIN)) {
            if (ROUTE_SUBSTITUTION.equals(headers.get(SUBSYSTEM))) {
                // ignore my own merging request
                if (!Platform.getInstance().getOrigin().equals(headers.get(ORIGIN))) {
                    if (MERGE.equals(headers.get(TYPE)) && body instanceof Map) {
                        Map<String, String> entries = (Map<String, String>) body;
                        log.info("Merging {} route substitution {} with {}",
                                entries.size(), entries.size() == 1? "entry" : "entries", headers.get(ORIGIN));
                        for (String r: entries.keySet()) {
                            po.addRouteSubstitution(r, entries.get(r));
                        }
                    }
                }
                // synchronization request - this instance will broadcast its route substitution list
                if (SYNC.equals(headers.get(TYPE))) {
                    // broadcast my route substitution to other application instances of this module
                    Map<String, String> entries = po.getRouteSubstitutionList();
                    if (!entries.isEmpty()) {
                        log.info("Sending route substitution table to other application instances of this module");
                        po.broadcast(platform.getRouteManagerName(), entries,
                                new Kv(SUBSYSTEM, ROUTE_SUBSTITUTION),
                                new Kv(TYPE, MERGE), new Kv(ORIGIN, platform.getOrigin()));
                    }
                }
                // adding or removing a route substitution entry
                if (headers.containsKey(TYPE) && headers.containsKey(ROUTE)) {
                    String type = headers.get(TYPE);
                    String route = headers.get(ROUTE);
                    if (ADD.equals(type) && headers.containsKey(REPLACEMENT)) {
                        po.addRouteSubstitution(route, headers.get(REPLACEMENT));
                    }
                    if (REMOVE.equals(type)) {
                        po.removeRouteSubstitution(route);
                    }
                }
            }
        }
        return null;
    }


}
