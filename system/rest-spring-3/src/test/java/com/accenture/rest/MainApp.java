package com.accenture.rest;

import org.platformlambda.core.annotations.MainApplication;
import org.platformlambda.core.models.EntryPoint;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;

import java.io.IOException;
import java.util.Date;
import java.util.HashMap;
import java.util.Map;

@MainApplication
public class MainApp implements EntryPoint {

    @Override
    public void start(String[] args) throws IOException {
        LambdaFunction f = (headers, input, instance) -> {
            Map<String, Object> result = new HashMap<>();
            result.put("time", new Date());
            result.put("greeting", input);
            return result;
        };
        Platform.getInstance().registerPrivate("hello.world", f, 10);
    }
}
