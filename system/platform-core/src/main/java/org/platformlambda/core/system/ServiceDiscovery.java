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

package org.platformlambda.core.system;

import org.platformlambda.core.models.LambdaClient;

import java.util.ArrayList;
import java.util.Date;
import java.util.List;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

public class ServiceDiscovery {

    public static final String SERVICE_REGISTRY = "system.service.registry";
    public static final String SERVICE_QUERY = "system.service.query";
    public static final String TYPE = "type";
    public static final String ORIGIN = "origin";
    public static final String TX_PATH = "tx_path";
    public static final String ROUTE = "route";
    public static final String ADD = "add";
    public static final String REMOVE = "remove";
    public static final String UNREGISTER = "unregister";
    public static final String TRUE = "true";
    public static final String FIND = "find";

    /*
     * lambda routing table
     *
     * Note:
     * boolean values for origins and routes are always set to true.
     * They are there to support concurrent updates.
     *
     * clients: origin to LambdaClient(txPaths, personality)
     * origins: origin to (route, true)
     * routes: route to (origin, true)
     */
    protected static final ConcurrentMap<String, LambdaClient> clients = new ConcurrentHashMap<>();
    protected static final ConcurrentMap<String, ConcurrentMap<String, Boolean>> origins = new ConcurrentHashMap<>();
    protected static final ConcurrentMap<String, ConcurrentMap<String, Date>> routes = new ConcurrentHashMap<>();

    public static void createLambdaClient(String origin, String txPath) {
        clients.put(origin, new LambdaClient(txPath));
    }

    public static void setPersonality(String origin, String name, String personality) {
        LambdaClient client = clients.get(origin);
        if (client != null) {
            client.name = name;
            client.personality = personality;
        }
    }

    public static String getTxPath(String origin) {
        if (origin != null) {
            LambdaClient client = clients.get(origin);
            if (client != null) {
                return client.txPath;
            }
        }
        return null;
    }

    public static String getPersonality(String origin) {
        if (origin != null) {
            LambdaClient client = clients.get(origin);
            if (client != null) {
                return client.personality;
            }
        }
        return null;
    }

    public static String getName(String origin) {
        if (origin != null) {
            LambdaClient client = clients.get(origin);
            if (client != null) {
                return client.name;
            }
        }
        return null;
    }

    public static boolean originExists(String origin) {
        return origin != null && clients.containsKey(origin);
    }

    public static List<String> getRoutes(String origin) {
        List<String> result = new ArrayList<>();
        if (origin == null) {
            return result;
        }
        ConcurrentMap<String, Boolean> entries = origins.get(origin);
        if (entries != null) {
            for (String r: entries.keySet()) {
                result.add(r);
            }
        }
        return result;
    }

    public static List<String> getAllPaths(String route) {
        if (route == null) {
            return null;
        }
        ConcurrentMap<String, Date> entries = routes.get(route);
        if (entries == null || entries.isEmpty()) {
            return null;
        }
        List<String> allPaths = new ArrayList<>();
        for (String origin: entries.keySet()) {
            String path = getTxPath(origin);
            if (path != null) {
                allPaths.add(path);
            }
        }
        return allPaths;
    }

}
