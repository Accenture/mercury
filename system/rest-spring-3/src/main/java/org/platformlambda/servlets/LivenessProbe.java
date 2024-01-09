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

import java.io.IOException;
import java.io.Serial;

@WebServlet(urlPatterns={"/livenessprobe"}, asyncSupported=true)
public class LivenessProbe extends ServletBase {
    @Serial
    private static final long serialVersionUID = 3607030982796747671L;
    private static final String LIVENESSPROBE = "livenessprobe";

    @Override
    protected void doGet(HttpServletRequest request, HttpServletResponse response) throws IOException {
        submit(LIVENESSPROBE, request, response);
    }

}
