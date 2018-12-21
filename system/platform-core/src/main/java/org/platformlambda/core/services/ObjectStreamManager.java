/*

    Copyright 2018 Accenture Technology

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

package org.platformlambda.core.services;

import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.Date;
import java.util.HashMap;
import java.util.Map;

public class ObjectStreamManager implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(ObjectStreamManager.class);

    private static final String TYPE = "type";
    private static final String CREATE = "create";
    private static final String NAME = "name";
    private static final String DESTROY = "destroy";
    private static final String QUERY = "query";
    private static final String TOTAL = "total";
    private static final String STREAMS = "streams";

    private static final Map<String, Date> streams = new HashMap<>();

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws IOException {

        Platform platform = Platform.getInstance();
        if (CREATE.equals(headers.get(TYPE))) {
            ObjectStreamService service = new ObjectStreamService();
            platform.registerPrivate(service.getPath(), service, 1);
            streams.put(service.getPath(), new Date());
            log.info("{} created", service.getPath());
            return service.getPath()+"@"+platform.getOrigin();
        } else if (DESTROY.equals(headers.get(TYPE)) && headers.containsKey(NAME)) {
            String fqName = headers.get(NAME);
            String name = fqName.contains("@") ? fqName.substring(0, fqName.indexOf('@')) : fqName;
            Date created = streams.get(name);
            if (created != null) {
                streams.remove(name);
                platform.release(name);
                log.info("{} ({}) destroyed", name, Utility.getInstance().date2str(created));
                return true;
            } else {
                throw new IllegalArgumentException(name + " not found");
            }
        } else if (QUERY.equals(headers.get(TYPE))) {
            Map<String, Object> result = new HashMap<>();
            Map<String, Object> map = new HashMap<>();
            for (String k: streams.keySet()) {
                map.put(k, streams.get(k));
            }
            result.put(STREAMS, map);
            result.put(TOTAL, map.size());
            return result;

        } else {
            throw new IllegalArgumentException("Usage: set header (type=create) to create a new I/O stream");
        }
    }

}
