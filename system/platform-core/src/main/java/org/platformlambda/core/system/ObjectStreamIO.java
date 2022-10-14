/*

    Copyright 2018-2022 Accenture Technology

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

package org.platformlambda.core.system;

import org.platformlambda.core.annotations.EventInterceptor;
import org.platformlambda.core.annotations.ZeroTracing;
import org.platformlambda.core.models.*;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.*;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentLinkedQueue;
import java.util.concurrent.ConcurrentMap;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.atomic.AtomicInteger;

public class ObjectStreamIO {
    private static final Logger log = LoggerFactory.getLogger(ObjectStreamIO.class);

    private static final ConcurrentMap<String, StreamInfo> streams = new ConcurrentHashMap<>();
    private static final AtomicInteger counter = new AtomicInteger(0);

    public static final int DEFAULT_TIMEOUT = 1800;

    private static final String TYPE = "type";
    private static final String READ = "read";
    private static final String CLOSE = "close";
    private static final String DATA = "data";
    private static final String END_OF_STREAM = "eof";
    private static final String STREAM_PREFIX = "stream.";
    private static final String IN = ".in";
    private static final String OUT = ".out";
    private static boolean loaded = false;
    private String inputStreamId;
    private String outputStreamId;
    private String streamRoute;
    private final int expirySeconds;
    private final AtomicBoolean eof = new AtomicBoolean(false);
    private final ConcurrentLinkedQueue<String> callbacks = new ConcurrentLinkedQueue<>();

    public ObjectStreamIO() throws IOException {
        this.expirySeconds = DEFAULT_TIMEOUT;
        this.createStream();
    }

    public ObjectStreamIO(int expirySeconds) throws IOException {
        this.expirySeconds = Math.max(1, expirySeconds);
        this.createStream();
    }

    public int getExpirySeconds() {
        return expirySeconds;
    }

    private void createStream() throws IOException {
        Utility util = Utility.getInstance();
        Platform platform = Platform.getInstance();
        if (counter.incrementAndGet() == 1 && !loaded) {
            loaded = true;
            HouseKeeper houseKeeper = new HouseKeeper();
            houseKeeper.start();
        }
        String id = util.getUuid();
        String in = STREAM_PREFIX+id+IN;
        String out = STREAM_PREFIX+id+OUT;
        this.inputStreamId = in + "@" + platform.getOrigin();
        this.outputStreamId = out + "@" + platform.getOrigin();
        StreamPublisher publisher = new StreamPublisher();
        StreamConsumer consumer = new StreamConsumer(publisher, in, out);
        platform.registerPrivateStream(out, publisher);
        platform.registerPrivate(in, consumer, 1);
        streams.put(in, new StreamInfo(expirySeconds));
        log.info("Stream {} created with expiry of {} seconds", id, expirySeconds);
    }

    public String getInputStreamId() {
        return inputStreamId;
    }

    public String getOutputStreamId() {
        return outputStreamId;
    }

    public static Map<String, Object> getStreamInfo() {
        Utility util = Utility.getInstance();
        Map<String, Object> result = new HashMap<>();
        for (String id: streams.keySet()) {
            StreamInfo info = streams.get(id);
            Map<String, Object> metadata = new HashMap<>();
            metadata.put("created", util.date2str(new Date(info.created)));
            metadata.put("last_read", util.date2str(new Date(info.updated)));
            metadata.put("expiry_seconds", info.expiryMills / 1000);
            result.put(id, metadata);
        }
        result.put("count", streams.size());
        return result;
    }

    public static void touch(String id) {
        StreamInfo info = streams.get(id);
        if (info != null) {
            info.updated = System.currentTimeMillis();
        }
    }

    public static void removeExpiredStreams() {
        PostOffice po = PostOffice.getInstance();
        Utility util = Utility.getInstance();
        long now = System.currentTimeMillis();
        List<String> list = new ArrayList<>(streams.keySet());
        for (String id : list) {
            StreamInfo info = streams.get(id);
            if (now - info.updated > info.expiryMills) {
                try {
                    String createdTime = util.date2str(new Date(info.created));
                    String updatedTime = util.date2str(new Date(info.updated));
                    log.warn("{} expired. Inactivity for {} seconds ({} - {})", id, info.expiryMills / 1000,
                            createdTime, updatedTime);
                    po.send(id, new Kv(TYPE, CLOSE));
                } catch (IOException e) {
                    log.error("Unable to remove expired {} - {}", id, e.getMessage());
                } finally {
                    streams.remove(id);
                }
            }
        }
    }

    private class StreamPublisher implements StreamFunction {

        @Override
        public void init(String manager) {
            streamRoute = manager;
        }

        @Override
        public String getManager() {
            return streamRoute;
        }

        @Override
        public void handleEvent(Map<String, String> headers, Object body) throws Exception {
            if (DATA.equals(headers.get(TYPE))) {
                if (!eof.get()) {
                    String cb = callbacks.poll();
                    if (cb != null) {
                        sendReply(cb, body, DATA);
                    }
                }
            } else if (END_OF_STREAM.equals(headers.get(TYPE))) {
                if (!eof.get()) {
                    eof.set(true);
                    String cb = callbacks.poll();
                    if (cb != null) {
                        sendReply(cb, body, END_OF_STREAM);
                    }
                }
            }
        }

        private void sendReply(String cb, Object body, String type) throws IOException {
            PostOffice po = PostOffice.getInstance();
            if (cb.contains("|")) {
                int sep = cb.indexOf('|');
                String callback = cb.substring(0, sep);
                String extra = cb.substring(sep+1);
                EventEnvelope eventExtra = new EventEnvelope();
                eventExtra.setTo(callback).setExtra(extra).setHeader(TYPE, type).setBody(body);
                po.send(eventExtra);
            } else {
                po.send(cb, body, new Kv(TYPE, type));
            }
        }
    }

    @EventInterceptor
    @ZeroTracing
    private class StreamConsumer implements LambdaFunction {
        private final StreamPublisher publisher;
        private final String in;
        private final String out;

        public StreamConsumer(StreamPublisher publisher, String in, String out) {
            this.publisher = publisher;
            this.in = in;
            this.out = out;
        }

        @Override
        public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
            Platform platform = Platform.getInstance();
            PostOffice po = PostOffice.getInstance();
            EventEnvelope event = (EventEnvelope) body;
            String type = event.getHeaders().get(TYPE);
            String cb = event.getReplyTo();
            String extra = event.getExtra();
            if (READ.equals(type) && cb != null) {
                if (extra != null) {
                    callbacks.offer(cb + "|" + extra);
                } else {
                    callbacks.offer(cb);
                }
                publisher.get();
                touch(in);
            }
            if (CLOSE.equals(type)) {
                platform.release(in);
                platform.release(out);
                streams.remove(in);
                if (cb != null) {
                    if (extra != null) {
                        EventEnvelope eventExtra = new EventEnvelope();
                        eventExtra.setTo(cb).setExtra(extra).setBody(true);
                        po.send(eventExtra);
                    } else {
                        po.send(cb, true);
                    }
                }
            }
            return null;
        }
    }

    private static class HouseKeeper extends Thread {

        private static final long INTERVAL = 10000;
        private boolean normal = true;

        @Override
        public void run() {
            Runtime.getRuntime().addShutdownHook(new Thread(this::shutdown));
            log.info("Started");
            long t1 = System.currentTimeMillis() - INTERVAL;
            while (normal) {
                long now = System.currentTimeMillis();
                // scan every 10 seconds
                if (now - t1 > INTERVAL) {
                    t1 = now;
                    removeExpiredStreams();
                }
                try {
                    Thread.sleep(1000);
                } catch (InterruptedException e) {
                    // ok to ignore
                }
            }
            log.info("Stopped");
        }

        private void shutdown() {
            normal = false;
        }
    }

}
