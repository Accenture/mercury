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

package org.platformlambda.automation.models;

import io.vertx.core.http.HttpServerRequest;

public class AsyncContextHolder {

    public HttpServerRequest request;
    public long timeout;
    public long lastAccess;
    public String url, resHeaderId, accept, method;

    public AsyncContextHolder(HttpServerRequest request) {
        this.request = request;
        this.timeout = 30 * 1000;
        this.touch();
    }

    public AsyncContextHolder(HttpServerRequest request, long timeout) {
        this.request = request;
        this.timeout = timeout;
        this.touch();
    }

    public AsyncContextHolder setTimeout(long timeout) {
        this.timeout = timeout;
        return this;
    }

    public AsyncContextHolder setUrl(String url) {
        this.url = url;
        return this;
    }

    public AsyncContextHolder setResHeaderId(String resHeaderId) {
        this.resHeaderId = resHeaderId;
        return this;
    }

    public AsyncContextHolder setMethod(String method) {
        this.method = method;
        return this;
    }

    public AsyncContextHolder setAccept(String accept) {
        this.accept = accept;
        return this;
    }

    public void touch() {
        this.lastAccess = System.currentTimeMillis();
    }

}
