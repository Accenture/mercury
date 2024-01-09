/*

    Copyright 2018-2024 Accenture Technology

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

package org.platformlambda.example;

import org.platformlambda.core.annotations.MainApplication;
import org.platformlambda.core.models.EntryPoint;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.AppStarter;
import org.platformlambda.core.system.Platform;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.HashMap;
import java.util.Map;

@MainApplication
public class MainApp implements EntryPoint {
    private static final Logger log = LoggerFactory.getLogger(MainApp.class);

    public static void main(String[] args) {
        AppStarter.main(args);
    }

    @Override
    public void start(String[] args) throws Exception {
        // Obtain the platform singleton instance
        Platform platform = Platform.getInstance();
        // You can create a microservice as a lambda function inline or write it as a regular Java class
        LambdaFunction echo = (headers, input, instance) -> {
            log.info("echo #{} got a request", instance);
            Map<String, Object> result = new HashMap<>();
            result.put("body", input);
            result.put("instance", instance);
            result.put("origin", platform.getOrigin());
            return result;
        };
        // Register the above inline lambda function
        platform.register("hello.world", echo, 10);
        /*
         * There are a few demo services in the "services" folder.
         * They use the "PreLoad" annotation to load automatically.
         *
         * If you are using Kafka or other messaging system as a service mesh,
         * you can set the "cloud.connector" in application.properties
         * and call the "connectToCloud" method.
         */
        platform.connectToCloud();
    }

}
