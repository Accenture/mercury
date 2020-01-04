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

package org.platformlambda.rest.core.websocket;

import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.servlet.*;
import javax.servlet.annotation.WebFilter;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletRequestWrapper;
import javax.servlet.http.HttpServletResponse;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;

@WebFilter(asyncSupported = true, value = "/*")
public class InfoFilter implements Filter {
    private static final Logger log = LoggerFactory.getLogger(InfoFilter.class);

    private static final String IP = "ip";
    private static final String PROTOCOL = "x-forwarded-proto";
    private static final String HTTPS = "https";
    private static final String UPGRADE = "upgrade";
    private static final String TRANSPORT_SECURITY_KEY = "Strict-Transport-Security";
    private static final String TRANSPORT_SECURITY_VALUE = "max-age=31536000; includeSubDomains";
    private static final String LOCALHOST = "127.0.0.1";

    private static boolean loaded = false;
    private static List<String> protectedRestEndpoints = new ArrayList<>();
    private static String apiKeyLabel, infoApiKey;

    @Override
    public void init(FilterConfig filterConfig) {
        if (!loaded) {
            loaded = true;
            AppConfigReader reader = AppConfigReader.getInstance();
            String endpoints = reader.getProperty("protected.info.endpoints");
            if (endpoints != null) {
                protectedRestEndpoints = Utility.getInstance().split(endpoints, ", ");
                Utility util = Utility.getInstance();
                apiKeyLabel = reader.getProperty("info.api.key.label", "X-Info-Key");
                infoApiKey = reader.getProperty("info.api.key");
                if (infoApiKey == null) {
                    infoApiKey = util.getUuid();
                    log.error("{} disabled because info.api.key is missing in application.properties or INFO_API_KEY in environment", endpoints);
                }
                log.info("Started. {} loaded.", apiKeyLabel);
            }
        }
    }

    @Override
    public void doFilter(ServletRequest request, ServletResponse response, FilterChain chain) throws IOException, ServletException {
        if (request instanceof HttpServletRequest && response instanceof HttpServletResponse) {
            HttpServletRequest req = (HttpServletRequest) request;
            HttpServletResponse res = (HttpServletResponse) response;
            if (req.getHeader(UPGRADE) != null) {
                /*
                 * update query string with caller IP address so that the websocket server endpoint can retrieve it.
                 */
                chain.doFilter(new IpWrapper(req), res);
            } else {
                /*
                 * HTTP Strict Transport Security (HSTS)
                 * https://tools.ietf.org/html/rfc6797
                 *
                 * If HTTPS, add "Strict Transport Security" header.
                 */
                if (HTTPS.equals(req.getHeader(PROTOCOL))) {
                    res.setHeader(TRANSPORT_SECURITY_KEY, TRANSPORT_SECURITY_VALUE);
                }
                if (isProtected(req)) {
                    String apiKey = req.getHeader(apiKeyLabel);
                    if (apiKey == null) {
                        res.sendError(404, "Not found");
                        return;
                    } else {
                        if (!apiKey.equals(infoApiKey)) {
                            res.sendError(401, "Unauthorized");
                            return;
                        }
                    }
                }
                chain.doFilter(req, res);
            }
        } else {
            chain.doFilter(request, response);
        }
    }

    @Override
    public void destroy() {
        // no-op
    }

    private boolean isProtected(HttpServletRequest req) {
        if (!LOCALHOST.equals(req.getRemoteAddr()) && !protectedRestEndpoints.isEmpty()) {
            String uri = req.getRequestURI();
            for (String ep: protectedRestEndpoints) {
                if (equals(uri, ep)) {
                    return true;
                }
            }
        }
        return false;
    }

    private boolean equals(String uri, String rule) {
        Utility util = Utility.getInstance();
        List<String> uriParts = util.split(uri, "/");
        List<String> ruleParts = util.split(rule, "/");
        if (uriParts.size() < ruleParts.size()) {
            return false;
        }
        for (int i=0; i < ruleParts.size(); i++) {
            if (!ruleParts.get(i).equals(uriParts.get(i))) {
                return false;
            }
        }
        return true;
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
