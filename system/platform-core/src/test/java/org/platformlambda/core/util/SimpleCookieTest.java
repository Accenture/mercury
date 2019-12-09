package org.platformlambda.core.util;

import org.junit.Test;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.SimpleHttpCookie;

import static org.junit.Assert.assertFalse;
import static org.junit.Assert.assertTrue;

public class SimpleCookieTest {

    private static final String SET_COOKIE = "Set-Cookie";

    private SimpleHttpCookie createCookie() {
        String name = "hello";
        String value = "world";
        SimpleHttpCookie cookie = new SimpleHttpCookie(name, value);
        cookie.setPath("/");
        cookie.setHttpOnly(true);
        cookie.setSecure(true);
        cookie.setMaxAge(60);
        return cookie;
    }

    @Test
    public void validateCookie() {
        String cookie = createCookie().toString();
        assertTrue(cookie.contains("Path=/;"));
        assertTrue(cookie.contains("HttpOnly"));
        assertTrue(cookie.contains("Secure;"));
        assertTrue(cookie.contains("hello=world;"));
        assertTrue(cookie.contains("Max-Age=60;"));
        assertTrue(cookie.contains("GMT;"));
    }

    @Test
    public void cookieInEnvelope() {
        EventEnvelope event = new EventEnvelope();
        // "Set-Cookie" is the only header that supports multiple values
        event.setHeader(SET_COOKIE, createCookie());
        event.setHeader(SET_COOKIE, createCookie());
        String HELLO = "hello";
        String WORLD = "world";
        event.setHeader(HELLO, WORLD);
        assertTrue(event.getHeaders().get(SET_COOKIE).contains("|"));
        assertFalse(event.getHeaders().get(HELLO).contains("|"));
    }

}
