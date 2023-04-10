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

package org.platformlambda.core.system;

import org.platformlambda.core.annotations.CoroutineRunner;
import org.platformlambda.core.annotations.EventInterceptor;
import org.platformlambda.core.annotations.ZeroTracing;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.KotlinLambdaFunction;
import org.platformlambda.core.models.StreamFunction;
import org.platformlambda.core.models.TypedLambdaFunction;
import org.platformlambda.core.util.Utility;

import java.lang.reflect.Method;
import java.util.Date;

public class ServiceDef {
    private static final String HANDLE_EVENT = "handleEvent";
    private static final int MAX_INSTANCES = 1000;
    private final String route;
    @SuppressWarnings("rawtypes")
    private final TypedLambdaFunction lambda;
    private final StreamFunction stream;
    @SuppressWarnings("rawtypes")
    private final KotlinLambdaFunction suspendFunction;
    private final String id;
    private final boolean trackable;
    private final boolean coroutine;
    private final boolean interceptor;
    private final Date created = new Date();
    private boolean isPrivateFunction = false;
    private final boolean runAsStream;
    private final boolean runAsKotlin;
    private ServiceQueue manager;
    private Class<?> inputClass;
    private int instances = 1;

    @SuppressWarnings("rawtypes")
    public ServiceDef(String route, TypedLambdaFunction lambda) {
        this.trackable = lambda.getClass().getAnnotation(ZeroTracing.class) == null;
        this.interceptor = lambda.getClass().getAnnotation(EventInterceptor.class) != null;
        this.coroutine = lambda.getClass().getAnnotation(CoroutineRunner.class) != null;
        this.id = Utility.getInstance().getUuid();
        this.route = route;
        this.lambda = lambda;
        this.stream = null;
        this.suspendFunction = null;
        this.runAsStream = false;
        this.runAsKotlin = false;
        Method[] methods = lambda.getClass().getDeclaredMethods();
        for (Method m: methods) {
            Class<?>[] arguments = m.getParameterTypes();
            // HANDLE_EVENT method may be found more than once
            if (HANDLE_EVENT.equals(m.getName()) && arguments.length == 3) {
                String clsName = arguments[1].getName();
                if (clsName.contains(".") && !clsName.startsWith("java.")) {
                    inputClass = arguments[1];
                }
            }
        }
    }

    public ServiceDef(String route, StreamFunction lambda) {
        this.trackable = lambda.getClass().getAnnotation(ZeroTracing.class) == null;
        this.interceptor = lambda.getClass().getAnnotation(EventInterceptor.class) != null;
        this.coroutine = lambda.getClass().getAnnotation(CoroutineRunner.class) != null;
        this.id = Utility.getInstance().getUuid();
        this.route = route;
        this.stream = lambda;
        this.lambda = null;
        this.suspendFunction = null;
        this.runAsStream = true;
        this.runAsKotlin = false;
    }

    @SuppressWarnings("rawtypes")
    public ServiceDef(String route, KotlinLambdaFunction lambda) {
        this.trackable = lambda.getClass().getAnnotation(ZeroTracing.class) == null;
        this.interceptor = lambda.getClass().getAnnotation(EventInterceptor.class) != null;
        this.coroutine = true;
        this.id = Utility.getInstance().getUuid();
        this.route = route;
        this.lambda = null;
        this.stream = null;
        this.suspendFunction = lambda;
        this.runAsKotlin = true;
        this.runAsStream = false;
        Method[] methods = lambda.getClass().getDeclaredMethods();
        for (Method m: methods) {
            Class<?>[] arguments = m.getParameterTypes();
            // HANDLE_EVENT method may be found more than once
            // KotlinLambdaFunction is a "suspend" function and thus the last argument is the "Continuation" class
            if (HANDLE_EVENT.equals(m.getName()) && arguments.length == 4) {
                String clsName = arguments[1].getName();
                if (clsName.contains(".") && !clsName.startsWith("java.")) {
                    inputClass = arguments[1];
                }
            }
        }
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

    @SuppressWarnings("rawtypes")
    public KotlinLambdaFunction getSuspendFunction() {
        return suspendFunction;
    }

    public boolean isPrivate() {
        return isPrivateFunction;
    }

    public boolean isTrackable() {
        return trackable;
    }

    public boolean isCoroutine() {
        return coroutine;
    }

    public boolean isInterceptor() {
        return interceptor;
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

    public boolean isStream() {
        return runAsStream;
    }

    public boolean isKotlin() {
        return runAsKotlin;
    }

    public boolean inputIsEnvelope() {
        return EventEnvelope.class == inputClass;
    }

    public Class<?> getInputClass() {
        return inputClass;
    }

}
