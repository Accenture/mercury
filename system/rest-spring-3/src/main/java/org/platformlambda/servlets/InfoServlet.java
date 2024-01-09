/*

    Copyright 2018-2024 Accenture Technology

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

import jakarta.servlet.annotation.WebServlet;
import jakarta.servlet.http.HttpServletRequest;
import jakarta.servlet.http.HttpServletResponse;
import org.platformlambda.core.util.Utility;

import java.io.IOException;
import java.io.Serial;
import java.util.List;

@WebServlet(urlPatterns={"/info/*"}, asyncSupported=true)
public class InfoServlet extends ServletBase {
	@Serial
    private static final long serialVersionUID = 376901501172978505L;

    private static final String INFO = "info";
    private static final String ROUTES = "routes";
    private static final String LIB = "lib";

    @Override
    protected void doGet(HttpServletRequest request, HttpServletResponse response) throws IOException {
        final String type;
        List<String> paths = Utility.getInstance().split(request.getPathInfo(), "/");
        if (paths.size() == 1 && LIB.equals(paths.get(0))) {
            type = LIB;
        } else if (paths.size() == 1 && ROUTES.equals(paths.get(0))) {
            type = ROUTES;
        } else {
            type = INFO;
        }
        submit(type, request, response);
    }

}