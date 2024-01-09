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

package org.platformlambda.main;

import org.platformlambda.core.annotations.MainApplication;
import org.platformlambda.core.models.EntryPoint;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

@MainApplication
public class MainApp implements EntryPoint{
    private static final Logger log = LoggerFactory.getLogger(MainApp.class);

    @Override
    public void start(String[] args) {
        /*
         * This main application is added because the Unit Test assumes
         * it is running a Spring Boot application
         */
        log.info("Started");
    }
}
