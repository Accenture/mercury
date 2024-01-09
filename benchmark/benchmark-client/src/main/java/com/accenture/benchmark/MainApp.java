/*

    Copyright 2018-2024 Accenture Technology

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

package com.accenture.benchmark;

import com.accenture.services.BenchmarkService;
import com.accenture.services.Echo;
import com.accenture.services.InMemoryEventHealthCheck;
import com.accenture.services.ReceiveOnly;
import org.platformlambda.core.annotations.MainApplication;
import org.platformlambda.core.models.EntryPoint;
import org.platformlambda.core.system.AppStarter;
import org.platformlambda.core.system.LocalPubSub;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.util.AppConfigReader;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;

@MainApplication(sequence=1)
public class MainApp implements EntryPoint {
    private static final Logger log = LoggerFactory.getLogger(MainApp.class);

    private static final String BENCHMARK_USERS = "benchmark.users";
    private static final String BENCHMARK_SERVICE = "benchmark.service";

    /**
     * This main class is only used when testing the app from the IDE.
     *
     * @param args - command line arguments
     */
    public static void main(String[] args) {
        AppStarter.main(args);
    }

    @Override
    public void start(String[] args) throws IOException {
        Platform platform = Platform.getInstance();
        AppConfigReader config = AppConfigReader.getInstance();
        // if there is no network connector, the benchmark will be conducted during the in-memory event system
        if ("none".equals(config.getProperty("cloud.connector", "none"))) {
            platform.register("network.echo", new Echo(), 200);
            platform.register("network.one.way", new ReceiveOnly(), 200);
            platform.registerKotlinPrivate("cloud.connector.health", new InMemoryEventHealthCheck(), 5);
        }
        platform.connectToCloud();
        platform.registerPrivate(BENCHMARK_SERVICE, new BenchmarkService(), 10);
        // use local pub/sub to broadcast benchmark result to all users
        LocalPubSub ps = LocalPubSub.getInstance();
        ps.createTopic(BENCHMARK_USERS);
        log.info("Started");
    }

}
