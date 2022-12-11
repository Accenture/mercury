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
package org.platformlambda.cloud;

import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.ConfigReader;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.*;

public class ConnectorConfig {
    private static final Logger log = LoggerFactory.getLogger(ConnectorConfig.class);

    private static final String INVALID_ROUTE = "Invalid route ";

    private static Map<String, String> topicReplacements;
    private static String serviceName, displayUrl;

    private static final AppConfigReader config = AppConfigReader.getInstance();
    private static final String TOPIC_SUBSTITUTION = "application.feature.topic.substitution";
    private static final boolean FEATURE_ENABLED = "true".equalsIgnoreCase(config.getProperty(TOPIC_SUBSTITUTION));

    public static boolean topicSubstitutionEnabled() {
        return FEATURE_ENABLED;
    }

    public static int substitutionCount() throws IOException {
        return getTopicSubstitution().size();
    }

    /**
     * The service name must be set by the corresponding cloud connector implementation class
     * <p>
     * @param serviceName for the connector
     */
    public static void setServiceName(String serviceName) {
        ConnectorConfig.serviceName = serviceName;
    }

    /**
     * The URL must be set by the corresponding cloud connector implementation class
     * <p>
     * @param displayUrl for the connector
     */
    public static void setDisplayUrl(String displayUrl) {
        ConnectorConfig.displayUrl = displayUrl;
    }

    public static String getServiceName() {
        return serviceName;
    }

    public static String getDisplayUrl() {
        return displayUrl;
    }

    public static ConfigReader getConfig(String key, String defaultValue) throws IOException {
        AppConfigReader reader = AppConfigReader.getInstance();
        String location = reader.getProperty(key);
        if (location == null) {
            log.warn("Config parameter {} not defined - using default location: {}", key, defaultValue);
            location = defaultValue;
        }
        List<String> paths = Utility.getInstance().split(location, ", ");
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

    public static Map<String, String> getTopicSubstitution() throws IOException {
        if (topicReplacements == null) {
            Utility util = Utility.getInstance();
            Map<String, String> result = new HashMap<>();
            AppConfigReader config = AppConfigReader.getInstance();
            String useStaticTopics = config.getProperty("application.feature.topic.substitution", "false");
            if ("true".equalsIgnoreCase(useStaticTopics)) {
                ConfigReader topicConfig = getConfig("topic.substitution.file",
                        "file:/tmp/config/topic-substitution.yaml,classpath:/topic-substitution.yaml");
                Map<String, Object> map = util.getFlatMap(topicConfig.getMap());
                if (map.isEmpty()) {
                    log.error("Application cannot start because topic-substitution.yaml is empty");
                    System.exit(-1);
                }
                log.info("Using pre-allocated topics - virtual topic mapping below:");
                List<String> normalizedTopics = new ArrayList<>();
                for (String t : map.keySet()) {
                    result.put(t, map.get(t).toString());
                    int dot = t.lastIndexOf(".");
                    if (dot > 0) {
                        int partition = util.str2int(t.substring(dot+1));
                        String vt = t.substring(0, dot) + "-" + util.zeroFill(partition, 999);
                        normalizedTopics.add(vt + "|" + t);
                    }
                }
                Collections.sort(normalizedTopics);
                for (String nt : normalizedTopics) {
                    int sep = nt.lastIndexOf('|');
                    String vt = nt.substring(0, sep);
                    String t = nt.substring(sep+1);
                    log.info("{} -> {}", vt, map.get(t));
                }
            }
            topicReplacements = result;
        }
        return topicReplacements;
    }

    public static void validateTopicName(String route) throws IOException {
        // topic name can be upper case or lower case
        String name = route.toLowerCase();
        Utility util = Utility.getInstance();
        if (!util.validServiceName(name)) {
            throw new IOException("Invalid route name - use 0-9, a-z, A-Z, period, hyphen or underscore characters");
        }
        String path = util.filteredServiceName(name);
        if (path.length() == 0) {
            throw new IOException("Invalid route name");
        }
        if (!path.contains(".")) {
            throw new IOException(INVALID_ROUTE+route+" because it is missing dot separator(s). e.g. hello.world");
        }
        if (util.reservedExtension(path)) {
            throw new IOException(INVALID_ROUTE+route+" which is a reserved extension");
        }
        if (util.reservedFilename(path)) {
            throw new IOException(INVALID_ROUTE+route+" which is a reserved Windows filename");
        }
    }

}
