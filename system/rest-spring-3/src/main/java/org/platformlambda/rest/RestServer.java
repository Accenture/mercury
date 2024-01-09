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

    public static void main(String[] args) {
        // Declare as a Spring Boot application so that AppStarter will defer loading main applications
        AppStarter.runAsSpringBootApp();
        // Execute BeforeApplication(s)
        AppStarter.main(args);
        // Spring Boot will invoke the MainAppLoader to load MainApplication(s)
        SpringApplication.run(RestServer.class, args);
    }

}
