package org.platformlambda.core.services;

import org.platformlambda.core.annotations.EventInterceptor;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.PostOffice;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.List;
import java.util.Map;

@EventInterceptor
public class Multicaster implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(Multicaster.class);

    private final String source;
    private final List<String> targets;

    public Multicaster(String source, List<String> targets) {
        this.source = source;
        this.targets = targets;
        log.info("Multicast routing - {} -> {}", source, targets);
    }

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) {
        PostOffice po = PostOffice.getInstance();
        EventEnvelope event = (EventEnvelope) body;
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
