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

package org.platformlambda.tracing.services;

import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.HashMap;
import java.util.Map;

public class Tracer implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(Tracer.class);

    public static final String PATH = "path";
    public static final String QUERY = "query";
    private static final String EXEC_TIME = "exec_time";
    private static final String SUCCESS = "success";
    private static final String STATUS = "status";

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
        /*
         * headers is a map of text key-values for the trace metrics and exception
         * body is a map of text key-values for transaction specific annotations
         */
        Map<String, Object> data = new HashMap<>();
        data.put("type", "trace");
        data.put("trace", transform(headers));
        data.put("data", body);
        /*
         * this is a demo. Therefore, we just print out the perf metrics.
         *
         * In a production system, you should forward to metrics to a centralized dashboard for visualization.
         */
        String json = SimpleMapper.getInstance().getMapper().writeValueAsString(data);
        log.info("\n{}", json);
        return true;
    }

    private Map<String, Object> transform(Map<String, String> headers) {
        // restore the original types for selected key-values
        Utility util = Utility.getInstance();
        Map<String, Object> result = new HashMap<>();
        for (String key: headers.keySet()) {
            switch (key) {
                case PATH:
                    String path = headers.get(key);
                    if (path.contains("?")) {
                        int sep = path.indexOf('?');
                        String p = path.substring(0, sep);
                        String q = path.substring(sep+1);
                        result.put(PATH, p);
                        result.put(QUERY, q);
                    } else {
                        result.put(PATH, path);
                    }
                    break;
                case SUCCESS:
                    result.put(key, "true".equalsIgnoreCase(headers.get(key)));
                    break;
                case EXEC_TIME:
                    result.put(key, util.str2float(headers.get(key)));
                    break;
                case STATUS:
                    result.put(key, util.str2int(headers.get(key)));
                    break;
                default:
                    result.put(key, headers.get(key));
            }
        }
        return result;
    }

}
