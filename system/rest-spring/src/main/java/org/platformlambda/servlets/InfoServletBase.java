/*

    Copyright 2018-2021 Accenture Technology

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

package org.platformlambda.servlets;

import org.platformlambda.core.util.AppConfigReader;

import javax.servlet.http.HttpServlet;
import javax.servlet.http.HttpServletRequest;

public abstract class InfoServletBase extends HttpServlet {

    protected static final AppConfigReader config = AppConfigReader.getInstance();
    protected static final boolean protectEndpoint = "true".equals(
            config.getProperty("protect.info.endpoints", "false"));

    protected boolean isLocalHost(HttpServletRequest request) {
        String host = request.getHeader("host");
        return host != null && host.contains("127.0.0.1");
    }

}
