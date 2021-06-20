/*

    Copyright 2018-2021 Accenture Technology

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

import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.File;

public class ServerPersonality {
    private static final Logger log = LoggerFactory.getLogger(ServerPersonality.class);
    private static final ServerPersonality instance = new ServerPersonality();

    /**
     * REST is the user facing container that handles REST and websocket requests
     * WEB is the web-tier microservices container
     * APP is the application-tier microservices container
     * RESOURCES is the resources-tier microservices container
     */
    public enum Type {
        REST, WEB, APP, RESOURCES
    }
    private Type type = Type.APP;

    public ServerPersonality() {
        Utility util = Utility.getInstance();
        ensureDirExists(util.getWorkFolder());
    }

    public static ServerPersonality getInstance() {
        return instance;
    }

    public Type getType() {
        return type;
    }

    public void setType(Type type) {
        if (type == null) {
            throw new IllegalArgumentException("Personality cannot be null");
        }
        this.type = type;
        log.info("Setting personality as {}", type);
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

}
