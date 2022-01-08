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

package org.platformlambda.rest;

import org.platformlambda.core.system.AppStarter;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.boot.web.servlet.ServletComponentScan;
import org.springframework.boot.web.servlet.support.SpringBootServletInitializer;
import org.springframework.context.annotation.ComponentScan;

@ServletComponentScan({"org.platformlambda"})
@ComponentScan({"org.platformlambda", "${web.component.scan}"})
@SpringBootApplication
public class RestServer extends SpringBootServletInitializer {
    /**
     * This is started by Spring Boot when the WAR file is deployed
     * @param args may not be relevant when deployed as WAR
     */
    public static void main(String[] args) {
        /*
         * execute preparation steps if any
         * - this allows application to do preparation such as setting environment variables,
         *   overriding application.properties, etc.
         */
        AppStarter.setWebApp(true);
        AppStarter.main(args);
        // start Spring Boot
        SpringApplication.run(RestServer.class, args);
    }

}
