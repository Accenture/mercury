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

        public int helloWorld = 30;

    }
}
