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

package org.platformlambda.core.websocket.common;

import javax.websocket.Session;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;


public class WsConfigurator {
    private static final Logger log = LoggerFactory.getLogger(WsConfigurator.class);

    private static final int CONTROL_BYTES = 200;
    private static final int MIN_TIMEOUT = 10;
    private static final String DEFAULT_PAYLOAD_SIZE = "65536";
    private static final String DEFAULT_IDLE_TIMEOUT = "60";
    private static final String TEXT_SIZE = "websocket.text.size";
    private static final String BINARY_SIZE = "websocket.binary.size";
    private static final String IDLE_TIMEOUT = "websocket.idle.timeout";
    private static boolean idleTimerLogged = false;
    private static boolean textSizeLogged = false;
    private static boolean binarySizeLogged = false;
    private int idleTimeout, textSize, binarySize, maxBinaryPayload;
    private static final WsConfigurator instance = new WsConfigurator();

    private WsConfigurator() {
        Utility util = Utility.getInstance();
        AppConfigReader reader = AppConfigReader.getInstance();
        this.textSize = util.str2int(reader.getProperty(TEXT_SIZE, DEFAULT_PAYLOAD_SIZE));
        this.binarySize = util.str2int(reader.getProperty(BINARY_SIZE, DEFAULT_PAYLOAD_SIZE));
        this.maxBinaryPayload = this.binarySize - CONTROL_BYTES;
        this.idleTimeout = util.str2int(reader.getProperty(IDLE_TIMEOUT, DEFAULT_IDLE_TIMEOUT));
        if (this.idleTimeout < MIN_TIMEOUT) {
            this.idleTimeout = MIN_TIMEOUT;
        }
    }

    public static WsConfigurator getInstance() {
        return instance;
    }


    public int getIdleTimeout() {
        return idleTimeout;
    }

    public int getTextSize() {
        return textSize;
    }

    public int getBinarySize() {
        return binarySize;
    }

    public int getMaxBinaryPayload() {
        return maxBinaryPayload;
    }

    public void update(Session session) {
        session.setMaxIdleTimeout(getIdleTimeout() * 1000);
        if (!idleTimerLogged) {
            idleTimerLogged = true;
            log.info("{} = {} seconds", IDLE_TIMEOUT, session.getMaxIdleTimeout() / 1000);
        }
        // adjust web socket buffer size
        int originalTextSize = session.getMaxTextMessageBufferSize();
        int originalBinarySize = session.getMaxBinaryMessageBufferSize();
        if (originalTextSize != getTextSize()) {
            session.setMaxTextMessageBufferSize(getTextSize());
            if (!textSizeLogged) {
                textSizeLogged = true;
                log.warn("{} changed from {} to {}", TEXT_SIZE, originalTextSize, session.getMaxTextMessageBufferSize());
            }
        }
        if (originalBinarySize != getBinarySize()) {
            session.setMaxBinaryMessageBufferSize(getBinarySize());
            if (!binarySizeLogged) {
                binarySizeLogged = true;
                log.warn("{} changed from {} to {}", BINARY_SIZE, originalBinarySize, session.getMaxBinaryMessageBufferSize());
            }
        }
    }

}
