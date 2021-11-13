/*

    Copyright 2018-2021 Accenture Technology

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

package org.platformlambda.services;

import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.models.SamplePoJo;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.Date;
import java.util.Map;
import java.util.concurrent.atomic.AtomicInteger;

/**
 * IMPORTANT - for LambdaFunction, the handleEvent method is the event handler.
 * Please do not use any global scope variables. All variables must be in functional scope.
 * If you must use global scope variables, you may use Java Concurrent collections.
 */
public class HelloPoJo implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(HelloPoJo.class);

    private static final AtomicInteger counter = new AtomicInteger(0);

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws AppException {
        String id = headers.get("id");
        if (id == null) {
            throw new IllegalArgumentException("Missing parameter 'id'");
        }
        if (id.equals("1")) {
            // return some place-holder values to demonstrate the PoJo can be transported over the network
            SamplePoJo mock = new SamplePoJo(1, "Simple PoJo class", "100 World Blvd, Planet Earth");
            // set current timestamp to indicate that the object is a new one
            mock.setDate(new Date());
            // set instance count and service origin ID to show that the object comes from a different instance
            mock.setInstance(instance);
            mock.setOrigin(Platform.getInstance().getOrigin());
            mock.setSeq(counter.incrementAndGet());
            log.info("Pojo delivered by instance #{}", instance);
            return mock;
        } else {
            throw new AppException(404, "Not found. Try id = 1");
        }
    }

}
