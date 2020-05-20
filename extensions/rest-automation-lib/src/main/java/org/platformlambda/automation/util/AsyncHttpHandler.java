/*

    Copyright 2018-2020 Accenture Technology

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

package org.platformlambda.automation.util;

import org.platformlambda.automation.models.AsyncContextHolder;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.servlet.AsyncEvent;
import javax.servlet.AsyncListener;
import java.util.concurrent.ConcurrentMap;

public class AsyncHttpHandler implements AsyncListener {
    private static final Logger log = LoggerFactory.getLogger(AsyncHttpHandler.class);

    private ConcurrentMap<String, AsyncContextHolder> contexts;
    private String id;

    public AsyncHttpHandler(ConcurrentMap<String, AsyncContextHolder> contexts, String id) {
        this.contexts = contexts;
        this.id = id;
    }

    @Override
    public void onComplete(AsyncEvent event) {
        if (contexts.containsKey(id)) {
            contexts.remove(id);
            log.debug("Async HTTP Context {} completed, remaining {}", id, contexts.size());
        }
    }

    @Override
    public void onTimeout(AsyncEvent event) {
        // this should not occur as we use our own async timeout handler
        contexts.remove(id);
    }

    @Override
    public void onError(AsyncEvent event) {
        if (contexts.containsKey(id)) {
            contexts.remove(id);
            log.warn("Async HTTP Context {} exception {}", id, event.getThrowable().getMessage());
        }
    }

    @Override
    public void onStartAsync(AsyncEvent event) {
        // no-op
    }
}
