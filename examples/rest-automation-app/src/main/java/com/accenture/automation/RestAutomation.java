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

package com.accenture.automation;

import org.platformlambda.core.annotations.MainApplication;
import org.platformlambda.core.models.EntryPoint;
import org.platformlambda.core.system.AppStarter;
import org.platformlambda.core.system.Platform;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

@MainApplication(sequence=1)
public class RestAutomation implements EntryPoint {
    private static final Logger log = LoggerFactory.getLogger(RestAutomation.class);

    /**
     * This main class is only used when testing the app from the IDE.
     *
     * @param args - command line arguments
     */
    public static void main(String[] args) {
        AppStarter.main(args);
    }

    @Override
    public void start(String[] args) {
        /*
         * Since the startup sequence is 1, this module will start before
         * the underlying rest-automation (which is configured as sequence=2) begins.
         *
         * If you want to execute your preparation code after the rest-automation initialization,
         * you can set sequence to a larger value.
         */
        log.info("Started");
        Platform.getInstance().connectToCloud();
    }

}
