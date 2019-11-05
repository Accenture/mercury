package org.platformlambda.core.util;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class SimpleRBAC {
    private static final Logger log = LoggerFactory.getLogger(SimpleRBAC.class);

    private static final String ROLES = "roles";
    private static final String SERVICES = "services";
    private static final List<String> userRoles = new ArrayList<>();
    private static final Map<String, List<String>> accessMap = new HashMap<>();
    private static final SimpleRBAC instance = new SimpleRBAC();

    private SimpleRBAC() {
        ConfigReader config = getConfig();
        if (config == null) {
            log.error("RBAC configuration file is not available" );
        } else {
            int roles = loadRoles(config);
            int access = loadRbac(config);
            log.info("Loaded {} role{} and {} RBAC {}",
                    roles, roles == 1? "": "s", access, access == 1? "entry" : "entries");
        }
    }

    public static SimpleRBAC getInstance() {
        return instance;
    }

    public List<String> getUserRoles() {
        return userRoles;
    }
    
    public Map<String, List<String>> getAccessMap() {
        return accessMap;
    }

    public boolean permitted(String service, String... roles) {
        if (accessMap.containsKey(service)) {
            List<String> list = accessMap.get(service);
            for (String s: list) {
                for (String r: roles) {
                    if (s.equalsIgnoreCase(r)) {
                        return true;
                    }
                }
            }
            return false;

        } else {
            // Unrestricted access if there are no RBAC entries for the service
            return true;
        }
    }

    @SuppressWarnings("unchecked")
    private int loadRoles(ConfigReader config) {
        Object roles = config.get(ROLES);
        if (roles instanceof List) {
            List<Object> labels = (List<Object>) roles;
            for (Object o: labels) {
                userRoles.add(o.toString());
            }
            return userRoles.size();
        } else {
            log.error("Unable to load roles because 'roles' is not a list");
        }
        return 0;
    }

    @SuppressWarnings("unchecked")
    private int loadRbac(ConfigReader config) {
        Object rbac = config.get(SERVICES);
        if (rbac instanceof Map) {
            Map<String, Object> rights = (Map<String, Object>) rbac;
            for (Object k: rights.keySet()) {
                Object v = rights.get(k);
                if (v instanceof List) {
                    List<Object> labels = (List<Object>) v;
                    List<String> list = new ArrayList<>();
                    for (Object o: labels) {
                        list.add(o.toString());
                    }
                    if (list.isEmpty()) {
                        log.error("Invalid RBAC entry ({}) because it is empty", k);
                    } else {
                        accessMap.put(k.toString(), list);
                    }
                } else {
                    log.error("Invalid RBAC entry ({}) because it is not a list", k);
                }
            }
            return accessMap.size();
        } else {
            log.error("Unable to load roles because 'roles' is not a list");
        }
        return 0;
    }

    private ConfigReader getConfig() {
        AppConfigReader reader = AppConfigReader.getInstance();
        List<String> paths = Utility.getInstance().split(reader.getProperty("rbac.config",
                "file:/tmp/config/rbac.yaml, classpath:/rbac.yaml"), ", ");
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
        return null;
    }
}
