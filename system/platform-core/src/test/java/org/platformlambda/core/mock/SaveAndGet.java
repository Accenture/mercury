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

package org.platformlambda.core.mock;

import org.platformlambda.core.annotations.PreLoad;
import org.platformlambda.core.models.LambdaFunction;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.Map;

@PreLoad(route="event.save.get", isPrivate = false)
public class SaveAndGet implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(SaveAndGet.class);

    private static Object store = "?";

    @Override
    public Object handleEvent(Map<String, String> headers, Object input, int instance) throws Exception {
        log.info("Got headers={}, body={}", headers, input);
        String type = headers.get("type");
        if ("save".equals(type)) {
            store = input;
            return "saved";
        }
        if ("get".equals(type)) {
            return store;
        }
        throw new IllegalArgumentException("Invalid request type - must be save or get");
    }
}
