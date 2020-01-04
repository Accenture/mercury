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

package org.platformlambda.example;

import org.platformlambda.core.annotations.MainApplication;
import org.platformlambda.core.models.EntryPoint;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.AppStarter;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.ServerPersonality;
import org.platformlambda.services.HelloGeneric;
import org.platformlambda.services.HelloPoJo;
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
        // Start the platform
        Platform platform = Platform.getInstance();

        // You can create a microservice as a lambda function inline or write it as a regular Java class
        LambdaFunction echo = (headers, body, instance) -> {
            log.info("echo @"+instance+" received - "+headers+", "+body);
            Map<String, Object> result = new HashMap<>();
            result.put("headers", headers);
            result.put("body", body);
            result.put("instance", instance);
            result.put("origin", platform.getOrigin());
            return result;
        };

        // register your services
        platform.register("hello.world", echo, 10);
        platform.register("hello.pojo", new HelloPoJo(), 5);
        platform.register("hello.generic", new HelloGeneric(), 5);

        // connect to cloud according to "cloud.connector" in application.properties
        platform.connectToCloud();
    }

}
