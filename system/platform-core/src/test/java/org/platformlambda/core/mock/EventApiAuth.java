package org.platformlambda.core.mock;

import org.platformlambda.core.annotations.CoroutineRunner;
import org.platformlambda.core.annotations.PreLoad;
import org.platformlambda.core.models.AsyncHttpRequest;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.TypedLambdaFunction;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.Map;

@CoroutineRunner
@PreLoad(route="event.api.auth")
public class EventApiAuth implements TypedLambdaFunction<AsyncHttpRequest, Object> {
    private static final Logger log = LoggerFactory.getLogger(EventApiAuth.class);
    @Override
    public Object handleEvent(Map<String, String> headers, AsyncHttpRequest input, int instance) throws Exception {
        log.info("Perform Event API authentication for {} {}", input.getMethod(), input.getUrl());
        return new EventEnvelope().setBody(true).setHeader("user", "demo");
    }
}
