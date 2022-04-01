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

package org.platformlambda.core.models;

import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;

public abstract class InboxBase {

    protected static final ExecutorService executor = Executors.newCachedThreadPool();
    protected static final String ASYNC_INBOX = "async.inbox";

    protected String id;

    protected static final ConcurrentMap<String, InboxBase> inboxes = new ConcurrentHashMap<>();

    public static InboxBase getHolder(String inboxId) {
        return inboxes.get(inboxId);
    }

    public String getId() {
        return id;
    }

}
