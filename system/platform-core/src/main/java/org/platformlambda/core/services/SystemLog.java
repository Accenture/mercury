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

package org.platformlambda.core.services;

import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.util.CryptoApi;
import org.platformlambda.core.util.ManagedCache;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

public class SystemLog implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(SystemLog.class);

    public static final String LEVEL = "level";
    public static final String INFO = "info";
    public static final String DEBUG = "debug";
    public static final String WARNING = "warn";
    public static final String ERROR = "error";
    public static final String FINISH = "finish";

    private static final ConcurrentMap<String, Boolean> completed = new ConcurrentHashMap<>();
    private static final ManagedCache cache = ManagedCache.createCache("log.cache", 30000);
    private static final CryptoApi crypto = new CryptoApi();
    private static final Utility util = Utility.getInstance();

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) {

        if (headers.containsKey(LEVEL) && body instanceof String) {
            String message = (String) body;
            String hash = util.getUTF(crypto.getMd5(util.getUTF(message)));
            if (completed.containsKey(hash)) {
                return true;
            }
            if (headers.containsKey(FINISH)) {
                completed.put(hash, true);
            }
            if (!cache.exists(hash)) {
                cache.put(hash, true);
                switch(headers.get(LEVEL)) {
                    case INFO:
                        log.info(message);
                        break;
                    case DEBUG:
                        log.debug(message);
                        break;
                    case WARNING:
                        log.warn(message);
                        break;
                    default:
                        log.error(message);
                        break;
                }
            }
        }
        return null;
    }

}
