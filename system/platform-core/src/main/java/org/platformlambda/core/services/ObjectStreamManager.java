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
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.*;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

@ZeroTracing
public class ObjectStreamManager implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(ObjectStreamManager.class);

    private static final String TYPE = "type";
    private static final String CREATE = "create";
    private static final String CLOSE = ObjectStreamService.CLOSE;
    private static final String NAME = "name";
    private static final String DESTROY = "destroy";
    private static final String QUERY = "query";
    private static final String TRANSACTIONS = "transactions";
    private static final String EXPIRY_SEC = "expiry_seconds";
    private static final String CREATED = "created";
    private static final String UPDATED = "updated";
    private static final int ONE_MINUTE = 60;
    private static final int DEFAULT_EXPIRY = 30 * ONE_MINUTE;

    private static final ConcurrentMap<String, StreamInfo> streams = new ConcurrentHashMap<>();
    private static StreamHouseKeeper houseKeeper;

    public ObjectStreamManager() {
        if (houseKeeper == null) {
            houseKeeper = new StreamHouseKeeper();
            houseKeeper.start();
        }
    }

    public static void touch(String id) {
        StreamInfo info = streams.get(id);
        if (info != null) {
            info.updated = System.currentTimeMillis();
        }
    }

    public static void increment(String id) {
        StreamInfo info = streams.get(id);
        if (info != null) {
            info.increment();
        }
    }

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws IOException {

        Utility util = Utility.getInstance();
        Platform platform = Platform.getInstance();
        if (CREATE.equals(headers.get(TYPE))) {
            int expirySeconds = headers.containsKey(EXPIRY_SEC) ? util.str2int(headers.get(EXPIRY_SEC)) : DEFAULT_EXPIRY;
            // minimum inactivity expiry timer is 10 seconds. Default is 30 minutes.
            if (expirySeconds < 10) {
                expirySeconds = 10;
            }
            ObjectStreamService service = new ObjectStreamService();
            platform.registerPrivate(service.getPath(), service, 1);
            streams.put(service.getPath(), new StreamInfo(expirySeconds));
            log.info("{} created with inactivity expiry of {} seconds", service.getPath(), expirySeconds);
            // return fully qualified name
            return service.getPath()+"@"+platform.getOrigin();
        } else if (DESTROY.equals(headers.get(TYPE)) && headers.containsKey(NAME)) {
            String fqName = headers.get(NAME);
            String name = fqName.contains("@") ? fqName.substring(0, fqName.indexOf('@')) : fqName;
            if (streams.containsKey(name)) {
                StreamInfo info = streams.get(name);
                streams.remove(name);
                platform.release(name);
                log.info("{} closed ({} - {}, transactions={})", name,
                        util.date2str(new Date(info.created), true),
                        util.date2str(new Date(info.updated), true), info.count);
                return true;
            } else {
                throw new IllegalArgumentException(name + " not found");
            }
        } else if (QUERY.equals(headers.get(TYPE))) {
            Map<String, Map<String, Object>> result = new HashMap<>();
            for (String key: streams.keySet()) {
                StreamInfo info = streams.get(key);
                Map<String, Object> map = new HashMap<>();
                map.put(CREATED, new Date(info.created));
                map.put(UPDATED, new Date(info.updated));
                map.put(EXPIRY_SEC, info.expiryMills / 1000);
                map.put(TRANSACTIONS, info.count);
                result.put(key, map);
            }
            return result;

        } else {
            throw new IllegalArgumentException("type must be create, destroy or query");
        }
    }

    private class StreamInfo {

        public long created = System.currentTimeMillis();
        public long updated;
        public long expiryMills;
        public int count = 0;

        public StreamInfo(long expirySeconds) {
            this.updated = this.created;
            // seconds to milliseconds
            this.expiryMills = expirySeconds * 1000;
        }

        public void increment() {
            count += 1;
        }

    }

    private class StreamHouseKeeper extends Thread {

        private static final long INTERVAL = 10000;
        private boolean normal = true;

        public StreamHouseKeeper() {
            Runtime.getRuntime().addShutdownHook(new Thread(this::shutdown));
        }

        @Override
        public void run() {

            log.info("Started");
            Utility util = Utility.getInstance();
            Platform platform = Platform.getInstance();
            long t1 = System.currentTimeMillis();
            while (normal) {
                long now = System.currentTimeMillis();
                // scan every 10 seconds
                if (now - t1 > INTERVAL) {
                    t1 = now;
                    if (!streams.isEmpty()) {
                        List<String> list = new ArrayList<>(streams.keySet());
                        for (String id : list) {
                            StreamInfo info = streams.get(id);
                            if (now - info.updated > info.expiryMills) {
                                try {
                                    log.warn("{} expired. Inactivity for {} seconds ({} - {}, transactions={})", id,
                                            info.expiryMills / 1000,
                                            util.date2str(new Date(info.created), true),
                                            util.date2str(new Date(info.updated), true), info.count);
                                    // tell service to destroy elastic queue object
                                    PostOffice.getInstance().send(id, new Kv(TYPE, CLOSE));
                                } catch (IOException e) {
                                    // perhaps stream service route is closed concurrently
                                    log.error("Unable to release {} - {}", id, e.getMessage());
                                    streams.remove(id);
                                    try {
                                        platform.release(id);
                                    } catch (IOException ex) {
                                        // this should not occur
                                    }
                                }

                            }
                        }
                    }
                }
                try {
                    Thread.sleep(1000);
                } catch (InterruptedException e) {
                    // ok to ignore
                }
            }
            log.info("Stopped");
        }

        private void shutdown() {
            normal = false;
        }

    }

}
