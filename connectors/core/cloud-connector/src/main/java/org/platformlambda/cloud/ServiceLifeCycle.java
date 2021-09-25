package org.platformlambda.cloud;

import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.PubSub;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.*;
import java.util.concurrent.atomic.AtomicInteger;

public class ServiceLifeCycle {
    private static final Logger log = LoggerFactory.getLogger(ServiceLifeCycle.class);

    public static final String TYPE = "type";
    public static final String INIT = "init";
    public static final String TOKEN = "token";
    public static final long INITIALIZE = -100;
    private static final String SEQUENCE = "seq";
    private static final long INTERVAL = 3000;
    private final String topic;
    private final String token;
    private final int partition;
    private static LambdaFunction resetHandler;

    /**
     * When offset is set to the special value INITIALIZE, initial load
     * will send an initialization token to the EventConsumer to make sure
     * the consumer is ready to read new events.
     *
     * @param topic that the consumer uses
     * @param partition for the topic
     * @param token of random value
     */
    public ServiceLifeCycle(String topic, int partition, String token) {
        this.topic = topic;
        this.partition = partition;
        this.token = token;
    }

    public void start() {
        final Platform platform = Platform.getInstance();
        final PostOffice po = PostOffice.getInstance();
        final PubSub ps = PubSub.getInstance();
        final String INIT_HANDLER = INIT + "." + (partition < 0? topic : topic + "." + partition);
        final AtomicInteger seq = new AtomicInteger(-1);
        final List<String> task = new ArrayList<>();
        LambdaFunction f = (headers, body, instance) -> {
            if (INIT.equals(body)) {
                int n = seq.incrementAndGet();
                try {
                    Map<String, String> event = new HashMap<>();
                    event.put(TYPE, INIT);
                    event.put(TOKEN, token);
                    event.put(SEQUENCE, String.valueOf(n));
                    if (n > 0) {
                        log.info("Contacting {}, partition {}, sequence {}", topic, partition, n);
                    }
                    ps.publish(topic, partition, event, INIT);
                    task.clear();
                    String handle = po.sendLater(new EventEnvelope().setTo(INIT_HANDLER).setBody(INIT),
                                        new Date(System.currentTimeMillis() + INTERVAL));
                    task.add(handle);
                } catch (IOException e) {
                    log.error("Unable to send initToken to consumer - {}", e.getMessage());
                }
            } else {
                if (!task.isEmpty()) {
                    po.cancelFutureEvent(task.get(0));
                }
                platform.release(INIT_HANDLER);
                log.info("{}, partition {} ready", topic, partition);
            }
            return true;
        };
        try {
            platform.registerPrivate(INIT_HANDLER, f, 1);
            po.sendLater(new EventEnvelope().setTo(INIT_HANDLER).setBody(INIT),
                    new Date(System.currentTimeMillis() + 1000));
        } catch (IOException e) {
            log.error("Unable to register {} - {}", INIT_HANDLER, e.getMessage());
        }
    }

    public static void setResetHandler(LambdaFunction resetHandler) {
        ServiceLifeCycle.resetHandler = resetHandler;
    }

    public static void releaseConnecton(String reason) {
        if (resetHandler != null) {
            try {
                resetHandler.handleEvent(new HashMap<>(), reason, 1);
            } catch (Exception e) {
                log.error("Unable to release connection - {}", e.getMessage());
            }
        }
    }

}
