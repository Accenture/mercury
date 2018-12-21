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

package org.platformlambda.core.system;

import org.platformlambda.core.util.CryptoApi;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.File;
import java.security.KeyPair;

public class ServerPersonality {
    private static final Logger log = LoggerFactory.getLogger(ServerPersonality.class);

    private static CryptoApi crypto = new CryptoApi();
    private static final ServerPersonality instance = new ServerPersonality();

    /**
     * REST is the user facing container that handles REST and websocket requests
     * WEB is the web-tier microservices container
     * APP is the application-tier microservices container
     * RESOURCES is the resources-tier microservices container
     *
     * DEVOPS is a microservices container for DevOps to monitor the system
     *
     * PLATFORM is the Event Node for routing events among microservices containers
     */
    public enum Type {
        REST, WEB, APP, RESOURCES, PLATFORM, DEVOPS, UNDEFINED
    }

    private Type type = Type.UNDEFINED;
    private byte[] publicKey, privateKey;

    private ServerPersonality() {
        // singleton
    }

    public static ServerPersonality getInstance() {
        return instance;
    }

    public Type getType() {
        return type;
    }

    public void setType(Type type) {
        if (type == null || type == Type.UNDEFINED) {
            Type[] types = Type.values();
            StringBuilder sb = new StringBuilder();
            sb.append("You must select one from [");
            for (Type t: types) {
                sb.append(t.toString());
                sb.append(", ");
            }
            throw new IllegalArgumentException(sb.substring(0, sb.length()-2) + "]");
        }
        if (this.type == Type.UNDEFINED) {
            this.type = type;
            Platform platform = Platform.getInstance();
            String origin = platform.getOrigin();
            log.info("Setting personality for {} to {}", origin, type.toString());
            loadKeyPair();
        } else {
            throw new IllegalArgumentException("Unable to override because personality has already been set as "+this.type);
        }
    }

    public byte[] getPublicKey() {
        return publicKey;
    }

    public byte[] getPrivateKey() {
        return privateKey;
    }

    private void loadKeyPair() {
        Utility util = Utility.getInstance();
        File dir = util.getIdentityFolder();
        if (!dir.exists()) {
            dir.mkdirs();
        }
        String fileName = getKeyFileName();
        String pubName = fileName+CryptoApi.PUBLIC;
        File pubFile = new File(dir, pubName);
        File priFile = new File(dir, getKeyFileName()+CryptoApi.PRIVATE);
        if (!pubFile.exists() || !priFile.exists()) {
            generateRsaKey();
        }
        // check if the key files exist
        if (pubFile.exists() && priFile.exists()) {
            publicKey = crypto.readPem(util.file2str(pubFile));
            privateKey = crypto.readPem(util.file2str(priFile));
            // save public key so it is easy to do application development and end-to-end platform tests locally
            File pk = this.type == Type.PLATFORM?
                    new File(util.getEventNodeCredentials(), pubName) : new File(util.getLambdaCredentials(), pubName);
            if (!pk.exists()) {
                util.bytes2file(pk, util.file2bytes(pubFile));
            }
        }
    }

    private void generateRsaKey() {
        Utility util = Utility.getInstance();
        File dir = util.getIdentityFolder();
        String fileName = getKeyFileName();
        String pubName = fileName+CryptoApi.PUBLIC;
        File pubFile = new File(dir, pubName);
        String priName = fileName+CryptoApi.PRIVATE;
        File priFile = new File(dir, priName);
        boolean generated = false;
        if (!pubFile.exists() || !priFile.exists()) {
            generated = true;
            KeyPair kp = crypto.generateRsaKey();
            // save public key
            byte[] pub = crypto.getEncodedPublicKey(kp);
            util.str2file(pubFile, crypto.writePem(pub, "rsa public key"));
            // save private key
            byte[] pri = crypto.getEncodedPrivateKey(kp);
            util.str2file(priFile, crypto.writePem(pri, "rsa private key"));
        }
        // check file system again
        if (generated) {
            if (pubFile.exists() && priFile.exists()) {
                log.info("RSA keypair generated");
            } else {
                log.error("Unable to generate RSA keypair. Check local file system access rights.");
            }
        }

    }

    public String getKeyFileName() {
        String nodeId = Platform.getInstance().getNodeId();
        Utility util = Utility.getInstance();
        return util.getPackageName()+"."+nodeId;
    }

}
