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

package org.platformlambda;

import org.platformlambda.core.annotations.MainApplication;
import org.platformlambda.core.models.EntryPoint;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.ServerPersonality;
import org.platformlambda.kafka.PresenceHandler;
import org.platformlambda.rest.RestServer;
import org.platformlambda.services.*;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.concurrent.TimeoutException;

@MainApplication
public class MainApp implements EntryPoint {
    private static final Logger log = LoggerFactory.getLogger(MainApp.class);

    public static final String MANAGER = "kafka.manager";
    public static final String PRESENCE_HANDLER = "presence.service";
    public static final String PRESENCE_HOUSEKEEPER = "presence.housekeeper";
    private static final String ADDITIONAL_INFO = "additional.info";
    private static final String CLOUD_CONNECTOR = PostOffice.CLOUD_CONNECTOR;

    public static void main(String[] args) {
        RestServer.main(args);
    }

    @Override
    public void start(String[] args) throws TimeoutException, IOException {
        ServerPersonality.getInstance().setType(ServerPersonality.Type.RESOURCES);

        Platform platform = Platform.getInstance();
        platform.connectToCloud();
        platform.waitForProvider(CLOUD_CONNECTOR, 20);
        platform.waitForProvider(MANAGER, 20);
        // start additional info service
        platform.registerPrivate(ADDITIONAL_INFO, new AdditionalInfo(), 3);
        // ping connected application instances periodically
        new PingScheduler().start();
        // broadcast heart beat to presence monitor peers
        new KeepAlive().start();
        // setup presence handler
        platform.registerPrivate(PRESENCE_HANDLER, new PresenceHandler(), 1);
        // setup presence housekeeper that removes expired Kafka topics
        platform.registerPrivate(PRESENCE_HOUSEKEEPER, new HouseKeeper(), 1);
        log.info("Started");
    }


}
