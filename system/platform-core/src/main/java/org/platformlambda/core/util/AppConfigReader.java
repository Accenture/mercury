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

package org.platformlambda.core.util;

import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;

public class AppConfigReader extends ConfigReader {
    private static final Logger log = LoggerFactory.getLogger(AppConfigReader.class);

    public static final AppConfigReader reader = new AppConfigReader();

    public static AppConfigReader getInstance() {
        return reader;
    }

    private static final String APP_PROPS = "classpath:/application.properties";
    private static final String APP_YML = "classpath:/application.yml";

    private AppConfigReader() {
        // load application.properties
        try {
            /*
             * skip parameter value substitution because AppConfigReader is the base configuration.
             * Other ConfigReader may use the dollar bracket convention to substitution parameter values
             * with those in the AppConfigReader.
             *
             */
            doSubstitution = false;
            load(APP_PROPS);
            if (!config.isEmpty()) {
                log.info("Loaded {} item{} from {}", config.size(),
                        config.size() == 1? "" : "s", APP_PROPS);
                return;
            }
        } catch (IOException e) {
            log.error("Unable to load {}, {}", APP_PROPS, e.getMessage());
        }
        // try YAML
        try {
            load(APP_YML);
            if (!config.isEmpty()) {
                /*
                 * flatten the map so it can be read in dot and bracket format directly
                 * e.g. "application.name"
                 */
                flattenMap();
                log.info("Loaded {} item{} from {}", config.size(), config.size() == 1? "" : "s", APP_YML);
            }
        } catch (IOException e) {
            log.error("Unable to load {}, {}", APP_YML, e.getMessage());
        }

    }

}
