package com.accenture.setup;

import org.platformlambda.core.annotations.BeforeApplication;
import org.platformlambda.core.models.EntryPoint;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

@BeforeApplication
public class SetupEnv implements EntryPoint {
    private static final Logger log = LoggerFactory.getLogger(SetupEnv.class);

    /**
     * Preparation steps before your MainApp starts
     *
     * @param args if you want to read command line arguments
     */
    @Override
    public void start(String[] args) {
        //
        // your code here to retrieve credentials and configuration from a cloud secret manager
        // and to generate the required config files in the "/tmp/config" folder.
        // 
        // Please refer to README in the benchmark root folder for sample config files.
        // 
        log.info("Placeholder to setup kafka.properties and presence.properties");
    }
}
