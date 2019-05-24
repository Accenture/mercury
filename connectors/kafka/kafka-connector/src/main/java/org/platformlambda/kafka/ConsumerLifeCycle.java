package org.platformlambda.kafka;

import org.apache.kafka.clients.consumer.ConsumerRebalanceListener;
import org.apache.kafka.common.TopicPartition;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.AppConfigReader;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.Collection;

public class ConsumerLifeCycle implements ConsumerRebalanceListener {
    private static final Logger log = LoggerFactory.getLogger(ConsumerLifeCycle.class);

    private static final String TYPE = "type";
    private static final String INIT = "init";
    private static boolean ready = false;
    private boolean serviceMonitor;


    public ConsumerLifeCycle() {
        AppConfigReader reader = AppConfigReader.getInstance();
        serviceMonitor = "true".equals(reader.getProperty("service.monitor", "false"));
    }

    public static boolean isReady() {
        return ready;
    }

    @Override
    public void onPartitionsRevoked(Collection<TopicPartition> collection) {
        ready = false;
        log.info("Standing by");
    }

    @Override
    public void onPartitionsAssigned(Collection<TopicPartition> collection) {
        try {
            // sending round-trip message to tell the event consumer that system is ready
            if (!serviceMonitor) {
                PostOffice.getInstance().send(PostOffice.CLOUD_CONNECTOR, new Kv(TYPE, INIT));
            }
            ready = true;
            log.info("Ready");
        } catch (IOException e) {
            log.error("Unable to send initialization request to {} - {}", PostOffice.CLOUD_CONNECTOR, e.getMessage());
        }
    }

}