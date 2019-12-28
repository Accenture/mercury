package org.platformlambda.automation.servlets;

import com.google.api.client.http.*;
import com.google.api.client.http.javanet.NetHttpTransport;
import org.platformlambda.automation.util.Housekeeper;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.AsyncHttpRequest;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.serializers.SimpleXmlParser;
import org.platformlambda.core.serializers.SimpleXmlWriter;
import org.platformlambda.core.system.ObjectStreamIO;
import org.platformlambda.core.system.ObjectStreamReader;
import org.platformlambda.core.system.ObjectStreamWriter;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.ws.rs.core.MediaType;
import java.io.BufferedInputStream;
import java.io.File;
import java.io.FileOutputStream;
import java.io.IOException;
import java.net.URLEncoder;
import java.security.GeneralSecurityException;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

public class HttpRelay implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(HttpRelay.class);
    private static final SimpleXmlParser xmlReader = new SimpleXmlParser();
    private static final SimpleXmlWriter xmlWriter = new SimpleXmlWriter();
    private static final ConcurrentMap<String, HttpRequestFactory> httpFactory = new ConcurrentHashMap<>();

    private static final String REGULAR_FACTORY = "regular.";
    private static final String TRUST_ALL_FACTORY = "trust_all.";
    private static final String COOKIE = "cookie";
    private static final String GET = "GET";
    private static final String PUT = "PUT";
    private static final String POST = "POST";
    private static final String DELETE = "DELETE";
    private static final String PATCH = "PATCH";
    private static final String HEAD = "HEAD";
    private static final String STREAM = "stream";
    private static final String STREAM_PREFIX = "stream.";
    private static final String USER_AGENT = "user-agent";
    private static final String CONTENT_TYPE = "content-type";
    private static final String ACCEPT = "accept";
    private static final String AUTHORIZATION = "authorization";
    private static final String BOUNDARY = "boundary";
    private static final String CONTENT_DISPOSITION = "Content-Disposition";
    private static final int BUFFER_SIZE = 2048;
    /*
     * Some headers must be dropped because they are not relevant for HTTP relay
     * e.g. "content-encoding" and "transfer-encoding" will break HTTP response rendering.
     */
    private static final String[] MUST_DROP_HEADERS = { "content-encoding", "transfer-encoding", "host", "connection",
                                                        "upgrade-insecure-requests", "accept-encoding",
                                                        "sec-fetch-mode", "sec-fetch-site", "sec-fetch-user" };
    private static File tempDir;

    public HttpRelay() {
        if (tempDir == null) {
            // create temp upload directory
            AppConfigReader reader = AppConfigReader.getInstance();
            String temp = reader.getProperty("app.temp.dir", "/tmp/downloads");
            tempDir = new File(temp);
            if (!tempDir.exists()) {
                tempDir.mkdirs();
                log.info("Temporary work directory {} created", tempDir);
            }
            Housekeeper housekeeper = new Housekeeper(tempDir);
            housekeeper.start();
        }
    }

    private HttpRequestFactory getHttpFactory(int instance, boolean trustAll) throws GeneralSecurityException {
        String key = (trustAll? TRUST_ALL_FACTORY : REGULAR_FACTORY) + instance;
        if (httpFactory.containsKey(key)) {
            return httpFactory.get(key);
        }
        if (trustAll) {
            HttpRequestFactory factory = new NetHttpTransport.Builder().doNotValidateCertificate().build().createRequestFactory();
            httpFactory.put(key, factory);
            log.info("Loaded HTTP factory {}", key);
            return factory;
        } else {
            HttpRequestFactory factory = new NetHttpTransport().createRequestFactory();
            httpFactory.put(key, factory);
            log.info("Loaded HTTP factory {}", key);
            return factory;
        }
    }

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
        Utility util = Utility.getInstance();
        AsyncHttpRequest request = new AsyncHttpRequest(body);
        String relay = request.getRelay();
        if (relay != null) {
            // select a http request factory
            HttpRequestFactory factory = getHttpFactory(instance, request.isTrustAllCert());
            // construct target URL
            String qs = request.getQueryString();
            String url = getUrl(relay, request.getUrl()) + (qs == null? "" : "?"+qs);
            boolean multipartUpload = false;
            // get request body if any
            HttpContent content = null;
            String method = request.getMethod();
            String contentType = request.getHeader(CONTENT_TYPE);
            File temp = null;
            if (POST.equals(method) || PUT.equals(method) || PATCH.equals(method)) {
                Object reqBody = request.getBody();
                if (reqBody instanceof byte[]) {
                    content = new ByteArrayContent(contentType, (byte[]) reqBody);
                }
                if (reqBody instanceof String) {
                    content = ByteArrayContent.fromString(contentType, (String) reqBody);
                }
                if (reqBody instanceof Map) {
                    boolean xml = contentType != null && contentType.startsWith(MediaType.APPLICATION_XML);
                    if (xml) {
                        String v = xmlWriter.write(reqBody);
                        content = ByteArrayContent.fromString(contentType, v);
                    } else {
                        byte[] json = SimpleMapper.getInstance().getMapper().writeValueAsBytes(reqBody);
                        content = new ByteArrayContent(contentType, json);
                    }
                }
                if (reqBody instanceof List) {
                    byte[] json = SimpleMapper.getInstance().getMapper().writeValueAsBytes(reqBody);
                    content = new ByteArrayContent(contentType, json);
                }
                if (reqBody == null) {
                    String streamId = request.getStreamRoute();
                    if (streamId != null && streamId.startsWith(STREAM_PREFIX) && streamId.contains("@")) {
                        temp = stream2file(streamId, request.getTimeoutSeconds());
                        int contentLen = request.getContentLength();
                        if (contentLen > 0) {
                            String filename = request.getFileName();
                            if (contentType != null && contentType.startsWith(MediaType.MULTIPART_FORM_DATA) &&
                                                                        POST.equals(method) && filename != null) {
                                String id = util.getUuid();
                                String tag = request.getUploadTag();
                                MultipartContent multipartContent = new MultipartContent().setMediaType(
                                        new HttpMediaType(MediaType.MULTIPART_FORM_DATA).setParameter(BOUNDARY, id));
                                FileContent fileContent = new FileContent(MediaType.APPLICATION_OCTET_STREAM, temp);
                                MultipartContent.Part part = new MultipartContent.Part(fileContent);
                                String disposition = getContentDisposition(tag, filename);
                                part.setHeaders(new HttpHeaders().set(CONTENT_DISPOSITION, disposition));
                                multipartContent.addPart(part);
                                content = multipartContent;
                                multipartUpload = true;
                            } else {
                                // uploading some raw data as bytes
                                content = new FileContent(contentType, temp);
                            }
                        }
                    }
                }
            }
            if (content == null) {
                content = ByteArrayContent.fromString(contentType, "");
            }
            // annotate trace if any
            PostOffice po = PostOffice.getInstance();
            po.annotateTrace("target", url);
            // construct outgoing HTTP request
            GenericUrl target = new GenericUrl(url);
            HttpRequest http;
            if (GET.equals(request.getMethod())) {
                http = factory.buildGetRequest(target);
            } else if (PUT.equals(request.getMethod())) {
                http = factory.buildPutRequest(target, content);
            } else if (POST.equals(request.getMethod())) {
                http = factory.buildPostRequest(target, content);
            } else if (PATCH.equals(request.getMethod())) {
                http = factory.buildPatchRequest(target, content);
            } else if (DELETE.equals(request.getMethod())) {
                http = factory.buildDeleteRequest(target);
            } else if (HEAD.equals(request.getMethod())) {
                http = factory.buildPutRequest(target, content);
            } else {
                throw new AppException(405, "Method not allowed");
            }
            boolean update = false;
            HttpHeaders httpHeaders = new HttpHeaders();
            Map<String, String> reqHeaders = request.getHeaders();
            for (String h: reqHeaders.keySet()) {
                if (allowedHeader(h) && !(multipartUpload && h.equalsIgnoreCase(CONTENT_TYPE))) {
                    if (h.equalsIgnoreCase(USER_AGENT)) {
                        http.setSuppressUserAgentSuffix(true);
                        httpHeaders.setUserAgent(reqHeaders.get(h));
                    } else if (h.equalsIgnoreCase(ACCEPT)) {
                        httpHeaders.setAccept(reqHeaders.get(h));
                    } else if (h.equalsIgnoreCase(AUTHORIZATION)) {
                        httpHeaders.setAuthorization(reqHeaders.get(h));
                    } else {
                        httpHeaders.set(h, Collections.singletonList(reqHeaders.get(h)));
                    }
                    update = true;
                }
            }
            // set cookies if any
            Map<String, String> cookies  = request.getCookies();
            StringBuilder sb = new StringBuilder();
            for (String k: cookies.keySet()) {
                String v = cookies.get(k);
                sb.append(k);
                sb.append('=');
                sb.append(URLEncoder.encode(v, "UTF-8"));
                sb.append("; ");
            }
            if (sb.length() > 0) {
                httpHeaders.set(COOKIE, sb.substring(0, sb.length()-2));
                update = true;
            }
            if (update) {
                http.setHeaders(httpHeaders);
            }
            HttpResponse response = null;
            try {
                response = http.execute();
                EventEnvelope resEvent = new EventEnvelope();
                resEvent.setStatus(response.getStatusCode());
                setResponseHeaders(resEvent, response.getHeaders());
                Long contentLen = response.getHeaders().getContentLength();
                if (contentLen == null) {
                    BufferedInputStream in = new BufferedInputStream(response.getContent());
                    int len;
                    byte[] buffer = new byte[BUFFER_SIZE];
                    ObjectStreamIO stream = null;
                    ObjectStreamWriter out = null;
                    while ((len = in.read(buffer, 0, buffer.length)) != -1) {
                        if (out == null) {
                            stream = new ObjectStreamIO(request.getTimeoutSeconds());
                            out = stream.getOutputStream();
                        }
                        out.write(buffer, 0, len);
                    }
                    if (out != null) {
                        out.close();
                        resEvent.setHeader(STREAM, stream.getRoute());
                    } else {
                        resEvent.setBody("");
                    }
                    return resEvent;
                } else {
                    String resContentType = response.getHeaders().getFirstHeaderStringValue(CONTENT_TYPE);
                    if (resContentType != null) {
                        if (resContentType.startsWith(MediaType.APPLICATION_JSON)) {
                            // request body is assumed to be JSON
                            String text = util.getUTF(util.stream2bytes(response.getContent(), false)).trim();
                            if (text.length() == 0) {
                                return resEvent.setBody(new HashMap<>());
                            } else {
                                if (text.startsWith("{") && text.endsWith("}")) {
                                    return resEvent.setBody(SimpleMapper.getInstance().getMapper().readValue(text, Map.class));
                                } else if (text.startsWith("[") && text.endsWith("]")) {
                                    return resEvent.setBody(SimpleMapper.getInstance().getMapper().readValue(text, List.class));
                                } else {
                                    return resEvent.setBody(text);
                                }
                            }

                        } else if (resContentType.startsWith(MediaType.APPLICATION_XML)) {
                            // request body is assumed to be XML
                            String text = util.getUTF(util.stream2bytes(response.getContent(), false));
                            try {
                                return resEvent.setBody(text.isEmpty()? new HashMap<>() : xmlReader.parse(text));
                            } catch (Exception e) {
                                return resEvent.setBody(text);
                            }
                        } else if (resContentType.startsWith(MediaType.TEXT_HTML) ||
                                   resContentType.startsWith(MediaType.TEXT_PLAIN)) {
                            String text = util.getUTF(util.stream2bytes(response.getContent(), false));
                            return resEvent.setBody(text);
                        }
                    }
                    return resEvent.setBody(util.stream2bytes(response.getContent(), false));
                }

            } catch (HttpResponseException e) {
                EventEnvelope resEvent = new EventEnvelope();
                resEvent.setStatus(e.getStatusCode());
                resEvent.setBody(e.getContent());
                if (response != null) {
                    setResponseHeaders(resEvent, response.getHeaders());
                }
                return resEvent;

            } finally {
                if (response != null) {
                    response.disconnect();
                }
                if (temp != null) {
                    temp.delete();
                }
            }

        }
        throw new IllegalArgumentException("Invalid request");
    }

    private void setResponseHeaders(EventEnvelope event, HttpHeaders headers) {
        for (String h: headers.keySet()) {
            /*
             * with the exception of "set-cookie" that allows multiples,
             * all other headers are set as single value.
             */
            List<String> values = headers.getHeaderStringValues(h);
            for (String v: values) {
                if (allowedHeader(h)) {
                    event.setHeader(h, v);
                }
            }
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

    private String getContentDisposition(String tag, String filename) {
        return String.format("form-data; name=\"%s\"; filename=\"%s\"",
                tag, filename.replace("\"", "\'"));
    }

    private String getUrl(String host, String url) {
        return (host.endsWith("/")? host.substring(0, host.length()-1) : host) + (url.startsWith("/")? url : "/" + url);
    }

    private File stream2file(String streamId, int timeout) throws IOException, AppException {
        Utility util = Utility.getInstance();
        File temp = new File(tempDir, util.getUuid());
        FileOutputStream out = new FileOutputStream(temp);
        ObjectStreamIO consumer = new ObjectStreamIO(streamId);
        ObjectStreamReader in = consumer.getInputStream(timeout);
        int i = 0;
        while (!in.isEof()) {
            try {
                for (Object block : in) {
                    /*
                     * only bytes or text are supported when using output stream
                     * e.g. for downloading a large file
                     */
                    if (block instanceof byte[]) {
                        out.write((byte[]) block);
                    }
                    if (block instanceof String) {
                        out.write(util.getUTF((String) block));
                    }
                }
            } catch (RuntimeException e) {
                log.warn("Input stream {} interrupted - {}", streamId, e.getMessage());
                if (e.getMessage().contains("timeout")) {
                    throw new AppException(408, e.getMessage());
                } else {
                    throw new AppException(500, e.getMessage());
                }
            } finally {
                in.close();
                out.close();
            }
        }
        return temp;
    }

}
