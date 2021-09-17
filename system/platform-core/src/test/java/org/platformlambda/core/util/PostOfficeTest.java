/*

    Copyright 2018-2021 Accenture Technology

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

package org.platformlambda.core.util;

import io.vertx.core.Future;
import org.apache.logging.log4j.ThreadContext;
import org.junit.Assert;
import org.junit.BeforeClass;
import org.junit.Test;
import org.platformlambda.core.annotations.EventInterceptor;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.models.TypedLambdaFunction;
import org.platformlambda.core.system.AppStarter;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.ServiceDef;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.*;
import java.util.concurrent.*;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.concurrent.atomic.AtomicLong;

public class PostOfficeTest {
    private static final Logger log = LoggerFactory.getLogger(PostOfficeTest.class);

    private static final BlockingQueue<String> bench = new ArrayBlockingQueue<>(1);
    private static final String HELLO_WORLD = "hello.world";

    @BeforeClass
    public static void setup() throws IOException {
        AppStarter.main(new String[0]);
        Platform platform = Platform.getInstance();
        LambdaFunction echo = (headers, body, instance) -> {
            int c = body instanceof Integer? (int) body : 2;
            if (c % 2 == 0) {
                Thread.sleep(1000);
                // timeout the incoming request
            }
            Map<String, Object> result = new HashMap<>();
            result.put("headers", headers);
            result.put("body", body);
            result.put("instance", instance);
            result.put("counter", c);
            result.put("origin", platform.getOrigin());
            return result;
        };
        // private function
        platform.registerPrivate(HELLO_WORLD, echo, 10);
        // you can convert a private function to public when needed
        platform.makePublic(HELLO_WORLD);
    }

    @Test
    public void findProviderThatExists() throws TimeoutException {
        Platform platform = Platform.getInstance();
        platform.waitForProvider("cloud.connector", 10);
    }

    @Test(expected = TimeoutException.class)
    public void findProviderThatDoesNotExists() throws TimeoutException {
        Platform platform = Platform.getInstance();
        platform.waitForProvider("no.such.service", 1);
    }

    @Test
    public void findProviderThatIsPending() throws TimeoutException, IOException {
        Platform platform = Platform.getInstance();
        LambdaFunction noOp = (headers, body, instance) -> true;
        LambdaFunction f = (headers, body, instance) -> {
            platform.register("no.op", noOp, 1);
            return true;
        };
        platform.registerPrivate("pending.service", f, 1);
        PostOffice po = PostOffice.getInstance();
        // start service 3 seconds later so we can test the waitForProvider method
        po.sendLater(new EventEnvelope().setTo("pending.service").setBody("hi"),
                new Date(System.currentTimeMillis()+3000));
        platform.waitForProvider("no.op", 5);
    }

    @Test
    public void youCanSetUniqueAppId() {
        /*
         * usually you do not need to set application-ID
         * When you set it, the origin-ID will be generated from the app-ID
         * so that you can correlate user specific information for tracking purpose.
         */
        String id = Utility.getInstance().getDateUuid()+"-"+System.getProperty("user.name");
        Platform.setAppId(id);
        Platform platform = Platform.getInstance();
        Assert.assertEquals(id, platform.getAppId());
    }

    @Test(expected = IOException.class)
    public void registerInvalidRoute() throws IOException {
        Platform platform = Platform.getInstance();
        LambdaFunction noOp = (headers, body, instance) -> true;
        platform.register("invalidFormat", noOp, 1);
    }

    @Test(expected = IOException.class)
    public void registerNullRoute() throws IOException {
        Platform platform = Platform.getInstance();
        LambdaFunction noOp = (headers, body, instance) -> true;
        platform.register(null, noOp, 1);
    }

    @Test(expected = IOException.class)
    public void registerNullService() throws IOException {
        Platform platform = Platform.getInstance();
        platform.register("no.service", null, 1);
    }

    @Test(expected = IOException.class)
    public void reservedExtensionNotAllowed() throws IOException {
        Platform platform = Platform.getInstance();
        LambdaFunction noOp = (headers, body, instance) -> true;
        platform.register("nothing.com", noOp, 1);
    }

    @Test(expected = IOException.class)
    public void reservedFilenameNotAllowed() throws IOException {
        Platform platform = Platform.getInstance();
        LambdaFunction noOp = (headers, body, instance) -> true;
        platform.register("thumbs.db", noOp, 1);
    }

    @Test
    public void reloadPublicServiceAsPrivate() throws IOException, AppException, TimeoutException {
        String SERVICE = "reloadable.service";
        Platform platform = Platform.getInstance();
        PostOffice po = PostOffice.getInstance();
        LambdaFunction TRUE_FUNCTION = (headers, body, instance) -> true;
        LambdaFunction FALSE_FUNCTION = (headers, body, instance) -> false;
        platform.register(SERVICE, TRUE_FUNCTION, 1);
        EventEnvelope result = po.request(SERVICE, 5000, "HELLO");
        Assert.assertEquals(true, result.getBody());
        // reload as private
        platform.registerPrivate(SERVICE, FALSE_FUNCTION, 1);
        result = po.request(SERVICE, 5000, "HELLO");
        Assert.assertEquals(false, result.getBody());
        // convert to public
        platform.makePublic(SERVICE);
    }

    @Test(expected = IOException.class)
    public void emptyRouteNotAllowed() throws IOException {
        Platform platform = Platform.getInstance();
        LambdaFunction noOp = (headers, body, instance) -> true;
        platform.register("", noOp, 1);
    }

    @Test
    public void checkLocalRouting() {
        Platform platform = Platform.getInstance();
        ConcurrentMap<String, ServiceDef> routes = platform.getLocalRoutingTable();
        Assert.assertFalse(routes.isEmpty());
    }

    @Test
    public void testExists() throws TimeoutException {
        Platform platform = Platform.getInstance();
        platform.waitForProvider("system.service.registry", 5000);
        PostOffice po = PostOffice.getInstance();
        Assert.assertFalse(po.exists());
        Assert.assertTrue(po.exists(HELLO_WORLD));
        Assert.assertFalse(po.exists(HELLO_WORLD, "unknown.service"));
        Assert.assertFalse(po.exists(HELLO_WORLD, "unknown.1", "unknown.2"));
        Assert.assertNotNull(po.search(HELLO_WORLD));
    }

    @Test(expected = IOException.class)
    public void testNonExistRoute() throws IOException {
        PostOffice po = PostOffice.getInstance();
        po.send("undefined.route", "OK");
    }

    @Test
    public void deferredDelivery() throws IOException {
        long FIVE_SECONDS = 5000;
        long now = System.currentTimeMillis();
        PostOffice po = PostOffice.getInstance();
        EventEnvelope event1 = new EventEnvelope().setTo(HELLO_WORLD).setTraceId("24680").setTracePath("GET /1");
        EventEnvelope event2 = new EventEnvelope().setTo(HELLO_WORLD).setTraceId("12345").setTracePath("GET /2");
        String id1 = po.sendLater(event1, new Date(now+(FIVE_SECONDS/10)));
        String id2 = po.sendLater(event2, new Date(now+FIVE_SECONDS));
        Assert.assertEquals(event1.getId(), id1);
        Assert.assertEquals(event2.getId(), id2);
        List<String> future1 = po.getFutureEvents(HELLO_WORLD);
        Assert.assertEquals(2, future1.size());
        Assert.assertTrue(future1.contains(id1));
        Assert.assertTrue(future1.contains(id2));
        List<String> futureRoutes = po.getAllFutureEvents();
        Assert.assertTrue(futureRoutes.contains(HELLO_WORLD));
        Date time = po.getFutureEventTime(id2);
        long diff = time.getTime() - now;
        Assert.assertEquals(FIVE_SECONDS, diff);
        po.cancelFutureEvent(id2);
        Date notFound = po.getFutureEventTime(id2);
        Assert.assertNull(notFound);
    }

    @Test
    public void pingTest() throws AppException, IOException, TimeoutException {
        PostOffice po = PostOffice.getInstance();
        EventEnvelope result = po.ping(HELLO_WORLD, 5000);
        // ping does not execute function so the execution time is -1
        Assert.assertTrue(result.getExecutionTime() < 0);
        Assert.assertTrue(result.getRoundTrip() >= 0);
    }

    @SuppressWarnings("unchecked")
    @Test
    public void broadcastTest() throws IOException, InterruptedException {
        BlockingQueue<String> bench = new ArrayBlockingQueue<>(1);
        String CALLBACK = "my.callback";
        String MESSAGE = "test";
        String DONE = "done";
        Platform platform = Platform.getInstance();
        LambdaFunction callback = (headers, body, instance) -> {
            if (body instanceof Map) {
                if (MESSAGE.equals(((Map<String, Object>) body).get("body"))) {
                    bench.offer(DONE);
                }
            }
            return null;
        };
        platform.registerPrivate(CALLBACK, callback, 1);
        PostOffice po = PostOffice.getInstance();
        po.send(new EventEnvelope().setTo(HELLO_WORLD).setBody(MESSAGE).setReplyTo(CALLBACK));
        String result = bench.poll(10, TimeUnit.SECONDS);
        Assert.assertEquals(DONE, result);
        // these are drop-n-forget since there are no reply-to address
        po.send(HELLO_WORLD, "some message", new Kv("hello", "world"));
        po.broadcast(HELLO_WORLD, "another message");
        po.broadcast(HELLO_WORLD, "another message", new Kv("key", "value"));
        po.broadcast(HELLO_WORLD, new Kv("hello", "world"), new Kv("key", "value"));
        // this one has replyTo
        po.broadcast(new EventEnvelope().setTo(HELLO_WORLD).setBody(MESSAGE).setReplyTo(CALLBACK));
        result = bench.poll(10, TimeUnit.SECONDS);
        Assert.assertEquals(DONE, result);
        platform.release("my.callback");
    }

    @Test
    public void eventHasFromAddress() throws IOException, InterruptedException {
        String TRACE_ON = "trace";
        String FIRST = "hello.world.1";
        String SECOND = "hello.world.2";
        Platform platform = Platform.getInstance();
        PostOffice po = PostOffice.getInstance();
        LambdaFunction f1 = (headers, body, instance) -> {
            if (TRACE_ON.equals(body)) {
                po.startTracing("12345", "TRACE addr://test");
            }
            po.send(SECOND, true);
            return Optional.empty();
        };
        platform.register(FIRST, f1, 1);
        platform.register(SECOND, new SimpleInterceptor(), 1);
        // without tracing
        po.send(FIRST, Optional.empty());
        String result = bench.poll(2, TimeUnit.SECONDS);
        // validate the "from" address
        Assert.assertEquals(FIRST, result);
        // with tracing
        po.send(FIRST, TRACE_ON);
        result = bench.poll(2, TimeUnit.SECONDS);
        Assert.assertEquals(FIRST, result);
    }

    @Test(expected = TimeoutException.class)
    public void singleRequestWithException() throws TimeoutException, IOException, AppException {
        PostOffice po = PostOffice.getInstance();
        po.request("hello.world", 500, 0);
    }

    @Test
    @SuppressWarnings("unchecked")
    public void singleRequest() throws TimeoutException, IOException, AppException {
        int input = 111;
        PostOffice po = PostOffice.getInstance();
        EventEnvelope response = po.request("hello.world", 500, input);
        Assert.assertEquals(HashMap.class, response.getBody().getClass());
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(input, result.get("body"));
    }

    @Test
    public void asyncRequestTest() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> success = new ArrayBlockingQueue<>(1);
        String SERVICE = "hello.future.1";
        String TEXT = "hello world";
        Platform platform = Platform.getInstance();
        PostOffice po = PostOffice.getInstance();
        LambdaFunction f = (headers, body, instance) -> body;
        platform.registerPrivate(SERVICE, f, 1);
        Future<EventEnvelope> future = po.asyncRequest(new EventEnvelope().setTo(SERVICE).setBody(TEXT), 1500);
        future.onSuccess(event -> {
            try {
                platform.release(SERVICE);
                success.offer(event);
            } catch (IOException e) {
                // ok to ignore
            }
        });
        EventEnvelope result = success.poll(5, TimeUnit.SECONDS);
        Assert.assertNotNull(result);
        Assert.assertEquals(200, (int) result.getStatus());
        Assert.assertEquals(TEXT, result.getBody());
    }

    @Test
    public void asyncRequestTimeout() throws IOException, InterruptedException {
        final long TIMEOUT = 500;
        final BlockingQueue<Throwable> exception = new ArrayBlockingQueue<>(1);
        String SERVICE = "hello.future.2";
        String TEXT = "hello world";
        Platform platform = Platform.getInstance();
        PostOffice po = PostOffice.getInstance();
        LambdaFunction f = (headers, body, instance) -> {
            Thread.sleep(1000);
            return body;
        };
        platform.registerPrivate(SERVICE, f, 1);
        Future<EventEnvelope> future = po.asyncRequest(new EventEnvelope().setTo(SERVICE).setBody(TEXT), TIMEOUT);
        future.onFailure(exception::offer);
        Throwable e = exception.poll(5, TimeUnit.SECONDS);
        Assert.assertNotNull(e);
        Assert.assertEquals(TimeoutException.class, e.getClass());
        Assert.assertEquals(SERVICE+" timeout for "+TIMEOUT+" ms", e.getMessage());
        platform.release(SERVICE);
    }

    @Test
    public void futureExceptionAsResult() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> completion = new ArrayBlockingQueue<>(1);
        int STATUS = 400;
        String ERROR = "some exception";
        String SERVICE = "hello.future.3";
        String TEXT = "hello world";
        Platform platform = Platform.getInstance();
        PostOffice po = PostOffice.getInstance();
        LambdaFunction f = (headers, body, instance) -> {
            throw new AppException(STATUS, ERROR);
        };
        platform.registerPrivate(SERVICE, f, 1);
        Future<EventEnvelope> future = po.asyncRequest(new EventEnvelope().setTo(SERVICE).setBody(TEXT), 5000);
        future.onSuccess(event -> {
            try {
                platform.release(SERVICE);
                completion.offer(event);
            } catch (IOException e) {
                // ok to ignore
            }
        });
        EventEnvelope result = completion.poll(5, TimeUnit.SECONDS);
        Assert.assertNotNull(result);
        Assert.assertEquals(STATUS, (int) result.getStatus());
        Assert.assertEquals(ERROR, result.getBody());
    }

    @Test
    @SuppressWarnings("unchecked")
    public void multilevelTrace() throws TimeoutException, IOException, AppException {
        final String ROUTE_ONE = "hello.level.1";
        final String ROUTE_TWO = "hello.level.2";
        final String TRACE_ID = "cid-123456";
        final String TRACE_PATH = "GET /api/hello/world";
        final String SOME_KEY = "some_key";
        final String SOME_VALUE = "some value";
        final String ANOTHER_KEY = "another_key";
        final String ANOTHER_VALUE = "another value";
        Platform platform = Platform.getInstance();
        LambdaFunction tier1 = (headers, body, instance) -> {
            PostOffice po = PostOffice.getInstance();
            Assert.assertEquals(ROUTE_ONE, po.getRoute());
            Map<String, Object> result = new HashMap<>();
            result.put("headers", headers);
            result.put("body", body);
            result.put("instance", instance);
            result.put("origin", platform.getOrigin());
            // verify trace ID and path
            Assert.assertEquals(TRACE_ID, po.getTraceId());
            Assert.assertEquals(TRACE_PATH, po.getTrace().path);
            // annotate trace
            po.annotateTrace(SOME_KEY, SOME_VALUE).annotateTrace(ANOTHER_KEY, ANOTHER_VALUE);
            // send to level-2 service
            EventEnvelope response = po.request(ROUTE_TWO, 5000, "test");
            Assert.assertEquals(TRACE_ID, response.getBody());
            return result;
        };
        LambdaFunction tier2 = (headers, body, instance) -> {
            PostOffice po = PostOffice.getInstance();
            Assert.assertEquals(ROUTE_TWO, po.getRoute());
            Assert.assertEquals(TRACE_ID, po.getTraceId());
            // annotations are local to a service and should not be transported to the next service
            Assert.assertTrue(po.getTrace().annotations.isEmpty());
            return po.getTraceId();
        };
        platform.register(ROUTE_ONE, tier1, 1);
        platform.register(ROUTE_TWO, tier2, 1);
        // test tracing to 2 levels
        String testMessage = "some message";
        EventEnvelope event = new EventEnvelope();
        event.setTo(ROUTE_ONE).setHeader("hello", "world").setBody(testMessage);
        event.setTrace(TRACE_ID, TRACE_PATH);
        PostOffice po = PostOffice.getInstance();
        EventEnvelope response = po.request(event, 5000);
        Assert.assertEquals(HashMap.class, response.getBody().getClass());
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertTrue(result.containsKey("body"));
        Assert.assertEquals(testMessage, result.get("body"));
    }

    @Test
    @SuppressWarnings("unchecked")
    public void parallelRequests() throws IOException {
        PostOffice po = PostOffice.getInstance();
        List<EventEnvelope> parallelEvents = new ArrayList<>();
        for (int i=0; i < 4; i++) {
            EventEnvelope event = new EventEnvelope();
            event.setTo("hello.world");
            event.setBody(i);
            event.setHeader("request", "#"+(i+1));
            parallelEvents.add(event);
        }
        List<EventEnvelope> results = po.request(parallelEvents, 500);
        // expect partial results of 2 items because the other two will timeout
        Assert.assertEquals(2, results.size());
        // check partial results
        for (EventEnvelope evt: results) {
            Assert.assertTrue(evt.getBody() instanceof Map);
            Map<String, Object> values = (Map<String, Object>) evt.getBody();
            Assert.assertTrue(values.containsKey("body"));
            Object v = values.get("body");
            Assert.assertTrue(v instanceof Integer);
            int val = (int) v;
            // expect body as odd number because even number will timeout
            Assert.assertTrue(val % 2 != 0);
        }
    }

    @Test
    @SuppressWarnings("unchecked")
    public void routeSubstitution() throws TimeoutException, IOException, AppException {
        int input = 111;
        PostOffice po = PostOffice.getInstance();
        // with route substitution in the application.properties, hello.test will route to hello.world
        EventEnvelope response = po.request("hello.test", 500, input);
        Assert.assertEquals(HashMap.class, response.getBody().getClass());
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(input, result.get("body"));
        Map<String, String> list = po.getRouteSubstitutionList();
        Assert.assertTrue(list.containsKey("hello.test"));
    }

    @Test
    public void traceIdInLogContext() throws IOException, TimeoutException, AppException {
        AppConfigReader config = AppConfigReader.getInstance();
        String traceLogHeader = config.getProperty("trace.log.header", "X-Trace-Id");
        LambdaFunction f = (headers, body, instance) -> {
            log.info("Got {} {}", headers, body);
            return ThreadContext.get(traceLogHeader);
        };
        String TRACE_ID = "12345";
        Platform platform = Platform.getInstance();
        platform.register("hello.log", f, 1);
        PostOffice po = PostOffice.getInstance();
        // enable trace
        EventEnvelope event = new EventEnvelope().setTo("hello.log").setHeader("hello", "world")
                .setBody("validate log context").setTrace(TRACE_ID, "GET /hello/log");
        EventEnvelope response = po.request(event, 2000);
        Assert.assertEquals(TRACE_ID, response.getBody());
    }

    @SuppressWarnings("unchecked")
    @Test
    public void healthTest() throws AppException, IOException, TimeoutException {
        Platform platform = Platform.getInstance();
        platform.waitForProvider("cloud.connector.health", 5000);
        PostOffice po = PostOffice.getInstance();
        EventEnvelope result = po.request(PostOffice.ACTUATOR_SERVICES, 5000, new Kv("type" ,"health"));
        Assert.assertTrue(result.getBody() instanceof Map);
        Map<String, Object> map = (Map<String, Object>) result.getBody();
        Assert.assertEquals("UP", map.get("status"));
        Assert.assertEquals("platform-core", map.get("name"));
        Assert.assertEquals(platform.getOrigin(), map.get("origin"));
        Object upstream = map.get("upstream");
        Assert.assertTrue(upstream instanceof List);
        List<Map<String, Object>> upstreamList = (List<Map<String, Object>>) upstream;
        Assert.assertEquals(1, upstreamList.size());
        Map<String, Object> health = upstreamList.get(0);
        Assert.assertEquals("mock.connector", health.get("service"));
        Assert.assertEquals("mock.topic", health.get("topics"));
        Assert.assertEquals("fine", health.get("message"));
        Assert.assertEquals("true", health.get("required").toString());
    }

    @Test
    public void infoTest() throws AppException, IOException, TimeoutException {
        PostOffice po = PostOffice.getInstance();
        EventEnvelope result = po.request(PostOffice.ACTUATOR_SERVICES, 5000, new Kv("type", "info"));
        Assert.assertTrue(result.getBody() instanceof Map);
    }

    @Test
    public void libTest() throws AppException, IOException, TimeoutException {
        PostOffice po = PostOffice.getInstance();
        EventEnvelope result = po.request(PostOffice.ACTUATOR_SERVICES, 5000, new Kv("type", "lib"));
        Assert.assertTrue(result.getBody() instanceof Map);
    }

    @Test
    public void infoRouteTest() throws AppException, IOException, TimeoutException {
        PostOffice po = PostOffice.getInstance();
        EventEnvelope result = po.request(PostOffice.ACTUATOR_SERVICES, 5000, new Kv("type", "routes"));
        Assert.assertTrue(result.getBody() instanceof Map);
    }

    @Test
    public void livenessTest() throws AppException, IOException, TimeoutException {
        PostOffice po = PostOffice.getInstance();
        EventEnvelope result = po.request(PostOffice.ACTUATOR_SERVICES, 5000, new Kv("type", "livenessprobe"));
        Assert.assertEquals("OK", result.getBody());
    }

    @Test
    public void envTest() throws AppException, IOException, TimeoutException {
        PostOffice po = PostOffice.getInstance();
        EventEnvelope result = po.request(PostOffice.ACTUATOR_SERVICES, 5000, new Kv("type", "env"));
        Assert.assertTrue(result.getBody() instanceof Map);
    }

    @Test
    public void resumeTest() throws AppException, IOException, TimeoutException {
        Platform platform = Platform.getInstance();
        platform.waitForProvider("system.service.registry", 5000);
        final String TYPE = "type";
        final String USER = "user";
        final String WHEN = "when";
        PostOffice po = PostOffice.getInstance();
        EventEnvelope result = po.request(PostOffice.ACTUATOR_SERVICES, 5000,
                new Kv(TYPE, "resume"), new Kv(USER, "someone"), new Kv(WHEN, "now"));
        Assert.assertEquals(false, result.getBody());
    }

    @Test
    public void envelopeArgumentTest() throws IOException, AppException, TimeoutException {
        String TARGET = "test.route.1";
        String MESSAGE = "hello world";
        PostOffice po = PostOffice.getInstance();
        Platform.getInstance().register(TARGET, new EventEnvelopeReader(), 1);
        EventEnvelope input = new EventEnvelope().setTo(TARGET).setBody(MESSAGE);
        EventEnvelope output = po.request(input, 5000);
        Assert.assertEquals(MESSAGE, output.getBody());
    }

    @Test
    public void threadPoolTest() throws IOException, InterruptedException {
        final int CYCLES = 200;
        final int WORKER_POOL = 50;
        final ConcurrentMap<Long, Boolean> threads = new ConcurrentHashMap<>();
        final BlockingQueue<Boolean> bench = new ArrayBlockingQueue<>(1);
        final AtomicInteger counter = new AtomicInteger(0);
        final AtomicLong last = new AtomicLong(0);
        String MULTI_CORES = "multi.cores";
        LambdaFunction f= (headers, body, instance) -> {
            int n = counter.incrementAndGet();
            long id = Thread.currentThread().getId();
            log.debug("Instance #{}, count={}, thread #{} {}", instance, n, id, body);
            threads.put(id, true);
            if (n == CYCLES) {
                last.set(System.currentTimeMillis());
                bench.offer(true);
            }
            return true;
        };
        Platform.getInstance().registerPrivate(MULTI_CORES, f, WORKER_POOL);
        PostOffice po = PostOffice.getInstance();
        long t1 = System.currentTimeMillis();
        for (int i=0; i < CYCLES; i++) {
            po.send(MULTI_CORES, "hello world");
        }
        Boolean result = bench.poll(10, TimeUnit.SECONDS);
        long diff = last.get() - t1;
        log.info("{} cycles done? {}, {} workers consumed {} threads in {} ms",
                CYCLES, result != null && result, WORKER_POOL, threads.size(), diff);
    }

    @EventInterceptor
    private static class SimpleInterceptor implements LambdaFunction {

        @Override
        public Object handleEvent(Map<String, String> headers, Object body, int instance) {
            EventEnvelope event = (EventEnvelope) body;
            bench.offer(event.getFrom());
            return Optional.empty();
        }
    }

    private static class EventEnvelopeReader implements TypedLambdaFunction<EventEnvelope, EventEnvelope> {

        @Override
        public EventEnvelope handleEvent(Map<String, String> headers, EventEnvelope body, int instance) {
            return new EventEnvelope().setBody(body.getBody());
        }
    }

}
