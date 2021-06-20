package org.platformlambda.core.util.main;

import org.platformlambda.core.annotations.MainApplication;
import org.platformlambda.core.annotations.OptionalService;
import org.platformlambda.core.models.EntryPoint;
import org.platformlambda.core.system.Platform;

@MainApplication
@OptionalService("server.port=8085")
public class MockApp implements EntryPoint {

    @Override
    public void start(String[] args) throws Exception {

        Platform platform = Platform.getInstance();
        platform.connectToCloud();
        System.out.println("started");
    }
}
