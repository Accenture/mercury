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

package org.platformlambda.hazelcast.services;

import com.hazelcast.core.LifecycleEvent;
import com.hazelcast.core.LifecycleListener;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class TopicLifecycleListener implements LifecycleListener {
    private static final Logger log = LoggerFactory.getLogger(TopicLifecycleListener.class);

    @Override
    public void stateChanged(LifecycleEvent event) {
        if (event.getState() == LifecycleEvent.LifecycleState.SHUTTING_DOWN) {
            log.error("Stopping application because Hazelcast is no longer available");
            System.exit(10);
        }
        if (event.getState() == LifecycleEvent.LifecycleState.CLIENT_DISCONNECTED) {
            log.error("Hazelcast is offline");
            System.exit(11);
        }
        if (event.getState() == LifecycleEvent.LifecycleState.CLIENT_CONNECTED) {
            log.info("Hazelcast is online");
        }
    }
}
