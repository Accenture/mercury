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
import io.vertx.core.Future;
import io.vertx.core.Promise;
import io.vertx.core.Vertx;
import io.vertx.core.eventbus.EventBus;
import org.platformlambda.core.annotations.CloudConnector;
import org.platformlambda.core.annotations.CloudService;
import org.platformlambda.core.models.*;
import org.platformlambda.core.util.*;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.lang.reflect.InvocationTargetException;
import java.util.ArrayList;
import java.util.Collections;
import java.util.Date;
import java.util.List;
import java.util.concurrent.*;
import java.util.concurrent.atomic.AtomicInteger;

public class Platform {
    private static final Logger log = LoggerFactory.getLogger(Platform.class);
    private static final CryptoApi crypto = new CryptoApi();
    private static final ConcurrentMap<String, ServiceDef> registry = new ConcurrentHashMap<>();
    private static final String PERSONALITY = "personality";
    private static final String SPRING_APPNAME = "spring.application.name";
    private static final String APPNAME = "application.name";
    private static final String DEFAULT_APPNAME = "application";
    private static final String CONNECTOR = "connector";
    private static final String SERVICE = "service";
    private static final String ROUTE = "Route ";
    private static final String NOT_FOUND = " not found";
    private static final String INVALID_ROUTE = "Invalid route ";
    private static final String RELOADING = "Reloading";
    private static String originId;
    private static boolean cloudSelected = false;
    private static boolean cloudServicesStarted = false;
    private static String appId;
    private static Vertx vertx;
    private static EventBus system;
    private static ExecutorService executor;
    private static SimpleCache cache;
    private final long startTime = System.currentTimeMillis();
    private static final AtomicInteger initCounter = new AtomicInteger(0);
    private static final Platform INSTANCE = new Platform();
    private String applicationName = null;

    private Platform() {
        // singleton
    }

    public static Platform getInstance() {
        initialize();
        return INSTANCE;
    }

    private static void initialize() {
        if (initCounter.incrementAndGet() == 1) {
            AppConfigReader config = AppConfigReader.getInstance();
            int poolSize = Math.max(32, Utility.getInstance().str2int(config.getProperty("event.worker.pool", "100")));
            system = Vertx.vertx().eventBus();
            vertx = Vertx.vertx();
            cache = SimpleCache.createCache("system.log.cache", 30000);
            executor = Executors.newWorkStealingPool(poolSize);
            log.info("Event system started with up to {} kernel threads", poolSize);
        }
        if (initCounter.get() > 10000) {
            initCounter.set(10);
        }
    }

    /**
     * IMPORTANT: If this OPTIONAL value is set, the origin ID will be derived from this value.
     * <p>
     * You MUST use unique ID for each application instance otherwise service routing would fail.
     * <p>
     * Application allows us to associate user specific information with the ID.
     * When appId is set, origin ID will derive its value from appId.
     * <p>
     *     If you want to set appId, it must be done before the "getOrigin" method is called.
     *     i.e. do it before platform starts. The best place for that is the "BeforeApplication" class.
     * <p>
     * For examples:
     * For production, you may use unique ID like Kubernetes pod-ID
     * For development in a laptop, you may use applicationName + timestamp + user.name
     * <p>
     * This method is static so that it can be set using BeforeApplication module
     * before the app starts.
     * <p>
     * @param id unique application name and instance identifier
     */
    public static void setAppId(String id) {
        if (Platform.appId == null) {
            Platform.appId = id;
            // reset originId
            Platform.originId = null;
            log.info("application instance ID set to {}", Platform.appId);
        } else {
            throw new IllegalArgumentException("application instance ID is already set");
        }
    }

    public String getAppId() {
        return Platform.appId;
    }

    /**
     * Internal API - This vertx instance must be used exclusively by the platform-core
     * <p>
     * This is used for running kotlin co-routines in the event loop
     * and spinning up worker threads for blocking code on demand.
     * <p>
     * Please do not use it at user application level to avoid blocking the event loop.
     * <p>
     * @return vertx engine
     */
    public Vertx getVertx() {
        return vertx;
    }

    /**
     * Internal API - The vertx event bus instance must be used exclusively by the platform-core
     * <p>
     * This is used for event delivery for the PostOffice
     * <p>
     * Please do not use it at user application level to avoid blocking the event loop.
     * <p>
     * @return memory event bus
     */
    public EventBus getEventSystem() {
        return system;
    }

    /**
     * Internal API - This method returns a lambda function executor for running worker in a kernel thread
     *
     * @return executor
     */
    public ExecutorService getEventExecutor() {
        return executor;
    }

    /**
     * This method returns application name
     * <p>
     *     Note: please set the same application name in pom.xml and application.properties
     *
     * @return app name
     */
    public String getName() {
        if (applicationName == null) {
            AppConfigReader config = AppConfigReader.getInstance();
            applicationName = config.getProperty(APPNAME, config.getProperty(SPRING_APPNAME, DEFAULT_APPNAME));
        }
        return applicationName;
    }

    /**
     * Origin ID is the unique identifier for an application instance.
     * <p>
     *     If you call the setAppId(name) method before the event system starts,
     *     origin ID will be derived from the given appId. Therefore, please use
     *     unique appId when running more than one application modules in a network.
     *
     * @return unique origin ID
     */
    public String getOrigin() {
        if (originId == null) {
            Utility util = Utility.getInstance();
            String id = util.getUuid();
            if (Platform.appId != null) {
                byte[] hash = crypto.getSHA256(util.getUTF(Platform.appId));
                id = util.bytes2hex(hash).substring(0, id.length());
            }
            originId = util.getDateOnly(new Date()) + id;
        }
        return originId;
    }

    public long getStartTime() {
        return startTime;
    }

    /**
     * Cloud services will be started automatically when your app call the connectToCloud() method
     * <p>
     *     Call this function only when you want to start cloud services without an event stream connector.
     */
    public synchronized void startCloudServices() {
        if (!Platform.cloudServicesStarted) {
            // guarantee to execute once
            Platform.cloudServicesStarted = true;
            AppConfigReader reader = AppConfigReader.getInstance();
            String cloudServices = reader.getProperty(EventEmitter.CLOUD_SERVICES);
            if (cloudServices != null) {
                List<String> list = Utility.getInstance().split(cloudServices, ", ");
                if (!list.isEmpty()) {
                    List<String> loaded = new ArrayList<>();
                    SimpleClassScanner scanner = SimpleClassScanner.getInstance();
                    List<ClassInfo> services = scanner.getAnnotatedClasses(CloudService.class, true);
                    for (String name: list) {
                        if (loaded.contains(name)) {
                            log.error("Cloud service ({}) already loaded", name);
                        } else {
                            if (startService(name, services, false)) {
                                loaded.add(name);
                            } else {
                                log.error("Cloud service ({}) not found", name);
                            }
                        }
                    }
                    if (loaded.isEmpty()) {
                        log.warn("No Cloud services are loaded");
                    } else {
                        log.info("Cloud services {} started", loaded);
                    }
                }
            }
        }
    }

    public static boolean isCloudSelected() {
        return Platform.cloudSelected;
    }

    /**
     * This will connect the app instance to a network event stream system
     * based on the "cloud.connector" parameter in the application.properties
     */
    public synchronized void connectToCloud() {
        if (!Platform.cloudSelected) {
            // guarantee to execute once
            Platform.cloudSelected = true;
            AppConfigReader reader = AppConfigReader.getInstance();
            String name = reader.getProperty(EventEmitter.CLOUD_CONNECTOR, "none");
            if ("none".equalsIgnoreCase(name)) {
                // there are no cloud connector. Check if there are cloud services.
                startCloudServices();
            } else {
                SimpleClassScanner scanner = SimpleClassScanner.getInstance();
                List<ClassInfo> services = scanner.getAnnotatedClasses(CloudConnector.class, true);
                if (!startService(name, services, true)) {
                    log.error("Cloud connector ({}) not found", name);
                }
            }
        }
    }

    private boolean startService(String name, List<ClassInfo> services, boolean isConnector) {
        if (name == null) {
            return false;
        }
        final String type = isConnector? CONNECTOR : SERVICE;
        for (ClassInfo info : services) {
            final Class<?> cls;
            try {
                cls = Class.forName(info.getName());
            } catch (ClassNotFoundException e) {
                log.error("Unable to start cloud {} ({}) - {}", type, info.getName(), e.getMessage());
                return false;
            }
            final String serviceName;
            final String original;
            if (isConnector) {
                CloudConnector connector = cls.getAnnotation(CloudConnector.class);
                serviceName = connector.name();
                original = connector.original();
            } else {
                CloudService connector = cls.getAnnotation(CloudService.class);
                serviceName = connector.name();
                original = connector.original();
            }
            final String nextService = original.equals(serviceName)? "" : original;
            if (name.equals(serviceName)) {
                try {
                    Object o = cls.getDeclaredConstructor().newInstance();
                    if (o instanceof CloudSetup) {
                        CloudSetup cloud = (CloudSetup) o;
                        Platform.getInstance().getEventExecutor().submit(() -> {
                            log.info("Starting cloud {} {} using {}", type, name, cls.getName());
                            cloud.initialize();
                            // execute next service if provided
                            if (!nextService.isEmpty()) {
                                startService(nextService, services, isConnector);
                            }
                        });
                        return true;
                    } else {
                        log.error("Unable to start cloud {} ({}) because it does not inherit {}",
                                type, cls.getName(), CloudSetup.class.getName());
                    }

                } catch (NoSuchMethodException | InvocationTargetException |
                        InstantiationException | IllegalAccessException e) {
                    log.error("Unable to start cloud {} ({}) - {}", type, info.getName(), e.getMessage());
                }
                break;
            }
        }
        return false;
    }

    /**
     * Internal API that returns local routing table
     *
     * @return routing table
     */
    public ConcurrentMap<String, ServiceDef> getLocalRoutingTable() {
        return registry;
    }

    /**
     * Register a public lambda function with one or more concurrent instances.
     * Its routing path will be published to the global service registry.
     *
     * @param route path
     * @param lambda function must be written in Java that implements the TypedLambdaFunction interface
     * @param instances for concurrent processing of events
     * @throws IOException in case of duplicated registration
     */
    @SuppressWarnings("rawtypes")
    public void register(String route, TypedLambdaFunction lambda, int instances) throws IOException {
        register(route, lambda, false, instances);
    }

    /**
     * Register a private lambda function with one or more concurrent instances.
     * Private function is only visible within a single execution unit.
     * Its routing path will not be published to the global service registry.
     *
     * @param route path
     * @param lambda function must be written in Java that implements the TypedLambdaFunction interface
     * @param instances for concurrent processing of events
     * @throws IOException in case of duplicated registration
     */
    @SuppressWarnings("rawtypes")
    public void registerPrivate(String route, TypedLambdaFunction lambda, int instances) throws IOException {
        register(route, lambda, true, instances);
    }

    /**
     * Register a public non-blocking lambda function with one or more concurrent instances.
     * Its routing path will be published to the global service registry.
     *
     * @param route path
     * @param lambda function must be written in Kotlin that implements the KotlinLambdaFunction interface
     * @param instances for concurrent processing of events
     * @throws IOException in case of duplicated registration
     */
    @SuppressWarnings("rawtypes")
    public void registerKotlin(String route, KotlinLambdaFunction lambda, int instances) throws IOException {
        registerAsync(route, lambda, false, instances);
    }

    /**
     * Register a private non-blocking lambda function with one or more concurrent instances.
     * Private function is only visible within a single execution unit.
     * Its routing path will not be published to the global service registry.
     *
     * @param route path
     * @param lambda function must be written in Kotlin that implements the KotlinLambdaFunction interface
     * @param instances for concurrent processing of events
     * @throws IOException in case of validation errors
     */
    @SuppressWarnings("rawtypes")
    public void registerKotlinPrivate(String route, KotlinLambdaFunction lambda, int instances) throws IOException {
        registerAsync(route, lambda, true, instances);
    }

    /**
     * Convert a private function into public
     *
     * @param route name of a service
     * @throws IllegalArgumentException if the route is not found
     * @throws IOException in case of routing error
     */
    public void makePublic(String route) throws IOException {
        if (!hasRoute(route)) {
            throw new IllegalArgumentException(ROUTE+route+NOT_FOUND);
        }
        ServiceDef service = registry.get(route);
        if (service == null) {
            throw new IllegalArgumentException(ROUTE+route+NOT_FOUND);
        }
        if (service.isPrivate()) {
            // set it to public
            service.setPrivate(false);
            log.info("Converted {} to PUBLIC", route);
            advertiseRoute(route);
        }
    }

    /**
     * Check the route registered in this application instance
     *
     * @param route name of a function
     * @return true if it is a private function
     */
    public boolean isPrivate(String route) {
        if (!hasRoute(route)) {
            throw new IllegalArgumentException(ROUTE+route+NOT_FOUND);
        }
        ServiceDef service = registry.get(route);
        if (service == null) {
            throw new IllegalArgumentException(ROUTE+route+NOT_FOUND);
        }
        return service.isPrivate();
    }

    /**
     * Check if the route is trackable
     *
     * @param route name of a function
     * @return true or false
     */
    public boolean isTrackable(String route) {
        ServiceDef service = registry.get(route);
        return service != null && service.isTrackable();
    }

    /**
     * Check if the route is an interceptor
     *
     * @param route name of a function
     * @return true or false
     */
    public boolean isInterceptor(String route) {
        ServiceDef service = registry.get(route);
        return service != null && service.isInterceptor();
    }

    /**
     * Check if the route is a coroutine
     *
     * @param route name of a function
     * @return true or false
     */
    public boolean isCoroutine(String route) {
        ServiceDef service = registry.get(route);
        return service != null && service.isCoroutine();
    }

    /**
     * Check if the route is a suspend function
     *
     * @param route name of a function
     * @return true or false
     */
    public boolean isSuspendFunction(String route) {
        ServiceDef service = registry.get(route);
        return service != null && service.isKotlin();
    }

    /**
     * Register a lambda function written in Java function that implements TypedLambdaFunction or LambdaFunction
     *
     * @param route name of the service
     * @param lambda function
     * @param isPrivate if true, it indicates the function is not visible outside this app instance
     * @param instances number of workers for this function
     * @throws IOException in case of routing error
     */
    @SuppressWarnings("rawtypes")
    private void register(String route, TypedLambdaFunction lambda, boolean isPrivate, int instances)
            throws IOException {
        if (lambda == null) {
            throw new IllegalArgumentException("Missing LambdaFunction instance");
        }
        String path = getValidatedRoute(route);
        if (registry.containsKey(path)) {
            log.warn("{} LambdaFunction {}", RELOADING, path);
            release(path);
        }
        ServiceDef service = new ServiceDef(path, lambda).setConcurrency(instances).setPrivate(isPrivate);
        ServiceQueue manager = new ServiceQueue(service);
        service.setManager(manager);
        // save into local registry
        registry.put(path, service);
        if (!isPrivate) {
            advertiseRoute(route);
        }
    }

    @SuppressWarnings("rawtypes")
    private void registerAsync(String route, KotlinLambdaFunction lambda, boolean isPrivate, int instances)
            throws IOException {
        if (lambda == null) {
            throw new IOException("Missing KotlinLambdaFunction instance");
        }
        String path = getValidatedRoute(route);
        if (registry.containsKey(path)) {
            log.warn("{} KotlinLambdaFunction {}", RELOADING, path);
            release(path);
        }
        ServiceDef service = new ServiceDef(path, lambda).setConcurrency(instances).setPrivate(isPrivate);
        ServiceQueue manager = new ServiceQueue(service);
        service.setManager(manager);
        // save into local registry
        registry.put(path, service);
        if (!isPrivate) {
            advertiseRoute(route);
        }
    }

    /**
     * Register a public stream function
     *
     * @param route name of the service
     * @param lambda function
     * @throws IOException if the route name is invalid
     */
    public void registerStream(String route, StreamFunction lambda) throws IOException {
        registerStream(route, lambda, false);
    }

    /**
     * Register a private stream function
     *
     * @param route name of the service
     * @param lambda function
     * @throws IOException if the route name is invalid
     */
    public void registerPrivateStream(String route, StreamFunction lambda) throws IOException {
        registerStream(route, lambda, true);
    }

    private void registerStream(String route, StreamFunction lambda, boolean isPrivate) throws IOException {
        if (lambda == null) {
            throw new IOException("Missing StreamFunction instance");
        }
        String path = getValidatedRoute(route);
        if (registry.containsKey(path)) {
            log.warn("{} StreamFunction {}", RELOADING, path);
            release(path);
        }
        ServiceDef service = new ServiceDef(path, lambda).setConcurrency(1).setPrivate(isPrivate);
        ServiceQueue manager = new ServiceQueue(service);
        service.setManager(manager);
        registry.put(path, service);
        if (!isPrivate) {
            advertiseRoute(route);
        }
    }

    private String getValidatedRoute(String route) {
        if (route == null) {
            throw new IllegalArgumentException("Missing service routing path");
        }
        // guarantee that only valid service name is registered
        Utility util = Utility.getInstance();
        if (!util.validServiceName(route)) {
            throw new IllegalArgumentException(INVALID_ROUTE +
                    "name - use 0-9, a-z, period, hyphen or underscore characters");
        }
        String path = util.filteredServiceName(route);
        if (path.length() == 0) {
            throw new IllegalArgumentException(INVALID_ROUTE + "name");
        }
        if (!path.contains(".")) {
            throw new IllegalArgumentException(INVALID_ROUTE + route +
                    " because it is missing dot separator(s). e.g. hello.world");
        }
        if (util.reservedExtension(path)) {
            throw new IllegalArgumentException(INVALID_ROUTE + route + " which is use a reserved extension");
        }
        if (util.reservedFilename(path)) {
            throw new IllegalArgumentException(INVALID_ROUTE + route + " which is a reserved Windows filename");
        }
        return path;
    }

    private void advertiseRoute(String route) throws IOException {
        TargetRoute cloud = EventEmitter.getInstance().getCloudRoute();
        if (cloud != null) {
            String personality = Platform.getInstance().getName()+", "+ServerPersonality.getInstance().getType().name();
            EventEmitter.getInstance().send(ServiceDiscovery.SERVICE_REGISTRY,
                    new Kv(PERSONALITY, personality),
                    new Kv(ServiceDiscovery.ROUTE, route),
                    new Kv(ServiceDiscovery.ORIGIN, getOrigin()),
                    new Kv(ServiceDiscovery.TYPE, ServiceDiscovery.ADD));
        }
    }

    /**
     * Unregister a route from this application instance
     *
     * @param route name of a service
     * @return true if successful
     */
    public boolean release(String route) {
        if (route != null && registry.containsKey(route)) {
            ServiceDef def = registry.get(route);
            if (!def.isPrivate()) {
                TargetRoute cloud = EventEmitter.getInstance().getCloudRoute();
                if (cloud != null) {
                    try {
                        EventEmitter.getInstance().send(ServiceDiscovery.SERVICE_REGISTRY,
                                new Kv(ServiceDiscovery.ROUTE, route),
                                new Kv(ServiceDiscovery.ORIGIN, getOrigin()),
                                new Kv(ServiceDiscovery.TYPE, ServiceDiscovery.UNREGISTER));
                    } catch (IOException e) {
                        // ok to ignore
                    }
                }
            }
            ServiceQueue manager = getManager(route);
            if (manager != null) {
                registry.remove(route);
                manager.stop();
            }
            return true;
        } else {
            return false;
        }
    }

    /**
     * Check if some route names are registered in this application instance
     *
     * @param routes of the services
     * @return true or false
     */
    public boolean hasRoute(List<String> routes) {
        if (routes.isEmpty()) {
            return false;
        }
        int n = 0;
        for (String r: routes) {
            if (hasRoute(r)) {
                n++;
            }
        }
        return n == routes.size();
    }

    /**
     * Check if a route name is registered in this application instance
     *
     * @param route name of a service
     * @return true or false
     */
    public boolean hasRoute(String route) {
        if (route == null) {
            return false;
        } else {
            String name = route.contains("@") ? route.substring(0, route.indexOf('@')) : route;
            return registry.containsKey(name);
        }
    }

    /**
     * Internal API - DO NOT use it in user application code
     *
     * @param route name of a service
     * @return manager instance
     */
    public ServiceQueue getManager(String route) {
        return route != null && registry.containsKey(route)? registry.get(route).getManager() : null;
    }

    /**
     * This method may be used during application startup.
     *
     * @param provider route name of a service that you want to check if it is ready
     * @param seconds to wait
     * @return future response of true or false
     */
    public Future<Boolean> waitForProvider(String provider, int seconds) {
        return waitForProviders(Collections.singletonList(provider), seconds);
    }

    /**
     * This method may be used during application startup.
     *
     * @param providers route names of services that you want to check if they are ready
     * @param seconds to wait
     * @return future response of true or false
     */
    public Future<Boolean> waitForProviders(List<String> providers, int seconds) {
        return Future.future(promise -> {
            if (hasRoute(providers)) {
                getEventExecutor().submit(() -> promise.complete(true));
            } else {
                getVertx().setTimer(2000, t ->
                        waitForProviders(promise, providers, 0, Math.max(2, seconds) / 2));
            }
        });
    }

    private void waitForProviders(Promise<Boolean> promise, List<String> providers, int attempt, int max) {
        int iteration = attempt + 1;
        if (hasRoute(providers)) {
            getEventExecutor().submit(() -> promise.complete(true));
        } else {
            if (iteration >= max) {
                getEventExecutor().submit(() -> promise.complete(false));
            } else {
                logRecently("info", "Waiting for " + providers + " to get ready... " + iteration);
                getVertx().setTimer(2000, t -> waitForProviders(promise, providers, iteration, max));
            }
        }
    }

    private void logRecently(String level, String message) {
        Utility util = Utility.getInstance();
        // this avoids printing duplicated log in a concurrent situation
        String hash = util.getUTF(crypto.getMd5(util.getUTF(message)));
        if (!cache.exists(hash)) {
            cache.put(hash, true);
            if ("error".equals(level)) {
                log.warn(message);
            }
            if ("info".equals(level)) {
                log.info(message);
            }
        }
    }

}
