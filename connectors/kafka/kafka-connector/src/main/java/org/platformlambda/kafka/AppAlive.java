package org.platformlambda.kafka;

import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;

public class AppAlive extends Thread {
    private static final Logger log = LoggerFactory.getLogger(AppAlive.class);

    private static final String TYPE = "type";
    private static final String ORIGIN = "origin";
    private static final String NAME = "name";
    private static final String TIMESTAMP = "timestamp";
    private static final String APP_ALIVE = "app_alive";
    private static final String PRESENCE_HOUSEKEEPER = "presence.housekeeper";
    private static final String TO = "to";
    private static final long INTERVAL = 30 * 1000;
    private static boolean normal = true;

    @Override
    public void run() {
        log.info("Started");
        Runtime.getRuntime().addShutdownHook(new Thread(this::shutdown));

        Platform platform = Platform.getInstance();
        Utility util = Utility.getInstance();
        String origin = platform.getOrigin();
        PostOffice po = PostOffice.getInstance();
        // the first message will be sent in 10 seconds
        long t0 = System.currentTimeMillis() - 10000;
        while (normal) {
            if (ConsumerLifeCycle.isReady()) {
                long now = System.currentTimeMillis();
                if (now - t0 > INTERVAL) {
                    t0 = now;
                    EventEnvelope event = new EventEnvelope();
                    event.setTo(PRESENCE_HOUSEKEEPER);
                    event.setHeader(ORIGIN, origin);
                    event.setHeader(TYPE, APP_ALIVE);
                    event.setHeader(NAME, Platform.getInstance().getName());
                    event.setHeader(TIMESTAMP, util.getTimestamp());
                    try {
                        po.send(PostOffice.CLOUD_CONNECTOR, event.toBytes(), new Kv(TO, "*"));
                    } catch (IOException e) {
                        log.error("Unable to send keep-alive to other presence monitors - {}", e.getMessage());
                    }

                }
            }
            try {
                Thread.sleep(1000);
            } catch (InterruptedException e) {
                // yield to the operating system
            }
        }
        log.info("Stopped");
    }

    private void shutdown() {
        normal = false;
    }
}
