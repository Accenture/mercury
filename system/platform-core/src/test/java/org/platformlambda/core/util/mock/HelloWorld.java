package org.platformlambda.core.util.mock;

import org.platformlambda.core.annotations.PreLoad;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;

import java.util.HashMap;
import java.util.Map;

@PreLoad(route="hello.world, hello.alias", instances = 10)
public class HelloWorld implements LambdaFunction {

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
        int c = body instanceof Integer? (int) body : 2;
        if (c % 2 == 0) {
            Thread.sleep(1000);
            // timeout the incoming request
        }
        Map<String, Object> result = new HashMap<>();
        result.put("headers", headers);
        result.put("body", body);
        result.put("instance", instance);
        result.put("counter", c);
        result.put("origin", Platform.getInstance().getOrigin());
        return result;
    }
}
