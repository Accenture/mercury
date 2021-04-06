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

package org.platformlambda.core.system;

import akka.actor.ActorRef;
import org.platformlambda.core.models.TypedLambdaFunction;
import org.platformlambda.core.util.Utility;

import java.util.Date;

public class ServiceDef {

    private static final int MAX_INSTANCES = 1000;

    private final String route;
    @SuppressWarnings("rawtypes")
    private final TypedLambdaFunction lambda;
    private final String id;
    private ActorRef manager;
    private final Date created = new Date();
    private boolean isPrivate = false;
    private int instances = 1;

    @SuppressWarnings("rawtypes")
    public ServiceDef(String route, TypedLambdaFunction lambda) {
        this.id = Utility.getInstance().getUuid();
        this.route = route;
        this.lambda = lambda;
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

    @SuppressWarnings("rawtypes")
    public TypedLambdaFunction getFunction() {
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

    public ActorRef getManager() {
        return manager;
    }

    public void setManager(ActorRef manager) {
        this.manager = manager;
    }

}
