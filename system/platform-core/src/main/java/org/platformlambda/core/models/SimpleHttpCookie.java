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

package org.platformlambda.core.models;

import org.platformlambda.core.util.Utility;

import java.io.UnsupportedEncodingException;
import java.net.URLEncoder;
import java.util.Date;

public class SimpleHttpCookie {

    private static final String SEPARATOR = "|";
    private final String name;
    private String value;
    private String domain;
    private String path;
    private long maxAge = 0;

    private boolean secure = false;
    private boolean httpOnly;

    public SimpleHttpCookie(String name, String value) {
        this.name = name;
        try {
            this.value = URLEncoder.encode(value, "UTF-8");
        } catch (UnsupportedEncodingException e) {
            this.value = value;
        }
    }

    public SimpleHttpCookie setSecure(boolean secure) {
        this.secure = secure;
        return this;
    }

    public SimpleHttpCookie setHttpOnly(boolean httpOnly) {
        this.httpOnly = httpOnly;
        return this;
    }

    public SimpleHttpCookie setMaxAge(long maxAge) {
        this.maxAge = maxAge;
        return this;
    }

    public SimpleHttpCookie setDomain(String domain) {
        if (domain.contains("/")) {
            throw new IllegalArgumentException("Cookie domain must not contain '/'");
        }
        if (domain.contains(SEPARATOR)) {
            throw new IllegalArgumentException("Cookie cannot contain separator '"+SEPARATOR+"'");
        }
        this.domain = domain.toLowerCase();
        return this;
    }

    public SimpleHttpCookie setPath(String path) {
        if (!path.startsWith("/")) {
            throw new IllegalArgumentException("Cookie path must start with '/'");
        }
        if (path.contains(SEPARATOR)) {
            throw new IllegalArgumentException("Cookie cannot contain separator '"+SEPARATOR+"'");
        }
        this.path = path;
        return this;
    }

    /**
     * This method is used to create a cookie string to be used for manually set cookie in HTTP response header.
     *
     * @return cookie string for the "Set-Cookie" HTTP response header
     */
    @Override
    public String toString() {
        StringBuilder sb = new StringBuilder();
        sb.append(name);
        sb.append('=');
        sb.append(value);
        if (maxAge > 0) {
            sb.append("; Max-Age=");
            sb.append(maxAge);
            sb.append("; Expires=");
            sb.append(Utility.getInstance().getHtmlDate(new Date(System.currentTimeMillis() + maxAge * 1000)));
        }
        if (domain != null) {
            sb.append("; Domain=");
            sb.append(domain);
        }
        if (path != null) {
            sb.append("; Path=");
            sb.append(path);
        }
        if (secure) {
            sb.append("; Secure");
        }
        if (httpOnly) {
            sb.append("; HttpOnly");
        }
        return sb.toString();
    }

}
