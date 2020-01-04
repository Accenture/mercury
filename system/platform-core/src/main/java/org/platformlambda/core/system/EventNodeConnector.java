/*

    Copyright 2018-2020 Accenture Technology

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

package org.platformlambda.core.system;

import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.models.WsEnvelope;
import org.platformlambda.core.serializers.MsgPack;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.CryptoApi;
import org.platformlambda.core.util.Utility;
import org.platformlambda.core.websocket.common.MultipartPayload;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.websocket.CloseReason;
import java.io.File;
import java.io.IOException;
import java.security.GeneralSecurityException;
import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.ConcurrentMap;

public class EventNodeConnector implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(EventNodeConnector.class);
    private static final CryptoApi crypto = new CryptoApi();
    private static final Utility util = Utility.getInstance();
    private static final MsgPack msgPack = new MsgPack();

    public static final String HELLO = "hello";
    public static final String CHALLENGE = "challenge";
    public static final String RESPONSE = "response";
    public static final String SIGNATURE = "signature";
    public static final String SIGNER = "signer";
    public static final String PERSONALITY = "personality";
    public static final String TOKEN = "token";
    public static final String ENCRYPT = "encrypt";
    private static final String EVENT_NODE = "EVENT NODE";
    private static final String ALIVE = "keep-alive";
    private static final String PUBLIC_KEY_USER_GROUP = "public.key.user.group";
    private static final String PUBLIC_KEY_ID = "public.key.id";
    private static final long MAX_ALIVE_WAIT = 8000;
    private static String publicKeyUserGroup;

    private State state = State.UNASSIGNED;
    private String txPath = null;
    private String personalityType;
    private byte[] challengeToken;
    private long handshakeStartTime = 0, aliveTime = 0, aliveSeq = 0;
    private boolean ready = false, checkingAlive = false;

    public enum State {
        UNASSIGNED, CONNECTING, CONNECTED, DISCONNECTED, ERROR
    }

    private static final EventNodeConnector instance = new EventNodeConnector();

    private EventNodeConnector() {
        // singleton
        ServerPersonality personality = ServerPersonality.getInstance();
        personalityType = personality.getType().name();
        // public.key.user.group is a convenient feature. MUST not exist in production.
        AppConfigReader reader = AppConfigReader.getInstance();
        String userGroup = reader.getProperty(PUBLIC_KEY_USER_GROUP);
        if (userGroup != null) {
            publicKeyUserGroup = userGroup;
        }
    }

    public static EventNodeConnector getInstance() {
        return instance;
    }

    public void setState(State state) {
        this.state = state;
    }

    public boolean isUnassigned() {
        return state == State.UNASSIGNED;
    }

    public boolean isConnected() {
        return state == State.CONNECTED;
    }

    public boolean isReady() {
        return ready;
    }

    public long getHandshakeStartTime() {
        return handshakeStartTime;
    }

    public String getTxPath() {
        return txPath;
    }

    @SuppressWarnings("unchecked")
    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {

        WsRegistry registry = WsRegistry.getInstance();
        PostOffice po = PostOffice.getInstance();
        if (headers.containsKey(WsEnvelope.TYPE)) {
            switch (headers.get(WsEnvelope.TYPE)) {
                case WsEnvelope.OPEN:
                    txPath = headers.get(WsEnvelope.TX_PATH);
                    ready = false;
                    checkingAlive = false;
                    aliveSeq = 0;
                    handshakeStartTime = System.currentTimeMillis();
                    log.debug("Connected {}, {}, {}", headers.get(WsEnvelope.ROUTE), headers.get(WsEnvelope.IP), headers.get(WsEnvelope.PATH));
                    setState(State.CONNECTED);
                    EventEnvelope hello = new EventEnvelope();
                    hello.setHeader(WsEnvelope.TYPE, EventNodeConnector.HELLO);
                    if (publicKeyUserGroup != null) {
                        hello.setHeader(PUBLIC_KEY_USER_GROUP, publicKeyUserGroup);
                        hello.setBody(ServerPersonality.getInstance().getPublicKey());
                    } else {
                        hello.setBody(EventNodeConnector.HELLO);
                    }
                    po.send(txPath, hello.toBytes());
                    break;
                case WsEnvelope.CLOSE:
                    ready = false;
                    checkingAlive = false;
                    setState(State.DISCONNECTED);
                    // release resources
                    log.debug("Disconnected {}", headers.get(WsEnvelope.ROUTE));
                    break;
                case WsEnvelope.BYTES:
                    WsEnvelope envelope = registry.get(headers.get(WsEnvelope.ROUTE));
                    if (envelope != null) {
                        if (ready) {
                            // handle data
                            EventEnvelope message = new EventEnvelope();
                            try {
                                byte[] payload = envelope.encrypt ?
                                        crypto.aesDecrypt((byte[]) body, envelope.sessionKey) : (byte[]) body;
                                message.load(payload);
                            } catch (GeneralSecurityException e) {
                                log.error("Unable to decrypt message from {}, {}", EVENT_NODE, e.getMessage());
                                util.closeConnection(txPath, CloseReason.CloseCodes.GOING_AWAY, "Secure transport failed");
                                return false;
                            }
                            MultipartPayload.getInstance().incoming(message);
                            EventNodeManager.touch();
                        } else {
                            // handle handshake
                            EventEnvelope message = new EventEnvelope();
                            message.load((byte[]) body);
                            Map<String, String> para = message.getHeaders();
                            if (HELLO.equals(para.get(WsEnvelope.TYPE)) && para.containsKey(PUBLIC_KEY_ID)) {
                                String platformKeyName = para.get(PUBLIC_KEY_ID);
                                byte[] platformPublicKey = null;
                                if (publicKeyUserGroup != null && para.containsKey(PUBLIC_KEY_USER_GROUP) && publicKeyUserGroup.equals(para.get(PUBLIC_KEY_USER_GROUP))) {
                                    if (message.getBody() instanceof byte[]) {
                                        platformPublicKey = (byte[]) message.getBody();
                                        log.warn("{} is configured so system is accepting event node public key automatically", PUBLIC_KEY_USER_GROUP);
                                        log.warn("+++ THIS MUST NOT BE A PRODUCTION SYSTEM +++");
                                    }
                                }
                                if (platformPublicKey == null) {
                                    // get platform public key
                                    File pkFile = new File(util.getEventNodeCredentials(), platformKeyName + CryptoApi.PUBLIC);
                                    if (!pkFile.exists()) {
                                        log.error("Unable to authenticate {}, public key not found in {}", EVENT_NODE, pkFile.getPath());
                                        util.closeConnection(txPath, CloseReason.CloseCodes.GOING_AWAY, personalityType + " does not have remote public key");
                                        return false;
                                    }
                                    // load public key bytes
                                    platformPublicKey = crypto.readPem(util.file2str(pkFile));
                                }
                                try {
                                    // initiate handshake
                                    EventEnvelope challenge = new EventEnvelope();
                                    challenge.setHeader(WsEnvelope.TYPE, CHALLENGE);
                                    // generate session key and challenge token
                                    byte[] sessionKey = crypto.generateAesKey(128);
                                    envelope.sessionKey = sessionKey;
                                    challengeToken = crypto.getRandomBytes(128);
                                    // encrypt the session key with the event node public key
                                    ServerPersonality personality = ServerPersonality.getInstance();
                                    byte[] eKey = crypto.rsaEncrypt(sessionKey, platformPublicKey);
                                    challenge.setHeader(CHALLENGE, util.bytesToBase64(eKey));
                                    byte[] signature = crypto.rsaSign(challengeToken, personality.getPrivateKey());
                                    // pack signer, token, signature and server personality and encrypt into the message body
                                    Map<String, Object> metadata = new HashMap<>();
                                    metadata.put(SIGNER, personality.getKeyName());
                                    metadata.put(TOKEN, challengeToken);
                                    metadata.put(SIGNATURE, signature);
                                    metadata.put(PERSONALITY, personality.getType().toString());
                                    byte[] ePayload = msgPack.pack(metadata);
                                    // encrypt challenge token as payload
                                    challenge.setBody(crypto.aesEncrypt(ePayload, sessionKey));
                                    po.send(envelope.txPath, challenge.toBytes());
                                } catch (GeneralSecurityException | IllegalArgumentException e) {
                                    log.error("Unable to authenticate {}, {}", EVENT_NODE, e.getMessage());
                                    util.closeConnection(txPath, CloseReason.CloseCodes.GOING_AWAY, personalityType + " gives up");
                                    return false;
                                }
                            }
                            if (RESPONSE.equals(para.get(WsEnvelope.TYPE)) && envelope.sessionKey != null) {
                                byte[] encryptedToken = (byte[]) message.getBody();
                                // verify if it can be decrypted back to the challenge token
                                Map<String, Object> confirm;
                                try {
                                    confirm = (Map<String, Object>) msgPack.unpack(crypto.aesDecrypt(encryptedToken, envelope.sessionKey));
                                } catch (GeneralSecurityException | IOException e) {
                                    log.error("Unable to authenticate {}, {}", EVENT_NODE, e.getMessage());
                                    util.closeConnection(txPath, CloseReason.CloseCodes.GOING_AWAY, personalityType + "gives up");
                                    return false;
                                }
                                byte[] token = null;
                                if (confirm.containsKey(TOKEN) && confirm.containsKey(ENCRYPT)) {
                                    envelope.encrypt = (boolean) confirm.get(ENCRYPT);
                                    token = (byte[]) confirm.get(TOKEN);
                                }
                                if (!Arrays.equals(challengeToken, token)) {
                                    log.error("Unable to authenticate {}, challenge-response failed", EVENT_NODE);
                                    util.closeConnection(txPath, CloseReason.CloseCodes.GOING_AWAY, personalityType + " gives up");
                                    return false;
                                } else {
                                    log.info("{} authenticated", EVENT_NODE);
                                    // now ready to received normal messages
                                    ready = true;
                                    // turn on secure transport
                                    if (envelope.encrypt) {
                                        po.send(envelope.txPath, new Kv(WsEnvelope.TYPE, WsEnvelope.ENCRYPT),
                                                new Kv(WsEnvelope.ENCRYPT, util.bytesToBase64(envelope.sessionKey)));
                                        log.info("Secure transport enabled, {} to {}", personalityType, EVENT_NODE);
                                    }
                                    // upload routing table
                                    ServerPersonality personality = ServerPersonality.getInstance();
                                    if (personality.getType() != ServerPersonality.Type.PLATFORM) {
                                        Platform platform = Platform.getInstance();
                                        ConcurrentMap<String, ServiceDef> routes = platform.getLocalRoutingTable();
                                        for (String r : routes.keySet()) {
                                            ServiceDef def = routes.get(r);
                                            if (!def.isPrivate()) {
                                                po.send(ServiceDiscovery.SERVICE_REGISTRY,
                                                        new Kv(PERSONALITY, personalityType),
                                                        new Kv(ServiceDiscovery.ROUTE, def.getRoute()),
                                                        new Kv(ServiceDiscovery.ORIGIN, platform.getOrigin()),
                                                        new Kv(ServiceDiscovery.TYPE, ServiceDiscovery.ADD));
                                                log.info("{} registered with {}", def.getRoute(), EVENT_NODE);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    break;
                case WsEnvelope.STRING:
                    String message = (String) body;
                    log.debug("{}", body);
                    if (message.contains(ALIVE)) {
                        aliveTime = System.currentTimeMillis();
                        checkingAlive = false;
                        EventNodeManager.touch();
                    }
                    break;
                default:
                    // this should not happen
                    log.error("Invalid event {} {}", headers, body);
                    break;
            }
        }
        return true;
    }

    public void isAlive() {
        if (checkingAlive && txPath != null && (System.currentTimeMillis() - aliveTime > MAX_ALIVE_WAIT)) {
            String message = "Event node failed to keep alive in "+(MAX_ALIVE_WAIT / 1000)+" seconds";
            log.error(message);
            try {
                util.closeConnection(txPath, CloseReason.CloseCodes.GOING_AWAY, message);
            } catch (IOException e) {
                // ok to ignore
            }
        }
    }

    public void keepAlive() {
        if (txPath != null) {
            checkingAlive = true;
            aliveTime = System.currentTimeMillis();
            aliveSeq++;
            try {
                PostOffice.getInstance().send(txPath, ALIVE+" "+aliveSeq);
            } catch (IOException e) {
                log.error("Unable to send keep alive to event node - {}", e.getMessage());
            }
        }
    }

}
