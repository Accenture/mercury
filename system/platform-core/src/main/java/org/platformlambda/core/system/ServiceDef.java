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

import org.platformlambda.core.models.StreamFunction;
import org.platformlambda.core.models.TypedLambdaFunction;
import org.platformlambda.core.util.Utility;

import java.util.Date;

public class ServiceDef {

    private static final int MAX_INSTANCES = 1000;

    private final String route;
    @SuppressWarnings("rawtypes")
    private final TypedLambdaFunction lambda;
    private final StreamFunction stream;
    private final String id;
    private ServiceQueue manager;
    private final Date created = new Date();
    private boolean isPrivateFunction = false;
    private boolean isStreamFunction = false;
    private int instances = 1;

    @SuppressWarnings("rawtypes")
    public ServiceDef(String route, TypedLambdaFunction lambda) {
        this.id = Utility.getInstance().getUuid();
        this.route = route;
        this.lambda = lambda;
        this.stream = null;
    }

    public ServiceDef(String route, StreamFunction stream) {
        this.id = Utility.getInstance().getUuid();
        this.route = route;
        this.stream = stream;
        this.lambda = null;
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

    public StreamFunction getStreamFunction() {
        return stream;
    }

    public boolean isPrivate() {
        return isPrivateFunction;
    }

    public int getConcurrency() {
        return instances;
    }

    public ServiceDef setConcurrency(int instances) {
        this.instances = Math.max(1, (Math.min(instances, MAX_INSTANCES)));
        return this;
    }

    public ServiceDef setPrivate(boolean isPrivateFunction) {
        this.isPrivateFunction = isPrivateFunction;
        return this;
    }

    public ServiceQueue getManager() {
        return manager;
    }

    public void setManager(ServiceQueue manager) {
        this.manager = manager;
    }

    public ServiceDef setStream(boolean isStreamFunction) {
        this.isStreamFunction = isStreamFunction;
        return this;
    }

    public boolean isStream() {
        return isStreamFunction;
    }

}
