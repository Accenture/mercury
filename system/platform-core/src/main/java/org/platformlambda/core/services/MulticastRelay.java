package org.platformlambda.core.services;

import org.platformlambda.core.annotations.CoroutineRunner;
import org.platformlambda.core.annotations.EventInterceptor;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.TypedLambdaFunction;
import org.platformlambda.core.system.EventEmitter;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;
import java.util.Map;

@CoroutineRunner
@EventInterceptor
public class MulticastRelay implements TypedLambdaFunction<EventEnvelope, Void> {
    private static final Logger log = LoggerFactory.getLogger(MulticastRelay.class);

    private final String source;
    private final List<String> targets;

    public MulticastRelay(String source, List<String> targets) {
        this.source = source;
        this.targets = targets;
        log.info("Multicast routing - {} -> {}", source, targets);
    }

    @Override
    public Void handleEvent(Map<String, String> headers, EventEnvelope event, int instance) {
        EventEmitter po = EventEmitter.getInstance();
        for (String service: targets) {
            if (po.exists(service)) {
                try {
                    po.send(event.copy().setTo(service));
                } catch (Exception e) {
                    log.warn("Unable to relay {} -> {} - {}", source, service, e.getMessage());
                }
            } else {
                log.warn("Unable to relay {} -> {} - target not reachable", source, service);
            }
        }
        return null;
    }
}
