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

package org.platformlambda.core.system;

import org.platformlambda.core.models.LambdaFunction;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.Map;

public class ShutdownService implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(ShutdownService.class);

    private static final String USER = "user";

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
        String user = headers.get(USER);
        if (user != null) {
            log.info("Shutdown requested by {}", user);
            System.exit(-2);
        }
        return true;
    }
}
