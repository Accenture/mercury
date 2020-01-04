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

package com.accenture.examples.services;

import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.util.Utility;

import java.util.Map;

public class DemoMath implements LambdaFunction {

    private static final Utility util = Utility.getInstance();

    /**
     * This service demonstrates a simple addition function
     *
     * @param headers containing parameters a and b
     * @param body not used for this function
     * @param instance to be provided at runtime
     * @return result as integer
     */
    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance)  {
        if (headers.containsKey("a") && headers.containsKey("b")) {
            String a = headers.get("a");
            String b = headers.get("b");
            if (util.isDigits(a) && util.isDigits(b)) {
                return util.str2int(a) + util.str2int(b);

            } else {
                throw new IllegalArgumentException("a and b must be numeric integers");
            }

        } else {
            throw new IllegalArgumentException("missing parameters a and b");
        }

    }

}