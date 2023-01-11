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

package org.platformlambda.core;

import io.vertx.core.Future;
import org.apache.logging.log4j.ThreadContext;
import org.junit.Assert;
import org.junit.Test;
import org.platformlambda.core.annotations.EventInterceptor;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.mock.TestBase;
import org.platformlambda.core.models.*;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.ServiceDef;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.MultiLevelMap;
import org.platformlambda.core.util.Utility;
import org.platformlambda.core.websocket.client.PersistentWsClient;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.sql.SQLException;
import java.util.*;
import java.util.concurrent.*;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.concurrent.atomic.AtomicLong;

public class PostOfficeTest extends TestBase {
    private static final Logger log = LoggerFactory.getLogger(PostOfficeTest.class);

    private static final BlockingQueue<String> bench = new ArrayBlockingQueue<>(1);
    private static final String HELLO_ALIAS = "hello.alias";

    private static final ExecutorService executor = Executors.newCachedThreadPool();

    @Test
    public void concurrentEventTest() throws IOException, InterruptedException {
        final String SERVICE = "concurrent.service";
        final String SLOW_SERVICE = "slow.service.1";
        final int CYCLES = 50;
        final long TIMEOUT = 10000;
        PostOffice po = PostOffice.getInstance();
        LambdaFunction f1 = (headers, body, instance) -> {
            // artificially blocking the thread
            Thread.sleep(500);
            return body;
        };
        // make nested RPC call from f2 to f1
        LambdaFunction f2 = (headers, body, instance) -> po.request(SLOW_SERVICE, TIMEOUT, body);
        Platform platform = Platform.getInstance();
        platform.registerPrivate(SLOW_SERVICE, f1, CYCLES);
        platform.registerPrivate(SERVICE, f2, CYCLES);
        long begin = System.currentTimeMillis();
        BlockingQueue<Boolean> wait = new ArrayBlockingQueue<>(1);
        AtomicInteger counter = new AtomicInteger(0);
        AtomicInteger passes = new AtomicInteger(0);
        for (int i=0; i < CYCLES; i++) {
            executor.submit(() -> {
                int count = counter.incrementAndGet();
                try {
                    String message = "hello world "+count;
                    EventEnvelope request = new EventEnvelope().setTo(SERVICE).setBody(message);
                    request.setTrace("12345", "/TEST");
                    EventEnvelope response = po.request(request, TIMEOUT);
                    if (message.equals(response.getBody())) {
                        passes.incrementAndGet();
                    }
                    if (passes.get() >= CYCLES) {
                        wait.offer(true);
                    }
                } catch (Exception e) {
                    log.error("Exception - {}", e.getMessage());
                }
            });
        }
        log.info("Wait for concurrent responses from service");
        wait.poll(TIMEOUT, TimeUnit.MILLISECONDS);
        long diff = System.currentTimeMillis() - begin;
        // demonstrate parallelism that the total time consumed should not be too far from the artificial delay
        log.info("Finished in {} ms", diff);
        Assert.assertEquals(CYCLES, passes.get());
        platform.release(SLOW_SERVICE);
        platform.release(SERVICE);
    }

    @SuppressWarnings("unchecked")
    @Test
    public void aliasRouteTest() throws AppException, IOException, TimeoutException {
        PostOffice po = PostOffice.getInstance();
        final String MESSAGE = "test message";
        EventEnvelope result = po.request(HELLO_ALIAS, 5000, MESSAGE);
        Assert.assertTrue(result.getBody() instanceof Map);
        Map<String, Object> body = (Map<String, Object>) result.getBody();
        Assert.assertEquals(MESSAGE, body.get("body"));
    }

    @Test
    public void nullRouteListTest() {
        PostOffice po = PostOffice.getInstance();
        Assert.assertFalse(po.exists((String[]) null));
        Assert.assertFalse(po.exists((String) null));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void rpcTagTest() throws AppException, IOException, TimeoutException {
        final PostOffice po = PostOffice.getInstance();
        final long TIMEOUT = 5000;
        final String RPC = "rpc";
        final String RPC_TIMEOUT_CHECK = "rpc.timeout.check";
        EventEnvelope request = new EventEnvelope().setTo(RPC_TIMEOUT_CHECK).setBody("OK");
        EventEnvelope response = po.request(request, TIMEOUT);
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(String.valueOf(TIMEOUT), result.get(RPC));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void parallelRpcTagTest() throws IOException {
        final PostOffice po = PostOffice.getInstance();
        final int CYCLE = 3;
        final long TIMEOUT = 5500;
        final String RPC = "rpc";
        final String BODY = "body";
        final String RPC_TIMEOUT_CHECK = "rpc.timeout.check";
        List<EventEnvelope> requests = new ArrayList<>();
        for (int i=0; i < CYCLE; i++) {
            requests.add(new EventEnvelope().setTo(RPC_TIMEOUT_CHECK).setBody(i+1)
                    .setTrace("unit-test", "TEST /api/rpc/timeout/tag"));
        }
        List<EventEnvelope> responses = po.request(requests, TIMEOUT);
        Assert.assertEquals(CYCLE, responses.size());
        List<Integer> payloads = new ArrayList<>();
        for (EventEnvelope response: responses) {
            Assert.assertTrue(response.getBody() instanceof Map);
            Map<String, Object> result = (Map<String, Object>) response.getBody();
            Assert.assertTrue(result.containsKey(BODY));
            Assert.assertTrue(result.get(BODY) instanceof Integer);
            payloads.add((Integer) result.get(BODY));
            Assert.assertEquals(String.valueOf(TIMEOUT), result.get(RPC));
        }
        Assert.assertEquals(CYCLE, payloads.size());
    }

    @Test
    public void wsTest() throws InterruptedException {
        final Utility util = Utility.getInstance();
        final AppConfigReader config = AppConfigReader.getInstance();
        final int PORT = util.str2int(config.getProperty("websocket.server.port", "8085"));
        final String WELCOME = "welcome";
        final String MESSAGE = "hello world";
        final String END = "end";
        final BlockingQueue<Boolean> bench = new ArrayBlockingQueue<>(1);
        final PostOffice po = PostOffice.getInstance();
        List<String> welcome = new ArrayList<>();
        List<String> txPaths = new ArrayList<>();
        LambdaFunction connector = (headers, body, instance) -> {
            if ("open".equals(headers.get("type"))) {
                String txPath = headers.get("tx_path");
                if (txPaths.isEmpty()) {
                    txPaths.add(txPath);
                }
                Assert.assertNotNull(txPath);
                po.send(txPath, WELCOME.getBytes());
                po.send(txPath, MESSAGE);
                po.send(txPath, END);
            }
            if ("string".equals(headers.get("type"))) {
                Assert.assertTrue(body instanceof String);
                String text = (String) body;
                Assert.assertEquals(MESSAGE, text);
                bench.offer(true);
            }
            if ("bytes".equals(headers.get("type"))) {
                Assert.assertTrue(body instanceof byte[]);
                welcome.add(util.getUTF( (byte[]) body));
            }
            return true;
        };
        for (int i=0; i < 3; i++) {
            if (util.portReady("127.0.0.1", PORT, 3000)) {
                break;
            } else {
                log.info("Waiting for websocket server at port-{} to get ready", PORT);
                Thread.sleep(1000);
            }
        }
        PersistentWsClient client = new PersistentWsClient(connector,
                Collections.singletonList("ws://127.0.0.1:"+PORT+"/ws/hello"));
        client.start();
        bench.poll(5, TimeUnit.SECONDS);
        Assert.assertEquals(1, welcome.size());
        Assert.assertEquals(WELCOME, welcome.get(0));
        client.close();
    }

    @Test
    public void testExceptionTransport() throws IOException {
        String ROUTE = "test.exception.1";
        String MESSAGE = "hello world";
        Platform platform = Platform.getInstance();
        LambdaFunction f = (headers, body, instance) -> {
            throw new IllegalArgumentException(MESSAGE);
        };
        platform.registerPrivate(ROUTE, f, 1);
        PostOffice po = PostOffice.getInstance();
        AppException ex = Assert.assertThrows(AppException.class, () -> {
            po.request(ROUTE, 5000, "demo");
        });
        Assert.assertEquals(400, ex.getStatus());
        Assert.assertEquals(MESSAGE, ex.getMessage());
        Assert.assertEquals(IllegalArgumentException.class, ex.getCause().getClass());
    }

    @Test
    public void testNestedExceptionTransport() throws IOException {
        String ROUTE = "test.exception.2";
        String MESSAGE = "hello world";
        String SQL_ERROR = "sql error";
        Platform platform = Platform.getInstance();
        LambdaFunction f = (headers, body, instance) -> {
            SQLException sqlEx = new SQLException(SQL_ERROR);
            throw new IllegalArgumentException(MESSAGE, sqlEx);
        };
        platform.registerPrivate(ROUTE, f, 1);
        PostOffice po = PostOffice.getInstance();
        AppException ex = Assert.assertThrows(AppException.class, () -> {
            po.request(ROUTE, 5000, "demo");
        });
        // the root cause is surfaced in the immediate exception
        Assert.assertEquals(500, ex.getStatus());
        Assert.assertEquals(SQL_ERROR, ex.getMessage());
        Assert.assertEquals(IllegalArgumentException.class, ex.getCause().getClass());
        Assert.assertEquals(SQLException.class, ex.getCause().getCause().getClass());
        Assert.assertEquals(MESSAGE, ex.getCause().getMessage());
    }

    @Test
    public void findProviderThatExists() throws TimeoutException {
        Platform platform = Platform.getInstance();
        platform.waitForProvider("cloud.connector", 10);
    }

    @Test
    public void findProviderThatDoesNotExists() {
        Platform platform = Platform.getInstance();
        TimeoutException ex = Assert.assertThrows(TimeoutException.class, () ->
                platform.waitForProvider("no.such.service", 1));
        Assert.assertTrue(ex.getMessage().startsWith("Giving up no.such.service because it is not ready"));
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
        // start service 3 seconds later, so we can test the waitForProvider method
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

    @Test
    public void registerInvalidRoute() {
        Platform platform = Platform.getInstance();
        LambdaFunction noOp = (headers, body, instance) -> true;
        IOException ex = Assert.assertThrows(IOException.class, () ->
                platform.register("invalidFormat", noOp, 1));
        Assert.assertEquals("Invalid route name - use 0-9, a-z, period, hyphen or underscore characters",
                ex.getMessage());
    }

    @Test
    public void registerNullRoute() {
        Platform platform = Platform.getInstance();
        LambdaFunction noOp = (headers, body, instance) -> true;
        IOException ex = Assert.assertThrows(IOException.class, () ->
                platform.register(null, noOp, 1));
        Assert.assertEquals("Missing service routing path", ex.getMessage());
    }

    @Test
    public void registerNullService() {
        Platform platform = Platform.getInstance();
        IOException ex = Assert.assertThrows(IOException.class, () ->
                platform.register("no.service", null, 1));
        Assert.assertEquals("Missing lambda function", ex.getMessage());
    }

    @Test
    public void reservedExtensionNotAllowed() {
        Platform platform = Platform.getInstance();
        LambdaFunction noOp = (headers, body, instance) -> true;
        IOException ex = Assert.assertThrows(IOException.class, () ->
                platform.register("nothing.com", noOp, 1));
        Assert.assertEquals("Invalid route nothing.com which is use a reserved extension", ex.getMessage());
    }

    @Test
    public void reservedFilenameNotAllowed() {
        Platform platform = Platform.getInstance();
        LambdaFunction noOp = (headers, body, instance) -> true;
        IOException ex = Assert.assertThrows(IOException.class, () ->
                platform.register("thumbs.db", noOp, 1));
        Assert.assertEquals("Invalid route thumbs.db which is a reserved Windows filename", ex.getMessage());
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

    @Test
    public void emptyRouteNotAllowed() {
        Platform platform = Platform.getInstance();
        LambdaFunction noOp = (headers, body, instance) -> true;
        IOException ex = Assert.assertThrows(IOException.class, () ->
                platform.register("", noOp, 1));
        Assert.assertEquals("Invalid route name - use 0-9, a-z, period, hyphen or underscore characters",
                ex.getMessage());
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
        List<String> origins = po.search(HELLO_WORLD);
        Assert.assertTrue(origins.contains(platform.getOrigin()));
        List<String> remoteOrigins = po.search(HELLO_WORLD, true);
        Assert.assertTrue(remoteOrigins.isEmpty());
        Assert.assertTrue(po.exists(platform.getOrigin()));
    }

    @Test
    public void testNonExistRoute() {
        PostOffice po = PostOffice.getInstance();
        IOException ex = Assert.assertThrows(IOException.class, () ->
                po.send("undefined.route", "OK"));
        Assert.assertEquals("Route undefined.route not found", ex.getMessage());
    }

    @Test
    public void cancelFutureEventTest() {
        long FIVE_SECONDS = 5000;
        long now = System.currentTimeMillis();
        String TRACE_ID = Utility.getInstance().getUuid();
        PostOffice po = PostOffice.getInstance();
        EventEnvelope event1 = new EventEnvelope().setTo(HELLO_WORLD)
                .setTraceId(TRACE_ID).setTracePath("GET /1").setBody(1);
        EventEnvelope event2 = new EventEnvelope().setTo(HELLO_WORLD)
                .setTraceId(TRACE_ID).setTracePath("GET /2").setBody(2);
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
        List<String> futureEvents = po.getFutureEvents(HELLO_WORLD);
        Assert.assertTrue(futureEvents.contains(id1));
        po.cancelFutureEvent(id1);
    }

    @Test
    public void journalYamlTest() {
        String MY_FUNCTION = "my.test.function";
        String ANOTHER_FUNCTION = "another.function";
        PostOffice po = PostOffice.getInstance();
        List<String> routes = po.getJournaledRoutes();
        Assert.assertEquals(2, routes.size());
        Assert.assertEquals(ANOTHER_FUNCTION, routes.get(0));
        Assert.assertEquals(MY_FUNCTION, routes.get(1));
    }

    @Test
    public void journalTest() throws IOException, InterruptedException {
        BlockingQueue<Map<String, Object>> bench = new ArrayBlockingQueue<>(1);
        AtomicInteger counter = new AtomicInteger(0);
        Platform platform = Platform.getInstance();
        PostOffice po = PostOffice.getInstance();
        String TRACE_ID = Utility.getInstance().getUuid();
        String FROM = "unit.test";
        String HELLO = "hello";
        String WORLD = "world";
        String RETURN_VALUE = "some_value";
        String MY_FUNCTION = "my.test.function";
        String TRACE_PROCESSOR = "distributed.trace.processor";
        LambdaFunction f = (headers, body, instance) -> {
            // only process the 2nd event
            if (counter.incrementAndGet() == 2) {
                Map<String, Object> result = new HashMap<>();
                result.put("metrics", headers);
                result.put("journal", body);
                bench.offer(result);
            }
            return null;
        };
        LambdaFunction myFunction = (headers, body, instance) -> {
            po.annotateTrace(HELLO, WORLD);
            return RETURN_VALUE;
        };
        platform.registerPrivate(TRACE_PROCESSOR, f, 1);
        platform.registerPrivate(MY_FUNCTION, myFunction, 1);
        EventEnvelope event = new EventEnvelope().setTo(MY_FUNCTION).setFrom(FROM)
                .setTraceId(TRACE_ID).setTracePath("GET /api/hello/world").setBody(HELLO);
        // send twice to test caching mechanism
        po.send(event);
        po.send(event);
        // wait for function completion
        Map<String, Object> result = bench.poll(10, TimeUnit.SECONDS);
        platform.release(TRACE_PROCESSOR);
        MultiLevelMap multi = new MultiLevelMap(result);
        Assert.assertEquals(MY_FUNCTION, multi.getElement("metrics.service"));
        Assert.assertEquals(FROM, multi.getElement("metrics.from"));
        Assert.assertEquals(TRACE_ID, multi.getElement("metrics.id"));
        Assert.assertEquals("true", multi.getElement("metrics.success"));
        Assert.assertEquals(HELLO, multi.getElement("journal.payload.input.body"));
        Assert.assertEquals(RETURN_VALUE, multi.getElement("journal.payload.output.body"));
        Assert.assertEquals(WORLD, multi.getElement("journal.annotations.hello"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void pingTest() throws AppException, IOException, TimeoutException {
        Platform platform = Platform.getInstance();
        PostOffice po = PostOffice.getInstance();
        EventEnvelope result = po.ping(HELLO_WORLD, 5000);
        // ping does not execute function so the execution time is -1
        Assert.assertTrue(result.getExecutionTime() < 0);
        Assert.assertTrue(result.getRoundTrip() >= 0);
        Assert.assertTrue(result.getBody() instanceof Map);
        Map<String, Object> response = (Map<String, Object>) result.getBody();
        Assert.assertTrue(response.containsKey("time"));
        Assert.assertTrue(response.containsKey("service"));
        Assert.assertEquals("pong", response.get("type"));
        Assert.assertEquals(platform.getName(), response.get("app"));
        Assert.assertEquals(platform.getOrigin(), response.get("origin"));
        Assert.assertEquals("This response is generated when you send an event without headers and body",
                response.get("reason"));
        log.info("Ping successfully - {}", response.get("message"));
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
        String FIRST = "hello.world.one";
        String SECOND = "hello.world.two";
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
        String result = bench.poll(5, TimeUnit.SECONDS);
        // validate the "from" address
        Assert.assertEquals(FIRST, result);
        // with tracing
        po.send(FIRST, TRACE_ON);
        result = bench.poll(5, TimeUnit.SECONDS);
        Assert.assertEquals(FIRST, result);
    }

    @Test
    public void singleRequestWithException() {
        PostOffice po = PostOffice.getInstance();
        TimeoutException ex = Assert.assertThrows(TimeoutException.class, () ->
                po.request("hello.world", 800, 2));
        Assert.assertEquals("hello.world timeout for 800 ms", ex.getMessage());
    }

    @Test
    public void singleRequestWithTimeoutAndException() {
        PostOffice po = PostOffice.getInstance();
        EventEnvelope request = new EventEnvelope().setTo("hello.world").setFrom("unit.test")
                                    .setTrace("test", "TEST /timeout/exception")
                                    .setHeader("exception", true).setBody(2);
        TimeoutException ex = Assert.assertThrows(TimeoutException.class, () ->
                                    po.request(request, 800));
        Assert.assertEquals("hello.world timeout for 800 ms", ex.getMessage());
    }

    @Test
    @SuppressWarnings("unchecked")
    public void singleRequest() throws TimeoutException, IOException, AppException {
        int input = 111;
        PostOffice po = PostOffice.getInstance();
        EventEnvelope response = po.request("hello.world", 800, input);
        Assert.assertEquals(HashMap.class, response.getBody().getClass());
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(input, result.get("body"));
    }

    @Test
    public void asyncRequestTest() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> success = new ArrayBlockingQueue<>(1);
        final String SERVICE = "hello.future.1";
        final String TEXT = "hello world";
        final Platform platform = Platform.getInstance();
        final PostOffice po = PostOffice.getInstance();
        final LambdaFunction f = (headers, body, instance) -> body;
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
        final String SERVICE = "hello.future.2";
        final String TEXT = "hello world";
        final Platform platform = Platform.getInstance();
        final PostOffice po = PostOffice.getInstance();
        final LambdaFunction f = (headers, body, instance) -> {
            log.info("Simulate timeout for {}", SERVICE);
            Thread.sleep(1000);
            return body;
        };
        platform.registerPrivate(SERVICE, f, 1);
        Future<EventEnvelope> future = po.asyncRequest(new EventEnvelope().setTo(SERVICE).setBody(TEXT), TIMEOUT);
        future.onFailure(exception::offer);
        Throwable e = exception.poll(5, TimeUnit.SECONDS);
        Assert.assertNotNull(e);
        Assert.assertEquals(TimeoutException.class, e.getClass());
        Assert.assertEquals("Timeout for "+TIMEOUT+" ms", e.getMessage());
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
    public void forkJoinTest() throws IOException {
        final String SERVICE = "hello.fork.n.join";
        final String SLOW_SERVICE = "slow.service.2";
        final int SERVICE_INSTANCES = 5;
        final long TIMEOUT = 10000;
        PostOffice po = PostOffice.getInstance();
        LambdaFunction f1 = (headers, body, instance) -> {
            // artificially blocking the thread
            Thread.sleep(500);
            return body;
        };
        // make nested RPC call from f2 to f1
        LambdaFunction f2 = (headers, body, instance) -> po.request(SLOW_SERVICE, TIMEOUT, body);
        Platform platform = Platform.getInstance();
        platform.registerPrivate(SLOW_SERVICE, f1, SERVICE_INSTANCES);
        platform.registerPrivate(SERVICE, f2, SERVICE_INSTANCES);
        final String TEXT = "hello world";
        final int INSTANCES = 10;
        final long begin = System.currentTimeMillis();
        List<EventEnvelope> request = new ArrayList<>();
        for (int i=1; i < INSTANCES + 1; i++) {
            request.add(new EventEnvelope().setTo(SERVICE).setBody(TEXT + "." + i));
        }
        List<EventEnvelope> result = po.request(request, 1500);
        long diff = System.currentTimeMillis() - begin;
        // Since the number of concurrent requests is 2x of the number of workers,
        // the total time should be slightly over one second
        log.info("Elapsed time = {} ms", diff);
        Assert.assertNotNull(result);
        Assert.assertEquals(INSTANCES, result.size());
        for (EventEnvelope r: result) {
            Assert.assertTrue(r.getBody() instanceof String);
            String text = (String) r.getBody();
            Assert.assertTrue(text.startsWith(TEXT));
            log.info("Received response #{} {} - {}, exec_time = {} ms, round_trip = {} ms",
                    r.getCorrelationId(), r.getId(), text, r.getExecutionTime(), r.getRoundTrip());
        }
        platform.release(SERVICE);
        platform.release(SLOW_SERVICE);
    }

    @Test
    public void asyncForkJoinTest() throws IOException, InterruptedException {
        final BlockingQueue<List<EventEnvelope>> success = new ArrayBlockingQueue<>(1);
        final String SERVICE = "hello.future.3";
        final String TEXT = "hello world";
        final int PARALLEL_INSTANCES = 5;
        final Platform platform = Platform.getInstance();
        final PostOffice po = PostOffice.getInstance();
        final LambdaFunction f1 = (headers, body, instance) -> body;
        platform.registerPrivate(SERVICE, f1, PARALLEL_INSTANCES);
        List<EventEnvelope> request = new ArrayList<>();
        for (int i=1; i < PARALLEL_INSTANCES + 1; i++) {
            request.add(new EventEnvelope().setTo(SERVICE).setBody(TEXT + "." + i));
        }
        Future<List<EventEnvelope>> future = po.asyncRequest(request, 1500);
        future.onSuccess(event -> {
            try {
                platform.release(SERVICE);
                success.offer(event);
            } catch (IOException e) {
                // ok to ignore
            }
        });
        List<EventEnvelope> result = success.poll(5, TimeUnit.SECONDS);
        Assert.assertNotNull(result);
        Assert.assertEquals(PARALLEL_INSTANCES, result.size());
        for (EventEnvelope r: result) {
            Assert.assertTrue(r.getBody() instanceof String);
            String text = (String) r.getBody();
            Assert.assertTrue(text.startsWith(TEXT));
            log.info("Received response #{} {} - {}", r.getCorrelationId(), r.getId(), text);
        }
    }

    @Test
    public void asyncForkJoinTimeoutTest() throws IOException, InterruptedException {
        final long TIMEOUT = 500;
        final BlockingQueue<Throwable> exception = new ArrayBlockingQueue<>(1);
        final String SERVICE = "hello.future.4";
        final String TIMEOUT_SERVICE = "hello.future.timeout";
        final String TEXT = "hello world";
        final int PARALLEL_INSTANCES = 5;
        final Platform platform = Platform.getInstance();
        final PostOffice po = PostOffice.getInstance();
        final LambdaFunction f1 = (headers, body, instance) -> {
            log.info("Received event {}, {}", headers, body);
            return body;
        };
        final LambdaFunction f2 = (headers, body, instance) -> {
            log.info("Simulate timeout for {}", TIMEOUT_SERVICE);
            Thread.sleep(1000);
            return body;
        };
        platform.registerPrivate(SERVICE, f1, PARALLEL_INSTANCES);
        platform.registerPrivate(TIMEOUT_SERVICE, f2, 1);
        List<EventEnvelope> request = new ArrayList<>();
        for (int i=1; i < PARALLEL_INSTANCES; i++) {
            request.add(new EventEnvelope().setTo(SERVICE).setBody(TEXT + "." + i));
        }
        request.add(new EventEnvelope().setTo(TIMEOUT_SERVICE).setBody(TEXT));
        Future<List<EventEnvelope>> future = po.asyncRequest(request, TIMEOUT);
        future.onFailure(exception::offer);
        Throwable e = exception.poll(5, TimeUnit.SECONDS);
        Assert.assertNotNull(e);
        Assert.assertEquals(TimeoutException.class, e.getClass());
        Assert.assertEquals("Timeout for "+TIMEOUT+" ms", e.getMessage());
        platform.release(SERVICE);
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
        List<EventEnvelope> results = po.request(parallelEvents, 800);
        // expect partial results of 2 items because the other two will time out
        Assert.assertEquals(2, results.size());
        // check partial results
        for (EventEnvelope evt: results) {
            Assert.assertTrue(evt.getBody() instanceof Map);
            Map<String, Object> values = (Map<String, Object>) evt.getBody();
            Assert.assertTrue(values.containsKey("body"));
            Object v = values.get("body");
            Assert.assertTrue(v instanceof Integer);
            int val = (int) v;
            // expect body as odd number because even number will time out
            Assert.assertTrue(val % 2 != 0);
        }
    }

    @Test
    @SuppressWarnings("unchecked")
    public void routeSubstitution() throws TimeoutException, IOException, AppException {
        int input = 111;
        PostOffice po = PostOffice.getInstance();
        // with route substitution in the application.properties, hello.test will route to hello.world
        EventEnvelope response = po.request("hello.test", 800, input);
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

    @SuppressWarnings("unchecked")
    @Test
    public void infoRouteTest() throws AppException, IOException, TimeoutException {
        String MY_FUNCTION = "my.test.function";
        String ANOTHER_FUNCTION = "another.function";
        PostOffice po = PostOffice.getInstance();
        EventEnvelope result = po.request(PostOffice.ACTUATOR_SERVICES, 5000, new Kv("type", "routes"));
        Assert.assertTrue(result.getBody() instanceof Map);
        MultiLevelMap multi = new MultiLevelMap((Map<String, Object>) result.getBody());
        Object journalRoutes = multi.getElement("journal");
        Assert.assertTrue(journalRoutes instanceof List);
        List<String> routes = (List<String>) journalRoutes;
        Assert.assertTrue(routes.contains(MY_FUNCTION));
        Assert.assertTrue(routes.contains(ANOTHER_FUNCTION));
    }

    @Test
    public void livenessProbeTest() throws AppException, IOException, TimeoutException {
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

    @Test
    public void testCallBackEventHandler() throws IOException, InterruptedException {
        final BlockingQueue<Object> bench = new ArrayBlockingQueue<>(1);
        String TRACE_ID = "10000";
        String HELLO = "hello";
        String POJO_HAPPY_CASE = "pojo.happy.case.1";
        String SIMPLE_CALLBACK = "simple.callback.1";
        Platform platform = Platform.getInstance();
        PostOffice po = PostOffice.getInstance();
        LambdaFunction f = (headers, body, instance) -> {
            PoJo pojo = new PoJo();
            pojo.setName((String) body);
            return pojo;
        };
        platform.registerPrivate(POJO_HAPPY_CASE, f, 1);
        platform.registerPrivate(SIMPLE_CALLBACK, new SimpleCallback(bench, TRACE_ID), 1);
        po.send(new EventEnvelope().setTo(POJO_HAPPY_CASE).setReplyTo(SIMPLE_CALLBACK).setBody(HELLO)
                .setTrace(TRACE_ID, "HAPPY /10000"));
        Object result = bench.poll(10, TimeUnit.SECONDS);
        Assert.assertNotNull(result);
        Assert.assertEquals(PoJo.class, result.getClass());
        Assert.assertEquals(HELLO, ((PoJo) result).getName());
        platform.release(POJO_HAPPY_CASE);
        platform.release(SIMPLE_CALLBACK);
    }

    @Test
    public void testCallBackErrorHandler() throws IOException, InterruptedException {
        final BlockingQueue<Object> bench = new ArrayBlockingQueue<>(1);
        String TRACE_ID = "20000";
        String HELLO = "hello";
        String DEMO_EXCEPTION = "demo-exception";
        String POJO_ERROR_CASE = "pojo.error.case.2";
        String SIMPLE_CALLBACK = "simple.callback.2";
        Platform platform = Platform.getInstance();
        PostOffice po = PostOffice.getInstance();
        LambdaFunction f = (headers, body, instance) -> {
            if (HELLO.equals(body)) {
                throw new IllegalArgumentException(DEMO_EXCEPTION);
            } else {
                PoJo pojo = new PoJo();
                pojo.setName((String) body);
                return pojo;
            }
        };
        platform.registerPrivate(POJO_ERROR_CASE, f, 1);
        platform.registerPrivate(SIMPLE_CALLBACK, new SimpleCallback(bench, TRACE_ID), 1);
        po.send(new EventEnvelope().setTo(POJO_ERROR_CASE).setReplyTo(SIMPLE_CALLBACK).setBody(HELLO)
                .setTrace(TRACE_ID, "ERROR /20000"));

        Object result = bench.poll(10, TimeUnit.SECONDS);
        Assert.assertNotNull(result);
        Assert.assertEquals(AppException.class, result.getClass());
        Assert.assertEquals(DEMO_EXCEPTION, ((AppException) result).getMessage());
        platform.release(POJO_ERROR_CASE);
        platform.release(SIMPLE_CALLBACK);
    }

    @Test
    public void testCallBackCastingException() throws IOException, InterruptedException {
        final BlockingQueue<Object> bench = new ArrayBlockingQueue<>(1);
        String TRACE_ID = "30000";
        String HELLO = "hello";
        // this casting error message is compatible from Java 1.8 to 16
        String CASTING_ERROR = "String cannot be cast to";
        String POJO_ERROR_CASE = "pojo.error.case.3";
        String SIMPLE_CALLBACK = "simple.callback.3";
        Platform platform = Platform.getInstance();
        PostOffice po = PostOffice.getInstance();
        LambdaFunction f = (headers, body, instance) -> HELLO;
        platform.registerPrivate(POJO_ERROR_CASE, f, 1);
        platform.registerPrivate(SIMPLE_CALLBACK, new SimpleCallback(bench, TRACE_ID), 1);
        po.send(new EventEnvelope().setTo(POJO_ERROR_CASE).setReplyTo(SIMPLE_CALLBACK).setBody(HELLO)
                .setTrace(TRACE_ID, "CAST /30000"));
        Object result = bench.poll(10, TimeUnit.SECONDS);
        Assert.assertNotNull(result);
        Assert.assertEquals(AppException.class, result.getClass());
        String error = ((AppException) result).getMessage();
        Assert.assertTrue(error.contains(CASTING_ERROR));
        platform.release(POJO_ERROR_CASE);
        platform.release(SIMPLE_CALLBACK);
    }

    @Test
    public void testAutoInputMapping() throws AppException, IOException, TimeoutException {
        String AUTO_MAPPING = "hello.auto.input.mapping";
        String HELLO_WORLD = "hello world";
        String NAME = "name";
        PostOffice po = PostOffice.getInstance();
        Map<String, Object> map = new HashMap<>();
        map.put(NAME, HELLO_WORLD);
        EventEnvelope response = po.request(AUTO_MAPPING, 5000, map);
        Assert.assertEquals(PoJo.class, response.getBody().getClass());
        PoJo pojo = (PoJo) response.getBody();
        Assert.assertEquals(HELLO_WORLD, pojo.getName());
    }

    @SuppressWarnings("unchecked")
    @Test
    public void testPrimitiveTransport() throws AppException, IOException, TimeoutException {
        String HELLO_WORLD = "hello.world";
        int number = 100;
        PostOffice po = PostOffice.getInstance();
        EventEnvelope response = po.request(HELLO_WORLD, 5000, number);
        Map<String, Object> map = (Map<String, Object>) response.getBody();
        Assert.assertEquals(number, map.get("body"));
        Date now = new Date();
        response = po.request(HELLO_WORLD, 5000, now);
        map = (Map<String, Object>) response.getBody();
        Assert.assertEquals(Utility.getInstance().date2str(now), map.get("body"));
    }

    private static class SimpleCallback implements TypedLambdaFunction<PoJo, Void>, ServiceExceptionHandler {

        private final BlockingQueue<Object> bench;
        private final String traceId;

        public SimpleCallback(BlockingQueue<Object> bench, String traceId) {
            this.bench = bench;
            this.traceId = traceId;
        }

        @Override
        public void onError(AppException e, EventEnvelope event) {
            PostOffice po = PostOffice.getInstance();
            if (traceId.equals(po.getTraceId())) {
                log.info("Found trace path '{}'", po.getTrace().path);
                bench.offer(e);
            }
        }

        @Override
        public Void handleEvent(Map<String, String> headers, PoJo body, int instance) {
            PostOffice po = PostOffice.getInstance();
            if (traceId.equals(po.getTraceId())) {
                log.info("Found trace path '{}'", po.getTrace().path);
                bench.offer(body);
            }
            return null;
        }

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
