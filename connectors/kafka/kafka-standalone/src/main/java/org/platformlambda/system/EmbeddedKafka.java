package org.platformlambda.system;

import kafka.server.KafkaConfig;
import kafka.server.KafkaServerStartable;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.File;
import java.io.IOException;
import java.io.InputStream;
import java.util.Properties;

public class EmbeddedKafka extends Thread {
    private static final Logger log = LoggerFactory.getLogger(EmbeddedKafka.class);

    private KafkaServerStartable kafka;
    private EmbeddedZk zookeeper;

    public EmbeddedKafka(EmbeddedZk zookeeper) {
        this.zookeeper = zookeeper;
    }

    @Override
    public void run() {

        try (InputStream stream = EmbeddedKafka.class.getResourceAsStream("/kafka.properties")) {
            if (stream == null) {
                throw new IOException("kafka.properties is not available as resource");
            }
            Properties p = new Properties();
            p.load(stream);

            String dir = p.getProperty("log.dirs");
            if (dir != null) {
                File reset = new File(dir);
                if (reset.exists() && reset.isDirectory()) {
                    Utility.getInstance().cleanupDir(reset);
                    log.info("Clean up transient Kafka working directory at {}", dir);
                }
            }
            kafka = new KafkaServerStartable(new KafkaConfig(p));
            kafka.startup();
            Runtime.getRuntime().addShutdownHook(new Thread(this::shutdown));

        } catch (IOException e) {
            log.error("Unable to start Kafka kafka - {}", e.getMessage());
            System.exit(-1);
        }
    }

    private void shutdown() {
        // orderly shutdown of kafka and zookeeper
        log.info("Shutting down");
        kafka.shutdown();
        zookeeper.shutdown();
    }



}
