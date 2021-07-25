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

package org.platformlambda.core.system;

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
import java.util.Date;
import java.util.List;
import java.util.UUID;
import java.util.concurrent.*;

public class Platform {
    private static final Logger log = LoggerFactory.getLogger(Platform.class);
    private static final ManagedCache cache = ManagedCache.createCache("system.log.cache", 30000);
    private static final CryptoApi crypto = new CryptoApi();
    private static final ConcurrentMap<String, BlockingQueue<Boolean>> serviceTokens = new ConcurrentHashMap<>();
    private static final ConcurrentMap<String, ServiceDef> registry = new ConcurrentHashMap<>();
    private static final String PERSONALITY = "personality";
    private static final String INIT = "init:";
    private static String originId;
    private static boolean cloudSelected = false, cloudServicesStarted = false;
    private static final Platform instance = new Platform();
    private static String appId;
    private final Vertx vertx;
    private final EventBus system;

    private Platform() {
        // singleton
        vertx = Vertx.vertx();
        system = vertx.eventBus();
    }

    public static Platform getInstance() {
        return instance;
    }

    /**
     * Internal use - DO NOT call this from user application
     * <p>
     * @param id of the service
     * @return blocking queue for service initialization
     */
    public BlockingQueue<Boolean> getServiceToken(String id) {
        return serviceTokens.get(id);
    }

    /**
     * IMPORTANT: If this OPTIONAL value is set, the origin ID will be derived from this value.
     * <p>
     * You MUST use unique ID for each application instance otherwise service routing would fail.
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
            log.info("application instance ID set to {}", Platform.appId);
        } else {
            log.error("Unable to change application instance ID as it was set as {}", Platform.appId);
        }
    }

    public String getAppId() {
        return Platform.appId;
    }

    /**
     * INTERNAL USE ONLY - The vertx event loop engine must be used exclusively by the platform-core
     * <p>
     * Please do not use it at user application level to avoid blocking the event loop.
     * <p>
     * @return vertx engine
     */
    public Vertx getVertx() {
        return vertx;
    }

    /**
     * INTERNAL USE ONLY - The vertx event bus must be used exclusively by the platform-core
     * <p>
     * Please do not use it at user application level to avoid blocking the event loop.
     * <p>
     * @return memory event bus
     */
    public EventBus getEventSystem() {
        return system;
    }

    public String getName() {
        return Utility.getInstance().getPackageName();
    }

    /**
     * Origin ID is the unique identifier for an application instance.
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

    /**
     * Cloud services will be started when your app makes a connectToCloud() request
     * <p>
     * Call this function only when you want to start cloud services without an event stream connector
     */
    public synchronized void startCloudServices() {
        if (!Platform.cloudServicesStarted) {
            // guarantee to execute once
            Platform.cloudServicesStarted = true;
            AppConfigReader reader = AppConfigReader.getInstance();
            String cloudServices = reader.getProperty(PostOffice.CLOUD_SERVICES);
            if (cloudServices != null) {
                List<String> list = Utility.getInstance().split(cloudServices, ", ");
                if (!list.isEmpty()) {
                    List<String> loaded = new ArrayList<>();
                    SimpleClassScanner scanner = SimpleClassScanner.getInstance();
                    List<Class<?>> services = scanner.getAnnotatedClasses(CloudService.class, true);
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
     * This will connect based on the "cloud.connector" parameter in the application.properties
     */
    public synchronized void connectToCloud() {
        if (!Platform.cloudSelected) {
            // guarantee to execute once
            Platform.cloudSelected = true;
            // set personality to APP automatically
            ServerPersonality personality = ServerPersonality.getInstance();
            AppConfigReader reader = AppConfigReader.getInstance();
            String name = reader.getProperty(PostOffice.CLOUD_CONNECTOR, "none");
            if (!"none".equalsIgnoreCase(name)) {
                SimpleClassScanner scanner = SimpleClassScanner.getInstance();
                List<Class<?>> services = scanner.getAnnotatedClasses(CloudConnector.class, true);
                if (!startService(name, services, true)) {
                    log.error("Cloud connector ({}) not found", name);
                }
            }
        }
    }

    private boolean startService(String name, List<Class<?>> services, boolean isConnector) {
        if (name == null) {
            return false;
        }
        for (Class<?> cls : services) {
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
                        new Thread(()-> {
                            log.info("Starting cloud {} {} using {}", isConnector? "connector" : "service", name, cls.getName());
                            cloud.initialize();
                            // execute next service if provided
                            if (!nextService.isEmpty()) {
                                startService(nextService, services, isConnector);
                            }
                        }).start();
                        return true;
                    } else {
                        log.error("Unable to start cloud {} ({}) because it does not inherit {}",
                                isConnector? "connector" : "service",
                                cls.getName(), CloudSetup.class.getName());
                    }

                } catch (InstantiationException | IllegalAccessException | NoSuchMethodException | InvocationTargetException e) {
                    log.error("Unable to start cloud {} ({}) - {}",
                            isConnector? "connector" : "service", cls.getName(), e.getMessage());
                }
                break;
            }
        }
        return false;
    }

    public ConcurrentMap<String, ServiceDef> getLocalRoutingTable() {
        return registry;
    }

    /**
     * Register a public lambda function with one or more concurrent instances.
     * Its routing path will be published to the global service registry.
     *
     * @param route path
     * @param lambda function
     * @param instances for concurrent processing of events
     * @throws IOException in case of duplicated registration
     */
    @SuppressWarnings("rawtypes")
    public void register(String route, TypedLambdaFunction lambda, int instances) throws IOException {
        register(route, lambda, false, instances);
    }

    /**
     * Private function is only visible within a single execution unit.
     * Its routing path will not be published to the global service registry.
     *
     * @param route path
     * @param lambda function
     * @param instances for concurrent processing of events
     * @throws IOException in case of duplicated registration
     */
    @SuppressWarnings("rawtypes")
    public void registerPrivate(String route, TypedLambdaFunction lambda, int instances) throws IOException {
        register(route, lambda, true, instances);
    }

    public void makePublic(String route) throws IOException {
        if (!hasRoute(route)) {
            throw new IOException("Route "+route+" not found");
        }
        ServiceDef service = registry.get(route);
        if (service == null) {
            throw new IOException("Route "+route+" not found");
        }
        if (!service.isPrivate()) {
            throw new IllegalArgumentException("Route "+route+" is already public");
        }
        // set it to public
        service.setPrivate(false);
        log.info("Converted {} to PUBLIC", route);
        advertiseRoute(route);
    }

    @SuppressWarnings("rawtypes")
    private void register(String route, TypedLambdaFunction lambda, boolean isPrivate, int instances)
            throws IOException {
        if (lambda == null) {
            throw new IOException("Missing lambda function");
        }
        String path = getValidatedRoute(route);
        if (registry.containsKey(path)) {
            log.warn("{} will be reloaded", path);
            release(path);
        }
        String uuid = UUID.randomUUID().toString();
        BlockingQueue<Boolean> signal = new ArrayBlockingQueue<>(1);
        ServiceDef service = new ServiceDef(path, lambda).setConcurrency(instances).setPrivate(isPrivate);
        ServiceQueue manager = new ServiceQueue(service);
        service.setManager(manager);
        // wait for service initialization
        try {
            serviceTokens.put(uuid, signal);
            system.send(service.getRoute(), INIT+uuid);
            signal.poll(2, TimeUnit.SECONDS);
        } catch (InterruptedException e) {
            log.error("{} took longer to initialize - the event system may be unhealthy", path);
        } finally {
            serviceTokens.remove(uuid);
        }
        // save into local registry
        registry.put(path, service);
        if (!isPrivate) {
            advertiseRoute(route);
        }
    }

    public void registerStream(String route, StreamFunction lambda) throws IOException {
        registerStream(route, lambda, false);
    }

    public void registerPrivateStream(String route, StreamFunction lambda) throws IOException {
        registerStream(route, lambda, true);
    }

    private void registerStream(String route, StreamFunction lambda, boolean isPrivate) throws IOException {
        if (lambda == null) {
            throw new IOException("Missing lambda function");
        }
        String path = getValidatedRoute(route);
        if (registry.containsKey(path)) {
            log.warn("{} will be reloaded", path);
            release(path);
        }
        String uuid = UUID.randomUUID().toString();
        BlockingQueue<Boolean> signal = new ArrayBlockingQueue<>(1);
        ServiceDef service = new ServiceDef(path, lambda).setConcurrency(1).setPrivate(isPrivate).setStream(true);
        ServiceQueue manager = new ServiceQueue(service);
        service.setManager(manager);
        try {
            serviceTokens.put(uuid, signal);
            system.send(service.getRoute(), INIT+uuid);
            signal.poll(2, TimeUnit.SECONDS);
        } catch (InterruptedException e) {
            log.error("{} took longer to initialize - the event system may be unhealthy", path);
        } finally {
            serviceTokens.remove(uuid);
        }
        registry.put(path, service);
        if (!isPrivate) {
            advertiseRoute(route);
        }
    }

    private String getValidatedRoute(String route) throws IOException {
        if (route == null) {
            throw new IOException("Missing service routing path");
        }
        // guarantee that only valid service name is registered
        Utility util = Utility.getInstance();
        if (!util.validServiceName(route)) {
            throw new IOException("Invalid route name - use 0-9, a-z, period, hyphen or underscore characters");
        }
        String path = util.filteredServiceName(route);
        if (path.length() == 0) {
            throw new IOException("Invalid route name");
        }
        if (!path.contains(".")) {
            throw new IOException("Invalid route "+route+" because it is missing dot separator(s). e.g. hello.world");
        }
        if (util.reservedExtension(path)) {
            throw new IOException("Invalid route "+route+" because it cannot use a reserved extension");
        }
        if (util.reservedFilename(path)) {
            throw new IOException("Invalid route "+route+" which is a reserved Windows filename");
        }
        return path;
    }

    private void advertiseRoute(String route) throws IOException {
        TargetRoute cloud = PostOffice.getInstance().getCloudRoute();
        if (cloud != null) {
            String personality = Platform.getInstance().getName()+", "+ServerPersonality.getInstance().getType().name();
            PostOffice.getInstance().send(ServiceDiscovery.SERVICE_REGISTRY,
                    new Kv(PERSONALITY, personality),
                    new Kv(ServiceDiscovery.ROUTE, route),
                    new Kv(ServiceDiscovery.ORIGIN, getOrigin()),
                    new Kv(ServiceDiscovery.TYPE, ServiceDiscovery.ADD));
        }
    }

    public void release(String route) throws IOException {
        if (route != null && registry.containsKey(route)) {
            ServiceDef def = registry.get(route);
            if (!def.isPrivate()) {
                TargetRoute cloud = PostOffice.getInstance().getCloudRoute();
                if (cloud != null) {
                    PostOffice.getInstance().send(ServiceDiscovery.SERVICE_REGISTRY,
                            new Kv(ServiceDiscovery.ROUTE, route),
                            new Kv(ServiceDiscovery.ORIGIN, getOrigin()),
                            new Kv(ServiceDiscovery.TYPE, ServiceDiscovery.UNREGISTER));
                }
            }
            ServiceQueue manager = getManager(route);
            if (manager != null) {
                registry.remove(route);
                manager.stop();
            }

        } else {
            throw new IOException("Route "+route+" not found");
        }
    }

    public boolean hasRoute(String route) {
        return route != null && registry.containsKey(route);
    }

    public ServiceQueue getManager(String route) {
        return route != null && registry.containsKey(route)? registry.get(route).getManager() : null;
    }

    public void waitForProvider(String provider, int seconds) throws TimeoutException {
        if (!hasRoute(provider)) {
            int waitTime = Math.max(2, seconds);
            int waitCycle = waitTime / 2;
            int count = 0;
            while (count < waitCycle && !hasRoute(provider)) {
                count++;
                try {
                    Thread.sleep(2000);
                } catch (InterruptedException e) {
                    // ok to ignore
                }
                if (count > 1) {
                    logRecently("info", "Waiting for " + provider + " to get ready... " + count);
                }
            }
            if (hasRoute(provider)) {
                logRecently("info", provider + " is ready");
            } else {
                String message = "Giving up " + provider + " because it is not ready after " + waitTime + " seconds";
                logRecently("error", message);
                throw new TimeoutException(message);
            }
        }
    }

    private void logRecently(String level, String message) {
        Utility util = Utility.getInstance();
        // this avoids printing duplicated log in a concurrent situation
        String hash = util.getUTF(crypto.getMd5(util.getUTF(message)));
        if (!cache.exists(hash)) {
            cache.put(hash, true);
            if (level.equals("error")) {
                log.warn(message);
            }
            if (level.equals("info")) {
                log.info(message);
            }
        }
    }

}
