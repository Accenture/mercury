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
        LambdaFunction mock = (headers, input, instance) -> {
            log.info("Received {} {}", headers.isEmpty()? "*" : headers, input);
            return true;
        };
        platform.register("hello.world", mock, 5);
        platform.connectToCloud();
        log.info("started");
    }
}
