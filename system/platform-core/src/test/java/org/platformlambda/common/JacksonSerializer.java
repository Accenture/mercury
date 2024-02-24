/*

    Copyright 2018-2024 Accenture Technology

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

package org.platformlambda.common;

import com.fasterxml.jackson.databind.DeserializationFeature;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.json.JsonMapper;
import org.platformlambda.core.models.CustomSerializer;

import java.util.Map;

public class JacksonSerializer implements CustomSerializer {

    private static final ObjectMapper mapper;

    static {
        mapper = JsonMapper.builder().build();
        mapper.disable(DeserializationFeature.FAIL_ON_UNKNOWN_PROPERTIES);
    }

    @SuppressWarnings("unchecked")
    @Override
    public Map<String, Object> toMap(Object obj) {
        return (Map<String, Object>) mapper.convertValue(obj, Map.class);
    }

    @Override
    public <T> T toPoJo(Object obj, Class<T> toValueType) {
        return mapper.convertValue(obj, toValueType);
    }
}
