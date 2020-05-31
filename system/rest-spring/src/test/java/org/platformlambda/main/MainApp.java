package org.platformlambda.main;

import org.platformlambda.core.annotations.MainApplication;
import org.platformlambda.core.models.EntryPoint;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

@MainApplication
public class MainApp implements EntryPoint{
    private static final Logger log = LoggerFactory.getLogger(MainApp.class);

    @Override
    public void start(String[] args) {
        /*
         * This main application is added because the Unit Test assumes
         * it is running a Spring Boot application
         */
        log.info("Started");
    }
}
