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

package org.platformlambda.rest;

import org.platformlambda.core.system.AppStarter;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.boot.web.servlet.ServletComponentScan;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.ComponentScan;

import javax.servlet.ServletContextEvent;
import javax.servlet.ServletContextListener;

@ServletComponentScan({"org.platformlambda"})
@ComponentScan({"org.platformlambda", "${web.component.scan}"})
@SpringBootApplication
public class RestServer {
    private static final Logger log = LoggerFactory.getLogger(RestServer.class);

    private static String[] args;

    public static void main(String[] args) {
        // save command line arguments
        RestServer.args = args;
        /*
         * execute preparation steps if any
         * - this allows application to do preparation such as setting environment variables,
         *   overriding application.properties, etc.
         */
        AppStarter.prepare(args);
        // start Spring Boot
        SpringApplication.run(RestServer.class, args);
    }

    @Bean
    protected ServletContextListener startMainApps() {
        return new ServletContextListener() {

            @Override
            public void contextInitialized(ServletContextEvent sce) {
                AppStarter.main(args);
            }

            @Override
            public void contextDestroyed(ServletContextEvent sce) {
                log.info("Stopped");
            }

        };
    }

}
