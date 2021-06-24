package org.platformlambda.mock;

import org.junit.BeforeClass;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;
import org.platformlambda.rest.RestServer;

import java.util.concurrent.atomic.AtomicInteger;

public class TestBase {
    protected static int port;

    private static final AtomicInteger startCounter = new AtomicInteger(0);

    @BeforeClass
    public static void setup() {
        if (startCounter.incrementAndGet() == 1) {
            Utility util = Utility.getInstance();
            AppConfigReader config = AppConfigReader.getInstance();
            port = util.str2int(config.getProperty("server.port", "8085"));
            RestServer.main(new String[0]);
        }
    }
}
