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

package org.platformlambda.core.system;

import io.github.classgraph.ClassInfo;
import io.vertx.core.Vertx;
import io.vertx.core.http.HttpServer;
import io.vertx.core.http.HttpServerOptions;
import org.platformlambda.automation.config.RoutingEntry;
import org.platformlambda.automation.http.HttpRelay;
import org.platformlambda.automation.http.HttpRequestHandler;
import org.platformlambda.automation.models.AsyncContextHolder;
import org.platformlambda.automation.services.ServiceGateway;
import org.platformlambda.automation.services.ServiceResponseHandler;
import org.platformlambda.automation.util.AsyncTimeoutHandler;
import org.platformlambda.core.annotations.BeforeApplication;
import org.platformlambda.core.annotations.MainApplication;
import org.platformlambda.core.annotations.WebSocketService;
import org.platformlambda.core.models.EntryPoint;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.util.*;
import org.platformlambda.core.websocket.server.MinimalistHttpHandler;
import org.platformlambda.core.websocket.server.WsRequestHandler;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.lang.reflect.InvocationTargetException;
import java.util.*;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

public class AppStarter {
    private static final Logger log = LoggerFactory.getLogger(AppStarter.class);
    private static final ConcurrentMap<String, LambdaFunction> lambdas = new ConcurrentHashMap<>();

    public static final String ASYNC_HTTP_REQUEST = "async.http.request";
    public static final String ASYNC_HTTP_RESPONSE = "async.http.response";

    private static final int MAX_SEQ = 999;
    private static boolean loaded = false;
    private static String[] args = new String[0];

    public static void main(String[] args) {
        if (!loaded) {
            loaded = true;
            AppStarter.args = args;
            AppStarter begin = new AppStarter();
            // Run "BeforeApplication" modules
            begin.doApps(args, false);
            /*
             * Initialize event system before loading "MainApplication" modules.
             * This ensures all "preload" services are loaded.
             */
            PostOffice po = PostOffice.getInstance();
            log.info("Starting application instance {}", po.getAppInstanceId());
            // Run "MainApplication" modules
            begin.doApps(args, true);
            // Setup websocket server if required
            try {
                begin.startHttpServerIfAny();
            } catch (IOException e) {
                log.error("Unable to start HTTP server", e);
            }
        }
    }

    public static String[] getArgs() {
        return args;
    }

    private void doApps(String[] args, boolean main) {
        // find and execute optional preparation modules
        Utility util = Utility.getInstance();
        SimpleClassScanner scanner = SimpleClassScanner.getInstance();
        Set<String> packages = scanner.getPackages(true);
        int n = 0;
        Map<String, Class<?>> steps = new HashMap<>();
        for (String p : packages) {
            List<ClassInfo> services = scanner.getAnnotatedClasses(p, main?
                                        MainApplication.class : BeforeApplication.class);
            for (ClassInfo info : services) {
                try {
                    Class<?> cls = Class.forName(info.getName());
                    if (Feature.isRequired(cls)) {
                        int seq = getSequence(cls, main);
                        String key = util.zeroFill(seq, MAX_SEQ) + "." + util.zeroFill(++n, MAX_SEQ);
                        steps.put(key, cls);
                    } else {
                        log.info("Skipping optional {}", cls);
                    }
                } catch (ClassNotFoundException e) {
                    log.error("Class {} not found", info.getName());
                }
            }
        }
        executeOrderly(steps, args, main);
    }

    private int getSequence(Class<?> cls, boolean main) {
        if (main) {
            MainApplication mainApp = cls.getAnnotation(MainApplication.class);
            return Math.min(MAX_SEQ, mainApp.sequence());
        } else {
            BeforeApplication beforeApp = cls.getAnnotation(BeforeApplication.class);
            return Math.min(MAX_SEQ, beforeApp.sequence());
        }
    }

    private void executeOrderly(Map<String, Class<?>> steps, String[] args, boolean main) {
        List<String> list = new ArrayList<>(steps.keySet());
        if (list.size() > 1) {
            Collections.sort(list);
        }
        int n = 0;
        int error = 0;
        for (String seq : list) {
            Class<?> cls = steps.get(seq);
            try {
                Object o = cls.getDeclaredConstructor().newInstance();
                if (o instanceof EntryPoint) {
                    EntryPoint app = (EntryPoint) o;
                    log.info("Starting {}", app.getClass().getName());
                    app.start(args);
                    n++;
                } else {
                    error++;
                    log.error("Unable to start {} because it is not an instance of {}",
                            cls.getName(), EntryPoint.class.getName());
                }
            } catch (Exception e) {
                error++;
                log.error("Unable to start - " + cls.getName(), e);
            }
        }
        if (main && error == 0 && n == 0) {
            log.error("Missing MainApplication\n\n{}\n{}\n\n",
                    "Did you forget to annotate your main module with @MainApplication that implements EntryPoint?",
                    "and ensure the package parent is defined in 'web.component.scan' of application.properties.");
        }
    }

    private void startHttpServerIfAny() throws IOException {
        // find and execute optional preparation modules
        SimpleClassScanner scanner = SimpleClassScanner.getInstance();
        Set<String> packages = scanner.getPackages(true);
        for (String p : packages) {
            List<ClassInfo> services = scanner.getAnnotatedClasses(p, WebSocketService.class);
            for (ClassInfo info : services) {
                try {
                    Class<?> cls = Class.forName(info.getName());
                    if (Feature.isRequired(cls)) {
                        WebSocketService annotation = cls.getAnnotation(WebSocketService.class);
                        if (annotation.value().length() > 0) {
                            if (!Utility.getInstance().validServiceName(annotation.value())) {
                                log.error("Unable to load {} ({}) because the path is not a valid service name",
                                        cls.getName(), annotation.value());
                            }
                            loadLambda(cls, annotation.namespace(), annotation.value());
                        }
                    } else {
                        log.info("Skipping optional {}", cls);
                    }
                } catch (ClassNotFoundException e) {
                    log.error("Class {} not found", info.getName());
                }
            }
        }
        // start HTTP/websocket server
        AppConfigReader config = AppConfigReader.getInstance();
        boolean enableRest = "true".equals(config.getProperty("rest.automation", "false"));
        if (enableRest || !lambdas.isEmpty()) {
            Utility util = Utility.getInstance();
            int port = util.str2int(config.getProperty("rest.server.port",
                                    config.getProperty("websocket.server.port",
                                    config.getProperty("server.port", "8085"))));
            if (port > 0) {
                final ConcurrentMap<String, AsyncContextHolder> contexts;
                Vertx vertx = Vertx.vertx();
                HttpServerOptions options = new HttpServerOptions().setTcpKeepAlive(true);
                HttpServer server = vertx.createHttpServer(options);
                if (enableRest) {
                    ConfigReader restConfig = getRestConfig();
                    RoutingEntry restRouting = RoutingEntry.getInstance();
                    restRouting.load(restConfig);
                    // Start HTTP request and response handlers
                    ServiceGateway gateway = new ServiceGateway();
                    contexts = gateway.getContexts();
                    server.requestHandler(new HttpRequestHandler(gateway));
                    // Start websocket server if there are websocket endpoints
                    if (!lambdas.isEmpty()) {
                        server.webSocketHandler(new WsRequestHandler(lambdas));
                    }
                } else {
                    /*
                     * REST automation not configured
                     * 1. start minimalist HTTP handlers to provide actuator endpoints
                     * 2. start websocket server
                     */
                    contexts = null;
                    server.requestHandler(new MinimalistHttpHandler());
                    server.webSocketHandler(new WsRequestHandler(lambdas));
                }
                server.listen(port)
                .onSuccess(service -> {
                    if (contexts != null) {
                        try {
                            Platform platform = Platform.getInstance();
                            platform.registerPrivate(ASYNC_HTTP_REQUEST, new HttpRelay(), 300);
                            platform.registerPrivate(ASYNC_HTTP_RESPONSE,
                                                        new ServiceResponseHandler(contexts), 300);
                        } catch (IOException e) {
                            log.error("Unable to register HTTP request/response handlers  - {}", e.getMessage());
                        }
                        // start timeout handler
                        AsyncTimeoutHandler timeoutHandler = new AsyncTimeoutHandler(contexts);
                        timeoutHandler.start();
                        log.info("Reactive HTTP server running on port-{}", service.actualPort());
                    }
                    if (!lambdas.isEmpty()) {
                        log.info("Websocket server running on port-{}", service.actualPort());
                    }
                })
                .onFailure(ex -> {
                    log.error("Unable to start - {}", ex.getMessage());
                    System.exit(-1);
                });
            }
        }
    }

    private void loadLambda(Class<?> cls, String namespace, String value) {
        Utility util = Utility.getInstance();
        List<String> parts = util.split(namespace + "/" + value, "/");
        StringBuilder sb = new StringBuilder();
        for (String p: parts) {
            sb.append('/');
            sb.append(p);
        }
        String path = sb.toString();
        String wsEndpoint = path + "/{handle}";
        try {
            Object o = cls.getDeclaredConstructor().newInstance();
            if (o instanceof LambdaFunction) {
                lambdas.put(path, (LambdaFunction) o);
                log.info("{} loaded as WEBSOCKET SERVER endpoint {}", cls.getName(), wsEndpoint);
            } else {
                log.error("Unable to load {} ({}) because it is not an instance of {}",
                        cls.getName(), wsEndpoint, LambdaFunction.class.getName());
            }

        } catch (InstantiationException | IllegalAccessException | NoSuchMethodException |
                 InvocationTargetException e) {
            log.error("Unable to load {} ({}) - {}", cls.getName(), wsEndpoint, e.getMessage());
        }
    }

    private ConfigReader getRestConfig() throws IOException {
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
