/*

    Copyright 2018-2023 Accenture Technology

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

package org.platformlambda.lang;

import org.platformlambda.cloud.services.ServiceQuery;
import org.platformlambda.core.annotations.MainApplication;
import org.platformlambda.core.models.EntryPoint;
import org.platformlambda.core.system.*;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.lang.websocket.server.LanguageConnector;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

@MainApplication
public class MainApp implements EntryPoint {
    private static final Logger log = LoggerFactory.getLogger(MainApp.class);

    public static void main(String[] args) {
        AppStarter.main(args);
    }

    @Override
    public void start(String[] args) throws Exception {
        ServerPersonality.getInstance().setType(ServerPersonality.Type.RESOURCES);
        /*
         * get ready to accept language pack client connections
         */
        LanguageConnector.initialize();
        Platform platform = Platform.getInstance();
        AppConfigReader reader = AppConfigReader.getInstance();
        platform.connectToCloud();
        String connector = reader.getProperty(PostOffice.CLOUD_CONNECTOR, "none");
        if ("none".equalsIgnoreCase(connector)) {
            log.info("Running in standalone mode");
            platform.register(ServiceDiscovery.SERVICE_QUERY, new ServiceQuery(), 10);
        }
    }

}
