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

package org.platformlambda.core.system;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

public class ServerPersonality {
    private static final Logger log = LoggerFactory.getLogger(ServerPersonality.class);
    private static final ServerPersonality INSTANCE = new ServerPersonality();

    public enum Type {
        REST, APP, RESOURCES
    }
    private Type type = Type.APP;

    public static ServerPersonality getInstance() {
        return INSTANCE;
    }

    public Type getType() {
        return type;
    }

    /**
     * REST indicates that the deployed application is user facing
     * <p>
     * APP signals that the deployed application holds business logic
     * <p>
     * RESOURCES is a resources-tier service. e.g. database service, MQ gateway, legacy service proxy or utility.
     *
     * @param type can be REST, APP or RESOURCES
     */
    public void setType(Type type) {
        if (type == null) {
            throw new IllegalArgumentException("Personality cannot be null");
        }
        this.type = type;
        log.info("Setting personality as {}", type);
    }

}
