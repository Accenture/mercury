/*
    Copyright 2018-2022 Accenture Technology
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

import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.serializers.MsgPack;
import org.platformlambda.core.serializers.PayloadMapper;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.*;
import java.math.BigDecimal;
import java.math.RoundingMode;
import java.util.*;

public class EventEnvelope {
    private static final Logger log = LoggerFactory.getLogger(EventEnvelope.class);

    private static final MsgPack msgPack = new MsgPack();
    private static final PayloadMapper converter = PayloadMapper.getInstance();

    // message-ID
    private static final String ID = "0";
    // metrics
    private static final String EXECUTION = "1";
    private static final String ROUND_TRIP = "2";
    // extra routing information for a request from a language pack client
    private static final String EXTRA = "3";
    // route paths
    private static final String TO = "T";
    private static final String REPLY_TO = "R";
    private static final String FROM = "F";
    // status
    private static final String STATUS = "S";
    // message headers and body
    private static final String HEADERS = "H";
    private static final String BODY = "B";
    // distributed trace ID for tracking a transaction from the edge to multiple levels of services
    private static final String TRACE_ID = "t";
    private static final String TRACE_PATH = "p";
    // optional correlation ID
    private static final String CID = "X";
    // object type for automatic serialization
    private static final String OBJ_TYPE = "O";
    private static final String PARA_TYPES = "P";
    // final destination
    private static final String END_ROUTE = "E";
    // broadcast
    private static final String BROADCAST_LEVEL = "b";
    // optional
    private static final String OPTIONAL = "+";
    private static final String JSON_TRANSPORT = "j";
    // serialized exception object
    private static final String EXCEPTION = "4";
    // special header for setting HTTP cookie for rest-automation
    private static final String SET_COOKIE = "set-cookie";

    private final Map<String, String> headers = new HashMap<>();
    private String id, from, to, replyTo, traceId, tracePath, cid, extra, type, parametricType;
    private Integer status;
    private Object body;
    private Object encodedBody;
    private byte[] exceptionBytes;
    private Throwable exception;
    private Float executionTime, roundTrip;
    private boolean endOfRoute = false, binary = true, optional = false, encoded = false, exRestored = false;
    private int broadcastLevel = 0;

    public EventEnvelope() {
        this.id = Utility.getInstance().getUuid();
    }

    public EventEnvelope(byte[] event) throws IOException {
        load(event);
    }

    public String getId() {
        return id;
    }

    public String getTo() {
        return to;
    }

    public String getFrom() {
        return from;
    }

    public String getReplyTo() {
        return replyTo;
    }

    public String getTraceId() {
        return traceId;
    }

    public String getTracePath() {
        return tracePath;
    }

    public String getCorrelationId() {
        return cid;
    }

    public String getExtra() {
        return extra;
    }

    public Integer getStatus() {
        return status == null? 200 : status;
    }

    public Float getExecutionTime() {
        return executionTime == null? -1.0f : executionTime;
    }

    public Float getRoundTrip() {
        return roundTrip == null? 0.0f : roundTrip;
    }

    public int getBroadcastLevel() {
        return broadcastLevel;
    }

    public boolean isEndOfRoute() {
        return endOfRoute;
    }

    public boolean isBinary() {
        return binary;
    }

    public boolean hasError() {
        return status != null && status != 200;
    }

    public String getError() {
        if (hasError()) {
            // body is used to store error message if status is not 200
            return body == null? "null" : (body instanceof String? (String) body : body.toString());
        } else {
            return null;
        }
    }

    public Map<String, String> getHeaders() {
        return headers;
    }

    public String getParametricType() {
        return parametricType;
    }

    public String getType() {
        return type;
    }

    /**
     * Get raw form of event body in Map or Java primitive
     * <p>
     * @return body in map or primitive form
     */
    public Object getRawBody() {
        return body;
    }

    /**
     * Get event body
     *
     * @return original or restored event body
     */
    @SuppressWarnings("unchecked")
    public Object getBody() {
        if (!encoded) {
            if (type == null) {
                setBody(body);
            }
            TypedPayload typed = new TypedPayload(type, body);
            if (parametricType != null) {
                typed.setParametricType(parametricType);
            }
            try {
                // validate class name in safe data model list
                if (typed.getType().contains(".")) {
                    SimpleMapper.getInstance().getSafeMapper(typed.getType());
                }
                Object obj = converter.decode(typed);
                if (obj instanceof PoJoList) {
                    PoJoList<Object> list = (PoJoList<Object>) obj;
                    encodedBody = list.getList();
                } else {
                    encodedBody = obj;
                }

            } catch (Exception e) {
                log.warn("Fall back to Map - {}", simpleError(e.getMessage()));
                encodedBody = body;
            }
            encoded = true;
        }
        return optional? Optional.ofNullable(encodedBody) : encodedBody;
    }

    /**
     * Get event exception if any
     *
     * @return exception cause
     */
    public Throwable getException() {
        if (!exRestored) {
            if (exceptionBytes != null) {
                try (ObjectInputStream in = new ObjectInputStream(new ByteArrayInputStream(exceptionBytes))) {
                    exception = (Throwable) in.readObject();
                } catch (IOException | ClassNotFoundException e) {
                    // ignore it because there is nothing we can do
                }
            }
            exRestored = true;
        }
        return exception;
    }

    /**
     * Convert body to another class type
     * (Best effort conversion - some fields may be lost if interface contracts are not compatible)
     * <p>
     * @param toValueType target class type
     * @param <T> class type
     * @return converted body
     */
    public <T> T getBody(Class<T> toValueType) {
        return SimpleMapper.getInstance().getMapper().readValue(body, toValueType);
    }

    /**
     * Convert body to another class type
     * (Best effort conversion - some fields may be lost if interface contracts are not compatible)
     * <p>
     * @param toValueType target class type
     * @param parameterClass one or more parameter classes
     * @param <T> class type
     * @return converted body
     */
    @SuppressWarnings("unchecked")
    public <T> T getBody(Class<T> toValueType, Class<?>... parameterClass) {
        if (parameterClass.length == 0) {
            throw new IllegalArgumentException("Missing parameter class");
        }
        StringBuilder sb = new StringBuilder();
        for (Class<?> cls: parameterClass) {
            sb.append(cls.getName());
            sb.append(',');
        }
        String parametricType = sb.substring(0, sb.length()-1);
        TypedPayload typed = new TypedPayload(toValueType.getName(), body).setParametricType(parametricType);
        try {
            return (T) converter.decode(typed);
        } catch (ClassNotFoundException e) {
            // this should not occur because the classes are given as arguments
            throw new IllegalArgumentException(e.getMessage());
        }
    }

    public EventEnvelope setId(String id) {
        this.id = id;
        return this;
    }

    /**
     * Set the target route
     *
     * @param to target route
     * @return event envelope
     */
    public EventEnvelope setTo(String to) {
        this.to = to;
        return this;
    }

    /**
     * Optionally provide the sender
     *
     * @param from the sender
     * @return event envelope
     */
    public EventEnvelope setFrom(String from) {
        this.from = from;
        return this;
    }

    /**
     * Optionally set the replyTo address
     *
     * @param replyTo route
     * @return event envelope
     */
    public EventEnvelope setReplyTo(String replyTo) {
        this.replyTo = replyTo;
        return this;
    }

    /**
     * A transaction trace should only be created by the rest-automation application
     * or a service at the "edge".
     *
     * @param traceId unique ID for distributed tracing
     * @param tracePath path at the edge such as HTTP method and URI
     * @return event envelope
     */
    public EventEnvelope setTrace(String traceId, String tracePath) {
        this.traceId = traceId;
        this.tracePath = tracePath;
        return this;
    }

    /**
     * A transaction trace should only be created by the rest-automation application
     * or a service at the "edge".
     *
     * @param traceId unique ID for distributed tracing
     * @return event envelope
     */
    public EventEnvelope setTraceId(String traceId) {
        this.traceId = traceId;
        return this;
    }

    /**
     * A transaction trace should only be created by the rest-automation application
     * or a service at the "edge".
     *
     * @param tracePath path at the edge such as HTTP method and URI
     * @return event envelope
     */
    public EventEnvelope setTracePath(String tracePath) {
        this.tracePath = tracePath;
        return this;
    }

    /**
     * Optionally set a correlation-ID
     *
     * @param cid correlation-ID
     * @return event envelope
     */
    public EventEnvelope setCorrelationId(String cid) {
        this.cid = cid;
        return this;
    }

    /**
     * This extra field is used to hold a small number of key-values for tagging purpose.
     * The language pack uses this feature to support special routing to python and node.js applications.
     *
     * @param extra language pack extra routing information
     * @return event envelope
     */
    public EventEnvelope setExtra(String extra) {
        this.extra = extra;
        return this;
    }

    private Map<String, String> extraToKeyValues() {
        Map<String, String> map = new HashMap<>();
        List<String> tags = Utility.getInstance().split(this.extra, "|");
        for (String t: tags) {
            int sep = t.indexOf('=');
            if (sep != -1) {
                map.put(t.substring(0, sep), t.substring(sep+1));
            } else {
                map.put(t, "");
            }
        }
        return map;
    }

    private String mapToString(Map<String, String> map) {
        if (map == null || map.isEmpty()) {
            return "";
        }
        StringBuilder sb = new StringBuilder();
        for (String k: map.keySet()) {
            sb.append(k);
            String v = map.get(k);
            if (v != null && v.length() > 0) {
                sb.append('=');
                sb.append(v);
            }
            sb.append('|');
        }
        return sb.substring(0, sb.length()-1);
    }

    /**
     * Add a tag without value to the extra field
     *
     * @param key without value
     * @return event envelope
     */
    public EventEnvelope addTag(String key) {
        return addTag(key, null);
    }

    /**
     * Add a tag with key-value to the extra field
     *
     * @param key for a new tag
     * @param value for a new tag
     * @return event envelope
     */
    public EventEnvelope addTag(String key, String value) {
        if (key != null && key.length() > 0) {
            Map<String, String> map = extraToKeyValues();
            map.put(key, value == null? "" : value);
            this.extra = mapToString(map);
        }
        return this;
    }

    /**
     * Remove a tag from the extra field
     *
     * @param key for a tag
     * @return event envelope
     */
    public EventEnvelope removeTag(String key) {
        if (key != null && key.length() > 0) {
            Map<String, String> map = extraToKeyValues();
            map.remove(key);
            this.extra = map.isEmpty()? null : mapToString(map);
        }
        return this;
    }

    /**
     * Retrieve a tag value
     *
     * @param key for a tag
     * @return event envelope
     */
    public String getTag(String key) {
        return key != null && key.length() > 0? extraToKeyValues().get(key) : null;
    }

    /**
     * Optionally set status code (use HTTP compatible response code) if the return object is an event envelope.
     *
     * @param status 200 for normal response
     * @return event envelope
     */
    public EventEnvelope setStatus(int status) {
        this.status = status;
        return this;
    }

    /**
     * Optionally set a parameter
     *
     * @param key of a parameter
     * @param value of a parameter
     * @return event envelope
     */
    public EventEnvelope setHeader(String key, Object value) {
        if (key != null) {
            String v;
            // null value is transported as an empty string
            if (value == null) {
                v = "";
            } else if (value instanceof String) {
                v = (String) value;
            } else if (value instanceof Date) {
                v = Utility.getInstance().date2str((Date) value);
            } else {
                v = value.toString();
            }
            if (SET_COOKIE.equalsIgnoreCase(key)) {
                if (this.headers.containsKey(key)) {
                    String prev = this.headers.get(key);
                    this.headers.put(key, prev + "|" + v);
                } else {
                    this.headers.put(key, v);
                }

            } else {
                this.headers.put(key, v);
            }
        }
        return this;
    }

    public EventEnvelope setHeaders(Map<String, String> headers) {
        // make a shallow copy
        if (headers != null) {
            for (String h : headers.keySet()) {
                this.setHeader(h, headers.get(h));
            }
        }
        return this;
    }

    /**
     * Set payload
     *
     * @param body Usually a PoJo, a Map or Java primitive
     * @return event envelope
     */
    @SuppressWarnings("unchecked")
    public EventEnvelope setBody(Object body) {
        final Object payload;
        if (body instanceof Optional) {
            this.optional = true;
            Optional<Object> o = (Optional<Object>) body;
            payload = o.orElse(null);
        } else if (body instanceof EventEnvelope) {
            EventEnvelope nested = (EventEnvelope) body;
            log.warn("Setting body from nested EventEnvelope is discouraged - system will remove the outer envelope");
            return setBody(nested.getBody());
        } else {
            payload = body;
        }
        // encode body and save object type
        this.encoded = true;
        this.encodedBody = payload instanceof Date? Utility.getInstance().date2str((Date) payload) : payload;
        TypedPayload typed = converter.encode(payload, binary);
        this.body = typed.getPayload();
        this.type = typed.getType();
        this.parametricType = typed.getParametricType();
        return this;
    }

    /**
     * Set exception cause
     *
     * @param cause of an exception
     * @return event envelope
     */
    public EventEnvelope setException(Throwable cause) {
        if (cause != null) {
            ByteArrayOutputStream out = new ByteArrayOutputStream();
            try (ObjectOutputStream stream = new ObjectOutputStream(out)) {
                stream.writeObject(cause);
                exceptionBytes = out.toByteArray();
                // for compatibility with language pack (Python and Node.js)
                this.addTag("exception");
            } catch (IOException e) {
                // this won't happen
            }
        }
        return this;
    }

    /**
     * DO NOT set this manually. The system will set it.
     *
     * @param milliseconds spent
     * @return event envelope
     */
    public EventEnvelope setExecutionTime(float milliseconds) {
        BigDecimal number = new BigDecimal(milliseconds).setScale(3, RoundingMode.HALF_EVEN);
        this.executionTime = number.floatValue();
        return this;
    }

    /**
     * DO NOT set this manually. The system will set it.
     *
     * @param milliseconds spent
     * @return event envelope
     */
    public EventEnvelope setRoundTrip(float milliseconds) {
        BigDecimal number = new BigDecimal(milliseconds).setScale(3, RoundingMode.HALF_EVEN);
        this.roundTrip = number.floatValue();
        return this;
    }

    /**
     * DO NOT set this manually. The system will set it.
     *
     * @param level 0 to 2
     * @return event envelope
     */
    public EventEnvelope setBroadcastLevel(int level) {
        this.broadcastLevel = level;
        return this;
    }

    /**
     * DO NOT set this manually. The system will set it when needed.
     *
     * @return event envelope
     */
    public EventEnvelope setEndOfRoute() {
        this.endOfRoute = true;
        return this;
    }

    /**
     * By default, payload will be encoded as binary.
     * In some rare case where the built-in MsgPack binary serialization
     * does not work, you may turn off binary mode by setting it to false.
     *
     * When it is set to false, the Java object in the payload will be encoded
     * using JSON (jackson serialization). This option should be used as the
     * last resort because it would reduce performance and increase encoded payload size.
     *
     * @param binary true or false
     * @return this EventEnvelope
     */
    public EventEnvelope setBinary(boolean binary) {
        this.binary = binary;
        return this;
    }

    /**
     * You should not set the object type normally.
     * It will be automatically set when a PoJo is set as the body.
     * This method is used by unit tests or other use cases.
     *
     * @param type of the body object
     * @return this EventEnvelope
     */
    public EventEnvelope setType(String type) {
        this.type = type;
        return this;
    }

    /**
     * For Java object with generic types, you may tell EventEnvelope to transport
     * the parameterized Class(es) so that the object can be deserialized correctly.
     *
     * @param parameterClass one or more parameter classes
     * @return this EventEnvelope
     */
    public EventEnvelope setParametricType(Class<?>... parameterClass) {
        StringBuilder sb = new StringBuilder();
        for (Class<?> cls: parameterClass) {
            sb.append(cls.getName());
            sb.append(',');
        }
        if (sb.length() > 0) {
            this.parametricType = sb.substring(0, sb.length()-1);
        }
        return this;
    }

    /**
     * For Java object with generic types, you may tell EventEnvelope to transport
     * the parameterized Class so that the object can be deserialized correctly.
     *
     * @param parametricType a single parameter class
     * @return this EventEnvelope
     */
    public EventEnvelope setParametricType(String parametricType) {
        this.parametricType = parametricType;
        return this;
    }

    public EventEnvelope copy() {
        EventEnvelope event = new EventEnvelope();
        event.encodedBody = this.encodedBody;
        event.body = this.body;
        event.setTo(this.getTo());
        event.setHeaders(this.getHeaders());
        event.setType(this.getType());
        event.setParametricType(this.getParametricType());
        event.setBroadcastLevel(this.getBroadcastLevel());
        event.setFrom(this.getFrom());
        event.setBinary(this.isBinary());
        event.setCorrelationId(this.getCorrelationId());
        if (this.isEndOfRoute()) {
            event.setEndOfRoute();
        }
        event.setExtra(this.getExtra());
        event.setStatus(this.getStatus());
        event.setReplyTo(this.getReplyTo());
        event.setTraceId(this.getTraceId());
        event.setTracePath(this.getTracePath());
        return event;
    }

    /**
     * DeSerialize the EventEnvelope from a byte array
     *
     * @param bytes encoded payload
     * @throws IOException in case of decoding errors
     */
    @SuppressWarnings("unchecked")
    public void load(byte[] bytes) throws IOException {
        Object o = msgPack.unpack(bytes);
        if (o instanceof Map) {
            Map<String, Object> message = (Map<String, Object>) o;
            if (message.containsKey(ID)) {
                id = (String) message.get(ID);
            }
            if (message.containsKey(TO)) {
                to = (String) message.get(TO);
            }
            if (message.containsKey(FROM)) {
                from = (String) message.get(FROM);
            }
            if (message.containsKey(REPLY_TO)) {
                replyTo = (String) message.get(REPLY_TO);
            }
            if (message.containsKey(TRACE_ID)) {
                traceId = (String) message.get(TRACE_ID);
            }
            if (message.containsKey(TRACE_PATH)) {
                tracePath = (String) message.get(TRACE_PATH);
            }
            if (message.containsKey(CID)) {
                cid = (String) message.get(CID);
            }
            if (message.containsKey(EXTRA)) {
                extra = (String) message.get(EXTRA);
            }
            if (message.containsKey(OPTIONAL)) {
                optional = true;
            }
            if (message.containsKey(STATUS)) {
                if (message.get(STATUS) instanceof Integer) {
                    status = (Integer) message.get(STATUS);
                } else {
                    status = Utility.getInstance().str2int(message.get(STATUS).toString());
                }
            }
            if (message.containsKey(HEADERS)) {
                setHeaders((Map<String, String>) message.get(HEADERS));
            }
            if (message.containsKey(END_ROUTE)) {
                endOfRoute = (Boolean) message.get(END_ROUTE);
            }
            if (message.containsKey(BROADCAST_LEVEL) && message.get(BROADCAST_LEVEL) instanceof Integer) {
                broadcastLevel = (Integer) message.get(BROADCAST_LEVEL);
            }
            if (message.containsKey(BODY)) {
                body = message.get(BODY);
            }
            if (message.containsKey(EXCEPTION)) {
                exceptionBytes = (byte[]) message.get(EXCEPTION);
            }
            if (message.containsKey(OBJ_TYPE)) {
                type = (String) message.get(OBJ_TYPE);
            }
            if (message.containsKey(PARA_TYPES)) {
                parametricType = (String) message.get(PARA_TYPES);
            }
            if (message.containsKey(EXECUTION)) {
                if (message.get(EXECUTION) instanceof Float) {
                    executionTime = (Float) message.get(EXECUTION);
                } else {
                    executionTime = Utility.getInstance().str2float((message.get(EXECUTION).toString()));
                }
            }
            if (message.containsKey(ROUND_TRIP)) {
                if (message.get(ROUND_TRIP) instanceof Float) {
                    roundTrip = (Float) message.get(ROUND_TRIP);
                } else {
                    roundTrip = Utility.getInstance().str2float((message.get(ROUND_TRIP).toString()));
                }
            }
            if (message.containsKey(JSON_TRANSPORT)) {
                binary = false;
            }
        }
    }

    private String simpleError(String message) {
        if (message == null) {
            return "null";
        }
        int sep = message.indexOf(": ");
        if (sep > 3) {
            String cls = message.substring(0, sep);
            return cls.contains(".")? message.substring(sep+2) : message;
        } else {
            return message;
        }
    }

    /**
     * Serialize the EventEnvelope as a byte array
     *
     * @return byte array
     * @throws IOException in case of encoding errors
     */
    public byte[] toBytes() throws IOException {
        Map<String, Object> message = new HashMap<>();
        if (id != null) {
            message.put(ID, id);
        }
        if (to != null) {
            message.put(TO, to);
        }
        if (from != null) {
            message.put(FROM, from);
        }
        if (replyTo != null) {
            message.put(REPLY_TO, replyTo);
        }
        if (traceId != null) {
            message.put(TRACE_ID, traceId);
        }
        if (tracePath != null) {
            message.put(TRACE_PATH, tracePath);
        }
        if (cid != null) {
            message.put(CID, cid);
        }
        if (extra != null) {
            message.put(EXTRA, extra);
        }
        if (status != null) {
            message.put(STATUS, status);
        }
        if (!headers.isEmpty()) {
            message.put(HEADERS, headers);
        }
        if (endOfRoute) {
            message.put(END_ROUTE, true);
        }
        if (broadcastLevel > 0) {
            message.put(BROADCAST_LEVEL, broadcastLevel);
        }
        if (optional) {
            message.put(OPTIONAL, true);
        }
        if (body != null) {
            message.put(BODY, body);
        }
        if (exceptionBytes != null) {
            message.put(EXCEPTION, exceptionBytes);
        }
        if (type != null) {
            message.put(OBJ_TYPE, type);
        }
        if (parametricType != null) {
            message.put(PARA_TYPES, parametricType);
        }
        if (executionTime != null) {
            message.put(EXECUTION, executionTime);
        }
        if (roundTrip != null) {
            message.put(ROUND_TRIP, roundTrip);
        }
        if (!binary) {
            message.put(JSON_TRANSPORT, true);
        }
        return msgPack.pack(message);
    }

}