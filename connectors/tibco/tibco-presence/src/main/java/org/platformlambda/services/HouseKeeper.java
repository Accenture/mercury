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

package org.platformlambda.services;

import org.platformlambda.MainApp;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.*;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

public class HouseKeeper implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(HouseKeeper.class);

    private static final String MONITOR_PARTITION = MainApp.MONITOR_PARTITION;
    private static final String MONITOR_ALIVE = MainApp.MONITOR_ALIVE;
    private static final String TYPE = "type";
    private static final String DOWNLOAD = "download";
    private static final String INIT = "init";
    private static final String ORIGIN = "origin";
    private static final String INSTANCE = "instance";
    private static final long ONE_MINUTE = 60 * 1000;
    private static final ConcurrentMap<String, MonitorInstance> monitors = new ConcurrentHashMap<>();

    public static Map<String, String> getMonitors() {
        Utility util = Utility.getInstance();
        Map<String, String> result = new HashMap<>();
        for (String m: monitors.keySet()) {
            MonitorInstance monitor = monitors.get(m);
            result.put(m, m.equals(monitor.instance) ? util.date2str(new Date(monitor.updated)) :
                    monitor.instance+", "+util.date2str(new Date(monitor.updated)));
        }
        return result;
    }

    @Override
    @SuppressWarnings("unchecked")
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
        PostOffice po = PostOffice.getInstance();
        String myOrigin = Platform.getInstance().getOrigin();
        String type = headers.get(TYPE);
        // when a new presence monitor joins the system
        if (INIT.equals(type)) {
            if (myOrigin.equals(headers.get(ORIGIN))) {
                if (!monitors.containsKey(myOrigin)) {
                    String appId = headers.get(INSTANCE);
                    monitors.put(myOrigin, new MonitorInstance(appId == null ? myOrigin : appId,
                            System.currentTimeMillis()));
                    log.info("Registered monitor (me) {}", myOrigin);
                }
            } else {
                po.send(MainApp.PRESENCE_HOUSEKEEPER+MONITOR_PARTITION,
                        new ArrayList<>(MonitorService.getConnections().keySet()),
                        new Kv(ORIGIN, myOrigin), new Kv(TYPE, MONITOR_ALIVE));
            }
        }
        // when a monitor sends keep-alive
        if (MONITOR_ALIVE.equals(type) && headers.containsKey(ORIGIN)) {
            String origin = headers.get(ORIGIN);
            String me = Platform.getInstance().getOrigin();
            if (!monitors.containsKey(origin)) {
                log.info("Registered monitor {} {}", me.equals(origin) ? "(me)" : "(peer)", origin);
            }
            String appId = headers.get(INSTANCE);
            monitors.put(origin, new MonitorInstance(appId == null ? origin : appId, System.currentTimeMillis()));
            removeExpiredMonitors();
            if (body instanceof List) {
                // compare connection list
                Map<String, Object> connections = MonitorService.getConnections();
                List<String> myConnections = new ArrayList<>(connections.keySet());
                List<String> peerConnections = (List<String>) body;
                if (!sameList(myConnections, peerConnections)) {
                    log.debug("Sync up because my list ({}) does not match peer ({})",
                            myConnections.size(), peerConnections.size());
                    // download current connections from peers
                    PostOffice.getInstance().send(MainApp.PRESENCE_HANDLER+MONITOR_PARTITION,
                            new Kv(TYPE, DOWNLOAD), new Kv(ORIGIN, me));
                }
            }
        }
        return true;
    }

    private boolean sameList(List<String> a, List<String> b) {
        if (a.size() != b.size()) {
            return false;
        }
        if (a.size() > 1) {
            Collections.sort(a);
        }
        if (b.size() > 1) {
            Collections.sort(b);
        }
        return a.toString().equals(b.toString());
    }

    private void removeExpiredMonitors() {
        long now = System.currentTimeMillis();
        List<String> expired = new ArrayList<>();
        for (String k: monitors.keySet()) {
            MonitorInstance monitor = monitors.get(k);
            if (now - monitor.updated > ONE_MINUTE) {
                expired.add(k);
            }
        }
        if (!expired.isEmpty()) {
            for (String k: expired) {
                monitors.remove(k);
                log.info("Removed monitor {}", k);
            }
        }
    }

    private static class MonitorInstance {
        public String instance;
        public long updated;

        public MonitorInstance(String instance, long updated) {
            this.instance = instance;
            this.updated = updated;
        }
    }

}
