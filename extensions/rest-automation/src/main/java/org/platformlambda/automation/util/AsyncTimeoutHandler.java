package org.platformlambda.automation.util;

import org.platformlambda.automation.models.AsyncContextHolder;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.servlet.ServletResponse;
import javax.servlet.http.HttpServletResponse;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ConcurrentMap;

public class AsyncTimeoutHandler extends Thread {
    private static final Logger log = LoggerFactory.getLogger(AsyncTimeoutHandler.class);

    private static ConcurrentMap<String, AsyncContextHolder> contexts;
    private boolean normal = true;

    public AsyncTimeoutHandler(ConcurrentMap<String, AsyncContextHolder> contexts) {
        this.contexts = contexts;
        Runtime.getRuntime().addShutdownHook(new Thread(this::shutdown));
    }

    @Override
    public void run() {
        log.info("Async HTTP timeout handler started");
        while (normal) {
            try {
                Thread.sleep(2000);
            } catch (InterruptedException e) {
                // ok to ignore
            }
            // check async context timeout
            if (!contexts.isEmpty()) {
                List<String> contextList = new ArrayList<>(contexts.keySet());
                long now = System.currentTimeMillis();
                for (String id : contextList) {
                    AsyncContextHolder holder = contexts.get(id);
                    long t1 = holder.lastAccess;
                    if (now - t1 > holder.timeout) {
                        ServletResponse res = holder.context.getResponse();
                        if (res instanceof HttpServletResponse) {
                            contexts.remove(id);
                            log.warn("Async HTTP Context {} timeout for {} ms", id, now - t1);
                            HttpServletResponse response = (HttpServletResponse) res;
                            try {
                                response.sendError(408, "Timeout for " + (holder.timeout / 1000) + " seconds");
                                holder.context.complete();
                            } catch (IOException e) {
                                log.error("Unable to send timeout exception to async context {}", id);
                            }
                        }
                    }
                }
            }
        }
        log.info("Async HTTP timeout handler stopped");
    }

    private void shutdown() {
        normal = false;
    }
}

