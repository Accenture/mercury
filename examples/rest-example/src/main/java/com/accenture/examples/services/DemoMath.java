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