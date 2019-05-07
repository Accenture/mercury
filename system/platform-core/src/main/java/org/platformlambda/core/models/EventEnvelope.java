/*

    Copyright 2018-2019 Accenture Technology

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

import org.platformlambda.core.serializers.MsgPack;
import org.platformlambda.core.serializers.PayloadMapper;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.system.ServerPersonality;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
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
    // route paths
    private static final String TO = "T";
    private static final String REPLY_TO = "R";
    private static final String FROM = "F";
    // status
    private static final String STATUS = "S";
    // message headers and body
    private static final String HEADERS = "H";
    private static final String BODY = "B";
    // optional correlation ID
    private static final String CID = "X";
    // object type for automatic serialization
    private static final String OBJ_TYPE = "O";
    private static final String PARA_TYPES = "P";
    private static final String NOTES = "N";
    // final destination
    private static final String END_ROUTE = "E";
    // broadcast
    private static final String BROADCAST_LEVEL = "b";

    private String id, from, to, replyTo, cid, notes, type, parametricType;
    private Integer status;
    private Map<String, String> headers = new HashMap<>();
    private Object body;
    private Float executionTime, roundTrip;
    private boolean endOfRoute = false, binary = true;
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

    public String getCorrelationId() {
        return cid;
    }

    public String getNotes() {
        return notes;
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

    public Object getBody() {
        return body;
    }

    public EventEnvelope setId(String id) {
        this.id = id;
        return this;
    }

    public EventEnvelope setTo(String to) {
        this.to = to;
        return this;
    }

    public EventEnvelope setFrom(String from) {
        this.from = from;
        return this;
    }

    public EventEnvelope setReplyTo(String replyTo) {
        this.replyTo = replyTo;
        return this;
    }

    public EventEnvelope setCorrelationId(String cid) {
        this.cid = cid;
        return this;
    }

    public EventEnvelope setNotes(String notes) {
        this.notes = notes;
        return this;
    }

    public EventEnvelope setStatus(int status) {
        this.status = status;
        return this;
    }

    public EventEnvelope setHeader(String key, Object value) {
        if (key != null && value != null) {
            this.headers.put(key, value instanceof String? (String) value : value.toString());
        }
        return this;
    }

    public EventEnvelope setBody(Object body) {
        this.body = body;
        return this;
    }

    public EventEnvelope setExecutionTime(float milliseconds) {
        String ms = String.format("%.3f", milliseconds);
        this.executionTime = Float.parseFloat(ms);
        return this;
    }

    public EventEnvelope setRoundTrip(float milliseconds) {
        String ms = String.format("%.3f", milliseconds);
        this.roundTrip = Float.parseFloat(ms);
        return this;
    }

    public EventEnvelope setBroadcastLevel(int level) {
        this.broadcastLevel = level;
        return this;
    }

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
     * For Java object with generic types, you may tell EventEnvelope to transport
     * the parameterized Class(es) so that the object can be deserialized correctly.
     * (Note that only simple typing is supported)
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

    public EventEnvelope setParametricType(String parametricType) {
        this.parametricType = parametricType;
        return this;
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
            if (message.containsKey(CID)) {
                cid = (String) message.get(CID);
            }
            if (message.containsKey(NOTES)) {
                notes = (String) message.get(NOTES);
            }
            if (message.containsKey(STATUS)) {
                if (message.get(STATUS) instanceof Integer) {
                    status = (Integer) message.get(STATUS);
                } else {
                    status = Utility.getInstance().str2int(message.get(STATUS).toString());
                }
            }
            if (message.containsKey(HEADERS)) {
                Map<String, String> map = (Map<String, String>) message.get(HEADERS);
                for (String h: map.keySet()) {
                    setHeader(h, map.get(h));
                }
            }
            if (message.containsKey(END_ROUTE)) {
                endOfRoute = (Boolean) message.get(END_ROUTE);
            }
            if (message.containsKey(BROADCAST_LEVEL) && message.get(BROADCAST_LEVEL) instanceof Integer) {
                broadcastLevel = (Integer) message.get(BROADCAST_LEVEL);
            }
            if (message.containsKey(PARA_TYPES)) {
                parametricType = (String) message.get(PARA_TYPES);
            }
            if (message.containsKey(BODY) && message.containsKey(OBJ_TYPE)) {
                TypedPayload typed = new TypedPayload((String) message.get(OBJ_TYPE), message.get(BODY));
                // for binary encoding, the payload is a Map
                if (typed.getType().contains(".")) {
                    binary = typed.getPayload() instanceof Map;
                }
                if (parametricType != null) {
                    typed.setParametricType(parametricType);
                }
                try {
                    // validate class name in white list if any
                    SimpleMapper.getInstance().getWhiteListMapper(typed.getType());
                    body = converter.decode(typed);
                } catch (Exception e) {
                    /*
                     * When the EventEnvelope is being relayed, the event node does not have the PoJo class
                     * so this will throw exception. Therefore, the object type must be preserved.
                     *
                     * The target microservice should be able to restore the object properly
                     * assuming source and target have the same PoJo class definition.
                     */
                    if (ServerPersonality.getInstance().getType() != ServerPersonality.Type.PLATFORM) {
                        log.warn("Fall back to Map - {}", simpleError(e.getMessage()));
                    }
                    type = (String) message.get(OBJ_TYPE);
                    body = message.get(BODY);
                }
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
        }
    }

    private String simpleError(String message) {
        int sep = message.indexOf(": ");
        if (sep > 3) {
            String cls = message.substring(0, sep);
            return cls.contains(".")? message.substring(sep+2) : message;
        } else {
            return message;
        }
    }

    /**
     * Encode the EventEnvelope into a byte array
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
        if (cid != null) {
            message.put(CID, cid);
        }
        if (notes != null) {
            message.put(NOTES, notes);
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
        if (parametricType != null) {
            message.put(PARA_TYPES, parametricType);
        }
        if (body != null) {
            if (type == null) {
                // encode body and save object type
                TypedPayload typed = converter.encode(body, binary);
                message.put(OBJ_TYPE, typed.getType());
                message.put(BODY, typed.getPayload());
            } else {
                // preserve encoded type and original body
                message.put(OBJ_TYPE, type);
                message.put(BODY, body);
            }
        }
        if (executionTime != null) {
            message.put(EXECUTION, executionTime);
        }
        if (roundTrip != null) {
            message.put(ROUND_TRIP, roundTrip);
        }
        return msgPack.pack(message);
    }

}
