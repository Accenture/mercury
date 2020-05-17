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

package org.platformlambda.hazelcast;

import com.hazelcast.cluster.Member;
import com.hazelcast.cluster.MembershipEvent;
import com.hazelcast.cluster.MembershipListener;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.ServiceDiscovery;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.*;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

public class ClusterListener implements MembershipListener {
    private static final Logger log = LoggerFactory.getLogger(ClusterListener.class);

    private static final String TYPE = ServiceDiscovery.TYPE;
    private static final String SETUP_CONSUMER = TopicLifecycleListener.SETUP_CONSUMER;
    private static final String CLUSTER_CHANGED = TopicLifecycleListener.CLUSTER_CHANGED;
    private static final ConcurrentMap<String, String> nodes = new ConcurrentHashMap<>();

    public static Map<String, String> getMembers() {
        return new HashMap<>(nodes);
    }

    public static void setMembers(Set<Member> members) {
        for (Member m: members) {
            String now = Utility.getInstance().date2str(new Date());
            nodes.put(m.getUuid().toString(), m.getAddress().toString()+", "+now);
        }
    }

    @Override
    public void memberAdded(MembershipEvent event) {
        String now = Utility.getInstance().date2str(new Date());
        nodes.put(event.getMember().getUuid().toString(), event.getMember().getAddress().toString()+", "+now);
        log.info("Added {}", event.getMember());
        reloadClient();
    }

    @Override
    public void memberRemoved(MembershipEvent event) {
        nodes.remove(event.getMember().getUuid().toString());
        log.info("Removed {}", event.getMember());
        reloadClient();
    }

    private void reloadClient() {
        /*
         * When cluster membership changes, the hazelcast client event listener would become unreliable.
         * The event listener must be restarted to improve event stream stability.
         */
        try {
            PostOffice.getInstance().send(SETUP_CONSUMER, new Kv(TYPE, CLUSTER_CHANGED));
        } catch (IOException e) {
            log.error("Unable to reset event consumer - {}", e.getMessage());
        }
    }

}
