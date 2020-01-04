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

package com.accenture.automation;

import org.platformlambda.core.annotations.OptionalService;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.ManagedCache;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.servlet.*;
import javax.servlet.annotation.WebFilter;
import javax.servlet.http.Cookie;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;

/**
 * To avoid unintended side effect, this module must sit in a different folder than the base package
 * so the OptionalService annotation would be evaluated.
 */
@WebFilter(asyncSupported = true, value = "/*")
@OptionalService("sso.enabled")
public class LoginFilter implements Filter {
    private static final Logger log = LoggerFactory.getLogger(LoginFilter.class);

    private static final ManagedCache cache = ManagedCache.createCache("user.session", 10000);
    private static final String PROTOCOL = "x-forwarded-proto";
    private static final String HTTP = "http";
    private static final String HTTPS = "https";
    private static final String UPGRADE = "upgrade";
    private static final String HOST = "host";
    private static final String TYPE = "type";
    private static final String EXISTS = "exists";
    private static final String SESSION_ID = "session_id";
    private static final String X_SESSION_ID = "x-session-id";
    private static final String X_TERM_ID = "X-Term-Id";
    private static String sessionService, apiLoginPage;
    private static boolean enforceHttps;
    private static List<String> protectedPages = new ArrayList<>();
    private static List<String> wildcardPages = new ArrayList<>();

    @Override
    public void init(FilterConfig filterConfig) {
        if (sessionService == null) {
            Utility util = Utility.getInstance();
            AppConfigReader reader = AppConfigReader.getInstance();
            sessionService = reader.getProperty("session.service", "v1.session.manager");
            apiLoginPage = reader.getProperty("sso.login", "/api/login");
            enforceHttps = "true".equals(reader.getProperty("enforce.https", "false"));
            List<String> pages = util.split(reader.getProperty("sso.protected.pages", "/, /index.*"), ", ");
            List<String> wildcardList = new ArrayList<>();
            for (String p : pages) {
                if (p.startsWith("*")) {
                    log.error("Ignore entry in sso.protected.pages because it cannot start with '*' - ({})", p);
                    continue;
                }
                if (p.endsWith("*")) {
                    String wc = p.substring(0, p.indexOf('*'));
                    wildcardPages.add(wc);
                    wildcardList.add(wc + "*");
                } else {
                    protectedPages.add(p);
                }
            }
            log.info("SSO protected pages {} {}", protectedPages, wildcardList);
        }
    }

    @Override
    public void doFilter(ServletRequest request, ServletResponse response, FilterChain chain) throws IOException, ServletException {
        if (request instanceof HttpServletRequest && response instanceof HttpServletResponse) {
            HttpServletRequest req = (HttpServletRequest) request;
            HttpServletResponse res = (HttpServletResponse) response;
            if (req.getHeader(UPGRADE) != null) {
                // let websocket traffic passes thru
                chain.doFilter(request, response);
            } else {
                String uri = req.getRequestURI();
                String qs = req.getQueryString();
                String target = uri+(qs != null? "?"+qs : "");
                if (enforceHttps && HTTP.equals(req.getHeader(PROTOCOL)) && req.getHeader(HOST) != null) {
                    res.sendRedirect(HTTPS + "://" + req.getHeader(HOST) + target);
                } else {
                    if (isProtected(uri)) {
                        disableBrowserCache(res);
                        String sessionId = getCookie(req, X_SESSION_ID);
                        String longTermId = getCookie(req, X_TERM_ID);
                        if (longTermId != null && longTermId.equals(sessionId) && isValidSession(sessionId)) {
                            chain.doFilter(request, response);
                        } else {
                            res.sendRedirect(apiLoginPage + "?return=" + target);
                        }
                    } else {
                        chain.doFilter(request, response);
                    }
                }
            }
        } else {
            chain.doFilter(request, response);
        }
    }

    private boolean isProtected(String uri) {
        if (protectedPages.contains(uri)) {
            return true;
        }
        for (String p : wildcardPages) {
            if (uri.startsWith(p)) {
                return true;
            }
        }
        return false;
    }

    private void disableBrowserCache(HttpServletResponse response) {
        response.setHeader("Cache-Control", "no-cache, no-store");
        response.setHeader("Pragma", "no-cache");
        response.setDateHeader("Expires", 0);
    }

    private String getCookie(HttpServletRequest request, String name) {
        Cookie[] cookies = request.getCookies();
        if (cookies != null) {
            for (Cookie c: cookies) {
                if (c.getName().equalsIgnoreCase(name)) {
                    return c.getValue();
                }
            }
        }
        return null;
    }

    private boolean isValidSession(String sessionId) throws IOException {
        if (sessionId == null) {
            return false;
        }
        // see if session is cached
        Object o = cache.get(sessionId);
        if (o instanceof Boolean) {
            return (boolean) o;
        }
        PostOffice po = PostOffice.getInstance();
        if (!po.exists(sessionService)) {
            throw new IOException("Session service "+sessionService+" not reachable");
        }
        try {
            EventEnvelope response = po.request(sessionService, 5000,
                    new Kv(TYPE, EXISTS), new Kv(SESSION_ID, sessionId));
            boolean result = response.getBody() instanceof Boolean && (boolean) response.getBody();
            cache.put(sessionId, result);
            return result;
        } catch (Exception e) {
            throw new IOException(e.getMessage());
        }
    }

    @Override
    public void destroy() {
        // no-op
    }
}