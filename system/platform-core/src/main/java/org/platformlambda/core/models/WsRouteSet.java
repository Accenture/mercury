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

package org.platformlambda.core.models;

import org.platformlambda.core.util.CryptoApi;

import java.util.concurrent.atomic.AtomicInteger;

public class WsRouteSet {

    private static final AtomicInteger counter = new AtomicInteger(0);
    private static final CryptoApi crypto = new CryptoApi();

    private String route, txPath;

    public WsRouteSet(String path) {
        // generate a 5-digit random number as identifier
        int r = crypto.nextInt(10000, 100000);
        // ensure uniqueness using a monotonically increasing sequence number
        int n = counter.incrementAndGet();
        this.route = path + ".in." + r +"." + n;
        this.txPath = path + ".out." + r +"." + n;
    }

    public String getRoute() {
        return route;
    }

    public String getTxPath() {
        return txPath;
    }

}
