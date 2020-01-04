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

package org.platformlambda.core.system;

import akka.actor.ActorRef;
import akka.actor.ActorSystem;
import org.platformlambda.core.annotations.CloudConnector;
import org.platformlambda.core.annotations.CloudService;
import org.platformlambda.core.models.CloudSetup;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.models.TargetRoute;
import org.platformlambda.core.services.DistributedTrace;
import org.platformlambda.core.services.ObjectStreamManager;
import org.platformlambda.core.services.RouteSubstitutionManager;
import org.platformlambda.core.util.*;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;
import java.util.concurrent.TimeoutException;

public class Platform {
    private static final Logger log = LoggerFactory.getLogger(Platform.class);
    private static final ManagedCache cache = ManagedCache.createCache("system.log.cache", 30000);
    private static final CryptoApi crypto = new CryptoApi();

    public static final String STREAM_MANAGER = "object.streams.io";
    public static final String DISTRIBUTED_TRACING = "distributed.tracing";
    private static final String ROUTE_MAPPER = ".route.mapper";
    private static final ConcurrentMap<String, ServiceDef> registry = new ConcurrentHashMap<>();
    private static final StopSignal STOP = new StopSignal();
    private static final String ROUTE_SUBSTITUTION_FEATURE = "application.feature.route.substitution";
    private static final String LAMBDA = "lambda";
    private static ActorSystem system;
    private static String lambdaId, namespace;
    private static boolean cloudSelected = false, cloudServicesStarted = false;
    private static Platform instance = new Platform();

    private Platform() {
        // initialize instance because the registration method needs it
        instance = this;
        // start built-in services
        AppConfigReader config = AppConfigReader.getInstance();
        boolean substitute = config.getProperty(ROUTE_SUBSTITUTION_FEATURE, "false").equals("true");
        try {
            registerPrivate(DISTRIBUTED_TRACING, new DistributedTrace(), 1);
            // streaming becomes a standard feature since v1.12.0
            registerPrivate(STREAM_MANAGER, new ObjectStreamManager(), 1);
            if (substitute) {
                // route substitution manager is registered as PUBLIC so it can sync with its peers
                register(getRouteManagerName(), new RouteSubstitutionManager(), 1);
            }
        } catch (IOException e) {
            log.error("Unable to create {} - {}", STREAM_MANAGER, e.getMessage());
        }
    }

    public static Platform getInstance() {
        return instance;
    }

    public ActorSystem getEventSystem() {
        if (system == null) {
            system = ActorSystem.create(LAMBDA);
        }
        return system;
    }

    public String getName() {
        return Utility.getInstance().getPackageName();
    }

    public String getRouteManagerName() {
        return getName()+ ROUTE_MAPPER;
    }

    /**
     * Namespace will be null if multi.tenancy.namespace is not configured in application.properties
     * @return namespace
     */
    public String getNamespace() {
        return namespace;
    }

    /**
     * Origin ID is the unique identifier for an application instance.
     *
     * @return unique origin ID
     */
    public String getOrigin() {
        if (lambdaId == null) {
            Utility util = Utility.getInstance();
            AppConfigReader config = AppConfigReader.getInstance();
            namespace = config.getProperty("multi.tenancy.namespace");
            lambdaId = namespace == null? Utility.getInstance().getDateUuid() :
                    Utility.getInstance().getDateUuid() + "." + util.filteredServiceName(namespace);
        }
        return lambdaId;
    }

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
            if (personality.getType() == ServerPersonality.Type.UNDEFINED) {
                personality.setType(ServerPersonality.Type.APP);
            }
            AppConfigReader reader = AppConfigReader.getInstance();
            String name = reader.getProperty(PostOffice.CLOUD_CONNECTOR, PostOffice.EVENT_NODE);
            SimpleClassScanner scanner = SimpleClassScanner.getInstance();
            List<Class<?>> services = scanner.getAnnotatedClasses(CloudConnector.class, true);
            if (!startService(name, services, true)) {
                log.error("Cloud connector ({}) not found", name);
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
            if (name.equals(serviceName)) {
                try {
                    Object o = cls.newInstance();
                    if (o instanceof CloudSetup) {
                        CloudSetup cloud = (CloudSetup) o;
                        new Thread(()-> {
                            log.info("Starting cloud {} {} using {}", isConnector? "connector" : "service", name, cls.getName());
                            cloud.initialize();
                            /*
                             * For wrapper, the system will execute original connector or service after initialization.
                             */
                            if (original.length() > 0) {
                                startService(original, services, isConnector);
                            }
                        }).start();
                        return true;
                    } else {
                        log.error("Unable to start cloud {} ({}) because it does not inherit {}",
                                isConnector? "connector" : "service",
                                cls.getName(), CloudSetup.class.getName());
                    }

                } catch (InstantiationException  | IllegalAccessException e) {
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
    public void register(String route, LambdaFunction lambda, int instances) throws IOException {
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
    public void registerPrivate(String route, LambdaFunction lambda, int instances) throws IOException {
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
        if (ServerPersonality.getInstance().getType() != ServerPersonality.Type.PLATFORM) {
            advertiseRoute(route);
        }
    }

    private void register(String route, LambdaFunction lambda, Boolean isPrivate, Integer instances)
            throws IOException {
        if (route == null) {
            throw new IOException("Missing service routing path");
        }
        if (lambda == null) {
            throw new IOException("Missing lambda function");
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
        if (registry.containsKey(path)) {
            throw new IOException("Route "+path+" already exists");
        }
        ActorRef manager = Platform.getInstance().getEventSystem().actorOf(ServiceQueue.props(path), path);
        ServiceDef service = new ServiceDef(path, lambda, manager).setConcurrency(instances).setPrivate(isPrivate);
        // tell manager to start workers
        manager.tell(service, ActorRef.noSender());
        // save into local registry
        registry.put(path, service);
        // automatically set personality if not defined
        ServerPersonality personality = ServerPersonality.getInstance();
        if (personality.getType() == ServerPersonality.Type.UNDEFINED) {
            personality.setType(ServerPersonality.Type.APP);
        }
        if (!isPrivate && personality.getType() != ServerPersonality.Type.PLATFORM) {
            advertiseRoute(route);
        }
    }

    private void advertiseRoute(String route) throws IOException {
        TargetRoute cloud = PostOffice.getInstance().getCloudRoute();
        if (cloud != null) {
            boolean tell = false;
            if (cloud.isEventNode()) {
                // if platform connection is ready, register to the event node
                EventNodeConnector connector = EventNodeConnector.getInstance();
                if (connector.isConnected() && connector.isReady()) {
                    // event node does not have local buffering so we can only send when it is connected
                    tell = true;
                }
            } else {
                // MQ has local buffering so we can send any time
                tell = true;
            }
            if (tell) {
                PostOffice.getInstance().send(ServiceDiscovery.SERVICE_REGISTRY,
                        new Kv(EventNodeConnector.PERSONALITY, ServerPersonality.getInstance().getType().name()),
                        new Kv(ServiceDiscovery.ROUTE, route),
                        new Kv(ServiceDiscovery.ORIGIN, getOrigin()),
                        new Kv(ServiceDiscovery.TYPE, ServiceDiscovery.ADD));
            }
        }
    }

    public void release(String route) throws IOException {
        if (route != null && registry.containsKey(route)) {
            ServiceDef def = registry.get(route);
            if (!def.isPrivate() && ServerPersonality.getInstance().getType() != ServerPersonality.Type.PLATFORM) {
                TargetRoute cloud = PostOffice.getInstance().getCloudRoute();
                if (cloud != null) {
                    boolean tell = false;
                    if (cloud.isEventNode()) {
                        EventNodeConnector connector = EventNodeConnector.getInstance();
                        if (connector.isConnected() && connector.isReady()) {
                            // event node does not have local buffering so we can only send when it is connected
                            tell = true;
                        }
                    } else {
                        // MQ has local buffering so we can send any time
                        tell = true;
                    }
                    if (tell) {
                        PostOffice.getInstance().send(ServiceDiscovery.SERVICE_REGISTRY,
                                new Kv(ServiceDiscovery.ROUTE, route),
                                new Kv(ServiceDiscovery.ORIGIN, getOrigin()),
                                new Kv(ServiceDiscovery.TYPE, ServiceDiscovery.UNREGISTER));
                    }
                }
            }
            ActorRef manager = getManager(route);
            if (manager != null) {
                registry.remove(route);
                manager.tell(STOP, ActorRef.noSender());
            }

        } else {
            throw new IOException("Route "+route+" not found");
        }
    }

    public boolean hasRoute(String route) {
        return route != null && registry.containsKey(route);
    }

    public ActorRef getManager(String route) {
        return route != null && registry.containsKey(route)? registry.get(route).getManager() : null;
    }

    public void waitForProvider(String provider, int seconds) throws TimeoutException {
        if (!hasRoute(provider)) {
            int cycles = seconds / 2;
            int count = 1;
            do {
                // retry every 2 seconds
                try {
                    Thread.sleep(2000);
                } catch (InterruptedException e) {
                    // ok to ignore
                }
                logRecently("info", "Waiting for " + provider + " to get ready... " + count);
                // taking too much time?
                if (++count >= cycles) {
                    String message = "Giving up " + provider + " because it is not ready after " + seconds + " seconds";
                    logRecently("error", message);
                    throw new TimeoutException(message);
                }
            } while (!hasRoute(provider));
            logRecently("info", provider + " is ready");
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
            if (level.equals("warn")) {
                log.warn(message);
            }
            if (level.equals("info")) {
                log.info(message);
            }
        }
    }

}
