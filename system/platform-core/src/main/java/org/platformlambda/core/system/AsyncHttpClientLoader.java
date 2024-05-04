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

package org.platformlambda.core.system;

import org.platformlambda.automation.http.AsyncHttpClient;
import org.platformlambda.core.annotations.BeforeApplication;
import org.platformlambda.core.models.EntryPoint;
import org.platformlambda.core.services.DistributedTrace;

import java.io.IOException;

/**
 * This module loads the AsyncHttpClient before loading other user application start-up code
 * so that its service is available to user application modules including those with
 * the "BeforeApplication" annotation. It is recommended that your start-up modules use
 * sequence from 3 onwards.
 */
@BeforeApplication(sequence = 2)
public class AsyncHttpClientLoader implements EntryPoint {

    @Override
    public void start(String[] args) throws IOException {
        Platform platform = Platform.getInstance();
        platform.registerPrivate("async.http.request", new AsyncHttpClient(), 200);
        platform.registerPrivate("distributed.tracing", new DistributedTrace(), 1);
    }

}
