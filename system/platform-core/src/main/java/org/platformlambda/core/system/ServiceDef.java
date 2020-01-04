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

import akka.actor.ActorRef;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.util.Utility;

import java.util.Date;

public class ServiceDef {

    private static final int MAX_INSTANCES = 1000;

    private String route;
    private LambdaFunction lambda;
    private String id;
    private boolean isPrivate = false;
    private int instances = 1;
    private ActorRef manager;
    private Date created = new Date();

    public ServiceDef(String route, LambdaFunction lambda, ActorRef manager) {
        this.id = Utility.getInstance().getUuid();
        this.route = route;
        this.lambda = lambda;
        this.manager = manager;
    }

    public String getId() {
        return id;
    }

    public Date getCreated() {
        return created;
    }

    public String getRoute() {
        return route;
    }

    public ActorRef getManager() {
        return manager;
    }

    public LambdaFunction getFunction() {
        return lambda;
    }

    public boolean isPrivate() {
        return isPrivate;
    }

    public int getConcurrency() {
        return instances;
    }

    public ServiceDef setConcurrency(Integer instances) {
        if (instances != null) {
            this.instances = instances < 1 ? 1 : (instances > MAX_INSTANCES ? MAX_INSTANCES : instances);
        }
        return this;
    }

    public ServiceDef setPrivate(Boolean isPrivate) {
        if (isPrivate != null) {
            this.isPrivate = isPrivate;
        }
        return this;
    }

}
