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

package org.platformlambda.core.util;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.PropertyNamingStrategy;
import org.junit.Test;
import org.platformlambda.core.serializers.SimpleMapper;

import static org.junit.Assert.assertTrue;

public class SnakeCaseTest {

    @Test
    public void snake() throws JsonProcessingException {

        PoJo pojo = new PoJo();
        ObjectMapper mapper = SimpleMapper.getInstance().getMapper();
        mapper.setPropertyNamingStrategy(PropertyNamingStrategy.SNAKE_CASE);
        String result = mapper.writeValueAsString(pojo);
        assertTrue(result.contains("hello_world"));
    }

    private class PoJo {

        private int helloWorld = 30;

		public int getHelloWorld() {
			return helloWorld;
		}

		public void setHelloWorld(int helloWorld) {
			this.helloWorld = helloWorld;
		}

    }
}
