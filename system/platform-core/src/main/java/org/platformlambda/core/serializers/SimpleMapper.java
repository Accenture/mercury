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

package org.platformlambda.core.serializers;

import com.fasterxml.jackson.annotation.JsonInclude;
import com.fasterxml.jackson.databind.*;
import com.fasterxml.jackson.databind.module.SimpleModule;
import com.fasterxml.jackson.databind.ser.std.ToStringSerializer;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.Date;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class SimpleMapper {
    private static final Logger log = LoggerFactory.getLogger(SimpleMapper.class);

    private static final String SNAKE_CASE_SERIALIZATION = "snake.case.serialization";
    private static final ObjectMapper mapper = new ObjectMapper();
    private static final Set<String> safeModels = new HashSet<>();
    private static final String[] SAFE_GROUPS = {"java.util.", "java.lang."};
    private static final SimpleMapper instance = new SimpleMapper();

    private SimpleMapper() {
        // Setup features
        mapper.enable(SerializationFeature.INDENT_OUTPUT);
        mapper.configure(MapperFeature.SORT_PROPERTIES_ALPHABETICALLY, true);
        mapper.configure(SerializationFeature.ORDER_MAP_ENTRIES_BY_KEYS, true);
        mapper.configure(SerializationFeature.FAIL_ON_EMPTY_BEANS, false);
        mapper.configure(SerializationFeature.WRITE_ENUMS_USING_TO_STRING, true);
        mapper.configure(SerializationFeature.INDENT_OUTPUT, true);
        mapper.configure(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES, false);
        // Ignore null values
        mapper.setSerializationInclusion(JsonInclude.Include.NON_NULL);
        // All 64 bit fields should be quoted. Use StringSerializer for them.
        SimpleModule module = new SimpleModule();
        module.addSerializer(Long.class, new ToStringSerializer());
        module.addSerializer(Double.class, new ToStringSerializer());
        // ISO-8601 date serializer
        mapper.setDateFormat(new FastDateFormatter());
        // Special treatment for SQL date which normally truncate the HH:mm:ss portion instead of ISO 8601 format
        module.addSerializer(java.sql.Date.class, new IsoDateSerializer());
        // ISO-8601 date deSerializer
        module.addDeserializer(Date.class, new IsoDateDeserializer());
        // register the module
        mapper.registerModule(module);
        AppConfigReader config = AppConfigReader.getInstance();
        boolean snake = config.getProperty(SNAKE_CASE_SERIALIZATION, "true").equals("true");
        if (snake) {
            mapper.setPropertyNamingStrategy(PropertyNamingStrategy.SNAKE_CASE);
            log.info("{} enabled", SNAKE_CASE_SERIALIZATION);
        }
        /*
         * This avoids a security vulnerability that input JSON string may contain arbitrary Java class name
         */
        mapper.disableDefaultTyping();
        /*
         * load white list for authorized PoJo
         */
        AppConfigReader reader = AppConfigReader.getInstance();
        String models = reader.getProperty("safe.data.models");
        if (models != null) {
            List<String> list = Utility.getInstance().split(models, ", ");
            for (String m: list) {
                safeModels.add(m.endsWith(".") ? m : m +".");
            }
            log.info("Safe data models {}", safeModels);
        }
    }

    public static SimpleMapper getInstance() {
        return instance;
    }

    public ObjectMapper getMapper() {
        return mapper;
    }

    public ObjectMapper getWhiteListMapper(Class<?> cls) {
        return getWhiteListMapper(cls.getTypeName());
    }

    public ObjectMapper getWhiteListMapper(String clsName) {
        if (permittedDataModel(clsName)) {
            return mapper;
        } else {
            throw new IllegalArgumentException("Class "+clsName+" not in safe.data.models");
        }
    }

    private boolean permittedDataModel(String clsName) {
        // accept all types if safe.data.models feature is not enabled
        if (safeModels.isEmpty()) {
            // feature not enabled
            return true;
        }
        // always allow primitive types including byte[]
        if (!clsName.contains(".")) {
            return true;
        }
        // accept safe java.util and java.lang classes
        for (String m: SAFE_GROUPS) {
            if (clsName.startsWith(m)) {
                return true;
            }
        }
        // validate with white list
        for (String m: safeModels) {
            if (clsName.startsWith(m)) {
                return true;
            }
        }
        return false;
    }

}
