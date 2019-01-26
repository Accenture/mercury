/*

    Copyright 2018-2019 Accenture Technology

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

package org.platformlambda.node.models;

public class ConnectionStatus {

    public boolean authenticated = false;
    public long connectTime, authTime;
    public String route, txPath;

    public ConnectionStatus(String route, String txPath) {
        this.route = route;
        this.txPath = txPath;
        this.connectTime = System.currentTimeMillis();
    }

    public void accept() {
        this.authenticated = true;
        this.authTime = System.currentTimeMillis();
    }

}
