/*

    Copyright 2018-2022 Accenture Technology

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

import org.platformlambda.core.annotations.MainApplication;
import org.platformlambda.core.models.EntryPoint;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.ServerPersonality;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.rest.RestServer;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.File;
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
        ServerPersonality.getInstance().setType(ServerPersonality.Type.APP);

        Platform platform = Platform.getInstance();
        // This service function is used for the demo swagger page
        LambdaFunction echo = (headers, body, instance) -> {
            Map<String, Object> result = new HashMap<>();
            result.put("headers", headers);
            result.put("body", body);
            result.put("instance", instance);
            result.put("origin", platform.getOrigin());
            return result;
        };
        // register the above echo service with some concurrent workers in this execution unit
        // Each deployment unit can be scaled horizontally by the cloud.
        platform.register("hello.world", echo, 20);

        // ensure the api-playground folder exists
        AppConfigReader config = AppConfigReader.getInstance();
        String playgroundFolder = config.getProperty("api.playground.apps", "/tmp/api-playground");
        File playground = new File(playgroundFolder);
        if (!playground.exists()) {
            playground.mkdirs();
            log.info("API Playground folder created - {}", playground);
        }
        String htmlFolder = config.getProperty("temp.html.folder", "/tmp/swagger-ui-js");
        File htmlDir = new File(htmlFolder);
        if (!htmlDir.exists()) {
            htmlDir.mkdirs();
            log.info("API Playground temp HTML folder created - {}", htmlDir);
        }
        log.info("Application started");
    }

}
