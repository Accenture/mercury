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

import io.vertx.core.Vertx;
import io.vertx.core.http.HttpServerOptions;
import org.platformlambda.automation.config.RoutingEntry;
import org.platformlambda.automation.config.WsEntry;
import org.platformlambda.automation.http.HttpRelay;
import org.platformlambda.automation.http.HttpRequestHandler;
import org.platformlambda.automation.models.AsyncContextHolder;
import org.platformlambda.automation.services.*;
import org.platformlambda.automation.util.AsyncTimeoutHandler;
import org.platformlambda.automation.ws.WsRequestHandler;
import org.platformlambda.automation.ws.WsTokenIssuer;
import org.platformlambda.core.annotations.MainApplication;
import org.platformlambda.core.models.EntryPoint;
import org.platformlambda.core.system.AppStarter;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.ServerPersonality;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.ConfigReader;
import org.platformlambda.core.util.Utility;
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
        AppStarter.main(args);
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
        AppConfigReader appConfig = AppConfigReader.getInstance();
        Utility util = Utility.getInstance();
        try {
            ConfigReader config = getConfig();
            RoutingEntry restRouting = RoutingEntry.getInstance();
            restRouting.load(config);
            WsEntry wsRouting = WsEntry.getInstance();
            wsRouting.load(config);
            // start service response handler
            ServiceGateway gateway = new ServiceGateway();
            ConcurrentMap<String, AsyncContextHolder> contexts = gateway.getContexts();
            // "async.http.request" is deployed as PUBLIC to provide "HttpClient as a service"
            platform.register(ASYNC_HTTP_REQUEST, new HttpRelay(), 300);
            // "async.http.response" must be PRIVATE because the AsyncContext objects are kept in local memory
            platform.registerPrivate(ASYNC_HTTP_RESPONSE, new ServiceResponseHandler(contexts), 300);
            // fall back to "server.port" if "rest.server.port" is not configured
            int port = util.str2int(appConfig.getProperty("rest.server.port",
                                    appConfig.getProperty("server.port", "8100")));

            Vertx vertx = Vertx.vertx();
            HttpServerOptions options = new HttpServerOptions().setTcpKeepAlive(true);
            vertx.createHttpServer(options)
                    .webSocketHandler(new WsRequestHandler())
                    .requestHandler(new HttpRequestHandler(gateway))
                    .listen(port)
                    .onSuccess(server -> log.info("REST automation running on port-{}", server.actualPort()))
                    .onFailure(ex -> {
                        log.error("Unable to start - {}", ex.getMessage());
                        System.exit(-1);
                    });

            AsyncTimeoutHandler timeoutHandler = new AsyncTimeoutHandler(contexts);
            timeoutHandler.start();
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
