package org.platformlambda.mock;

import org.platformlambda.core.annotations.MainApplication;
import org.platformlambda.core.models.EntryPoint;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;

@MainApplication
public class MockApp implements EntryPoint {
    private static final Logger log = LoggerFactory.getLogger(MockApp.class);

    @Override
    public void start(String[] args) throws IOException {
        Platform platform = Platform.getInstance();
        LambdaFunction mock = (headers, body, instance) -> {
            log.info("Received {} {}", headers, body);
            return true;
        };
        platform.connectToCloud();
        platform.register("hello.world", mock, 5);
        log.info("started");
    }
}
