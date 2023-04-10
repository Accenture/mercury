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

package org.platformlambda.core.system;

import io.github.classgraph.ClassInfo;
import io.vertx.core.Vertx;
import io.vertx.core.http.HttpServer;
import io.vertx.core.http.HttpServerOptions;
import org.platformlambda.automation.config.RoutingEntry;
import org.platformlambda.automation.http.HttpRequestHandler;
import org.platformlambda.automation.models.AsyncContextHolder;
import org.platformlambda.automation.services.ServiceGateway;
import org.platformlambda.automation.services.ServiceResponseHandler;
import org.platformlambda.automation.util.SimpleHttpUtility;
import org.platformlambda.core.annotations.BeforeApplication;
import org.platformlambda.core.annotations.MainApplication;
import org.platformlambda.core.annotations.PreLoad;
import org.platformlambda.core.annotations.WebSocketService;
import org.platformlambda.core.models.EntryPoint;
import org.platformlambda.core.models.KotlinLambdaFunction;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.models.TypedLambdaFunction;
import org.platformlambda.core.util.*;
import org.platformlambda.core.websocket.server.MinimalistHttpHandler;
import org.platformlambda.core.websocket.server.WsRequestHandler;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.lang.reflect.InvocationTargetException;
import java.text.NumberFormat;
import java.util.*;
import java.util.concurrent.*;
import java.util.concurrent.atomic.AtomicBoolean;

public class AppStarter {
    private static final Logger log = LoggerFactory.getLogger(AppStarter.class);
    private static final ConcurrentMap<String, LambdaFunction> wsLambdas = new ConcurrentHashMap<>();
    private static final AtomicBoolean housekeeperNotRunning = new AtomicBoolean(true);
    private static final long HOUSEKEEPING_INTERVAL = 10 * 1000L;    // 10 seconds
    public static final String ASYNC_HTTP_REQUEST = "async.http.request";
    public static final String ASYNC_HTTP_RESPONSE = "async.http.response";
    private static final String SKIP_OPTIONAL = "Skipping optional {}";
    private static final String CLASS_NOT_FOUND = "Class {} not found";

    private static final int MAX_SEQ = 999;
    private static boolean loaded = false;
    private static boolean mainAppLoaded = false;
    private static boolean springBoot = false;
    private static String[] args = new String[0];

    private static AppStarter instance;

    private final long startTime = System.currentTimeMillis();

    public static void main(String[] args) {
        if (!loaded) {
            loaded = true;
            AppStarter.args = args;
            instance = new AppStarter();
            // Run "BeforeApplication" modules
            instance.doApps(args, false);
            // preload services
            preload();
            // Setup REST automation and websocket server if needed
            try {
                instance.startHttpServerIfAny();
            } catch (IOException | InterruptedException e) {
                log.error("Unable to start HTTP server", e);
            }
            // Run "MainApplication" modules
            if (!springBoot) {
                mainAppLoaded = true;
                log.info("Loading user application");
                instance.doApps(args, true);
            }
        }
    }

    public static void runAsSpringBootApp() {
        springBoot = true;
    }

    public static void runMainApp() {
        if (instance != null && !mainAppLoaded) {
            mainAppLoaded = true;
            instance.doApps(args, true);
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
                        log.info(SKIP_OPTIONAL, cls);
                    }
                } catch (ClassNotFoundException e) {
                    log.error(CLASS_NOT_FOUND, info.getName());
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

    @SuppressWarnings("rawtypes")
    private static void preload() {
        log.info("Preloading started");
        Utility util = Utility.getInstance();
        Platform platform = Platform.getInstance();
        SimpleClassScanner scanner = SimpleClassScanner.getInstance();
        Set<String> packages = scanner.getPackages(true);
        for (String p : packages) {
            List<ClassInfo> services = scanner.getAnnotatedClasses(p, PreLoad.class);
            for (ClassInfo info : services) {
                String serviceName = info.getName();
                log.info("Loading service {}", serviceName);
                try {
                    Class<?> cls = Class.forName(serviceName);
                    if (Feature.isRequired(cls)) {
                        PreLoad preload = cls.getAnnotation(PreLoad.class);
                        List<String> routes = util.split(preload.route(), ", ");
                        if (routes.isEmpty()) {
                            log.error("Unable to preload {} - missing service route(s)", serviceName);
                        } else {
                            int instances = getInstancesFromEnv(preload.envInstances(), preload.instances());
                            boolean isPrivate = preload.isPrivate();
                            Object o = cls.getDeclaredConstructor().newInstance();
                            if (o instanceof TypedLambdaFunction) {
                                for (String r : routes) {
                                    if (isPrivate) {
                                        platform.registerPrivate(r, (TypedLambdaFunction) o, instances);
                                    } else {
                                        platform.register(r, (TypedLambdaFunction) o, instances);
                                    }
                                }
                            } else if (o instanceof KotlinLambdaFunction) {
                                for (String r : routes) {
                                    if (isPrivate) {
                                        platform.registerKotlinPrivate(r, (KotlinLambdaFunction) o, instances);
                                    } else {
                                        platform.registerKotlin(r, (KotlinLambdaFunction) o, instances);
                                    }
                                }
                            } else {
                                log.error("Unable to preload {} - {} does not implement {} or {}", serviceName,
                                        o.getClass(),
                                        TypedLambdaFunction.class.getSimpleName(),
                                        KotlinLambdaFunction.class.getSimpleName());
                            }
                        }
                    } else {
                        log.info(SKIP_OPTIONAL, cls);
                    }

                } catch (ClassNotFoundException | InvocationTargetException | InstantiationException |
                         IllegalAccessException | NoSuchMethodException | IOException e) {
                    log.error("Unable to preload {} - {}", serviceName, e.getMessage());
                }
            }
        }
        log.info("Preloading completed");
    }

    private static int getInstancesFromEnv(String envInstances, int instances) {
        if (envInstances == null || envInstances.isEmpty()) {
            return Math.max(1, instances);
        } else {
            final Utility util = Utility.getInstance();
            final String env;
            if (envInstances.contains("${") && envInstances.contains("}")) {
                env = util.getEnvVariable(envInstances);
            } else {
                AppConfigReader config = AppConfigReader.getInstance();
                env = config.getProperty(envInstances);
            }
            return Math.max(1, env != null? util.str2int(env) : instances);
        }
    }

    private void startHttpServerIfAny() throws IOException, InterruptedException {
        // find and execute optional preparation modules
        final SimpleClassScanner scanner = SimpleClassScanner.getInstance();
        final Set<String> packages = scanner.getPackages(true);
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
                            loadWebsocketServices(cls, annotation.namespace(), annotation.value());
                        }
                    } else {
                        log.info(SKIP_OPTIONAL, cls);
                    }
                } catch (ClassNotFoundException e) {
                    log.error(CLASS_NOT_FOUND, info.getName());
                }
            }
        }
        // start HTTP/websocket server
        final AppConfigReader config = AppConfigReader.getInstance();
        final boolean enableRest = "true".equals(config.getProperty("rest.automation", "false"));
        if (enableRest || !wsLambdas.isEmpty()) {
            final Utility util = Utility.getInstance();
            final int port = util.str2int(config.getProperty("websocket.server.port",
                                    config.getProperty("rest.server.port",
                                    config.getProperty("server.port", "8085"))));
            if (port > 0) {
                final BlockingQueue<Boolean> serverStatus = new ArrayBlockingQueue<>(1);
                final ConcurrentMap<String, AsyncContextHolder> contexts;
                // create a dedicated vertx event loop instance for the HTTP server
                final Vertx vertx = Vertx.vertx();
                final HttpServer server = vertx.createHttpServer(new HttpServerOptions().setTcpKeepAlive(true));
                if (enableRest) {
                    // start REST automation system
                    ConfigReader restConfig = getRestConfig();
                    RoutingEntry restRouting = RoutingEntry.getInstance();
                    restRouting.load(restConfig);
                    // Start HTTP request and response handlers
                    ServiceGateway gateway = new ServiceGateway();
                    contexts = gateway.getContexts();
                    server.requestHandler(new HttpRequestHandler(gateway));
                } else {
                    // start minimalist HTTP handlers to provide actuator endpoints
                    contexts = null;
                    server.requestHandler(new MinimalistHttpHandler());
                }
                // Start websocket server if there are websocket endpoints
                if (!wsLambdas.isEmpty()) {
                    server.webSocketHandler(new WsRequestHandler(wsLambdas));
                }
                server.listen(port)
                .onSuccess(service -> {
                    serverStatus.offer(true);
                    if (contexts != null) {
                        Platform platform = Platform.getInstance();
                        try {
                            platform.registerPrivate(ASYNC_HTTP_RESPONSE,
                                                        new ServiceResponseHandler(contexts), 200);
                        } catch (IOException e) {
                            log.error("Unable to register HTTP request/response handlers  - {}", e.getMessage());
                        }
                        // start timeout handler
                        Housekeeper housekeeper = new Housekeeper(contexts);
                        platform.getVertx().setPeriodic(HOUSEKEEPING_INTERVAL,
                                                        t -> housekeeper.removeExpiredConnections());
                        log.info("AsyncHttpContext housekeeper started");
                        log.info("Reactive HTTP server running on port-{}", service.actualPort());
                    }
                    if (!wsLambdas.isEmpty()) {
                        log.info("Websocket server running on port-{}", service.actualPort());
                    }
                })
                .onFailure(ex -> {
                    log.error("Unable to start - {}", ex.getMessage());
                    System.exit(-1);
                });
                Boolean ready = serverStatus.poll(20, TimeUnit.SECONDS);
                if (Boolean.TRUE.equals(ready)) {
                    String diff = NumberFormat.getInstance().format(System.currentTimeMillis() - startTime);
                    log.info("Modules loaded in {} ms", diff);
                }
            }
        }
    }

    private void loadWebsocketServices(Class<?> cls, String namespace, String value) {
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
                wsLambdas.put(path, (LambdaFunction) o);
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

    private static class Housekeeper {
        private final ConcurrentMap<String, AsyncContextHolder> contexts;

        private Housekeeper(ConcurrentMap<String, AsyncContextHolder> contexts) {
            this.contexts = contexts;
        }

        private void removeExpiredConnections() {
            if (housekeeperNotRunning.compareAndSet(true, false)) {
                Platform.getInstance().getEventExecutor().submit(() -> {
                    try {
                        checkAsyncTimeout();
                    } finally {
                        housekeeperNotRunning.set(true);
                    }
                });
            }
        }

        private void checkAsyncTimeout() {
            // check async context timeout
            if (!contexts.isEmpty()) {
                List<String> list = new ArrayList<>(contexts.keySet());
                long now = System.currentTimeMillis();
                for (String id : list) {
                    AsyncContextHolder holder = contexts.get(id);
                    long t1 = holder.lastAccess;
                    if (now - t1 > holder.timeout) {
                        log.warn("Async HTTP Context {} timeout for {} ms", id, now - t1);
                        SimpleHttpUtility httpUtil = SimpleHttpUtility.getInstance();
                        httpUtil.sendError(id, holder.request, 408,
                                "Timeout for " + (holder.timeout / 1000) + " seconds");
                    }
                }
            }
        }
    }

}
