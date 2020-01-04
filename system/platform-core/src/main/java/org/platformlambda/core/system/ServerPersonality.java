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

import org.platformlambda.core.util.AppConfigReader;
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
    private static boolean loaded = false;

    private ServerPersonality() {
        Utility util = Utility.getInstance();
        ensureDirExists(util.getIdentityFolder());
        ensureDirExists(util.getEventNodeCredentials());
        ensureDirExists(util.getLambdaCredentials());
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
        if (type == Type.PLATFORM) {
            AppConfigReader config = AppConfigReader.getInstance();
            if (!"this".equals(config.getProperty("event.node"))) {
                throw new IllegalArgumentException(Type.PLATFORM.name()+" is reserved for Event Node");
            }
        }
        this.type = type;
        loadKeyPair();
        log.info("Setting personality as {}", type);
    }

    public byte[] getPublicKey() {
        return publicKey;
    }

    public byte[] getPrivateKey() {
        return privateKey;
    }

    private void loadKeyPair() {
        if (!loaded) {
            loaded = true;
            Utility util = Utility.getInstance();
            File dir = util.getIdentityFolder();
            String fileName = getKeyName();
            String pubName = fileName+CryptoApi.PUBLIC;
            File pubFile = new File(dir, pubName);
            String priName = fileName+CryptoApi.PRIVATE;
            File priFile = new File(dir, priName);
            if (pubFile.exists() && priFile.exists()) {
                publicKey = crypto.readPem(util.file2str(pubFile));
                privateKey = crypto.readPem(util.file2str(priFile));
            } else {
                KeyPair kp = crypto.generateRsaKey();
                // save public key
                publicKey = crypto.getEncodedPublicKey(kp);
                util.str2file(pubFile, crypto.writePem(publicKey, "rsa public key"));
                // save private key
                privateKey = crypto.getEncodedPrivateKey(kp);
                util.str2file(priFile, crypto.writePem(privateKey, "rsa private key"));
                log.info("RSA keypair generated");
            }
            // save public key so it is easy to do application development and end-to-end platform tests locally
            File pk = this.type == Type.PLATFORM ?
                    new File(util.getEventNodeCredentials(), pubName) : new File(util.getLambdaCredentials(), pubName);
            if (!pk.exists()) {
                util.str2file(pk, crypto.writePem(publicKey, "rsa public key"));
                log.info("Public key saved {}", pk);
            }
        }
    }

    private void ensureDirExists(File dir) {
        if (!dir.exists()) {
            if (dir.mkdirs()) {
                log.info("Created {}", dir);
            } else {
                log.error("Unable to create {}", dir);
            }
        }
    }

    public String getKeyName() {
        return Platform.getInstance().getName();
    }

}
