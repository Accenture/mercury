/*

    Copyright 2018 Accenture Technology

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
import org.platformlambda.core.system.ServerPersonality;
import org.platformlambda.core.util.AppConfigReader;
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
    // final destination
    private static final String END_ROUTE = "E";
    // broadcast
    private static final String BROADCAST = "A";

    private String id, to, replyTo, cid, type, parametricType;
    private Integer status;
    private Map<String, String> headers = new HashMap<>();
    private Object body;
    private Float executionTime, roundTrip;
    private boolean endOfRoute = false, broadcast = false, binary = true;
    private static Set<String> safeModels = new HashSet<>();
    private static boolean loadSafeModels = false;

    public EventEnvelope() {
        this.id = Utility.getInstance().getUuid();
        if (!loadSafeModels) {
            loadSafeModels = true;
            AppConfigReader reader = AppConfigReader.getInstance();
            String models = reader.getProperty("safe.data.models");
            if (models != null) {
                List<String> list = Utility.getInstance().split(models, ", ");
                for (String m: list) {
                    safeModels.add(m);
                }
                log.info("Safe data models {}", safeModels);
            }
        }
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

    public String getReplyTo() {
        return replyTo;
    }

    public String getCorrelationId() {
        return cid;
    }

    public Integer getStatus() {
        return status == null? 200 : status;
    }

    public Float getExecutionTime() {
        return executionTime == null? 0.0f : executionTime;
    }

    public Float getRoundTrip() {
        return roundTrip == null? 0.0f : roundTrip;
    }

    public boolean isEndOfRoute() {
        return endOfRoute;
    }

    public boolean isBroadcast() {
        return broadcast;
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

    public EventEnvelope setReplyTo(String replyTo) {
        this.replyTo = replyTo;
        return this;
    }

    public EventEnvelope setCorrelationId(String cid) {
        this.cid = cid;
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

    public EventEnvelope setExecutionTime(float nanoSeconds) {
        this.executionTime = nanoSeconds;
        return this;
    }

    public EventEnvelope setRoundTrip(float nanoSeconds) {
        this.roundTrip = nanoSeconds;
        return this;
    }

    public EventEnvelope setEndOfRoute() {
        this.endOfRoute = true;
        return this;
    }

    public EventEnvelope setBroadcast() {
        this.broadcast = true;
        return this;
    }

    public EventEnvelope stopBroadcast() {
        this.broadcast = false;
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
            if (message.containsKey(REPLY_TO)) {
                replyTo = (String) message.get(REPLY_TO);
            }
            if (message.containsKey(CID)) {
                cid = (String) message.get(CID);
            }
            if (message.containsKey(STATUS)) {
                status = (Integer) message.get(STATUS);
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
            if (message.containsKey(BROADCAST)) {
                broadcast = (Boolean) message.get(BROADCAST);
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
                    if (!modelInWhiteList(typed.getType())) {
                        throw new IllegalArgumentException("Class not authorized in safe.data.models white-list");
                    }
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
                        log.warn("Fall back to HashMap. Unable to reconstruct {} - {}", typed.getType(), e.getMessage());
                    }
                    type = (String) message.get(OBJ_TYPE);
                    body = message.get(BODY);
                }
            }
            if (message.containsKey(EXECUTION)) {
                executionTime = (Float) message.get(EXECUTION);
            }
            if (message.containsKey(ROUND_TRIP)) {
                roundTrip = (Float) message.get(ROUND_TRIP);
            }
        }
    }

    private boolean modelInWhiteList(String clsName) {
        if (!clsName.contains(".")) {
            // primitive types
            return true;
        }
        if (safeModels.isEmpty()) {
            // feature not enabled
            return true;
        }
        for (String m: safeModels) {
            if (clsName.startsWith(m)) {
                return true;
            }
        }
        return false;
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
        if (replyTo != null) {
            message.put(REPLY_TO, replyTo);
        }
        if (cid != null) {
            message.put(CID, cid);
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
        if (broadcast) {
            message.put(BROADCAST, true);
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
