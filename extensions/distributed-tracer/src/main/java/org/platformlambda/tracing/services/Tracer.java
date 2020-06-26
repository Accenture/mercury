package org.platformlambda.tracing.services;

import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.Utility;
import org.platformlambda.tracing.ws.WsTrace;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.HashMap;
import java.util.Map;
import java.util.Set;

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
        data.put("annotations", body);
        // send the trace info to all connected UI clients
        PostOffice po = PostOffice.getInstance();
        Set<String> txPaths = WsTrace.getConnections();
        for (String path: txPaths) {
            po.send(path, data);
        }
        /*
         * do not log because the individual applications have already log the trace event.
         * Otherwise, there would be duplicated log if we forward the log to a centralized logging system
         * such as Splunk.
         */
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
