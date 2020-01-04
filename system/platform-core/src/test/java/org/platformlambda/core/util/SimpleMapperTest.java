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

package org.platformlambda.core.util;

import org.junit.Test;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.serializers.SimpleObjectMapper;
import org.platformlambda.core.util.models.PoJo;

import java.io.IOException;
import java.util.Date;
import java.util.HashMap;
import java.util.Map;

import static org.junit.Assert.assertEquals;

public class SimpleMapperTest {

    @Test
    @SuppressWarnings("unchecked")
    public void mapperSerializationTest() throws IOException {

        SimpleObjectMapper mapper = SimpleMapper.getInstance().getMapper();

        Date now = new Date();
        String iso8601 = Utility.getInstance().date2str(now);
        Map<String, Object> map = new HashMap<>();
        map.put("date", now);
        map.put("sql_date", new java.sql.Date(now.getTime()));
        map.put("sql_timestamp", new java.sql.Timestamp(now.getTime()));
        Map<String, Object> converted = mapper.readValue(mapper.writeValueAsString(map), HashMap.class);
        // verify that java.util.Date, java.sql.Date and java.sql.Timestamp can be serialized to ISO-8601 string format
        assertEquals(iso8601, converted.get("date"));
        // sql date is yyyy-mm-dd
        assertEquals(new java.sql.Date(now.getTime()).toString(), converted.get("sql_date"));
        assertEquals(iso8601, converted.get("sql_timestamp"));

        String name = "hello world";
        Map<String, Object> input = new HashMap<>();
        input.put("full_name", name);
        input.put("date", iso8601);
        PoJo pojo = mapper.readValue(input, PoJo.class);
        // verify that the time is restored correctly
        assertEquals(now.getTime(), pojo.getDate().getTime());
        // verify that snake case is deserialized correctly
        assertEquals(name, pojo.getFullName());

        // verify input timestamp can be in milliseconds too
        input.put("date", now.getTime());
        pojo = mapper.readValue(input, PoJo.class);
        assertEquals(now.getTime(), pojo.getDate().getTime());
    }

}
