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

package org.platformlambda.automation;

import org.platformlambda.automation.config.RoutingEntry;
import org.platformlambda.automation.config.WsEntry;
import org.platformlambda.automation.models.AsyncContextHolder;
import org.platformlambda.automation.services.NotificationManager;
import org.platformlambda.automation.services.NotificationQuery;
import org.platformlambda.automation.services.ServiceResponseHandler;
import org.platformlambda.automation.services.WsTokenIssuer;
import org.platformlambda.automation.servlets.HttpRelay;
import org.platformlambda.automation.servlets.ServiceGateway;
import org.platformlambda.automation.util.AsyncTimeoutHandler;
import org.platformlambda.core.annotations.MainApplication;
import org.platformlambda.core.models.EntryPoint;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.ServerPersonality;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.ConfigReader;
import org.platformlambda.core.util.Utility;
import org.platformlambda.rest.RestServer;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.List;
import java.util.concurrent.ConcurrentMap;

@MainApplication(sequence=2)
public class MainModule implements EntryPoint {
    private static final Logger log = LoggerFactory.getLogger(MainModule.class);

    public static final String ASYNC_HTTP_REQUEST = "async.http.request";
    public static final String ASYNC_HTTP_RESPONSE = "async.http.response";
    public static final String NOTIFICATION_MANAGER = "notification.manager";
    private static final String WS_TOKEN_ISSUER = "ws.token.issuer";
    private static final String NOTIFICATION_QUERY = "ws.notification";

    public static void main(String[] args) {
        RestServer.main(args);
    }

    /**
     * Starting point for rest-automation
     *
     * Note that this module will not start under IDE because
     * mercury dependencies are scoped as "provided" in the pom.xml
     *
     * However, you can always test it with a simple unit test by executing RestServer.main(new String[0]);
     *
     * @param args command line arguments if any
     */
    @Override
    public void start(String[] args) {
        /*
         * ServiceGateway will start first to load routing entries from rest.yaml
         * and start async.http.response service.
         *
         * The main app can then connect to the cloud.
         */
        ServerPersonality.getInstance().setType(ServerPersonality.Type.REST);
        Platform platform = Platform.getInstance();
        try {
            ConfigReader config = getConfig();
            RoutingEntry routing = RoutingEntry.getInstance();
            routing.load(config);
            WsEntry ws = WsEntry.getInstance();
            ws.load(config);
            // start service response handler
            ConcurrentMap<String, AsyncContextHolder> contexts = ServiceGateway.getContexts();
            // "async.http.request" is deployed as PUBLIC to provide "HttpClient as a service"
            platform.register(ASYNC_HTTP_REQUEST, new HttpRelay(), 300);
            // "async.http.response" must be PRIVATE because the AsyncContext objects are kept in local memory
            platform.registerPrivate(ASYNC_HTTP_RESPONSE, new ServiceResponseHandler(contexts), 300);
            /*
             * When AsyncContext timeout, the HttpServletResponse object is already closed.
             * Therefore, we use a custom timeout handler so we can control the timeout experience.
             */
            AsyncTimeoutHandler timeoutHandler = new AsyncTimeoutHandler(contexts);
            timeoutHandler.start();
            // ready to serve
            ServiceGateway.setReady();
            // start a generic notification service
            platform.register(NOTIFICATION_MANAGER, new NotificationManager(), 10);
            platform.registerPrivate(WS_TOKEN_ISSUER, new WsTokenIssuer(), 10);
            platform.registerPrivate(NOTIFICATION_QUERY, new NotificationQuery(), 10);

        } catch (Exception e) {
            log.error("Unable to start", e);
            System.exit(-1);
        }
        // connect to the event streams
        platform.connectToCloud();
        log.info("Application started");
    }

    private ConfigReader getConfig() throws IOException {
        AppConfigReader reader = AppConfigReader.getInstance();
        List<String> paths = Utility.getInstance().split(reader.getProperty("rest.automation.yaml",
                "file:/tmp/config/rest.yaml, classpath:/rest.yaml"), ", ");
        for (String p: paths) {
            ConfigReader config = new ConfigReader();
            try {
                config.load(p);
                log.info("Loading config from {}", p);
                return config;
            } catch (IOException e) {
                log.warn("Skipping {} - {}", p, e.getMessage());
            }
        }
        throw new IOException("Endpoint configuration not found in "+paths);
    }

}
