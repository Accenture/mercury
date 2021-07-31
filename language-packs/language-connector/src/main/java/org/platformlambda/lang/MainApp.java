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

package org.platformlambda.lang;

import org.platformlambda.core.annotations.MainApplication;
import org.platformlambda.core.models.EntryPoint;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.ServerPersonality;
import org.platformlambda.lang.websocket.server.LanguageConnector;
import org.platformlambda.rest.RestServer;

@MainApplication
public class MainApp implements EntryPoint {

    public static void main(String[] args) {
        RestServer.main(args);
    }

    @Override
    public void start(String[] args) throws Exception {
        ServerPersonality.getInstance().setType(ServerPersonality.Type.RESOURCES);
        /*
         * get ready to accept language pack client connections
         */
        LanguageConnector.initialize();
        Platform.getInstance().connectToCloud();
    }

}
