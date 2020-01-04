/*

    Copyright 2018-2020 Accenture Technology

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
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.models.ObjectWithGenericType;
import org.platformlambda.models.SamplePoJo;

import java.io.IOException;
import java.util.Date;
import java.util.Map;

public class HelloGeneric implements LambdaFunction {

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws AppException, IOException {

        String id = headers.get("id");
        if (id == null) {
            throw new IllegalArgumentException("Missing parameter 'id'");
        }

        if (id.equals("1")) {
            // to set status, key-values or parametric types, we can use EventEnvelope as a result wrapper
            EventEnvelope result = new EventEnvelope();

            ObjectWithGenericType<SamplePoJo> genericObject = new ObjectWithGenericType<>();
            // return some place-holder values to demonstrate the PoJo can be transported over the network
            SamplePoJo mock = new SamplePoJo(1, "Class with generic type resolved at run-time to be SamplePoJo", "200 World Blvd, Planet Earth");
            // set current timestamp to indicate that the object is a new one
            mock.setDate(new Date());
            // set instance count and service origin ID to show that the object comes from a different instance
            mock.setInstance(instance);
            mock.setOrigin(Platform.getInstance().getOrigin());

            genericObject.setId(101);
            genericObject.setContent(mock);

            result.setBody(genericObject);
            result.setParametricType(SamplePoJo.class);

            return result;
        } else {
            throw new AppException(404, "Not found. Try id = 1");
        }

    }

}
