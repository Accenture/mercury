package org.platformlambda.core.models;

import org.platformlambda.core.util.Utility;

import java.io.UnsupportedEncodingException;
import java.net.URLEncoder;
import java.util.Date;

public class SimpleHttpCookie {

    private static final String SEPARATOR = "|";
    private String name, value, domain, path;
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
