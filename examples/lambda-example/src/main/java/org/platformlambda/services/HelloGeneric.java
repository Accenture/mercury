/*

    Copyright 2018-2023 Accenture Technology

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

import org.platformlambda.core.annotations.CoroutineRunner;
import org.platformlambda.core.annotations.PreLoad;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.AsyncHttpRequest;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.TypedLambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.models.ObjectWithGenericType;
import org.platformlambda.models.SamplePoJo;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.Date;
import java.util.Map;
import java.util.concurrent.atomic.AtomicInteger;

/**
 * This demo function assumes the request comes from REST automation as an HTTP request event.
 * <p>
 * Since this function executes very fast, we set the function execution strategy
 * as "coroutine" using the CoroutineRunner annotation.
 * <p>
 * IMPORTANT: LambdaFunction should be self-contained without shared objects.
 *            If you must use shared objects, use "static final" atomic or concurrent version of the
 *            Java class.
 */
@CoroutineRunner
@PreLoad(route="hello.generic", instances=10)
public class HelloGeneric implements TypedLambdaFunction<AsyncHttpRequest, Object> {
    private static final Logger log = LoggerFactory.getLogger(HelloGeneric.class);
    private static final AtomicInteger counter = new AtomicInteger(0);

    @Override
    public Object handleEvent(Map<String, String> headers, AsyncHttpRequest input, int instance) throws AppException {
        // Demonstrate that authentication service can pass session information here
        log.info("Got session information {}", input.getSessionInfo());
        // simple validation
        String id = input.getPathParameter("id");
        if ("1".equals(id)) {
            // To set status, key-values or parametric types, we can use EventEnvelope as a result wrapper
            EventEnvelope result = new EventEnvelope();
            ObjectWithGenericType<SamplePoJo> genericObject = new ObjectWithGenericType<>();
            // Return some place-holder values to demonstrate the PoJo can be transported over the network
            SamplePoJo mock = new SamplePoJo(1, "Generic class with parametric type SamplePoJo",
                                    "200 World Blvd, Planet Earth");
            // Set current timestamp to indicate that the object is a new one
            mock.setDate(new Date());
            // Set instance count and service origin ID to show that the object comes from a different instance
            mock.setInstance(instance);
            mock.setOrigin(Platform.getInstance().getOrigin());
            // Increment the counter for demo purpose
            genericObject.setId(counter.incrementAndGet());
            genericObject.setContent(mock);
            // Set the sample pojo into the generic object holder
            result.setBody(genericObject);
            result.setParametricType(SamplePoJo.class);
            return result;
        } else {
            throw new AppException(404, "Not found. Try id = 1");
        }
    }

}
