/*

    Copyright 2018-2019 Accenture Technology

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

package com.accenture.examples.circuit.breaker;

import com.netflix.hystrix.HystrixCommand;
import com.netflix.hystrix.HystrixCommandGroupKey;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.system.PostOffice;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.concurrent.TimeoutException;

/**
 * This illustrates a simple use case of Netflix circuit breaker. It is just a demo.
 *
 * For advanced controls, please refer to https://github.com/Netflix/Hystrix/wiki/Configuration
 * e.g. setting thread pool, timeout, etc.
 */
public class CircuitBreakerCommand extends HystrixCommand<Object> {
    private static final Logger log = LoggerFactory.getLogger(CircuitBreakerCommand.class);

    private String primaryRoute, secondaryRoute;
    private long timeout;
    private Object payload;
    private Kv[] headers;

    public CircuitBreakerCommand(String primaryRoute, String secondaryRoute,
                                 long timeout, Object payload, Kv... headers) {
        super(HystrixCommandGroupKey.Factory.asKey(primaryRoute), (int) timeout);
        this.primaryRoute = primaryRoute;
        this.secondaryRoute = secondaryRoute;
        this.timeout = timeout;
        this.payload = payload;
        this.headers = headers;
    }

    @Override
    protected Object run() throws Exception {
        PostOffice po = PostOffice.getInstance();
        try {
            EventEnvelope response = po.request(primaryRoute, timeout, payload, headers);
            log.info("SUCCESSFULLY CALLED {}", primaryRoute);
            return response.getBody();
        } catch (IOException | TimeoutException | AppException e) {
            log.warn("FALL BACK TO SECONDARY SERVICE DUE TO {}", e.getMessage());
            throw e;
        }
    }

    @Override
    protected Object getFallback() {
        PostOffice po = PostOffice.getInstance();
        try {
            EventEnvelope response = po.request(secondaryRoute, timeout, payload, headers);
            return response.getBody();
        } catch (IOException | TimeoutException | AppException e) {
            log.error("SECONDARY ROUTE {} FAILED DUE TO {}", secondaryRoute, e.getMessage());
            throw new RuntimeException(e);
        }
    }


}
