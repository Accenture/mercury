/*

    Copyright 2018-2023 Accenture Technology

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
package org.platformlambda.rest.filter;

import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.servlet.*;
import javax.servlet.annotation.WebFilter;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;

@WebFilter(asyncSupported = true, value = "/*")
public class HstsFilter implements Filter {
    private static final Logger log = LoggerFactory.getLogger(HstsFilter.class);

    private static final String PROTOCOL = "x-forwarded-proto";
    private static final String HTTPS = "https";
    private static final String UPGRADE = "upgrade";
    private static final String TRANSPORT_SECURITY_KEY = "Strict-Transport-Security";
    private static final String TRANSPORT_SECURITY_VALUE = "max-age=31536000; includeSubDomains";

    private static boolean loaded = false;
    private static List<String> indexPageList = new ArrayList<>();
    private static String indexPage = "index.html";
    private static boolean hstsRequired = false;

    @Override
    public void init(FilterConfig filterConfig) {
        if (!loaded) {
            loaded = true;
            Utility util = Utility.getInstance();
            AppConfigReader reader = AppConfigReader.getInstance();
            // by default, HSTS header is enabled
            hstsRequired = "true".equals(reader.getProperty("hsts.feature", "true"));
            log.info("HSTS (RFC-6797) feature {}", hstsRequired? "enabled" : "disabled");
            // index.html redirection
            indexPage = reader.getProperty("index.page", "index.html");
            String indexList = reader.getProperty("index.redirection");
            if (indexList != null) {
                List<String> normalizedList = new ArrayList<>();
                util.split(indexList, ", ").forEach(s -> { normalizedList.add(s.endsWith("/")? s : s + "/"); });
                indexPageList = normalizedList;
                log.info("Index page redirection - {}", indexPageList);
            }
        }
    }

    @Override
    public void doFilter(ServletRequest request, ServletResponse response, FilterChain chain) throws IOException, ServletException {
        if (request instanceof HttpServletRequest && response instanceof HttpServletResponse) {
            HttpServletRequest req = (HttpServletRequest) request;
            HttpServletResponse res = (HttpServletResponse) response;
            if (req.getHeader(UPGRADE) == null) {
                /*
                 * HTTP Strict Transport Security (HSTS)
                 * https://tools.ietf.org/html/rfc6797
                 *
                 * If HTTPS, add "Strict Transport Security" header.
                 */
                if (hstsRequired && HTTPS.equals(req.getHeader(PROTOCOL))) {
                    res.setHeader(TRANSPORT_SECURITY_KEY, TRANSPORT_SECURITY_VALUE);
                }
                // perform redirection to index page if needed
                if (!indexPageList.isEmpty()) {
                    String uri = req.getRequestURI();
                    if (!uri.endsWith("/")) {
                        uri += "/";
                    }
                    for (String index : indexPageList) {
                        if (uri.equalsIgnoreCase(index)) {
                            String q = req.getQueryString();
                            res.sendRedirect(uri+indexPage + (q == null? "" : "?"+q));
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


}
