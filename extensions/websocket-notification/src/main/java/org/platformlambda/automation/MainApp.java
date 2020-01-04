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

package org.platformlambda.automation;

import org.platformlambda.automation.init.InitialLoad;
import org.platformlambda.automation.ws.WsAuthentication;
import org.platformlambda.automation.ws.WsNotification;
import org.platformlambda.core.annotations.MainApplication;
import org.platformlambda.core.models.EntryPoint;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.rest.RestServer;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;

@MainApplication
public class MainApp implements EntryPoint {
    private static final Logger log = LoggerFactory.getLogger(MainApp.class);

    public static final String INITIAL_LOAD = "initial.load";
    public static final String WS_AUTHENTICATION_SERVICE = "sample.ws.auth";
    public static final String WS_NOTIFICATION_SERVICE = "ws.notification";

    public static void main(String[] args) {
        RestServer.main(args);
    }

    @Override
    public void start(String[] args) throws IOException {
        /*
         * Starting the demo authenticator and notification service
         */
        Platform platform = Platform.getInstance();
        platform.register(WS_NOTIFICATION_SERVICE, new WsNotification(), 10);
        platform.register(WS_AUTHENTICATION_SERVICE, new WsAuthentication(), 5);
        platform.connectToCloud();

        // check if there are peers to recover
        PostOffice po = PostOffice.getInstance();
        platform.registerPrivate(INITIAL_LOAD, new InitialLoad(), 1);
        po.send(INITIAL_LOAD, "start");

        log.info("Application started");
    }

}
