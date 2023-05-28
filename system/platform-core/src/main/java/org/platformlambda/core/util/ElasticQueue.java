/*

    Copyright 2018-2023 Accenture Technology

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.

 */

package org.platformlambda.core.util;

import com.sleepycat.je.*;
import org.platformlambda.core.annotations.ZeroTracing;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.EventEmitter;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.File;
import java.io.IOException;
import java.nio.file.Files;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ConcurrentLinkedQueue;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.concurrent.locks.ReentrantLock;

public class ElasticQueue implements AutoCloseable {
    private static final Logger log = LoggerFactory.getLogger(ElasticQueue.class);

    private static final Utility util = Utility.getInstance();
    private static final AtomicInteger generation = new AtomicInteger(0);
    private static final ReentrantLock lock = new ReentrantLock();
    private static final AtomicInteger initCounter = new AtomicInteger(0);
    private static final AtomicBoolean housekeeperNotRunning = new AtomicBoolean(true);
    private static final AtomicBoolean keepAliveNotRunning = new AtomicBoolean(true);
    private static final long ONE_SECOND = 1000L;
    private static final long ONE_MINUTE = 60 * ONE_SECOND;
    private static final long ONE_HOUR = 60 * ONE_MINUTE;
    private static final long ONE_DAY = 24 * ONE_HOUR;
    private static final long KEEP_ALIVE_INTERVAL = 20 * ONE_SECOND;
    private static final long HOUSEKEEPING_INTERVAL = 10 * ONE_MINUTE;
    public static final int MEMORY_BUFFER = 20;
    private static final String RUNNING = "RUNNING";
    private static final String CLEAN_UP_TASK = "elastic.queue.cleanup";
    private static final String SLASH = "/";
    private static final int MAX_EVENTS = 100000000;
    private static Database db;
    private static Environment dbEnv;
    private static File dbFolder;
    private static boolean dbLoaded = false;
    private static boolean runningInCloud;
    private long readCounter;
    private long writeCounter;
    private boolean empty = false;
    private byte[] peeked = null;
    private int currentVersion = generation.get();
    private final String id;
    private final ConcurrentLinkedQueue<byte[]> memory = new ConcurrentLinkedQueue<>();

    /**
     * Two-stage elastic queue using memory and disk
     *
     * @param id service route path
     */
    public ElasticQueue(String id) {
        this.id = util.validServiceName(id)? id : util.filteredServiceName(id);
        resetCounter();
        if (initCounter.incrementAndGet() == 1) {
            Platform platform = Platform.getInstance();
            try {
                platform.registerPrivate(CLEAN_UP_TASK, new Cleanup(), 1);
            } catch (IOException e) {
                log.error("Unable to register {} - {}", CLEAN_UP_TASK, e.getMessage());
            }
            Runtime.getRuntime().addShutdownHook(new Thread(ElasticQueue::shutdown));
            AppConfigReader config = AppConfigReader.getInstance();
            runningInCloud = "true".equals(config.getProperty("running.in.cloud", "false"));
            File tmpRoot = new File(config.getProperty("transient.data.store", "/tmp/reactive"));
            if (runningInCloud) {
                dbFolder = tmpRoot;
            } else {
                String instanceId = platform.getName() + "-" + platform.getOrigin();
                dbFolder = new File(tmpRoot, instanceId);
            }
            if (!dbFolder.exists() && dbFolder.mkdirs()) {
                log.info("{} created", dbFolder);
            }
            // save a signature file first
            util.str2file(new File(dbFolder, RUNNING), util.getTimestamp());
            /*
             * Normally the system should initialize commit log before using the elastic queue.
             */
            boolean deferred = "true".equals(config.getProperty("deferred.commit.log", "false"));
            if (!deferred) {
                getDatabase();
                log.info("Commit log started");
            }
            scanExpiredStores(tmpRoot);
            platform.getVertx().setPeriodic(KEEP_ALIVE_INTERVAL, t -> keepAlive());
            platform.getVertx().setPeriodic(HOUSEKEEPING_INTERVAL, t -> housekeeping());
            log.info("Housekeeper started");
        }
        if (initCounter.get() > 10000) {
            initCounter.set(10);
        }
    }

    private void keepAlive() {
        if (keepAliveNotRunning.compareAndSet(true, false)) {
            Platform.getInstance().getEventExecutor().submit(() -> {
                try {
                    util.str2file(new File(dbFolder, RUNNING), util.getTimestamp());
                } finally {
                    keepAliveNotRunning.set(true);
                }
            });
        }
    }

    private void housekeeping() {
        if (housekeeperNotRunning.compareAndSet(true, false)) {
            Platform.getInstance().getEventExecutor().submit(() -> {
                try {
                    removeExpiredDbStatistics();
                } finally {
                    housekeeperNotRunning.set(true);
                }
            });
        }
    }

    private void removeExpiredDbStatistics() {
        long now = System.currentTimeMillis();
        List<File> outdated = new ArrayList<>();
        File[] files = dbFolder.listFiles();
        if (files != null) {
            for (File f : files) {
                String name = f.getName();
                if (name.startsWith("je.stat.") && name.endsWith(".csv")
                        && !name.equals("je.stat.csv") && now - f.lastModified() > ONE_DAY) {
                    outdated.add(f);
                }
            }
            for (File f : outdated) {
                try {
                    Files.deleteIfExists(f.toPath());
                    log.info("Outdated {} deleted", f);
                } catch (IOException e) {
                    log.error("Unable to delete outdated file {} - {}", f, e.getMessage());
                }
            }
        }
    }

    public String getId() {
        return id;
    }

    public long getReadCounter() {
        return readCounter;
    }

    public long getWriteCounter() {
        return writeCounter;
    }

    @Override
    public void close() {
        if (!isClosed()) {
            if (dbEnv != null && !dbEnv.isClosed()) {
                if (readCounter < writeCounter && writeCounter > MEMORY_BUFFER) {
                    try {
                        EventEmitter.getInstance().send(CLEAN_UP_TASK, id + SLASH + currentVersion);
                    } catch (IOException e) {
                        log.error("Unable to run {} - {}", CLEAN_UP_TASK, e.getMessage());
                    }
                } else {
                    dbEnv.cleanLog();
                }
            }
            resetCounter();
        }
    }

    /**
     * This method may be called when the route supported by this elastic queue is no longer in service
     */
    public void destroy() {
        close();
        if (dbEnv != null) {
            // perform final clean up
            try {
                EventEmitter.getInstance().send(CLEAN_UP_TASK, id);
            } catch (IOException e) {
                log.error("Unable to run {} - {}", CLEAN_UP_TASK, e.getMessage());
            }
        }
    }

    public boolean isClosed() {
        return writeCounter == 0;
    }

    private static void shutdown() {
        if (dbLoaded) {
            dbLoaded = false;
            try {
                db.close();
            } catch (Exception e) {
                log.debug("Exception while closing - {}", e.getMessage());
            }
            try {
                dbEnv.close();
            } catch (Exception e) {
                log.debug("Exception while closing - {}", e.getMessage());
            }
            if (dbFolder.exists()) {
                util.cleanupDir(dbFolder, runningInCloud);
                log.info("Holding area {} cleared", dbFolder);
            }
        }
    }

    private void resetCounter() {
        if (!empty) {
            empty = true;
            readCounter = writeCounter = 0;
            memory.clear();
            currentVersion = generation.incrementAndGet();
        }
    }

    private static void setupCommitLog(File dir) {
        try {
            long t1 = System.currentTimeMillis();
            dbEnv = new Environment(dir,
                    new EnvironmentConfig()
                            .setAllowCreate(true)
                            .setConfigParam(EnvironmentConfig.MAX_DISK, "0")
                            .setConfigParam(EnvironmentConfig.FREE_DISK, "0"));
            dbEnv.checkpoint(new CheckpointConfig().setMinutes(1));
            db = dbEnv.openDatabase(null, "kv",
                    new DatabaseConfig().setAllowCreate(true).setTemporary(false));
            dbLoaded = true;
            long diff = System.currentTimeMillis() - t1;
            log.info("Created holding area {} in {} ms", dir, diff);

        } catch (Exception e) {
            log.error("Unable to create holding area in {} - {}", dir, e.getMessage());
            System.exit(-1);
        }
    }

    private static Database getDatabase() {
        if (db == null) {
            lock.lock();
            try {
                if (dbEnv == null) {
                    setupCommitLog(dbFolder);
                }
            } finally {
                lock.unlock();
            }
        }
        return db;
    }

    public void write(byte[] event) {
        if (writeCounter < MEMORY_BUFFER) {
            // for highest performance, save to memory for the first few blocks
            memory.offer(event);
        } else {
            // otherwise, save to disk
            String key = id + SLASH + currentVersion + SLASH + util.zeroFill(writeCounter, MAX_EVENTS);
            DatabaseEntry k = new DatabaseEntry(util.getUTF(key));
            DatabaseEntry v = new DatabaseEntry(event);
            getDatabase().put(null, k, v);
        }
        writeCounter++;
        empty = false;
    }

    public byte[] peek() {
        if (peeked != null) {
            return peeked;
        }
        peeked = read();
        return peeked;
    }

    public byte[] read() {
        if (peeked != null) {
            byte[] result = peeked;
            peeked = null;
            return result;
        }
        if (readCounter >= writeCounter) {
            // catch up with writes and thus nothing to read
            close();
            return null;
        }
        if (readCounter < MEMORY_BUFFER) {
            byte[] event = memory.poll();
            if (event != null) {
                readCounter++;
            }
            return event;
        }
        boolean hasRecord = false;
        String key = id + SLASH + currentVersion + SLASH + util.zeroFill(readCounter, MAX_EVENTS);
        DatabaseEntry k = new DatabaseEntry(util.getUTF(key));
        DatabaseEntry v = new DatabaseEntry();
        try {
            OperationStatus status = getDatabase().get(null, k, v, LockMode.DEFAULT);
            if (status == OperationStatus.SUCCESS) {
                // must be an exact match
                String ks = util.getUTF(k.getData());
                if (ks.equals(key)) {
                    hasRecord = true;
                    readCounter++;
                    return v.getData();
                } else {
                    log.error("Expected {}, Actual: {}", key, ks);
                }
            }
            return null;
        } finally {
            if (hasRecord) {
                db.delete(null, k);
            }
        }
    }

    private void scanExpiredStores(File tmpRoot) {
        if (runningInCloud) {
            removeExpiredStore(tmpRoot);
        } else {
            File[] dirs = tmpRoot.listFiles();
            if (dirs != null) {
                for (File d : dirs) {
                    if (d.isDirectory()) {
                        removeExpiredStore(d);
                    }
                }
            }
        }
    }

    private void removeExpiredStore(File folder) {
        File f = new File(folder, RUNNING);
        if (f.exists()) {
            if (System.currentTimeMillis() - f.lastModified() > ONE_HOUR) {
                util.cleanupDir(folder, runningInCloud);
                log.info("Holding area {} expired", folder);
            }
        } else {
            util.cleanupDir(folder, runningInCloud);
            log.warn("Unknown holding area {} removed", folder);
        }
    }

    @ZeroTracing
    private static class Cleanup implements LambdaFunction {

        @Override
        public Object handleEvent(Map<String, String> headers, Object input, int instance) {
            if (input instanceof String && db != null && dbEnv != null) {
                Utility util = Utility.getInstance();
                int n = 0;
                String prefix = input + SLASH;
                DatabaseEntry k = new DatabaseEntry(util.getUTF(prefix));
                DatabaseEntry v = new DatabaseEntry();
                try (Cursor cursor = db.openCursor(null, new CursorConfig())) {
                    OperationStatus status = cursor.getSearchKeyRange(k, v, LockMode.DEFAULT);
                    while (status == OperationStatus.SUCCESS) {
                        String ks = util.getUTF(k.getData());
                        if (!ks.startsWith(prefix)) {
                            break;
                        }
                        db.delete(null, k);
                        n++;
                        status = cursor.getNext(k, v, LockMode.DEFAULT);
                    }
                    if (n > 0) {
                        dbEnv.cleanLog();
                        log.info("Cleared {} unread event{} for {}", n, n == 1? "" : "s", input);
                    }
                } catch (Exception e) {
                    log.debug("Unable to scan {} - {}", input, e.getMessage());
                }
            }
            return true;
        }
    }

}
