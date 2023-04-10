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
import org.junit.Assert;
import org.junit.Test;
import org.platformlambda.core.annotations.EventInterceptor;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.mock.TestBase;
import org.platformlambda.core.models.*;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.EventEmitter;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.ServiceDef;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.CryptoApi;
import org.platformlambda.core.util.MultiLevelMap;
import org.platformlambda.core.util.Utility;
import org.platformlambda.core.websocket.client.PersistentWsClient;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.sql.SQLException;
import java.time.Instant;
import java.time.LocalDateTime;
import java.time.ZoneId;
import java.util.*;
import java.util.concurrent.*;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.concurrent.atomic.AtomicLong;

public class PostOfficeTest extends TestBase {
    private static final Logger log = LoggerFactory.getLogger(PostOfficeTest.class);

    private static final CryptoApi crypto = new CryptoApi();
    private static final BlockingQueue<String> interceptorBench = new ArrayBlockingQueue<>(1);
    private static final String HELLO_ALIAS = "hello.alias";

    @Test
    public void concurrentEventTest() throws InterruptedException {
        final BlockingQueue<Boolean> wait = new ArrayBlockingQueue<>(1);
        final AppConfigReader config = AppConfigReader.getInstance();
        final Utility util = Utility.getInstance();
        int poolSize = Math.max(32, util.str2int(config.getProperty("event.worker.pool", "100")));
        final ExecutorService executor = Platform.getInstance().getEventExecutor();
        final String RPC_FORWARDER = "rpc.forwarder";
        final String SLOW_SERVICE = "artificial.delay";
        final int CYCLES = poolSize / 2;
        log.info("Test sync and blocking RPC with {} workers", CYCLES);
        final long TIMEOUT = 10000;
        PostOffice po = new PostOffice("unit.test", "12345", "/TEST");
        // make nested RPC calls
        long begin = System.currentTimeMillis();
        AtomicInteger counter = new AtomicInteger(0);
        AtomicInteger passes = new AtomicInteger(0);
        for (int i=0; i < CYCLES; i++) {
            /*
             * The blocking RPC will hold up a large number of worker threads = CYCLES
             * Therefore, this test will break if CYCLES > poolSize.
             */
            executor.submit(() -> {
                int count = counter.incrementAndGet();
                try {
                    final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
                    String message = "hello world "+count;
                    EventEnvelope request = new EventEnvelope().setTo(RPC_FORWARDER)
                            .setHeader("target", SLOW_SERVICE).setHeader("timeout", TIMEOUT).setBody(message);
                    po.asyncRequest(request, TIMEOUT, true).onSuccess(bench::offer);
                    EventEnvelope response = bench.poll(TIMEOUT, TimeUnit.MILLISECONDS);
                    assert response != null;
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
    }

    @SuppressWarnings("unchecked")
    @Test
    public void aliasRouteTest() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        final long TIMEOUT = 5000;
        EventEmitter po = EventEmitter.getInstance();
        final String MESSAGE = "test message";
        po.asyncRequest(new EventEnvelope().setTo(HELLO_ALIAS).setBody(MESSAGE), TIMEOUT).onSuccess(bench::offer);
        EventEnvelope response = bench.poll(TIMEOUT, TimeUnit.MILLISECONDS);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> body = (Map<String, Object>) response.getBody();
        Assert.assertEquals(MESSAGE, body.get("body"));
    }

    @Test
    public void nullRouteListTest() {
        EventEmitter po = EventEmitter.getInstance();
        Assert.assertFalse(po.exists((String[]) null));
        Assert.assertFalse(po.exists((String) null));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void rpcTagTest() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        final PostOffice po = new PostOffice("unit.test", "801", "TEST /rpc1/timeout/tag");
        final long TIMEOUT = 5000;
        final int BODY = 100;
        final String RPC_TIMEOUT_CHECK = "rpc.timeout.check";
        EventEnvelope request = new EventEnvelope().setTo(RPC_TIMEOUT_CHECK).setBody(BODY);
        po.asyncRequest(request, TIMEOUT).onSuccess(bench::offer);
        EventEnvelope response = bench.poll(TIMEOUT, TimeUnit.MILLISECONDS);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(String.valueOf(TIMEOUT), result.get(EventEmitter.RPC));
        Assert.assertEquals(BODY, result.get("body"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void parallelRpcTagTest() throws IOException, InterruptedException {
        final BlockingQueue<List<EventEnvelope>> bench = new ArrayBlockingQueue<>(1);
        final PostOffice po = new PostOffice("unit.test", "802", "TEST /rpc2/timeout/tag");
        final int CYCLE = 3;
        final long TIMEOUT = 5500;
        final String BODY = "body";
        final String RPC_TIMEOUT_CHECK = "rpc.timeout.check";
        List<EventEnvelope> requests = new ArrayList<>();
        for (int i=0; i < CYCLE; i++) {
            requests.add(new EventEnvelope().setTo(RPC_TIMEOUT_CHECK).setBody(i+1));
        }
        po.asyncRequest(requests, TIMEOUT).onSuccess(bench::offer);
        List<EventEnvelope> responses = bench.poll(TIMEOUT, TimeUnit.MILLISECONDS);
        assert responses != null;
        Assert.assertEquals(CYCLE, responses.size());
        List<Integer> payloads = new ArrayList<>();
        for (EventEnvelope response: responses) {
            Assert.assertTrue(response.getBody() instanceof Map);
            Map<String, Object> result = (Map<String, Object>) response.getBody();
            Assert.assertTrue(result.containsKey(BODY));
            Assert.assertTrue(result.get(BODY) instanceof Integer);
            payloads.add((Integer) result.get(BODY));
            Assert.assertEquals(String.valueOf(TIMEOUT), result.get(EventEmitter.RPC));
        }
        Assert.assertEquals(CYCLE, payloads.size());
    }

    @Test
    public void wsTest() throws InterruptedException {
        final Utility util = Utility.getInstance();
        final AppConfigReader config = AppConfigReader.getInstance();
        final int PORT = util.str2int(config.getProperty("websocket.server.port",
                                        config.getProperty("server.port", "8085")));
        final String WELCOME = "welcome";
        final String MESSAGE = "hello world";
        final String END = "end";
        final BlockingQueue<Boolean> bench = new ArrayBlockingQueue<>(1);
        final EventEmitter po = EventEmitter.getInstance();
        List<String> welcome = new ArrayList<>();
        List<String> txPaths = new ArrayList<>();
        LambdaFunction connector = (headers, input, instance) -> {
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
                Assert.assertTrue(input instanceof String);
                String text = (String) input;
                Assert.assertEquals(MESSAGE, text);
                bench.offer(true);
            }
            if ("bytes".equals(headers.get("type"))) {
                Assert.assertTrue(input instanceof byte[]);
                welcome.add(util.getUTF( (byte[]) input));
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
    public void testExceptionTransport() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        final long TIMEOUT = 5000;
        String EXCEPTION = "exception";
        String MESSAGE = "just a test";
        EventEmitter po = EventEmitter.getInstance();
        EventEnvelope request = new EventEnvelope().setTo("hello.world").setBody("demo").setHeader(EXCEPTION, true);
        po.asyncRequest(request, TIMEOUT).onSuccess(bench::offer);
        EventEnvelope response = bench.poll(TIMEOUT, TimeUnit.MILLISECONDS);
        assert response != null;
        Assert.assertEquals(400, response.getStatus());
        Assert.assertEquals(MESSAGE, response.getBody());
        Assert.assertEquals(AppException.class, response.getException().getClass());
        log.info("Exception transported - {}", response.getException().toString());
        log.info("Stack trace transported through the response event:");
        StackTraceElement[] elements = response.getException().getStackTrace();
        for (StackTraceElement e: elements) {
            if (e.getClassName().startsWith("org.platformlambda.")) {
                log.info("Found - {}", e);
            }
        }
    }

    @Test
    public void testNestedExceptionTransport() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        final long TIMEOUT = 5000;
        String NEST_EXCEPTION = "nested_exception";
        String MESSAGE = "just a test";
        String SQL_ERROR = "sql error";
        EventEmitter po = EventEmitter.getInstance();
        EventEnvelope request = new EventEnvelope().setTo("hello.world").setBody("hi").setHeader(NEST_EXCEPTION, true);
        po.asyncRequest(request, TIMEOUT).onSuccess(bench::offer);
        EventEnvelope response = bench.poll(TIMEOUT, TimeUnit.MILLISECONDS);
        assert response != null;
        Assert.assertEquals(400, response.getStatus());
        // event error is mapped to the root cause
        Assert.assertEquals(SQL_ERROR, response.getError());
        // nested exception is transported by the response event
        Throwable ex = response.getException();
        // immediate exception
        Assert.assertEquals(AppException.class, ex.getClass());
        AppException appEx = (AppException) ex;
        Assert.assertEquals(400, appEx.getStatus());
        Assert.assertEquals(MESSAGE, appEx.getMessage());
        // nested exception
        Throwable nested = ex.getCause();
        Assert.assertNotNull(nested);
        Assert.assertEquals(SQLException.class, nested.getClass());
        Assert.assertEquals(SQL_ERROR, nested.getMessage());
    }

    @Test
    public void findProviderThatExists() throws InterruptedException {
        BlockingQueue<Boolean> bench = new ArrayBlockingQueue<>(1);
        Platform platform = Platform.getInstance();
        Future<Boolean> status = platform.waitForProvider("cloud.connector", 10);
        status.onSuccess(bench::offer);
        Boolean result = bench.poll(5, TimeUnit.SECONDS);
        Assert.assertEquals(Boolean.TRUE, result);
    }

    @Test
    public void findProviderThatDoesNotExists() throws InterruptedException {
        BlockingQueue<Boolean> bench = new ArrayBlockingQueue<>(1);
        Platform platform = Platform.getInstance();
        Future<Boolean> status = platform.waitForProvider("no.such.service", 1);
        status.onSuccess(bench::offer);
        Boolean result = bench.poll(12, TimeUnit.SECONDS);
        Assert.assertNotEquals(Boolean.TRUE, result);
    }

    @Test
    public void findProviderThatIsPending() throws IOException, InterruptedException {
        final BlockingQueue<Boolean> bench1 = new ArrayBlockingQueue<>(1);
        final BlockingQueue<EventEnvelope> bench2 = new ArrayBlockingQueue<>(1);
        String NO_OP = "no.op";
        String PENDING_SERVICE = "pending.service";
        Platform platform = Platform.getInstance();
        LambdaFunction noOp = (headers, input, instance) -> true;
        LambdaFunction f = (headers, input, instance) -> {
            platform.register(NO_OP, noOp, 1);
            return true;
        };
        platform.registerPrivate(PENDING_SERVICE, f, 1);
        PostOffice po = new PostOffice("unit.test", "11", "CHECK /provider");
        // start service two seconds later, so we can test the waitForProvider method
        po.sendLater(new EventEnvelope().setTo(PENDING_SERVICE).setBody("hi"),
                new Date(System.currentTimeMillis()+2100));
        Future<Boolean> status = platform.waitForProvider(NO_OP, 5);
        status.onSuccess(bench1::offer);
        Boolean result = bench1.poll(12, TimeUnit.SECONDS);
        Assert.assertEquals(Boolean.TRUE, result);
        EventEnvelope request = new EventEnvelope().setTo(NO_OP).setBody("ok");
        po.asyncRequest(request, 5000).onSuccess(bench2::offer);
        EventEnvelope response = bench2.poll(12, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertEquals(true, response.getBody());
        platform.release(NO_OP);
        platform.release(PENDING_SERVICE);
    }

    @Test
    public void deriveOriginIdFromAppId() {
        /*
         * Usually you do not need to set application-ID
         * When you set it, the origin-ID will be generated from the app-ID
         * so that you can correlate user specific information for tracking purpose.
         *
         * Since appId must be set before the "getOrigin" method, the setId is done in the BeforeClass
         * for this unit test.
         */
        Platform platform = Platform.getInstance();
        Assert.assertEquals(APP_ID, platform.getAppId());
        Utility util = Utility.getInstance();
        // validate the hashing algorithm
        String id = util.getUuid();
        byte[] hash = crypto.getSHA256(util.getUTF(platform.getAppId()));
        id = util.bytes2hex(hash).substring(0, id.length());
        String originId = util.getDateOnly(new Date()) + id;
        Assert.assertEquals(platform.getOrigin(), originId);
    }

    @Test
    public void registerInvalidRoute() {
        Platform platform = Platform.getInstance();
        LambdaFunction noOp = (headers, input, instance) -> true;
        IllegalArgumentException ex = Assert.assertThrows(IllegalArgumentException.class, () ->
                platform.register("invalidFormat", noOp, 1));
        Assert.assertEquals("Invalid route name - use 0-9, a-z, period, hyphen or underscore characters",
                ex.getMessage());
    }

    @Test
    public void registerNullRoute() {
        Platform platform = Platform.getInstance();
        LambdaFunction noOp = (headers, input, instance) -> true;
        IllegalArgumentException ex = Assert.assertThrows(IllegalArgumentException.class, () ->
                platform.register(null, noOp, 1));
        Assert.assertEquals("Missing service routing path", ex.getMessage());
    }

    @Test
    public void registerNullService() {
        Platform platform = Platform.getInstance();
        IllegalArgumentException ex = Assert.assertThrows(IllegalArgumentException.class, () ->
                platform.register("no.service", null, 1));
        Assert.assertEquals("Missing LambdaFunction instance", ex.getMessage());
    }

    @Test
    public void reservedExtensionNotAllowed() {
        Platform platform = Platform.getInstance();
        LambdaFunction noOp = (headers, input, instance) -> true;
        IllegalArgumentException ex = Assert.assertThrows(IllegalArgumentException.class, () ->
                platform.register("nothing.com", noOp, 1));
        Assert.assertEquals("Invalid route nothing.com which is use a reserved extension", ex.getMessage());
    }

    @Test
    public void reservedFilenameNotAllowed() {
        Platform platform = Platform.getInstance();
        LambdaFunction noOp = (headers, input, instance) -> true;
        IllegalArgumentException ex = Assert.assertThrows(IllegalArgumentException.class, () ->
                platform.register("thumbs.db", noOp, 1));
        Assert.assertEquals("Invalid route thumbs.db which is a reserved Windows filename", ex.getMessage());
    }

    @Test
    public void reloadPublicServiceAsPrivate() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        String SERVICE = "reloadable.service";
        long TIMEOUT = 5000;
        Platform platform = Platform.getInstance();
        EventEmitter po = EventEmitter.getInstance();
        LambdaFunction TRUE_FUNCTION = (headers, input, instance) -> true;
        LambdaFunction FALSE_FUNCTION = (headers, input, instance) -> false;
        platform.register(SERVICE, TRUE_FUNCTION, 1);
        EventEnvelope request = new EventEnvelope().setTo(SERVICE).setBody("HELLO");
        po.asyncRequest(request, TIMEOUT).onSuccess(bench::offer);
        EventEnvelope result = bench.poll(10, TimeUnit.SECONDS);
        assert result != null;
        Assert.assertEquals(true, result.getBody());
        // reload as private
        platform.registerPrivate(SERVICE, FALSE_FUNCTION, 1);
        po.asyncRequest(request, TIMEOUT).onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertEquals(false, response.getBody());
        // convert to public
        platform.makePublic(SERVICE);
        platform.release(SERVICE);
    }

    @Test
    public void emptyRouteNotAllowed() {
        Platform platform = Platform.getInstance();
        LambdaFunction noOp = (headers, input, instance) -> true;
        IllegalArgumentException ex = Assert.assertThrows(IllegalArgumentException.class, () ->
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
    public void testExists() throws InterruptedException {
        BlockingQueue<List<String>> bench = new ArrayBlockingQueue<>(1);
        Platform platform = Platform.getInstance();
        EventEmitter po = EventEmitter.getInstance();
        Assert.assertFalse(po.exists());
        Assert.assertTrue(po.exists(HELLO_WORLD));
        Assert.assertFalse(po.exists(HELLO_WORLD, "unknown.service"));
        Assert.assertFalse(po.exists(HELLO_WORLD, "unknown.1", "unknown.2"));
        Future<List<String>> asyncResponse1 = po.search(HELLO_WORLD);
        asyncResponse1.onSuccess(bench::offer);
        List<String> origins = bench.poll(5, TimeUnit.SECONDS);
        assert origins != null;
        Assert.assertTrue(origins.contains(platform.getOrigin()));
        Future<List<String>> asyncResponse2 = po.search(HELLO_WORLD, true);
        asyncResponse2.onSuccess(bench::offer);
        List<String> remoteOrigins = bench.poll(5, TimeUnit.SECONDS);
        assert remoteOrigins != null;
        Assert.assertTrue(remoteOrigins.isEmpty());
        Assert.assertTrue(po.exists(platform.getOrigin()));
    }

    @Test
    public void testNonExistRoute() {
        EventEmitter po = EventEmitter.getInstance();
        IOException ex = Assert.assertThrows(IOException.class, () ->
                po.send("undefined.route", "OK"));
        Assert.assertEquals("Route undefined.route not found", ex.getMessage());
    }

    @Test
    public void cancelFutureEventTest() {
        long FIVE_SECONDS = 5000;
        long now = System.currentTimeMillis();
        String TRACE_ID = Utility.getInstance().getUuid();
        EventEmitter po = EventEmitter.getInstance();
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
        EventEmitter po = EventEmitter.getInstance();
        List<String> routes = po.getJournaledRoutes();
        Assert.assertEquals(2, routes.size());
        Assert.assertEquals(ANOTHER_FUNCTION, routes.get(0));
        Assert.assertEquals(MY_FUNCTION, routes.get(1));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void journalTest() throws IOException, InterruptedException {
        String TRANSACTION_JOURNAL_RECORDER = "transaction.journal.recorder";
        BlockingQueue<Map<String, Object>> bench = new ArrayBlockingQueue<>(1);
        Platform platform = Platform.getInstance();
        String TRACE_ID = Utility.getInstance().getUuid();
        String FROM = "unit.test";
        String HELLO = "hello";
        String WORLD = "world";
        String RETURN_VALUE = "some_value";
        String MY_FUNCTION = "my.test.function";
        LambdaFunction f = (headers, input, instance) -> {
            bench.offer((Map<String, Object>) input);
            return null;
        };
        LambdaFunction myFunction = (headers, input, instance) -> {
            PostOffice po = new PostOffice(headers, instance);
            po.annotateTrace(HELLO, WORLD);
            return RETURN_VALUE;
        };
        platform.registerPrivate(TRANSACTION_JOURNAL_RECORDER, f, 1);
        platform.registerPrivate(MY_FUNCTION, myFunction, 1);
        PostOffice po = new PostOffice(FROM, TRACE_ID, "GET /api/hello/journal");
        EventEnvelope event = new EventEnvelope().setTo(MY_FUNCTION).setBody(HELLO);
        po.send(event);
        // wait for function completion
        Map<String, Object> result = bench.poll(10, TimeUnit.SECONDS);
        platform.release(TRANSACTION_JOURNAL_RECORDER);
        MultiLevelMap multi = new MultiLevelMap(result);
        Assert.assertEquals(MY_FUNCTION, multi.getElement("trace.service"));
        Assert.assertEquals(FROM, multi.getElement("trace.from"));
        Assert.assertEquals(TRACE_ID, multi.getElement("trace.id"));
        Assert.assertEquals("true", multi.getElement("trace.success"));
        Assert.assertEquals(HELLO, multi.getElement("journal.input.body"));
        Assert.assertEquals(RETURN_VALUE, multi.getElement("journal.output.body"));
        Assert.assertEquals(WORLD, multi.getElement("annotations.hello"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void telemetryTest() throws IOException, InterruptedException {
        String DISTRIBUTED_TRACE_FORWARDER = "distributed.trace.forwarder";
        BlockingQueue<Map<String, Object>> bench = new ArrayBlockingQueue<>(1);
        Platform platform = Platform.getInstance();
        String TRACE_ID = Utility.getInstance().getUuid();
        String FROM = "unit.test";
        String HELLO = "hello";
        String WORLD = "world";
        String RETURN_VALUE = "some_value";
        String SIMPLE_FUNCTION = "another.simple.function";
        LambdaFunction f = (headers, input, instance) -> {
            bench.offer((Map<String, Object>) input);
            return null;
        };
        LambdaFunction myFunction = (headers, input, instance) -> {
            PostOffice po = new PostOffice(headers, instance);
            po.annotateTrace(HELLO, WORLD);
            return RETURN_VALUE;
        };
        platform.registerPrivate(DISTRIBUTED_TRACE_FORWARDER, f, 1);
        platform.registerPrivate(SIMPLE_FUNCTION, myFunction, 1);
        PostOffice po = new PostOffice(FROM, TRACE_ID, "GET /api/hello/telemetry");
        po.send(SIMPLE_FUNCTION, HELLO);
        // wait for function completion
        Map<String, Object> result = bench.poll(10, TimeUnit.SECONDS);
        platform.release(DISTRIBUTED_TRACE_FORWARDER);
        platform.release(SIMPLE_FUNCTION);
        MultiLevelMap multi = new MultiLevelMap(result);
        Assert.assertEquals(SIMPLE_FUNCTION, multi.getElement("trace.service"));
        Assert.assertEquals(FROM, multi.getElement("trace.from"));
        Assert.assertEquals(TRACE_ID, multi.getElement("trace.id"));
        Assert.assertEquals("true", multi.getElement("trace.success"));
        Assert.assertEquals(WORLD, multi.getElement("annotations.hello"));
    }

    @Test
    public void traceHeaderTest() throws IOException, InterruptedException {
        String TRACE_DETECTOR = "trace.detector";
        String TRACE_ID = "101";
        String TRACE_PATH = "GET /api/trace";
        BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        Platform platform = Platform.getInstance();
        EventEmitter po = EventEmitter.getInstance();
        LambdaFunction f = (headers, input, instance) -> {
            if (headers.containsKey("my_route") &&
                    headers.containsKey("my_trace_id") && headers.containsKey("my_trace_path")) {
                log.info("Trace detector got {}", headers);
                return true;
            } else {
                return false;
            }
        };
        platform.registerPrivate(TRACE_DETECTOR, f, 1);
        EventEnvelope request = new EventEnvelope().setTo(TRACE_DETECTOR)
                                        .setFrom("unit.test").setTrace(TRACE_ID, TRACE_PATH).setBody("ok");
        Future<EventEnvelope> response = po.asyncRequest(request, 5000);
        response.onSuccess(result -> bench.offer(result));
        platform.release(TRACE_DETECTOR);
        EventEnvelope result = bench.poll(5, TimeUnit.SECONDS);
        assert result != null;
        Assert.assertEquals(Boolean.TRUE, result.getBody());
    }

    @Test
    public void coroutineTraceHeaderTest() throws IOException, InterruptedException {
        String COROUTINE_TRACE_DETECTOR = "coroutine.trace.detector";
        String TRACE_ID = "101";
        String TRACE_PATH = "GET /api/trace";
        BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        EventEmitter po = EventEmitter.getInstance();
        EventEnvelope request = new EventEnvelope().setTo(COROUTINE_TRACE_DETECTOR)
                                        .setFrom("unit.test").setTrace(TRACE_ID, TRACE_PATH).setBody("ok");
        Future<EventEnvelope> response = po.asyncRequest(request, 5000);
        response.onSuccess(result -> bench.offer(result));
        EventEnvelope result = bench.poll(5, TimeUnit.SECONDS);
        assert result != null;
        Assert.assertEquals(Boolean.TRUE, result.getBody());
    }

    @SuppressWarnings("unchecked")
    @Test
    public void pingTest() throws IOException, InterruptedException {
        BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        Platform platform = Platform.getInstance();
        EventEmitter po = EventEmitter.getInstance();
        Future<EventEnvelope> asyncResponse = po.ping(HELLO_WORLD, 5000);
        asyncResponse.onSuccess(bench::offer);
        EventEnvelope result = bench.poll(5, TimeUnit.SECONDS);
        Assert.assertNotNull(result);
        // ping does not execute the target function so execution time should be zero
        Assert.assertEquals(0, result.getExecutionTime(), 0.0);
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
        LambdaFunction callback = (headers, input, instance) -> {
            if (input instanceof Map) {
                if (MESSAGE.equals(((Map<String, Object>) input).get("body"))) {
                    bench.offer(DONE);
                }
            }
            return null;
        };
        platform.registerPrivate(CALLBACK, callback, 1);
        PostOffice po = new PostOffice("unit.test", "222", "/broadcast/test");
        po.send(new EventEnvelope().setTo(HELLO_WORLD).setBody(MESSAGE).setReplyTo(CALLBACK));
        String result = bench.poll(10, TimeUnit.SECONDS);
        Assert.assertEquals(DONE, result);
        // these are drop-n-forget since there are no reply-to address
        po.send(HELLO_WORLD, new Kv("test", "message"), new Kv("key", "value"));
        po.send(HELLO_WORLD, "some message", new Kv("hello", "world"));
        po.broadcast(HELLO_WORLD, "another message");
        po.broadcast(HELLO_WORLD, "another message", new Kv("key", "value"));
        po.broadcast(HELLO_WORLD, new Kv("hello", "world"), new Kv("key", "value"));
        // this one has replyTo
        po.broadcast(new EventEnvelope().setTo(HELLO_WORLD).setBody(MESSAGE).setReplyTo(CALLBACK));
        result = bench.poll(10, TimeUnit.SECONDS);
        Assert.assertEquals(DONE, result);
        platform.release(CALLBACK);
    }

    @Test
    public void eventHasFromAddress() throws IOException, InterruptedException {
        String FIRST = "hello.world.one";
        String SECOND = "hello.world.two";
        Platform platform = Platform.getInstance();
        EventEmitter emitter = EventEmitter.getInstance();
        LambdaFunction f1 = (headers, input, instance) -> {
            PostOffice po = new PostOffice(headers, instance);
            po.send(SECOND, true);
            return Optional.empty();
        };
        platform.register(FIRST, f1, 1);
        platform.register(SECOND, new SimpleInterceptor(), 1);
        // without tracing
        emitter.send(FIRST, Optional.empty());
        String result = interceptorBench.poll(5, TimeUnit.SECONDS);
        // validate the "from" address
        Assert.assertEquals(FIRST, result);
    }

    @Test
    public void singleRequestWithTimeout() throws IOException, InterruptedException {
        BlockingQueue<Throwable> bench = new ArrayBlockingQueue<>(1);
        EventEmitter po = EventEmitter.getInstance();
        EventEnvelope request = new EventEnvelope().setTo("hello.world").setBody(2);
        po.asyncRequest(request, 800).onFailure(bench::offer);
        Throwable ex = bench.poll(10, TimeUnit.SECONDS);
        assert ex != null;
        Assert.assertEquals("Timeout for 800 ms", ex.getMessage());
        Assert.assertEquals(TimeoutException.class, ex.getClass());
    }

    @Test
    public void singleRequestWithException() throws IOException, InterruptedException {
        BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        EventEmitter po = EventEmitter.getInstance();
        EventEnvelope request = new EventEnvelope().setTo("hello.world").setFrom("unit.test")
                                    .setTrace("100", "TEST /timeout/exception")
                                    .setHeader("exception", true).setBody(1);
        po.asyncRequest(request, 800).onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertEquals(400, response.getStatus());
        Assert.assertEquals("just a test", response.getError());
        Assert.assertEquals(AppException.class, response.getException().getClass());
    }

    @Test
    @SuppressWarnings("unchecked")
    public void singleRequest() throws IOException, InterruptedException {
        BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        int input = 111;
        EventEmitter po = EventEmitter.getInstance();
        EventEnvelope request = new EventEnvelope().setTo("hello.world").setHeader("a", "b").setBody(input);
        po.asyncRequest(request, 800).onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
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
        final EventEmitter po = EventEmitter.getInstance();
        final LambdaFunction f = (headers, input, instance) -> input;
        platform.registerPrivate(SERVICE, f, 1);
        EventEnvelope request = new EventEnvelope().setTo(SERVICE)
                                    .setBody(TEXT).setTrace("1030", "TEST /api/async/request");
        Future<EventEnvelope> future = po.asyncRequest(request, 1500);
        future.onSuccess(event -> {
            platform.release(SERVICE);
            success.offer(event);
        });
        EventEnvelope result = success.poll(5, TimeUnit.SECONDS);
        Assert.assertNotNull(result);
        Assert.assertEquals(200, result.getStatus());
        Assert.assertEquals(TEXT, result.getBody());
    }

    @Test
    public void futureExceptionAsResult() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> completion = new ArrayBlockingQueue<>(1);
        int STATUS = 400;
        String ERROR = "some exception";
        String SERVICE = "hello.future.3";
        String TEXT = "hello world";
        Platform platform = Platform.getInstance();
        EventEmitter po = EventEmitter.getInstance();
        LambdaFunction f = (headers, input, instance) -> {
            throw new AppException(STATUS, ERROR);
        };
        platform.registerPrivate(SERVICE, f, 1);
        Future<EventEnvelope> future = po.asyncRequest(new EventEnvelope().setTo(SERVICE).setBody(TEXT), 5000);
        future.onSuccess(event -> {
            platform.release(SERVICE);
            completion.offer(event);
        });
        EventEnvelope result = completion.poll(5, TimeUnit.SECONDS);
        Assert.assertNotNull(result);
        Assert.assertEquals(STATUS, result.getStatus());
        Assert.assertEquals(ERROR, result.getBody());
    }

    @Test
    public void asyncForkJoinTest() throws IOException, InterruptedException {
        final BlockingQueue<List<EventEnvelope>> success = new ArrayBlockingQueue<>(1);
        final String from = "unit.test";
        final String traceId = "1020";
        final String tracePath = "TEST /async/fork-n-join";
        final String SERVICE = "hello.future.3";
        final String TEXT = "hello world";
        final int PARALLEL_INSTANCES = 5;
        final Platform platform = Platform.getInstance();
        final EventEmitter po = EventEmitter.getInstance();
        final LambdaFunction f1 = (headers, input, instance) -> input;
        platform.registerPrivate(SERVICE, f1, PARALLEL_INSTANCES);
        List<EventEnvelope> requests = new ArrayList<>();
        for (int i=1; i < PARALLEL_INSTANCES + 1; i++) {
            EventEnvelope req = new EventEnvelope().setTo(SERVICE).setBody(TEXT + "." + i)
                                    .setFrom(from).setTrace(traceId, tracePath);
            requests.add(req);
        }
        Future<List<EventEnvelope>> future = po.asyncRequest(requests, 1500);
        future.onSuccess(event -> {
            platform.release(SERVICE);
            success.offer(event);
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
        final String TEXT = "hello world";
        final int PARALLEL_INSTANCES = 5;
        final Platform platform = Platform.getInstance();
        final EventEmitter po = EventEmitter.getInstance();
        final LambdaFunction f1 = (headers, input, instance) -> {
            log.info("Received event {}, {}", headers, input);
            return input;
        };
        platform.registerPrivate(SERVICE, f1, PARALLEL_INSTANCES);
        List<EventEnvelope> requests = new ArrayList<>();
        for (int i=1; i <= PARALLEL_INSTANCES; i++) {
            requests.add(new EventEnvelope().setTo(SERVICE).setBody(TEXT + "." + i)
                            .setHeader("timeout_exception", true)
                            .setHeader("seq", i));
        }
        requests.add(new EventEnvelope().setTo("hello.world").setBody(2));
        Future<List<EventEnvelope>> future = po.asyncRequest(requests, TIMEOUT, true);
        future.onFailure(exception::offer);
        Throwable e = exception.poll(5, TimeUnit.SECONDS);
        Assert.assertNotNull(e);
        Assert.assertEquals(TimeoutException.class, e.getClass());
        Assert.assertEquals("Timeout for "+TIMEOUT+" ms", e.getMessage());
        platform.release(SERVICE);
    }

    @Test
    public void asyncForkJoinPartialResultTest() throws IOException, InterruptedException {
        final long TIMEOUT = 800;
        final BlockingQueue<List<EventEnvelope>> result = new ArrayBlockingQueue<>(1);
        final String SERVICE = "hello.future.5";
        final String TEXT = "hello world";
        final int PARALLEL_INSTANCES = 5;
        final Platform platform = Platform.getInstance();
        final EventEmitter po = EventEmitter.getInstance();
        final LambdaFunction f1 = (headers, input, instance) -> {
            log.info("Received event {}, {}", headers, input);
            return input;
        };
        platform.registerPrivate(SERVICE, f1, PARALLEL_INSTANCES);
        List<EventEnvelope> requests = new ArrayList<>();
        for (int i=1; i <= PARALLEL_INSTANCES; i++) {
            requests.add(new EventEnvelope().setTo(SERVICE).setBody(TEXT + "." + i)
                    .setHeader("partial_result", true)
                    .setHeader("seq", i));
        }
        // hello.world will make an artificial delay of one second so that it will not be included in the result set.
        requests.add(new EventEnvelope().setTo("hello.world").setBody(2));
        Future<List<EventEnvelope>> future = po.asyncRequest(requests, TIMEOUT, false);
        future.onSuccess(result::offer);
        List<EventEnvelope> responses = result.poll(10, TimeUnit.SECONDS);
        assert responses != null;
        Assert.assertEquals(PARALLEL_INSTANCES, responses.size());
        for (EventEnvelope evt: responses) {
            Assert.assertNotNull(evt.getBody());
            Assert.assertTrue(evt.getBody().toString().startsWith(TEXT));
        }
        platform.release(SERVICE);
    }

    @SuppressWarnings("unchecked")
    @Test
    public void nonBlockingRpcTest() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        final String HELLO_WORLD = "hello world";
        EventEnvelope request = new EventEnvelope().setTo("long.running.rpc.alias").setBody(HELLO_WORLD)
                        .setHeader("timeout", 2000)
                        .setTrace("10000", "/api/non-blocking/rpc").setFrom("unit.test");
        EventEmitter.getInstance().asyncRequest(request, 5000).onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        MultiLevelMap map = new MultiLevelMap((Map<String, Object>) response.getBody());
        Assert.assertEquals(2, map.getElement("body"));
        Assert.assertEquals(HELLO_WORLD, map.getElement("headers.body"));
    }

    @Test
    public void nonBlockingRpcTimeoutTest() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        final String HELLO_WORLD = "hello world";
        EventEnvelope request = new EventEnvelope().setTo("long.running.rpc.alias").setBody(HELLO_WORLD)
                .setHeader("timeout", 500)
                .setTrace("10001", "/api/non-blocking/rpc").setFrom("unit.test");
        /*
         * Since it is the nested service that throws TimeoutException,
         * the exception is transported as a regular response event.
         */
        EventEmitter.getInstance().asyncRequest(request, 5000).onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertEquals(408, response.getStatus());
        Assert.assertEquals("Timeout for 500 ms", response.getError());
    }

    @SuppressWarnings("unchecked")
    @Test
    public void nonBlockingForkAndJoinTest() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        final String FORK_N_JOIN = "fork-n-join";
        final String HELLO_WORLD = "hello world";
        EventEnvelope request = new EventEnvelope().setTo("long.running.rpc").setBody(HELLO_WORLD)
                .setFrom("unit.test")
                .setHeader(FORK_N_JOIN, true)
                .setHeader("timeout", 2000)
                .setTrace("20000", "/api/non-blocking/fork-n-join");
        EventEmitter.getInstance().asyncRequest(request, 5000).onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        MultiLevelMap map = new MultiLevelMap((Map<String, Object>) response.getBody());
        Assert.assertEquals(4, map.getMap().size());
        Assert.assertEquals(0, map.getElement("cid-0.body"));
        Assert.assertEquals(HELLO_WORLD, map.getElement("cid-0.headers.body"));
        Assert.assertEquals("true", map.getElement("cid-0.headers." + FORK_N_JOIN));
        Assert.assertEquals(1, map.getElement("cid-1.body"));
        Assert.assertEquals(HELLO_WORLD, map.getElement("cid-1.headers.body"));
        Assert.assertEquals("true", map.getElement("cid-1.headers." + FORK_N_JOIN));
        Assert.assertEquals(2, map.getElement("cid-2.body"));
        Assert.assertEquals(HELLO_WORLD, map.getElement("cid-2.headers.body"));
        Assert.assertEquals("true", map.getElement("cid-2.headers." + FORK_N_JOIN));
        Assert.assertEquals(3, map.getElement("cid-3.body"));
        Assert.assertEquals(HELLO_WORLD, map.getElement("cid-3.headers.body"));
        Assert.assertEquals("true", map.getElement("cid-3.headers." + FORK_N_JOIN));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void nonBlockingForkAndJoinTimeoutTest() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        final String FORK_N_JOIN = "fork-n-join";
        final String HELLO_WORLD = "hello world";
        EventEnvelope request = new EventEnvelope().setTo("long.running.rpc").setBody(HELLO_WORLD)
                .setFrom("unit.test")
                .setHeader(FORK_N_JOIN, true)
                .setHeader("timeout", 500)
                .setTrace("20000", "/api/non-blocking/fork-n-join");
        EventEmitter.getInstance().asyncRequest(request, 5000).onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        MultiLevelMap map = new MultiLevelMap((Map<String, Object>) response.getBody());
        // since 2 requests will time out with artificial delay of one second, there will only be 2 responses.
        Assert.assertEquals(2, map.getMap().size());
        Assert.assertEquals(1, map.getElement("cid-1.body"));
        Assert.assertEquals(HELLO_WORLD, map.getElement("cid-1.headers.body"));
        Assert.assertEquals("true", map.getElement("cid-1.headers." + FORK_N_JOIN));
        Assert.assertEquals(3, map.getElement("cid-3.body"));
        Assert.assertEquals(HELLO_WORLD, map.getElement("cid-3.headers.body"));
        Assert.assertEquals("true", map.getElement("cid-3.headers." + FORK_N_JOIN));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void remoteEventApiTest() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        long TIMEOUT = 3000;
        int NUMBER_THREE = 3;
        PostOffice po = new PostOffice("unit.test", "123", "TEST /remote/event");
        EventEnvelope event = new EventEnvelope().setTo("hello.world")
                .setBody(NUMBER_THREE).setHeader("hello", "world");
        Future<EventEnvelope> response = po.asyncRequest(event, TIMEOUT,
                "http://127.0.0.1:"+port+"/api/event", true);
        response.onSuccess(bench::offer);
        EventEnvelope result = bench.poll(TIMEOUT, TimeUnit.MILLISECONDS);
        Assert.assertNotNull(result);
        Assert.assertEquals(200, result.getStatus());
        Assert.assertTrue(result.getBody() instanceof Map);
        MultiLevelMap map = new MultiLevelMap((Map<String, Object>) result.getBody());
        Assert.assertEquals("world", map.getElement("headers.hello"));
        Assert.assertEquals(NUMBER_THREE, map.getElement("body"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void remoteEventApiKotlinTest() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        long TIMEOUT = 3000;
        int NUMBER_THREE = 3;
        PostOffice po = new PostOffice("unit.test", "123", "TEST /remote/event");
        EventEnvelope event = new EventEnvelope().setTo("hello.world")
                .setBody(NUMBER_THREE).setHeader("hello", "world");
        EventEnvelope forward = new EventEnvelope().setTo("event.api.forwarder")
                .setBody(event.toBytes()).setHeader("timeout", TIMEOUT).setHeader("rpc", true)
                .setHeader("endpoint", "http://127.0.0.1:"+port+"/api/event");
        Future<EventEnvelope> response = po.asyncRequest(forward, TIMEOUT);
        response.onSuccess(bench::offer);
        EventEnvelope result = bench.poll(TIMEOUT, TimeUnit.MILLISECONDS);
        Assert.assertNotNull(result);
        Assert.assertEquals(200, result.getStatus());
        Assert.assertTrue(result.getBody() instanceof Map);
        MultiLevelMap map = new MultiLevelMap((Map<String, Object>) result.getBody());
        Assert.assertEquals("world", map.getElement("headers.hello"));
        Assert.assertEquals(NUMBER_THREE, map.getElement("body"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void remoteEventApiWithLargePayloadTest() throws IOException, InterruptedException {
        // create a large payload of 100 KB
        StringBuilder sb = new StringBuilder();
        for (int i=0; i < 10000; i++) {
            sb.append("123456789.");
        }
        String PAYLOAD = sb.toString();
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        long TIMEOUT = 3000;
        PostOffice po = new PostOffice("unit.test", "1230", "TEST /remote/event/large");
        EventEnvelope event = new EventEnvelope();
        event.setTo("hello.world").setBody(PAYLOAD).setHeader("hello", "world");
        Future<EventEnvelope> response = po.asyncRequest(event, TIMEOUT,
                "http://127.0.0.1:"+port+"/api/event", true);
        response.onSuccess(bench::offer);
        // add 500 ms to the bench to capture HTTP-408 response if any
        EventEnvelope result = bench.poll(TIMEOUT + 500, TimeUnit.MILLISECONDS);
        Assert.assertNotNull(result);
        Assert.assertEquals(200, result.getStatus());
        Assert.assertTrue(result.getBody() instanceof Map);
        MultiLevelMap map = new MultiLevelMap((Map<String, Object>) result.getBody());
        Assert.assertEquals("world", map.getElement("headers.hello"));
        Assert.assertEquals(PAYLOAD, map.getElement("body"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void remoteEventApiWithLargePayloadKotlinTest() throws IOException, InterruptedException {
        // create a large payload of 100 KB
        StringBuilder sb = new StringBuilder();
        for (int i=0; i < 10000; i++) {
            sb.append("123456789.");
        }
        String PAYLOAD = sb.toString();
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        long TIMEOUT = 3000;
        PostOffice po = new PostOffice("unit.test", "1231", "TEST /remote/event/large/k");
        EventEnvelope event = new EventEnvelope().setTo("hello.world").setBody(PAYLOAD).setHeader("hello", "world");
        EventEnvelope forward = new EventEnvelope().setTo("event.api.forwarder")
                .setBody(event.toBytes()).setHeader("timeout", TIMEOUT).setHeader("rpc", true)
                .setHeader("endpoint", "http://127.0.0.1:"+port+"/api/event");
        Future<EventEnvelope> response = po.asyncRequest(forward, TIMEOUT);
        response.onSuccess(bench::offer);
        // add 500 ms to the bench to capture HTTP-408 response if any
        EventEnvelope result = bench.poll(TIMEOUT + 500, TimeUnit.MILLISECONDS);

        Assert.assertNotNull(result);
        Assert.assertEquals(200, result.getStatus());
        Assert.assertTrue(result.getBody() instanceof Map);
        MultiLevelMap map = new MultiLevelMap((Map<String, Object>) result.getBody());
        Assert.assertEquals("world", map.getElement("headers.hello"));
        Assert.assertEquals(PAYLOAD, map.getElement("body"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void remoteEventApiOneWayTest() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        long TIMEOUT = 3000;
        int NUMBER_THREE = 3;
        PostOffice po = new PostOffice("unit.test", "12002", "TEST /remote/event/oneway");
        EventEnvelope event = new EventEnvelope();
        event.setTo("hello.world").setBody(NUMBER_THREE).setHeader("hello", "world");
        Future<EventEnvelope> response = po.asyncRequest(event, TIMEOUT,
                "http://127.0.0.1:"+port+"/api/event", false);
        response.onSuccess(bench::offer);
        EventEnvelope result = bench.poll(5, TimeUnit.SECONDS);
        Assert.assertNotNull(result);
        // status code 202 indicates that a drop-n-forget event has been sent asynchronously
        Assert.assertEquals(202, result.getStatus());
        Assert.assertTrue(result.getBody() instanceof Map);
        Map<String, Object> map = (Map<String, Object>) result.getBody();
        Assert.assertTrue(map.containsKey("time"));
        Assert.assertEquals("async", map.get("type"));
        Assert.assertEquals(true, map.get("delivered"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void remoteEventApiOneWayKotlinTest() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        long TIMEOUT = 3000;
        int NUMBER_THREE = 3;
        PostOffice po = new PostOffice("unit.test", "12003", "TEST /remote/event/oneway/k");
        EventEnvelope event = new EventEnvelope();
        event.setTo("hello.world").setBody(NUMBER_THREE).setHeader("hello", "world");
        EventEnvelope forward = new EventEnvelope().setTo("event.api.forwarder")
                .setBody(event.toBytes()).setHeader("timeout", TIMEOUT).setHeader("rpc", false)
                .setHeader("endpoint", "http://127.0.0.1:"+port+"/api/event");
        Future<EventEnvelope> response = po.asyncRequest(forward, TIMEOUT);
        response.onSuccess(bench::offer);
        EventEnvelope result = bench.poll(5, TimeUnit.SECONDS);
        Assert.assertNotNull(result);
        // status code 202 indicates that a drop-n-forget event has been sent asynchronously
        Assert.assertEquals(202, result.getStatus());
        Assert.assertTrue(result.getBody() instanceof Map);
        Map<String, Object> map = (Map<String, Object>) result.getBody();
        Assert.assertTrue(map.containsKey("time"));
        Assert.assertEquals("async", map.get("type"));
        Assert.assertEquals(true, map.get("delivered"));
    }

    @Test
    @SuppressWarnings("unchecked")
    public void multilevelTrace() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        final String ROUTE_ONE = "hello.level.1";
        final String ROUTE_TWO = "hello.level.2";
        final String TRACE_ID = "cid-123456";
        final String TRACE_PATH = "GET /api/hello/world";
        Platform platform = Platform.getInstance();
        LambdaFunction tier2 = (headers, input, instance) -> {
            PostOffice po = new PostOffice(headers, instance);
            Assert.assertEquals(ROUTE_TWO, po.getRoute());
            Assert.assertEquals(TRACE_ID, po.getTraceId());
            // annotations are local to a service and should not be transported to the next service
            Assert.assertTrue(po.getTrace().annotations.isEmpty());
            return po.getTraceId();
        };
        platform.register(ROUTE_TWO, tier2, 1);
        // test tracing to 2 levels
        String testMessage = "some message";
        EventEnvelope event = new EventEnvelope();
        event.setTo(ROUTE_ONE).setHeader("hello", "world").setBody(testMessage);
        event.setTrace(TRACE_ID, TRACE_PATH).setFrom("unit.test");
        EventEmitter po = EventEmitter.getInstance();
        po.asyncRequest(event, 5000).onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertEquals(HashMap.class, response.getBody().getClass());
        MultiLevelMap map = new MultiLevelMap((Map<String, Object>) response.getBody());
        Assert.assertEquals("world", map.getElement("headers.hello"));
        Assert.assertEquals(testMessage, map.getElement("body"));
        Assert.assertEquals(TRACE_ID, map.getElement("trace_id"));
        Assert.assertEquals(TRACE_PATH, map.getElement("trace_path"));
        Assert.assertEquals(ROUTE_ONE, map.getElement("route_one"));
    }

    @Test
    @SuppressWarnings("unchecked")
    public void routeSubstitution() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        int input = 111;
        EventEmitter po = EventEmitter.getInstance();
        // with route substitution in the application.properties, hello.test will route to hello.world
        EventEnvelope request = new EventEnvelope().setTo("hello.test").setBody(input);
        po.asyncRequest(request, 800).onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertEquals(HashMap.class, response.getBody().getClass());
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertEquals(input, result.get("body"));
        Map<String, String> list = po.getRouteSubstitutionList();
        Assert.assertTrue(list.containsKey("hello.test"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void healthTest() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        Platform platform = Platform.getInstance();
        EventEmitter po = EventEmitter.getInstance();
        EventEnvelope request = new EventEnvelope().setTo(EventEmitter.ACTUATOR_SERVICES).setHeader("type" ,"health");
        po.asyncRequest(request, 5000).onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> map = (Map<String, Object>) response.getBody();
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

    @SuppressWarnings("unchecked")
    @Test
    public void infoTest() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        EventEmitter po = EventEmitter.getInstance();
        EventEnvelope request = new EventEnvelope().setTo(EventEmitter.ACTUATOR_SERVICES).setHeader("type" ,"info");
        po.asyncRequest(request, 5000).onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertTrue(result.containsKey("app"));
        Assert.assertTrue(result.containsKey("memory"));
        Assert.assertTrue(result.containsKey("personality"));
        Assert.assertTrue(result.containsKey("vm"));
        Assert.assertTrue(result.containsKey("streams"));
        Assert.assertTrue(result.containsKey("origin"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void libTest() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        EventEmitter po = EventEmitter.getInstance();
        EventEnvelope request = new EventEnvelope().setTo(EventEmitter.ACTUATOR_SERVICES).setHeader("type" ,"lib");
        po.asyncRequest(request, 5000).onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertTrue(result.containsKey("app"));
        Assert.assertTrue(result.containsKey("library"));
        System.out.println(result);
    }

    @SuppressWarnings("unchecked")
    @Test
    public void infoRouteTest() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        String MY_FUNCTION = "my.test.function";
        String ANOTHER_FUNCTION = "another.function";
        EventEmitter po = EventEmitter.getInstance();
        EventEnvelope request = new EventEnvelope().setTo(EventEmitter.ACTUATOR_SERVICES).setHeader("type" ,"routes");
        po.asyncRequest(request, 5000).onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        MultiLevelMap multi = new MultiLevelMap((Map<String, Object>) response.getBody());
        Object journalRoutes = multi.getElement("journal");
        Assert.assertTrue(journalRoutes instanceof List);
        List<String> routes = (List<String>) journalRoutes;
        Assert.assertTrue(routes.contains(MY_FUNCTION));
        Assert.assertTrue(routes.contains(ANOTHER_FUNCTION));
    }

    @Test
    public void livenessProbeTest() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        EventEmitter po = EventEmitter.getInstance();
        EventEnvelope request = new EventEnvelope()
                                    .setTo(EventEmitter.ACTUATOR_SERVICES).setHeader("type" ,"livenessprobe");
        po.asyncRequest(request, 5000).onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertEquals("OK", response.getBody());
    }

    @SuppressWarnings("unchecked")
    @Test
    public void envTest() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        EventEmitter po = EventEmitter.getInstance();
        EventEnvelope request = new EventEnvelope()
                .setTo(EventEmitter.ACTUATOR_SERVICES).setHeader("type" ,"env");
        po.asyncRequest(request, 5000).onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertTrue(response.getBody() instanceof Map);
        Map<String, Object> result = (Map<String, Object>) response.getBody();
        Assert.assertTrue(result.containsKey("app"));
        Assert.assertTrue(result.containsKey("routing"));
        Assert.assertTrue(result.containsKey("env"));
    }

    @Test
    public void resumeTest() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        final String USER = "user";
        final String WHEN = "when";
        EventEmitter po = EventEmitter.getInstance();
        EventEnvelope request = new EventEnvelope()
                .setTo(EventEmitter.ACTUATOR_SERVICES).setHeader("type" ,"resume")
                .setHeader(USER, "someone").setHeader(WHEN, "now");
        po.asyncRequest(request, 5000).onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertEquals(false, response.getBody());
    }

    @Test
    public void envelopeAsResponseTest() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        String TARGET = "test.route.1";
        String MESSAGE = "hello world";
        EventEmitter po = EventEmitter.getInstance();
        Platform.getInstance().register(TARGET, new EventEnvelopeReader(), 1);
        EventEnvelope request = new EventEnvelope().setTo(TARGET).setBody(MESSAGE);
        po.asyncRequest(request, 5000).onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertEquals(MESSAGE, response.getBody());
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
        LambdaFunction f= (headers, input, instance) -> {
            int n = counter.incrementAndGet();
            long id = Thread.currentThread().getId();
            log.debug("Instance #{}, count={}, thread #{} {}", instance, n, id, input);
            threads.put(id, true);
            if (n == CYCLES) {
                last.set(System.currentTimeMillis());
                bench.offer(true);
            }
            return true;
        };
        Platform.getInstance().registerPrivate(MULTI_CORES, f, WORKER_POOL);
        EventEmitter po = EventEmitter.getInstance();
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
        EventEmitter po = EventEmitter.getInstance();
        LambdaFunction f = (headers, input, instance) -> {
            PoJo pojo = new PoJo();
            pojo.setName((String) input);
            return pojo;
        };
        platform.registerPrivate(POJO_HAPPY_CASE, f, 1);
        platform.registerPrivate(SIMPLE_CALLBACK, new SimpleCallback(bench, TRACE_ID), 1);
        po.send(new EventEnvelope().setTo(POJO_HAPPY_CASE).setReplyTo(SIMPLE_CALLBACK).setBody(HELLO)
                        .setFrom("unit.test").setTrace(TRACE_ID, "HAPPY /10000"));
        Object result = bench.poll(10, TimeUnit.SECONDS);
        Assert.assertNotNull(result);
        Assert.assertEquals(PoJo.class, result.getClass());
        Assert.assertEquals(HELLO, ((PoJo) result).getName());
        platform.release(POJO_HAPPY_CASE);
        platform.release(SIMPLE_CALLBACK);
    }

    @Test
    public void testCallBackCastingException() throws IOException, InterruptedException {
        final BlockingQueue<Object> bench = new ArrayBlockingQueue<>(1);
        String TRACE_ID = "30000";
        String HELLO = "hello";
        String POJO_ERROR_CASE = "pojo.error.case.3";
        String SIMPLE_CALLBACK = "simple.callback.3";
        Platform platform = Platform.getInstance();
        EventEmitter po = EventEmitter.getInstance();
        LambdaFunction f = (headers, input, instance) -> HELLO;
        platform.registerPrivate(POJO_ERROR_CASE, f, 1);
        platform.registerPrivate(SIMPLE_CALLBACK, new SimpleCallback(bench, TRACE_ID), 1);
        po.send(new EventEnvelope().setTo(POJO_ERROR_CASE).setReplyTo(SIMPLE_CALLBACK).setBody(HELLO)
                .setTrace(TRACE_ID, "CAST /30000"));
        Object result = bench.poll(10, TimeUnit.SECONDS);
        Assert.assertNotNull(result);
        Assert.assertEquals(AppException.class, result.getClass());
        AppException ex = (AppException) result;
        Assert.assertEquals(500, ex.getStatus());
        Assert.assertTrue(ex.getMessage().contains("cannot be cast to"));
        platform.release(POJO_ERROR_CASE);
        platform.release(SIMPLE_CALLBACK);
    }

    @Test
    public void testInputObjectMapping() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        String TRACE_ID = "101010";
        String TRACE_PATH = "TEST /api/hello/input/mapping";
        String AUTO_MAPPING = "hello.input.mapping";
        String HELLO_WORLD = "hello world";
        String NAME = "name";
        String DATE = "date";
        String TIME = "time";
        Date now = new Date();
        LocalDateTime time = Instant.ofEpochMilli(now.getTime()).atZone(ZoneId.systemDefault()).toLocalDateTime();
        EventEmitter po = EventEmitter.getInstance();
        Map<String, Object> map = new HashMap<>();
        map.put(NAME, HELLO_WORLD);
        map.put(DATE, now);
        map.put(TIME, time);
        EventEnvelope request = new EventEnvelope().setTo(AUTO_MAPPING).setBody(map)
                                .setTrace(TRACE_ID,TRACE_PATH).setFrom("unit.test");
        po.asyncRequest(request, 5000).onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertEquals(PoJo.class, response.getBody().getClass());
        PoJo pojo = (PoJo) response.getBody();
        Assert.assertEquals(now, pojo.getDate());
        Assert.assertEquals(time, pojo.getTime());
        Assert.assertEquals(HELLO_WORLD, pojo.getName());
        // default values in PoJo
        Assert.assertEquals(0, pojo.getNumber());
        Assert.assertEquals(0L, pojo.getLongNumber());
        // the demo function is designed to return its function execution types
        Assert.assertEquals("true", response.getHeaders().get("coroutine"));
        Assert.assertEquals("false", response.getHeaders().get("suspend"));
        Assert.assertEquals("false", response.getHeaders().get("interceptor"));
        // the demo function will also echo the READ only route, trace ID and path
        Assert.assertEquals(AUTO_MAPPING, response.getHeaders().get("my_route"));
        Assert.assertEquals(TRACE_ID, response.getHeaders().get("my_trace_id"));
        Assert.assertEquals(TRACE_PATH, response.getHeaders().get("my_trace_path"));
        Assert.assertEquals("true", response.getHeaders().get("tracing"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void testPrimitiveTransport() throws IOException, InterruptedException {
        final BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        String HELLO_WORLD = "hello.world";
        int number = 101;
        EventEmitter po = EventEmitter.getInstance();
        EventEnvelope request = new EventEnvelope().setTo(HELLO_WORLD).setBody(number);
        po.asyncRequest(request, 5000).onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Map<String, Object> map = (Map<String, Object>) response.getBody();
        Assert.assertEquals(number, map.get("body"));
        Date now = new Date();
        request = new EventEnvelope().setTo(HELLO_WORLD).setBody(now);
        po.asyncRequest(request, 5000).onSuccess(bench::offer);
        response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        map = (Map<String, Object>) response.getBody();
        Assert.assertEquals(Utility.getInstance().date2str(now), map.get("body"));
    }

    private static class SimpleCallback implements TypedLambdaFunction<PoJo, Void>, PoJoMappingExceptionHandler {

        private final BlockingQueue<Object> bench;
        private final String traceId;

        public SimpleCallback(BlockingQueue<Object> bench, String traceId) {
            this.bench = bench;
            this.traceId = traceId;
        }

        @Override
        public void onError(String route, AppException e, EventEnvelope event, int instance) {
            EventEmitter po = EventEmitter.getInstance();
            TraceInfo trace = po.getTrace(route, instance);
            if (trace != null && traceId.equals(trace.id)) {
                log.info("Found trace path '{}'", trace.path);
                log.info("Caught casting exception, status={}, message={}", e.getStatus(), e.getMessage());
                bench.offer(e);
            }
        }

        @Override
        public Void handleEvent(Map<String, String> headers, PoJo body, int instance) {
            PostOffice po = new PostOffice(headers, instance);
            if (traceId.equals(po.getTraceId())) {
                log.info("Found trace path '{}'", po.getTrace().path);
                bench.offer(body);
            }
            return null;
        }

    }

    @EventInterceptor
    private static class SimpleInterceptor implements TypedLambdaFunction<EventEnvelope, Void> {

        @Override
        public Void handleEvent(Map<String, String> headers, EventEnvelope event, int instance) {
            log.info("{} received event from {}", headers, event.getFrom());
            interceptorBench.offer(event.getFrom());
            return null;
        }
    }

    private static class EventEnvelopeReader implements TypedLambdaFunction<EventEnvelope, EventEnvelope> {

        @Override
        public EventEnvelope handleEvent(Map<String, String> headers, EventEnvelope input, int instance) {
            return new EventEnvelope().setBody(input.getBody());
        }
    }

}
