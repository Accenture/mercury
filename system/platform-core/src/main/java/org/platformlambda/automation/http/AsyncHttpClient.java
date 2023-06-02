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

package org.platformlambda.automation.http;

import io.vertx.core.Future;
import io.vertx.core.Handler;
import io.vertx.core.MultiMap;
import io.vertx.core.Promise;
import io.vertx.core.buffer.Buffer;
import io.vertx.core.file.AsyncFile;
import io.vertx.core.file.FileSystem;
import io.vertx.core.file.OpenOptions;
import io.vertx.core.http.HttpMethod;
import io.vertx.ext.web.client.HttpRequest;
import io.vertx.ext.web.client.HttpResponse;
import io.vertx.ext.web.client.WebClient;
import io.vertx.ext.web.client.WebClientOptions;
import io.vertx.ext.web.codec.BodyCodec;
import io.vertx.ext.web.multipart.MultipartForm;
import org.platformlambda.automation.models.OutputStreamQueue;
import org.platformlambda.automation.services.ServiceGateway;
import org.platformlambda.core.annotations.CoroutineRunner;
import org.platformlambda.core.annotations.EventInterceptor;
import org.platformlambda.core.annotations.PreLoad;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.AsyncHttpRequest;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.TypedLambdaFunction;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.serializers.SimpleXmlParser;
import org.platformlambda.core.serializers.SimpleXmlWriter;
import org.platformlambda.core.system.*;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.*;
import java.net.URL;
import java.net.URLEncoder;
import java.nio.file.Files;
import java.util.*;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.atomic.AtomicInteger;

@CoroutineRunner
@EventInterceptor
@PreLoad(route="async.http.request", instances=200)
public class AsyncHttpClient implements TypedLambdaFunction<EventEnvelope, Void> {
    private static final Logger log = LoggerFactory.getLogger(AsyncHttpClient.class);
    private static final AtomicInteger initCounter = new AtomicInteger(0);
    private static final AtomicBoolean housekeeperNotRunning = new AtomicBoolean(true);
    private static final long HOUSEKEEPING_INTERVAL = 30 * 1000L;    // 30 seconds
    private static final long TEN_MINUTE = 10 * 60 * 1000L;
    private static final SimpleXmlParser xmlReader = new SimpleXmlParser();
    private static final SimpleXmlWriter xmlWriter = new SimpleXmlWriter();
    private static final ConcurrentMap<String, WebClient> webClients = new ConcurrentHashMap<>();
    private static final OpenOptions READ_THEN_DELETE = new OpenOptions().setRead(true).setDeleteOnClose(true);
    private static final String APPLICATION_OCTET_STREAM = "application/octet-stream";
    private static final String MULTIPART_FORM_DATA = "multipart/form-data";
    private static final String APPLICATION_JSON = "application/json";
    private static final String APPLICATION_XML = "application/xml";
    private static final String APPLICATION_JAVASCRIPT = "application/javascript";
    private static final String TEXT_PREFIX = "text/";
    private static final String REGULAR_FACTORY = "regular.";
    private static final String TRUST_ALL_FACTORY = "trust_all.";
    private static final String COOKIE = "cookie";
    private static final String DESTINATION = "destination";
    private static final String HTTP_RELAY = "async.http.request";
    private static final String GET = "GET";
    private static final String PUT = "PUT";
    private static final String POST = "POST";
    private static final String PATCH = "PATCH";
    private static final String DELETE = "DELETE";
    private static final String OPTIONS = "OPTIONS";
    private static final String HEAD = "HEAD";
    private static final String STREAM = "stream";
    private static final String STREAM_PREFIX = "stream.";
    private static final String INPUT_STREAM_SUFFIX = ".in";
    private static final String CONTENT_TYPE = "content-type";
    private static final String CONTENT_LENGTH = "content-length";
    private static final String X_CONTENT_LENGTH = "x-content-length";
    private static final String TIMEOUT = "timeout";
    /*
     * Some headers must be dropped because they are not relevant for HTTP relay
     * e.g. "content-encoding" and "transfer-encoding" will break HTTP response rendering.
     */
    private static final String[] MUST_DROP_HEADERS = { "content-encoding", "transfer-encoding", "host", "connection",
                                                        "upgrade-insecure-requests", "accept-encoding", "user-agent",
                                                        "sec-fetch-mode", "sec-fetch-site", "sec-fetch-user" };
    private final File tempDir;

    public AsyncHttpClient() {
        // create temp upload directory
        AppConfigReader reader = AppConfigReader.getInstance();
        String temp = reader.getProperty("app.temp.dir", "/tmp/temp_files_to_delete");
        tempDir = new File(temp);
        if (!tempDir.exists() && tempDir.mkdirs()) {
            log.info("Temporary work directory {} created", tempDir);
        }
        if (initCounter.incrementAndGet() == 1) {
            ServiceGateway.initialize();
            Platform.getInstance().getVertx().setPeriodic(HOUSEKEEPING_INTERVAL, t -> removeExpiredFiles());
            log.info("Housekeeper started");
        }
        if (initCounter.get() > 10000) {
            initCounter.set(10);
        }
    }

    private WebClient getWebClient(int instance, boolean trustAll) {
        String key = (trustAll? TRUST_ALL_FACTORY : REGULAR_FACTORY) + instance;
        if (webClients.containsKey(key)) {
            return webClients.get(key);
        }
        WebClientOptions options = new WebClientOptions().setUserAgent("async-http-client").setKeepAlive(true);
        options.setMaxHeaderSize(12 * 1024).setConnectTimeout(10000);
        if (trustAll) {
            options.setTrustAll(true);
        }
        WebClient client = WebClient.create(Platform.getInstance().getVertx(), options);
        log.info("Loaded HTTP web client {}", key);
        webClients.put(key, client);
        return client;
    }

    @SuppressWarnings("unchecked")
    private String queryParametersToString(AsyncHttpRequest request) {
        StringBuilder sb = new StringBuilder();
        Map<String, Object> params = request.getQueryParameters();
        if (params.isEmpty()) {
            return null;
        }
        for (Map.Entry<String, Object> kv: params.entrySet()) {
            String k = kv.getKey();
            Object v = kv.getValue();
            if (v instanceof String) {
                sb.append(k);
                sb.append('=');
                sb.append(v);
                sb.append('&');
            }
            if (v instanceof List) {
                List<String> list = (List<String>) v;
                for (String item: list) {
                    sb.append(k);
                    sb.append('=');
                    sb.append(item);
                    sb.append('&');
                }
            }
        }
        if (sb.length() == 0) {
            return null;
        }
        return sb.substring(0, sb.length()-1);
    }

    @Override
    public Void handleEvent(Map<String, String> headers, EventEnvelope input, int instance) {
        try {
            processRequest(headers, input, instance);
        } catch (Exception ex) {
            EventEnvelope response = new EventEnvelope();
            response.setException(ex).setBody(ex.getMessage());
            if (input.getReplyTo() != null) {
                if (ex instanceof AppException) {
                    AppException e = (AppException) ex;
                    response.setStatus(e.getStatus());
                } else if (ex instanceof IllegalArgumentException) {
                    response.setStatus(400);
                } else {
                    response.setStatus(500);
                }
                sendResponse(input, response);
            } else {
                log.error("Unhandled exception", ex);
            }
        }
        return null;
    }

    private HttpMethod getMethod(String method) {
        if (GET.equals(method)) {
            return HttpMethod.GET;
        }
        if (HEAD.equals(method)) {
            return HttpMethod.HEAD;
        }
        if (PUT.equals(method)) {
            return HttpMethod.PUT;
        }
        if (POST.equals(method)) {
            return HttpMethod.POST;
        }
        if (PATCH.equals(method)) {
            return HttpMethod.PATCH;
        }
        if (DELETE.equals(method)) {
            return HttpMethod.DELETE;
        }
        if (OPTIONS.equals(method)) {
            return HttpMethod.OPTIONS;
        }
        return null;
    }

    private void processRequest(Map<String, String> headers, EventEnvelope input, int instance)
            throws AppException, IOException {
        PostOffice po = new PostOffice(headers, instance);
        Utility util = Utility.getInstance();
        AsyncHttpRequest request = new AsyncHttpRequest(input.getBody());
        String method = request.getMethod();
        HttpMethod httpMethod = getMethod(method);
        if (httpMethod == null) {
            throw new AppException(405, "Method not allowed");
        }
        String targetHost = request.getTargetHost();
        if (targetHost == null) {
            throw new IllegalArgumentException("Missing target host. e.g. https://hostname");
        }
        final boolean secure;
        URL url = new URL(targetHost);
        String protocol = url.getProtocol();
        if ("http".equals(protocol)) {
            secure = false;
        } else if ("https".equals(protocol)) {
            secure = true;
        } else {
            throw new IllegalArgumentException("Protocol must be http or https");
        }
        String host = url.getHost().trim();
        if (host.length() == 0) {
            throw new IllegalArgumentException("Unable to resolve target host as domain or IP address");
        }
        int port = url.getPort();
        if (port < 0) {
            port = secure? 443 : 80;
        }
        String path = url.getPath();
        if (path.length() > 0) {
            throw new IllegalArgumentException("Target host must not contain URI path");
        }
        // normalize URI and query string
        final String uri;
        if (request.getUrl().contains("?")) {
            // when there are more than one query separator, drop the middle portion.
            int sep1 = request.getUrl().indexOf('?');
            int sep2 = request.getUrl().lastIndexOf('?');
            uri = encodeUri(util.getSafeDisplayUri(request.getUrl().substring(0, sep1)));
            String q = request.getUrl().substring(sep2+1).trim();
            if (!q.isEmpty()) {
                request.setQueryString(q);
            }
        } else {
            uri = encodeUri(util.getSafeDisplayUri(request.getUrl()));
        }
        // construct target URL
        String qs = request.getQueryString();
        String queryParams = queryParametersToString(request);
        if (queryParams != null) {
            qs = qs == null? queryParams : qs + "&" + queryParams;
        }
        String uriWithQuery = uri + (qs == null? "" : "?" + qs);
        po.annotateTrace(DESTINATION, url.getProtocol() + "://" + url.getHost() + ":" + port + uriWithQuery);
        WebClient client = getWebClient(instance, request.isTrustAllCert());
        HttpRequest<Buffer> http = client.request(httpMethod, port, host, uri).ssl(secure);
        if (qs != null) {
            Set<String> keys = new HashSet<>();
            List<String> parts = util.split(qs, "&");
            for (String kv: parts) {
                int eq = kv.indexOf('=');
                final String k;
                final String v;
                if (eq > 0) {
                    k = kv.substring(0, eq);
                    v = kv.substring(eq+1);
                } else {
                    k = kv;
                    v = "";
                }
                if (keys.contains(k)) {
                    http.addQueryParam(k, v);
                } else {
                    http.setQueryParam(k, v);
                    keys.add(k);
                }
            }
        }
        // optional read timeout
        int timeout = request.getTimeoutSeconds();
        if (timeout > 0) {
            http.timeout(timeout * 1000L);
        }
        Map<String, String> reqHeaders = request.getHeaders();
        // convert authentication session info into HTTP request headers
        Map<String, String> sessionInfo = request.getSessionInfo();
        reqHeaders.putAll(sessionInfo);
        for (Map.Entry<String, String> kv: reqHeaders.entrySet()) {
            if (allowedHeader(kv.getKey())) {
                http.putHeader(kv.getKey(), kv.getValue());
            }
        }
        // propagate X-Trace-Id when forwarding the HTTP request
        String traceId = po.getTraceId();
        if (traceId != null) {
            http.putHeader(ServiceGateway.getDefaultTraceIdLabel(), traceId);
        }
        // set cookies if any
        Map<String, String> cookies  = request.getCookies();
        StringBuilder sb = new StringBuilder();
        for (Map.Entry<String, String> kv: cookies.entrySet()) {
            sb.append(kv.getKey());
            sb.append('=');
            sb.append(URLEncoder.encode(kv.getValue(), "UTF-8"));
            sb.append("; ");
        }
        if (sb.length() > 0) {
            // remove the ending separator
            http.putHeader(COOKIE, sb.substring(0, sb.length()-2));
        }
        OutputStreamQueue queue = new OutputStreamQueue();
        HttpRequest<Void> httpRequest = http.as(BodyCodec.pipe(queue));
        Future<HttpResponse<Void>> httpResponse = null;
        // get request body if any
        String contentType = request.getHeader(CONTENT_TYPE);
        if (POST.equals(method) || PUT.equals(method) || PATCH.equals(method)) {
            Object reqBody = request.getBody();
            if (reqBody instanceof byte[]) {
                byte[] b = (byte[]) reqBody;
                httpRequest.putHeader(CONTENT_LENGTH, String.valueOf(b.length));
                httpResponse = httpRequest.sendBuffer(Buffer.buffer(b));
            }
            if (reqBody instanceof String) {
                byte[] b = util.getUTF((String) reqBody);
                httpRequest.putHeader(CONTENT_LENGTH, String.valueOf(b.length));
                httpResponse = httpRequest.sendBuffer(Buffer.buffer(b));
            }
            if (reqBody instanceof Map) {
                boolean xml = contentType != null && contentType.startsWith(APPLICATION_XML);
                byte[] b = xml? util.getUTF(xmlWriter.write(reqBody)) :
                            SimpleMapper.getInstance().getMapper().writeValueAsBytes(reqBody);
                httpRequest.putHeader(CONTENT_LENGTH, String.valueOf(b.length));
                httpResponse = httpRequest.sendBuffer(Buffer.buffer(b));
            }
            if (reqBody instanceof List) {
                byte[] b = SimpleMapper.getInstance().getMapper().writeValueAsBytes(reqBody);
                httpRequest.putHeader(CONTENT_LENGTH, String.valueOf(b.length));
                httpResponse = httpRequest.sendBuffer(Buffer.buffer(b));
            }
            final String streamId = request.getStreamRoute();
            if (reqBody == null && streamId != null && streamId.startsWith(STREAM_PREFIX)
                    && streamId.contains(INPUT_STREAM_SUFFIX)) {
                Platform.getInstance().getEventExecutor().submit(() ->
                        handleUpload(input, queue, request, httpRequest));
            }
        } else {
            httpResponse = httpRequest.send();
        }
        if (httpResponse != null) {
            httpResponse.onSuccess(new HttpResponseHandler(input, queue, request.getTimeoutSeconds()));
            httpResponse.onFailure(new HttpExceptionHandler(input, queue));
        }
    }

    private String encodeUri(String uri) throws UnsupportedEncodingException {
        Utility util = Utility.getInstance();
        List<String> parts = util.split(uri, "/");
        StringBuilder sb = new StringBuilder();
        for (String p: parts) {
            sb.append('/');
            sb.append(URLEncoder.encode(p, "UTF-8").replace("+", "%20"));
        }
        return sb.length() > 0? sb.toString() : "/";
    }

    private void handleUpload(EventEnvelope input, OutputStreamQueue queue,
                              AsyncHttpRequest request, HttpRequest<Void> httpRequest) {
        String streamId = request.getStreamRoute();
        String contentType = request.getHeader(CONTENT_TYPE);
        String method = request.getMethod();
        objectStream2file(streamId, request.getTimeoutSeconds())
            .onSuccess(temp -> {
                final Future<HttpResponse<Void>> future;
                int contentLen = request.getContentLength();
                if (contentLen > 0) {
                    String filename = request.getFileName();
                    if (contentType != null && contentType.startsWith(MULTIPART_FORM_DATA) &&
                            POST.equals(method) && filename != null) {
                        MultipartForm form = MultipartForm.create()
                                .binaryFileUpload(request.getUploadTag(), filename, temp.getPath(),
                                                    APPLICATION_OCTET_STREAM);
                        future = httpRequest.sendMultipartForm(form);
                    } else {
                        FileSystem fs = Platform.getInstance().getVertx().fileSystem();
                        AsyncFile file = fs.openBlocking(temp.getPath(), READ_THEN_DELETE);
                        future = httpRequest.sendStream(file);
                    }
                } else {
                    future = httpRequest.send();
                }
                future.onSuccess(new HttpResponseHandler(input, queue, request.getTimeoutSeconds()))
                        .onFailure(new HttpExceptionHandler(input, queue));
            })
            .onFailure(new HttpExceptionHandler(input, queue));
    }

    private void sendResponse(EventEnvelope input, EventEnvelope response) {
        response.setTo(input.getReplyTo()).setFrom(HTTP_RELAY)
                .setCorrelationId(input.getCorrelationId())
                .setTrace(input.getTraceId(), input.getTracePath());
        try {
            EventEmitter.getInstance().send(response);
        } catch (IOException e) {
            log.error("Unable to deliver response to {} - {}", input.getReplyTo(), e.getMessage());
        }
    }

    private boolean allowedHeader(String header) {
        for (String h: MUST_DROP_HEADERS) {
            if (header.equalsIgnoreCase(h)) {
                return false;
            }
        }
        return true;
    }

    private File getTempFile(String streamId) {
        int at = streamId.indexOf('@');
        return new File(tempDir, at > 0? streamId.substring(0, at) : streamId);
    }

    private Future<File> objectStream2file(String streamId, int timeoutSeconds) {
        return Future.future(promise -> {
            File temp = getTempFile(streamId);
            long timeout = Math.max(5000, timeoutSeconds * 1000L);
            AsyncObjectStreamReader in = new AsyncObjectStreamReader(streamId, timeout);
            try {
                fetchNextBlock(promise, temp, in, new FileOutputStream(temp));
            } catch (FileNotFoundException e) {
               promise.fail(e);
            }
        });
    }

    private void fetchNextBlock(Promise<File> promise, File temp, AsyncObjectStreamReader in, FileOutputStream out) {
        Future<Object> block = in.get();
        block.onSuccess(data -> {
            try {
                if (data != null) {
                    if (data instanceof byte[]) {
                        byte[] b = (byte[]) data;
                        if (b.length > 0) {
                            out.write(b);
                        }
                    }
                    if (data instanceof String) {
                        String text = (String) data;
                        if (!text.isEmpty()) {
                            out.write(Utility.getInstance().getUTF((String) data));
                        }
                    }
                    fetchNextBlock(promise, temp, in, out);
                } else {
                    in.close();
                    out.close();
                    promise.complete(temp);
                }
            } catch (IOException e) {
                promise.fail(e);
            }
        }).onFailure(promise::fail);
    }

    private void removeExpiredFiles() {
        if (housekeeperNotRunning.compareAndSet(true, false)) {
            Platform.getInstance().getEventExecutor().submit(() -> {
                try {
                    checkExpiredFiles();
                } finally {
                    housekeeperNotRunning.set(true);
                }
            });
        }
    }

    private void checkExpiredFiles() {
        /*
         * The temporary directory is used as a buffer for binary HTTP payload (including multi-part file upload).
         * They are removed immediately after relay.
         *
         * This housekeeper is designed as a "catch-all" mechanism to enable zero-maintenance.
         */
        long now = System.currentTimeMillis();
        List<File> expired = new ArrayList<>();
        File[] files = tempDir.listFiles();
        if (files != null && files.length > 0) {
            for (File f: files) {
                if (f.isFile() && now - f.lastModified() > TEN_MINUTE) {
                    expired.add(f);
                }
            }
            for (File f: expired) {
                try {
                    Files.deleteIfExists(f.toPath());
                    log.warn("Removing expired file {}", f);
                } catch (IOException e) {
                    log.error("Unable to delete expired file {} - {}", f, e.getMessage());
                }
            }
        }
    }

    private class HttpResponseHandler implements Handler<HttpResponse<Void>> {

        private final EventEnvelope input;
        private final OutputStreamQueue queue;
        private final int timeoutSeconds;

        public HttpResponseHandler(EventEnvelope input, OutputStreamQueue queue, int timeoutSeconds) {
            this.input = input;
            this.queue = queue;
            this.timeoutSeconds = Math.max(8, timeoutSeconds);
        }

        @Override
        public void handle(HttpResponse<Void> res) {
            Utility util = Utility.getInstance();
            EventEnvelope response = new EventEnvelope();
            response.setStatus(res.statusCode());
            MultiMap headers = res.headers();
            headers.forEach(kv -> response.setHeader(kv.getKey(), kv.getValue()));
            if (input.getReplyTo() != null) {
                String resContentType = res.getHeader(CONTENT_TYPE);
                String contentLen = res.getHeader(CONTENT_LENGTH);
                if (contentLen != null || isTextResponse(resContentType)) {
                    ByteArrayOutputStream out = new ByteArrayOutputStream();
                    try {
                        while (true) {
                            byte[] block = queue.read();
                            if (block == null) {
                                break;
                            } else {
                                try {
                                    out.write(block);
                                } catch (IOException e) {
                                    // ok to ignore
                                }
                            }
                        }
                    } finally {
                        queue.close();
                    }
                    byte[] b = out.toByteArray();
                    if (resContentType != null) {
                        if (resContentType.startsWith(APPLICATION_JSON)) {
                            // response body is assumed to be JSON
                            String text = util.getUTF(b).trim();
                            if (text.isEmpty()) {
                                sendResponse(input, response.setBody(new HashMap<>()));
                            } else {
                                if (text.startsWith("{") && text.endsWith("}")) {
                                    sendResponse(input, response.setBody(
                                            SimpleMapper.getInstance().getMapper().readValue(text, Map.class)));
                                } else if (text.startsWith("[") && text.endsWith("]")) {
                                    sendResponse(input, response.setBody(
                                            SimpleMapper.getInstance().getMapper().readValue(text, List.class)));
                                } else {
                                    sendResponse(input, response.setBody(text));
                                }
                            }

                        } else if (resContentType.startsWith(APPLICATION_XML)) {
                            // response body is assumed to be XML
                            String text = util.getUTF(b).trim();
                            try {
                                sendResponse(input, response.setBody(
                                        text.isEmpty() ? new HashMap<>() : xmlReader.parse(text)));
                            } catch (Exception e) {
                                sendResponse(input, response.setBody(text));
                            }
                        } else if (resContentType.startsWith(TEXT_PREFIX) ||
                                resContentType.startsWith(APPLICATION_JAVASCRIPT)) {
                            /*
                             * For API targetHost, the content-types are usually JSON or XML.
                             * HTML, CSS and JS are the best effort static file contents.
                             */
                            sendResponse(input, response.setBody(util.getUTF(b).trim()));
                        } else {
                            sendResponse(input, response.setBody(b));
                        }
                    } else {
                        sendResponse(input, response.setBody(b));
                    }
                } else {
                    Platform.getInstance().getEventExecutor().submit(() -> {
                        int len = 0;
                        ObjectStreamIO stream = null;
                        ObjectStreamWriter out = null;
                        try {
                            while (true) {
                                byte[] b = queue.read();
                                if (b == null) {
                                    break;
                                } else {
                                    if (out == null) {
                                        stream = new ObjectStreamIO(timeoutSeconds);
                                        out = new ObjectStreamWriter(stream.getOutputStreamId());
                                    }
                                    len += b.length;
                                    out.write(b);
                                }
                            }
                            if (out != null) {
                                response.setHeader(STREAM, stream.getInputStreamId())
                                        .setHeader(TIMEOUT, timeoutSeconds * 1000)
                                        .setHeader(X_CONTENT_LENGTH, len);
                                out.close();
                            } else {
                                response.setBody("");
                            }
                            sendResponse(input, response);
                        } catch (IOException e) {
                            // ok to ignore
                        } finally {
                            queue.close();
                        }
                    });
                }
            }
        }

        private boolean isTextResponse(String contentType) {
            return  contentType != null && (
                    contentType.startsWith(APPLICATION_JSON) || contentType.startsWith(APPLICATION_XML) ||
                    contentType.startsWith(TEXT_PREFIX) || contentType.startsWith(APPLICATION_JAVASCRIPT));
        }
    }

    private class HttpExceptionHandler implements Handler<Throwable> {

        private final EventEnvelope input;
        private final OutputStreamQueue queue;

        public HttpExceptionHandler(EventEnvelope input, OutputStreamQueue queue) {
            this.input = input;
            this.queue = queue;
        }

        @Override
        public void handle(Throwable ex) {
            try {
                EventEnvelope response = new EventEnvelope();
                response.setException(ex).setBody(ex.getMessage());
                if (input.getReplyTo() != null) {
                    if (ex instanceof AppException) {
                        AppException e = (AppException) ex;
                        response.setStatus(e.getStatus());
                    } else if (ex instanceof IllegalArgumentException) {
                        response.setStatus(400);
                    } else {
                        response.setStatus(500);
                    }
                    sendResponse(input, response);
                } else {
                    log.error("Unhandled exception", ex);
                }
            } finally {
                queue.close();
            }
        }
    }

}
