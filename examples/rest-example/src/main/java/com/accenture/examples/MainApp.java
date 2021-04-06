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

package com.accenture.examples;

import com.accenture.examples.services.DemoMath;
import org.platformlambda.core.annotations.MainApplication;
import org.platformlambda.core.models.EntryPoint;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.ServerPersonality;
import org.platformlambda.rest.RestServer;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.HashMap;
import java.util.Map;

@MainApplication
public class MainApp implements EntryPoint {
    private static final Logger log = LoggerFactory.getLogger(MainApp.class);

    public static void main(String[] args) {
        RestServer.main(args);
    }

    @Override
    public void start(String[] args) throws Exception {

        // Set personality to WEB - this must be done in the beginning
        ServerPersonality.getInstance().setType(ServerPersonality.Type.WEB);

        Platform platform = Platform.getInstance();
        // you can write simple service using anonymous function
        LambdaFunction echo = (headers, body, instance) -> {
            /*
             * Uncomment the "log.info" statement if you want to see this service receiving the event.
             * (Note that logging takes time so it will affect your function execution time.)
             */
//             log.info("echo @"+instance+" received - "+headers+", "+body);

            // your response object can be a Java primitive, hashmap or PoJo. No need to use JSON internally.
            Map<String, Object> result = new HashMap<>();
            result.put("headers", headers);
            result.put("body", body);
            result.put("instance", String.valueOf(instance));
            result.put("origin", platform.getOrigin());
            return result;
        };
        // register the above echo service with some concurrent workers in this execution unit
        // Each deployment unit can be scaled horizontally by the cloud.
        platform.register("hello.world", echo, 20);
        // Suppose DemoMath is more complex so we write it as a Java class implementing the LambdaFunction interface.
        platform.register("math.addition", new DemoMath(), 5);
        /*
         * In distributed mode, cloud services are started automatically when you connect to cloud.
         * In standalone mode, you can start cloud services programmatically.
         */
         platform.startCloudServices();
        // connect to the network event streams so it can automatically discover other services
        platform.connectToCloud();

        log.info("Application started");
    }

}
