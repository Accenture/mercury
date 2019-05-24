package org.platformlambda.system;

import org.apache.zookeeper.server.NIOServerCnxnFactory;
import org.apache.zookeeper.server.ServerCnxnFactory;
import org.apache.zookeeper.server.ZooKeeperServer;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.File;
import java.io.IOException;

public class EmbeddedZk extends Thread {
    private static final Logger log = LoggerFactory.getLogger(EmbeddedZk.class);

    private ServerCnxnFactory factory;

    @Override
    public void run() {

        Utility util = Utility.getInstance();
        AppConfigReader reader = AppConfigReader.getInstance();
        String zkDir = reader.getProperty("zk.dir", "/tmp/zk");
        int tickTime = util.str2int(reader.getProperty("zk.tick", "2000"));
        if (tickTime < 1000) {
            log.info("zk.tick is too small. Reset to 1000 ms");
            tickTime = 1000;
        }
        File baseDir = new File(zkDir);
        if (baseDir.exists()) {
            // this guarantees that a standalone zookeeper will start with a clean state
            util.cleanupDir(baseDir);
            log.info("Clean up transient Zookeeper working directory at {}", baseDir);
        }
        File snapshotDir = new File(baseDir, "snapshots");
        File logDir = new File(baseDir, "log");
        try {
            this.factory = NIOServerCnxnFactory.createFactory(2181, 512);
            factory.startup(new ZooKeeperServer(snapshotDir, logDir, tickTime));
        } catch (IOException | InterruptedException e) {
            log.error("Unable to start Zookeeper - {}", e.getMessage());
            System.exit(-1);
        }

    }

    public void shutdown() {
        log.info("Shutting down");
        factory.shutdown();
    }


}
