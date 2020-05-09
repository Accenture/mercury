package org.platformlambda.core.util;

import org.junit.Test;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.SimpleHttpCookie;

import org.junit.Assert;

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
        Assert.assertTrue(cookie.contains("Path=/;"));
        Assert.assertTrue(cookie.contains("HttpOnly"));
        Assert.assertTrue(cookie.contains("Secure;"));
        Assert.assertTrue(cookie.contains("hello=world;"));
        Assert.assertTrue(cookie.contains("Max-Age=60;"));
        Assert.assertTrue(cookie.contains("GMT;"));
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
        Assert.assertTrue(event.getHeaders().get(SET_COOKIE).contains("|"));
        Assert.assertFalse(event.getHeaders().get(HELLO).contains("|"));
    }

}
