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

package org.platformlambda.kafka;

import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.models.VersionInfo;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.ServerPersonality;
import org.platformlambda.core.system.ServiceDiscovery;
import org.platformlambda.core.util.Utility;

import java.util.Date;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class ServiceQuery implements LambdaFunction {

    private static String TYPE = ServiceDiscovery.TYPE;
    private static String ROUTE = ServiceDiscovery.ROUTE;
    private static String FIND = ServiceDiscovery.FIND;
    private static String DOWNLOAD = "download";
    private static String INFO = "info";

    @Override
    @SuppressWarnings("unchecked")
    public Object handleEvent(Map<String, String> headers, Object body, int instance) {
        String type = headers.get(TYPE);
        if (INFO.equals(type)) {
            Platform platform = Platform.getInstance();
            Map<String, Object> result = new HashMap<>();
            result.put("personality", ServerPersonality.getInstance().getType());
            VersionInfo info = Utility.getInstance().getVersionInfo();
            result.put("version", info.getVersion());
            result.put("name", platform.getName());
            result.put("origin", platform.getOrigin());
            result.put("group_id", info.getGroupId());
            result.put("artifact_id", info.getArtifactId());
            return result;

        } else if (DOWNLOAD.equals(type)) {
            Utility util = Utility.getInstance();
            Platform platform = Platform.getInstance();
            String me = platform.getName()+", v"+util.getVersionInfo().getVersion();
            Map<String, Object> result = new HashMap<>();
            result.put("routes", ServiceRegistry.getAllRoutes());
            result.put("nodes", ServiceRegistry.getAllOrigins());
            result.put("name", me);
            result.put("this", platform.getOrigin());
            result.put("time", new Date());
            return result;

        } else if (FIND.equals(type) && headers.containsKey(ROUTE)) {
            String route = headers.get(ROUTE);
            if (route.equals("*")) {
                if (body instanceof List) {
                    return exists((List<String>) body);
                } else {
                    return false;
                }
            } else {
                return exists(route);
            }

        } else {
            throw new IllegalArgumentException("Usage: type=download, info or (type=find, route=route_name)");
        }
    }

    private boolean exists(List<String> routes) {
        for (String r: routes) {
            if (!exists(r)) {
                return false;
            }
        }
        return true;
    }

    private boolean exists(String route) {
        if (route.contains(".")) {
            // normal route name
            if (Platform.getInstance().hasRoute(route)) {
                return true;
            }
            Map<String, String> targets = ServiceRegistry.getDestinations(route);
            return targets != null && !targets.isEmpty();
        } else {
            // origin-ID
            return ServiceRegistry.destinationExists(route);
        }
    }

}
