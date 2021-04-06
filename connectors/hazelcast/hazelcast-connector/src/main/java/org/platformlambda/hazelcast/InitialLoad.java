package org.platformlambda.hazelcast;

import org.platformlambda.core.system.PubSub;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.HashMap;
import java.util.Map;

public class InitialLoad extends Thread {
    private static final Logger log = LoggerFactory.getLogger(InitialLoad.class);

    public static final String TYPE = "type";
    public static final String INIT = "init";
    public static final String TOKEN = "token";
    public static final long INITIALIZE = -100;
    private static final String SEQUENCE = "seq";
    private static final long INTERVAL = 5000;
    private final String topic;
    private final String token;
    private final int partition;
    private int seq = 0;
    private boolean initializing = true;

    /**
     * When offset is set to the special value INITIALIZE, initial load
     * will send an initialization token to the EventConsumer to make sure
     * the consumer is ready to read new events.
     *
     * @param topic that the consumer uses
     * @param partition for the topic
     * @param token of random value
     */
    public InitialLoad(String topic, int partition, String token) {
        this.topic = topic;
        this.partition = partition;
        this.token = token;
    }

    @Override
    public void run() {
        PubSub ps = PubSub.getInstance();
        Runtime.getRuntime().addShutdownHook(new Thread(this::close));

        long t0 = 0;
        while (initializing) {
            long now = System.currentTimeMillis();
            if (now - t0 > INTERVAL) {
                t0 = now;
                try {
                    seq++;
                    Map<String, String> headers = new HashMap<>();
                    headers.put(TYPE, INIT);
                    headers.put(TOKEN, token);
                    headers.put(SEQUENCE, String.valueOf(seq));
                    log.info("Contacting {}, partition {}, sequence {}", topic, partition, seq);
                    ps.publish(topic, partition, headers, INIT);
                } catch (IOException e) {
                    log.error("Unable to send initToken to consumer - {}", e.getMessage());
                }
            }
            try {
                Thread.sleep(1000);
            } catch (InterruptedException e) {
                // ok to ignore
            }
        }
        log.info("{}, partition {} ready", topic, partition);
    }

    public void close() {
        initializing = false;
    }

}
