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

package org.platformlambda.models;

import io.vertx.core.http.HttpServerRequest;

public class AsyncContextHolder {

    public HttpServerRequest request;
    public long timeout;
    public long lastAccess;
    public String method, accept;

    public AsyncContextHolder(HttpServerRequest request) {
        this.request = request;
        this.timeout = 30 * 1000;
        this.method = request.method().name();
        this.accept = request.getHeader("accept");
        this.touch();
    }

    public void touch() {
        this.lastAccess = System.currentTimeMillis();
    }

}
