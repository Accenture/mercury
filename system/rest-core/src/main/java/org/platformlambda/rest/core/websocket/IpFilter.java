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

package org.platformlambda.rest.core.websocket;

import javax.servlet.*;
import javax.servlet.annotation.WebFilter;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletRequestWrapper;
import javax.servlet.http.HttpServletResponse;
import java.io.IOException;

@WebFilter(asyncSupported = true, value = "/*")
public class IpFilter implements Filter {

    private static final String UPGRADE = "upgrade";
    private static final String IP = "ip";

    @Override
    public void init(FilterConfig filterConfig) {
        // no-op
    }

    @Override
    public void doFilter(ServletRequest request, ServletResponse response, FilterChain chain) throws IOException, ServletException {
        if (request instanceof HttpServletRequest && response instanceof HttpServletResponse) {
            HttpServletRequest req = (HttpServletRequest) request;
            if (req.getHeader(UPGRADE) != null) {
                /*
                 * update query string with caller IP address so that the websocket server endpoint can retrieve it.
                 */
                chain.doFilter(new IpWrapper(req), response);
            } else {
                chain.doFilter(request, response);
            }
        } else {
            chain.doFilter(request, response);
        }
    }

    @Override
    public void destroy() {
        // no-op
    }

    private class IpWrapper extends HttpServletRequestWrapper {

        private HttpServletRequest request;

        /**
         * Constructs a request object wrapping the given request.
         *
         * @param request The request to wrap
         * @throws IllegalArgumentException if the request is null
         */
        public IpWrapper(HttpServletRequest request) {
            super(request);
            this.request = request;
        }

        @Override
        public String getQueryString() {
            String query = request.getQueryString();
            String ipAddr = IP+"="+request.getRemoteAddr();
            return query == null? ipAddr : ipAddr+"&"+query;
        }

    }

}
