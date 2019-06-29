package org.platformlambda.kafka.util;

import org.platformlambda.core.util.ConfigReader;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.File;
import java.io.IOException;
import java.util.List;

public class SetupUtil {
    private static final Logger log = LoggerFactory.getLogger(SetupUtil.class);

    private static final String FILEPATH = "file:";

    public static ConfigReader getConfig(String pathList) {
        ConfigReader config = new ConfigReader();
        List<String> paths = Utility.getInstance().split(pathList, ",");
        for (String p: paths) {
            // ignore trailing spaces
            String name = p.trim();
            try {
                log.info("Loading from {}", name);
                if (name.startsWith(FILEPATH)) {
                    File f = new File(name.substring(FILEPATH.length()));
                    if (!f.exists()) {
                        log.warn("{} not found", f);
                        continue;
                    }
                }
                config.load(name);
                return config;
            } catch (IOException e) {
                log.error("Unable to setup kafka from {} - {}", p, e.getMessage());
                System.exit(-1);
            }
        }
        return null;
    }

}
