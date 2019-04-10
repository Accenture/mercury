/*

    Copyright 2018-2019 Accenture Technology

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
import org.platformlambda.core.services.ObjectStreamManager;
import org.platformlambda.core.services.SystemLog;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.SimpleClassScanner;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.File;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.Set;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;
import java.util.concurrent.TimeoutException;

public class Platform {
    private static final Logger log = LoggerFactory.getLogger(Platform.class);

    public static final String STREAM_MANAGER = "system.streams.manager";
    private static final String SYSTEM_LOG = "system.log";
    private static final ConcurrentMap<String, ServiceDef> registry = new ConcurrentHashMap<>();
    private static final StopSignal STOP = new StopSignal();
    private static final String STREAMING_FEATURE = "application.feature.streaming";
    private static final String LAMBDA = "lambda";
    private static final String NODE_ID = "id";
    private static Platform instance = new Platform();
    private static ActorSystem system;
    private static String nodeId, lambdaId;
    private static boolean cloudSelected = false, cloudServicesStarted = false;

    private Platform() {
        // initialize instance
        instance = this;
        // start built-in services
        AppConfigReader config = AppConfigReader.getInstance();
        boolean streaming = config.getProperty(STREAMING_FEATURE, "false").equals("true");
        try {
            registerPrivate(SYSTEM_LOG, new SystemLog(), 1);
            if (streaming) {
                registerPrivate(STREAM_MANAGER, new ObjectStreamManager(), 1);
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

    /**
     * Node-ID is unique per machine.
     * It is only used by the Event Node (network event stream emulator).
     *
     * @return nodeId
     */
    public String getNodeId() {
        if (nodeId == null) {
            Utility util = Utility.getInstance();
            File dir = util.getIdentityFolder();
            if (!dir.exists()) {
                dir.mkdirs();
            }
            File file = new File(dir, NODE_ID);
            if (file.exists()) {
                nodeId = util.file2str(file).trim();
            } else {
                String uuid = util.getDateUuid();
                util.str2file(file, uuid + "\n");
                nodeId = uuid;
            }
        }
        return nodeId;
    }

    /**
     * Lambda-ID is a unique identifier for each application instance.
     * It is generated randomly each time when the app starts.
     * It is used as the originator for REST servers and LAMBDA executables.
     *
     * @return lambdaId
     */
    public String getLambdaId() {
        if (lambdaId == null) {
            lambdaId = Utility.getInstance().getDateUuid();
        }
        return lambdaId;
    }

    /**
     * Origin ID is the unique identifier for an application instance.
     *
     * @return unique origin ID
     */
    public String getOrigin() {
        return ServerPersonality.getInstance().getType() == ServerPersonality.Type.PLATFORM?
                getNodeId() : getLambdaId();
    }

    public synchronized void startCloudServices() {
        if (!Platform.cloudServicesStarted) {
            Platform.cloudServicesStarted = true;
            AppConfigReader reader = AppConfigReader.getInstance();
            String serviceList = reader.getProperty(PostOffice.CLOUD_SERVICES);
            if (serviceList != null) {
                List<String> list = Utility.getInstance().split(serviceList, ", ");
                if (!list.isEmpty()) {
                    List<String> loaded = new ArrayList<>();
                    SimpleClassScanner scanner = SimpleClassScanner.getInstance();
                    Set<String> packages = scanner.getPackages(true);
                    for (String p : packages) {
                        List<Class<?>> services = scanner.getAnnotatedClasses(p, CloudService.class);
                        for (Class<?> cls : services) {
                            CloudService connector = cls.getAnnotation(CloudService.class);
                            if (list.contains(connector.value())) {
                                try {
                                    Object o = cls.newInstance();
                                    if (o instanceof CloudSetup) {
                                        log.info("Starting cloud service ({})", cls.getName());
                                        loaded.add(connector.value());
                                        CloudSetup cloud = (CloudSetup) o;
                                        new Thread(cloud::initialize).start();

                                    } else {
                                        log.error("Unable start cloud service ({}) because it does not inherit {}",
                                                cls.getName(), CloudSetup.class.getName());
                                    }

                                } catch (InstantiationException  | IllegalAccessException e) {
                                    log.error("Unable start cloud service ({}) - {}", cls.getName(), e.getMessage());
                                }
                            }
                        }
                    }
                    if (loaded.isEmpty()) {
                        log.warn("No Cloud services are loaded");
                    } else {
                        log.info("Cloud services {} started", loaded);
                    }
                    if (loaded.size() != list.size()) {
                        log.warn("Incomplete cloud services - requested {}, found {}", list, loaded);
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
            // set personality to APP automatically
            ServerPersonality personality = ServerPersonality.getInstance();
            if (personality.getType() == ServerPersonality.Type.UNDEFINED) {
                personality.setType(ServerPersonality.Type.APP);
            }
            AppConfigReader reader = AppConfigReader.getInstance();
            String path = reader.getProperty(PostOffice.CLOUD_CONNECTOR, PostOffice.EVENT_NODE);
            boolean found = false;
            SimpleClassScanner scanner = SimpleClassScanner.getInstance();
            Set<String> packages = scanner.getPackages(true);
            for (String p : packages) {
                List<Class<?>> services = scanner.getAnnotatedClasses(p, CloudConnector.class);
                for (Class<?> cls : services) {
                    CloudConnector connector = cls.getAnnotation(CloudConnector.class);
                    if (connector.value().equals(path)) {
                        try {
                            Object o = cls.newInstance();
                            if (o instanceof CloudSetup) {
                                found = true;
                                CloudSetup cloud = (CloudSetup) o;
                                new Thread(()->{
                                    log.info("Starting cloud connector ({})", cls.getName());
                                    cloud.initialize();
                                    Platform.cloudSelected = true;

                                }).start();

                            } else {
                                log.error("Unable to start cloud connector ({}) because it does not inherit {}",
                                        cls.getName(), CloudSetup.class.getName());
                            }

                        } catch (InstantiationException  | IllegalAccessException e) {
                            log.error("Unable to start cloud connector ({}) - {}", cls.getName(), e.getMessage());
                        }
                        break;
                    }
                }
            }
            if (!found) {
                log.error("CLoud connector ({}) not found", path);
            }
        } else {
            log.error("Duplicated cloud connection request");
        }
    }

    public ConcurrentMap<String, ServiceDef> getLocalRoutingTable() {
        return registry;
    }

    /**
     * Stateless service is public lambda function with one or more concurrent instances.
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
        // start built
        // guarantee that only valid service name is registered
        Utility util = Utility.getInstance();
        if (!util.validServiceName(route)) {
            throw new IOException("Invalid route name - use 0-9, a-z, period, hyphen or underscore characters");
        }
        String path = util.filteredServiceName(route);
        if (path.length() == 0) {
            throw new IOException("Service routing path cannot be empty");
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
                        new Kv(EventNodeConnector.PERSONALITY, ServerPersonality.getInstance().getType().toString()),
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
        try {
            Platform platform = Platform.getInstance();
            PostOffice po = PostOffice.getInstance();
            int count = 1;
            while (!platform.hasRoute(provider)) {
                try {
                    Thread.sleep(1000);
                } catch (InterruptedException e) {
                    // ok to ignore
                }
                po.send(SYSTEM_LOG, "Waiting for " + provider + " to get ready... " + count, new Kv(SystemLog.LEVEL, SystemLog.INFO));
                // taking too much time?
                if (++count >= seconds) {
                    String message = "Giving up " + provider + " because it is not ready after " + seconds + " seconds";
                    po.send(SYSTEM_LOG, message, new Kv(SystemLog.LEVEL, SystemLog.ERROR));
                    throw new TimeoutException(message);
                }
            }
            po.send(SYSTEM_LOG, provider + " is ready", new Kv(SystemLog.LEVEL, SystemLog.INFO), new Kv(SystemLog.FINISH, true));
        } catch (IOException e) {
            log.error("Unable to check {} because {} not available", provider, SYSTEM_LOG);
        }
    }

}
