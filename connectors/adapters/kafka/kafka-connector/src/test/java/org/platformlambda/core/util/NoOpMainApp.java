package org.platformlambda.core.util;

import org.platformlambda.core.annotations.MainApplication;
import org.platformlambda.core.models.EntryPoint;
import org.platformlambda.core.system.Platform;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

@MainApplication
public class NoOpMainApp implements EntryPoint {
    private static final Logger log = LoggerFactory.getLogger(NoOpMainApp.class);

    @Override
    public void start(String[] args) {
        Platform.getInstance().connectToCloud();
        log.info("Started");
    }
}
