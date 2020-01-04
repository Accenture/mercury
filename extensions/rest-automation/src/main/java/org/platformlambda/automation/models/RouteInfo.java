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

package org.platformlambda.automation.models;

import java.util.List;

public class RouteInfo {

    public String url, service, authService, corsId, requestTransformId, responseTransformId;
    public int threshold = 50000;
    public boolean tracing = false;
    public List<String> methods;
    public int timeoutSeconds = 30;
    // optional for file upload
    public String upload = "file";
    // optional for HTTP relay
    public String host;
    public boolean trustAllCert = false;
    public List<String> urlRewrite;

}
