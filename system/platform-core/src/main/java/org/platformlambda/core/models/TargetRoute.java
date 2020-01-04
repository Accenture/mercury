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

package org.platformlambda.core.models;

import akka.actor.ActorRef;

import java.util.List;

public class TargetRoute {

    private List<String> txPaths = null;
    private ActorRef actor = null;
    private boolean cloud = false;

    public TargetRoute(List<String> txPaths) {
        this.txPaths = txPaths;
    }

    public TargetRoute(ActorRef actor, boolean cloud) {
        this.actor = actor;
        this.cloud = cloud;
    }

    public boolean isEventNode() {
        return txPaths != null;
    }

    public boolean isCloud() {
        return cloud;
    }

    public ActorRef getActor() {
        return actor;
    }

    public List<String> getTxPaths() {
        return txPaths;
    }

}
