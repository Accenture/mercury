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

package org.platformlambda.node.system;

import org.platformlambda.core.annotations.WebSocketService;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.models.WsEnvelope;
import org.platformlambda.core.serializers.MsgPack;
import org.platformlambda.core.system.ServiceDiscovery;
import org.platformlambda.core.system.EventNodeConnector;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.system.ServerPersonality;
import org.platformlambda.core.system.WsRegistry;
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
import java.util.HashMap;
import java.util.Map;

@WebSocketService("events")
public class LambdaRouter implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(LambdaRouter.class);
    private static final WsRegistry registry = WsRegistry.getInstance();
    private static final CryptoApi crypto = new CryptoApi();
    private static final Utility util = Utility.getInstance();
    private static final MsgPack msgPack = new MsgPack();
    private static final int AES128 = 128 / 8;
    private static final int AES192 = 192 / 8;
    private static final int AES256 = 256 / 8;
    private static final String ENCRYPT_EVENT_STREAM = "encrypt.event.stream";
    private static final String PUBLIC_KEY_USER_GROUP = "public.key.user.group";
    private static final String PUBLIC_KEY_ID = "public.key.id";
    private static final String TRUE = Boolean.TRUE.toString();
    private static final String FALSE = Boolean.FALSE.toString();
    private static Boolean strongCrypto, secureTransport;
    private static String publicKeyUserGroup;
    private static boolean started = false;

    private boolean ready = false;

    public static void begin() {
        started = true;
    }

    public LambdaRouter() {
        if (strongCrypto == null) {
            strongCrypto = crypto.strongCryptoSupported();
        }
        if (secureTransport == null) {
            AppConfigReader reader = AppConfigReader.getInstance();
            // secure transport introduces some overheads of one ms or more so the default is false
            secureTransport = TRUE.equals(reader.getProperty(ENCRYPT_EVENT_STREAM, FALSE));
            // public.key.user.group is a convenient feature. MUST not exist in production.
            String userGroup = reader.getProperty(PUBLIC_KEY_USER_GROUP);
            if (userGroup != null) {
                publicKeyUserGroup = userGroup;
            }
        }
    }

    @SuppressWarnings("unchecked")
    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {

        PostOffice po = PostOffice.getInstance();
        String route, txPath;

        switch(headers.get(WsEnvelope.TYPE)) {
            case WsEnvelope.OPEN:
                // the open event contains route, txPath, ip, path, query and token
                route = headers.get(WsEnvelope.ROUTE);
                txPath = headers.get(WsEnvelope.TX_PATH);
                if (!started) {
                    Utility.getInstance().closeConnection(txPath, CloseReason.CloseCodes.SERVICE_RESTART,
                                                        "Starting up. Please try again");
                    return false;
                }
                ServiceDiscovery.createLambdaClient(headers.get(WsEnvelope.TOKEN), txPath);
                ConnectionMonitor.addConnection(route, txPath);
                break;
            case WsEnvelope.CLOSE:
                route = headers.get(WsEnvelope.ROUTE);
                ConnectionMonitor.removeConnection(route);
                po.send(ServiceDiscovery.SERVICE_REGISTRY,
                        new Kv(ServiceDiscovery.TYPE, ServiceDiscovery.REMOVE),
                        new Kv(ServiceDiscovery.ORIGIN, headers.get(WsEnvelope.TOKEN)));
                ready = false;
                break;
            case WsEnvelope.BYTES:
                route = headers.get(WsEnvelope.ROUTE);
                WsEnvelope envelope = registry.get(route);
                if (envelope != null) {
                    if (ready) {
                        // handle data
                        EventEnvelope message = new EventEnvelope();
                        try {
                            byte[] payload = envelope.encrypt ?
                                    crypto.aesDecrypt((byte[]) body, envelope.sessionKey) : (byte[]) body;
                            message.load(payload);
                        } catch (GeneralSecurityException e) {
                            log.warn("Unable to decrypt message {}, {}", envelope.origin, e.getMessage());
                            util.closeConnection(envelope.txPath, CloseReason.CloseCodes.VIOLATED_POLICY,
                                    "Secure transport failed");
                            return false;
                        }
                        try {
                            // set end of route to avoid loopback
                            message.setEndOfRoute();
                            if (message.getTo() != null) {
                                po.send(message);
                            } else {
                                MultipartPayload.getInstance().incoming(message);
                            }
                        } catch (IOException e) {
                            if (message.getReplyTo() != null) {
                                po.discover(message.getReplyTo(), false);
                                EventEnvelope error = new EventEnvelope();
                                error.setTo(message.getReplyTo()).setStatus(404).setBody(e.getMessage());
                                po.send(error);

                            } else {
                                log.warn("Message not delivered to {} because {}", message.getTo(), e.getMessage());
                            }
                        }
                    } else {
                        // handle handshake
                        EventEnvelope message = new EventEnvelope();
                        message.load((byte[]) body);
                        Map<String, String> para = message.getHeaders();
                        String type = para.get(WsEnvelope.TYPE);
                        if (EventNodeConnector.HELLO.equals(type)) {
                            EventEnvelope hello = new EventEnvelope();
                            hello.setHeader(WsEnvelope.TYPE, EventNodeConnector.HELLO);
                            // send "hello" with the platform public key name
                            hello.setHeader(PUBLIC_KEY_ID, ServerPersonality.getInstance().getKeyName());
                            // supply public key value if public.key.user.group matches
                            if (publicKeyUserGroup != null && para.containsKey(PUBLIC_KEY_USER_GROUP) && publicKeyUserGroup.equals(para.get(PUBLIC_KEY_USER_GROUP))) {
                                hello.setHeader(PUBLIC_KEY_USER_GROUP, publicKeyUserGroup);
                                hello.setBody(ServerPersonality.getInstance().getPublicKey());
                                if (message.getBody() instanceof byte[]) {
                                    envelope.publicKey = (byte[]) message.getBody();
                                    log.warn("{} is configured so system is accepting lambda public key automatically", PUBLIC_KEY_USER_GROUP);
                                    log.warn("+++ THIS MUST BE A NON-PRODUCTION SYSTEM +++");
                                }
                            } else {
                                hello.setBody(EventNodeConnector.HELLO);
                            }
                            po.send(envelope.txPath, hello.toBytes());
                        }
                        if (EventNodeConnector.CHALLENGE.equals(type)) {
                            String eKey = para.get(EventNodeConnector.CHALLENGE);
                            byte[] ePayload = (byte[]) message.getBody();
                            String signer = null;
                            String personality;
                            EventEnvelope response = new EventEnvelope();
                            response.setHeader(WsEnvelope.TYPE, EventNodeConnector.RESPONSE);
                            try {
                                if (eKey == null || ePayload == null) {
                                    throw new GeneralSecurityException("Invalid challenge");
                                }
                                // decrypt the session key using platform private key
                                byte[] sessionKey = crypto.rsaDecrypt(util.base64ToBytes(eKey),
                                        ServerPersonality.getInstance().getPrivateKey());
                                if (!validSessionkey(sessionKey)) {
                                    throw new IOException("Session key length must be 128, 192 or 256 bits");
                                }
                                if (!LambdaRouter.strongCrypto && sessionKey.length > AES128) {
                                    throw new IOException("Session key length is restricted to 128 bits");
                                }
                                Object meta = msgPack.unpack(crypto.aesDecrypt(ePayload, sessionKey));
                                if (meta instanceof Map) {
                                    Map<String, Object> metadata = (Map<String, Object>) meta;
                                    signer = (String) metadata.get(EventNodeConnector.SIGNER);
                                    byte[] token = (byte[]) metadata.get(EventNodeConnector.TOKEN);
                                    byte[] signature = (byte[]) metadata.get(EventNodeConnector.SIGNATURE);
                                    personality = (String) metadata.get(EventNodeConnector.PERSONALITY);
                                    if (signer == null || token == null || signature == null || personality == null) {
                                        throw new GeneralSecurityException("Missing signer, token, signature or personality");
                                    }
                                    byte[] lambdaPublicKey = envelope.publicKey == null? getLambdaPublicKey(signer) : envelope.publicKey;
                                    if (lambdaPublicKey == null) {
                                        throw new IOException("Lambda public key not imported");
                                    }
                                    if (!crypto.rsaVerify(token, signature, lambdaPublicKey)) {
                                        throw new GeneralSecurityException("Invalid digital signature");
                                    }
                                    // indicate successful handshake
                                    Map<String, Object> confirm = new HashMap<>();
                                    confirm.put(EventNodeConnector.TOKEN, token);
                                    confirm.put(EventNodeConnector.ENCRYPT, LambdaRouter.secureTransport);
                                    // encrypt the clear token as a response
                                    response.setBody(crypto.aesEncrypt(msgPack.pack(confirm), sessionKey));
                                    // update session key
                                    envelope.sessionKey = sessionKey;
                                    envelope.encrypt = LambdaRouter.secureTransport;
                                    ServiceDiscovery.setPersonality(envelope.origin, signer, personality);
                                } else {
                                    throw new GeneralSecurityException("Invalid challenge");
                                }

                            } catch (GeneralSecurityException | IOException e) {
                                log.warn("Unable to authenticate {}{}, {}", envelope.origin,
                                        signer == null? "" : " with"+signer, e.getMessage());
                                util.closeConnection(envelope.txPath, CloseReason.CloseCodes.VIOLATED_POLICY,
                                        e instanceof IOException? e.getMessage() : "Unauthorized");
                                return false;
                            }
                            log.info("{}.{} from {} authenticated", personality, envelope.origin, signer);
                            ready = true;
                            po.send(envelope.txPath, response.toBytes());
                            ConnectionMonitor.acceptConnection(envelope.route);
                            if (LambdaRouter.secureTransport) {
                                po.send(envelope.txPath, new Kv(WsEnvelope.TYPE, WsEnvelope.ENCRYPT),
                                        new Kv(WsEnvelope.ENCRYPT, util.bytesToBase64(envelope.sessionKey)));
                                log.info("{}.{} secure transport enabled", personality, envelope.origin);
                            }

                        }
                    }
                }
                break;
            case WsEnvelope.STRING:
                // the data event for string payload contains route and txPath
                route = headers.get(WsEnvelope.ROUTE);
                txPath = headers.get(WsEnvelope.TX_PATH);
                // text message is used for keep-alive so just echo it back for a complete round trip
                po.send(txPath, body);
                break;
            default:
                // this should not happen
                log.error("Invalid event {} {}", headers, body);
                break;
        }
        return true;
    }

    private byte[] getLambdaPublicKey(String filename) {
        if (filename == null) {
            return null;
        }
        File f = new File(Utility.getInstance().getLambdaCredentials(), filename+CryptoApi.PUBLIC);
        if (!f.exists()) {
            log.warn("Lambda public key has not been imported as {}", f.getPath());
            return null;
        }
        return f.exists()? crypto.readPem(util.file2str(f)) : null;
    }

    private boolean validSessionkey(byte[] sessionKey) {
        return sessionKey.length == AES128 || sessionKey.length == AES192 || sessionKey.length == AES256;
    }

}
