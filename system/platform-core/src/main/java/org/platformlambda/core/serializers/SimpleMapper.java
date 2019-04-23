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

import com.google.gson.*;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.lang.reflect.Type;
import java.math.BigDecimal;
import java.math.BigInteger;
import java.util.*;

public class SimpleMapper {
    private static final Logger log = LoggerFactory.getLogger(SimpleMapper.class);

    private static final String SNAKE_CASE_SERIALIZATION = "snake.case.serialization";
    private static final Set<String> safeModels = new HashSet<>();
    private static final String[] SAFE_GROUPS = {"java.util.", "java.lang."};
    private static final SimpleMapper instance = new SimpleMapper();
    private SimpleObjectMapper mapper;

    private SimpleMapper() {
        // Camel or snake case
        AppConfigReader config = AppConfigReader.getInstance();
        boolean snake = config.getProperty(SNAKE_CASE_SERIALIZATION, "true").equals("true");
        if (snake) {
            log.info("{} enabled", SNAKE_CASE_SERIALIZATION);
        }
        // regularEngine converts numbers into Strings to preserve math precision
        Gson regularEngine = getJson(true, snake);
        Gson jsonReader = getJsonReader();
        Gson jsonWriter = getJson(false, snake);
        this.mapper = new SimpleObjectMapper(regularEngine, jsonReader, jsonWriter);
        /*
         * Optionally, load white list for authorized PoJo
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

    private Gson getJson(boolean numberAsString, boolean snake) {
        GsonBuilder builder = new GsonBuilder();
        // UTC date
        builder.registerTypeAdapter(Date.class, new UtcSerializer());
        builder.registerTypeAdapter(Date.class, new UtcDeserializer());
        // SQL date
        builder.registerTypeAdapter(java.sql.Date.class, new SqlDateSerializer());
        builder.registerTypeAdapter(java.sql.Date.class, new SqlDateDeserializer());
        builder.registerTypeAdapter(java.sql.Time.class, new SqlTimeSerializer());
        builder.registerTypeAdapter(java.sql.Time.class, new SqlTimeDeserializer());
        // Numbers
        if (numberAsString) {
            builder.registerTypeAdapter(Byte.class, new ByteSerializer());
            builder.registerTypeAdapter(Byte.class, new ByteDeserializer());
            builder.registerTypeAdapter(Short.class, new ShortSerializer());
            builder.registerTypeAdapter(Short.class, new ShortDeserializer());
            builder.registerTypeAdapter(Integer.class, new IntegerSerializer());
            builder.registerTypeAdapter(Integer.class, new IntegerDeserializer());
            builder.registerTypeAdapter(Float.class, new FloatSerializer());
            builder.registerTypeAdapter(Float.class, new FloatDeserializer());
        }
        // 64-bit numbers, BigInteger and BigDecimal are serialized as Strings
        builder.registerTypeAdapter(Long.class, new LongSerializer());
        builder.registerTypeAdapter(Long.class, new LongDeserializer());
        builder.registerTypeAdapter(Double.class, new DoubleSerializer());
        builder.registerTypeAdapter(Double.class, new DoubleDeserializer());
        builder.registerTypeAdapter(BigInteger.class, new BigIntegerSerializer());
        builder.registerTypeAdapter(BigInteger.class, new BigIntegerDeserializer());
        builder.registerTypeAdapter(BigDecimal.class, new BigDecimalSerializer());
        builder.registerTypeAdapter(BigDecimal.class, new BigDecimalDeserializer());
        // Indent JSON output
        builder.setPrettyPrinting();
        // Camel or snake case
        if (snake) {
            builder.setFieldNamingPolicy(FieldNamingPolicy.LOWER_CASE_WITH_UNDERSCORES);
        }
        return builder.create();
    }

    private Gson getJsonReader() {
        // this reader is used to parse a JSON string into Map or List
        GsonBuilder builder = new GsonBuilder();
        builder.registerTypeAdapter(Map.class, new MapDeserializer());
        builder.registerTypeAdapter(List.class, new ListDeserializer());
        return builder.create();
    }

    public static SimpleMapper getInstance() {
        return instance;
    }

    public SimpleObjectMapper getMapper() {
        return mapper;
    }

    public SimpleObjectMapper getWhiteListMapper(Class<?> cls) {
        return getWhiteListMapper(cls.getTypeName());
    }

    public SimpleObjectMapper getWhiteListMapper(String clsName) {
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

    /// Custom serializers ///

    private class UtcSerializer implements JsonSerializer<Date> {

        @Override
        public JsonElement serialize(Date date, Type type, JsonSerializationContext context) {
            return new JsonPrimitive(Utility.getInstance().date2str(date));
        }
    }

    private class UtcDeserializer implements JsonDeserializer<Date> {

        @Override
        public Date deserialize(JsonElement json, Type type, JsonDeserializationContext context) throws JsonParseException {
            return Utility.getInstance().str2date(json.getAsString());
        }
    }

    private class SqlDateSerializer implements JsonSerializer<java.sql.Date> {

        @Override
        public JsonElement serialize(java.sql.Date date, Type type, JsonSerializationContext context) {
            return new JsonPrimitive(date.toString());
        }
    }

    private class SqlDateDeserializer implements JsonDeserializer<java.sql.Date> {

        @Override
        public java.sql.Date deserialize(JsonElement json, Type type, JsonDeserializationContext context) throws JsonParseException {
            try {
                return java.sql.Date.valueOf(json.getAsString());
            } catch (IllegalArgumentException e) {
                // parse input as ISO-8601
                Date date = Utility.getInstance().str2date(json.getAsString());
                return new java.sql.Date(date.getTime());
            }
        }
    }

    private class SqlTimeSerializer implements JsonSerializer<java.sql.Time> {

        @Override
        public JsonElement serialize(java.sql.Time time, Type type, JsonSerializationContext context) {
            return new JsonPrimitive(time.toString());
        }
    }

    private class SqlTimeDeserializer implements JsonDeserializer<java.sql.Time> {

        @Override
        public java.sql.Time deserialize(JsonElement json, Type type, JsonDeserializationContext context) throws JsonParseException {
            return java.sql.Time.valueOf(json.getAsString());
        }
    }

    private class ByteSerializer implements JsonSerializer<Byte> {

        @Override
        public JsonElement serialize(Byte number, Type type, JsonSerializationContext context) {
            return new JsonPrimitive(String.valueOf(number));
        }
    }

    private class ByteDeserializer implements JsonDeserializer<Byte> {

        @Override
        public Byte deserialize(JsonElement json, Type type, JsonDeserializationContext context) throws JsonParseException {
            return Byte.parseByte(json.getAsString());
        }
    }

    private class ShortSerializer implements JsonSerializer<Short> {

        @Override
        public JsonElement serialize(Short number, Type type, JsonSerializationContext context) {
            return new JsonPrimitive(String.valueOf(number));
        }
    }

    private class ShortDeserializer implements JsonDeserializer<Short> {

        @Override
        public Short deserialize(JsonElement json, Type type, JsonDeserializationContext context) throws JsonParseException {
            return Short.parseShort(json.getAsString());
        }
    }

    private class IntegerSerializer implements JsonSerializer<Integer> {

        @Override
        public JsonElement serialize(Integer number, Type type, JsonSerializationContext context) {
            return new JsonPrimitive(String.valueOf(number));
        }
    }

    private class IntegerDeserializer implements JsonDeserializer<Integer> {

        @Override
        public Integer deserialize(JsonElement json, Type type, JsonDeserializationContext context) throws JsonParseException {
            return Integer.parseInt(json.getAsString());
        }
    }

    private class LongSerializer implements JsonSerializer<Long> {

        @Override
        public JsonElement serialize(Long number, Type type, JsonSerializationContext context) {
            return new JsonPrimitive(String.valueOf(number));
        }
    }

    private class LongDeserializer implements JsonDeserializer<Long> {

        @Override
        public Long deserialize(JsonElement json, Type type, JsonDeserializationContext context) throws JsonParseException {
            return Long.parseLong(json.getAsString());
        }
    }

    private class FloatSerializer implements JsonSerializer<Float> {

        @Override
        public JsonElement serialize(Float number, Type type, JsonSerializationContext context) {
            return new JsonPrimitive(number.toString());
        }
    }

    private class FloatDeserializer implements JsonDeserializer<Float> {

        @Override
        public Float deserialize(JsonElement json, Type type, JsonDeserializationContext context) throws JsonParseException {
            return Float.parseFloat(json.getAsString());
        }
    }

    private class DoubleSerializer implements JsonSerializer<Double> {

        @Override
        public JsonElement serialize(Double number, Type type, JsonSerializationContext context) {
            return new JsonPrimitive(number.toString());
        }
    }

    private class DoubleDeserializer implements JsonDeserializer<Double> {

        @Override
        public Double deserialize(JsonElement json, Type type, JsonDeserializationContext context) throws JsonParseException {
            return Double.parseDouble(json.getAsString());
        }
    }

    private class BigIntegerSerializer implements JsonSerializer<BigInteger> {

        @Override
        public JsonElement serialize(BigInteger number, Type type, JsonSerializationContext context) {
            return new JsonPrimitive(number.toString());
        }
    }

    private class BigIntegerDeserializer implements JsonDeserializer<BigInteger> {

        @Override
        public BigInteger deserialize(JsonElement json, Type type, JsonDeserializationContext context) throws JsonParseException {
            return new BigInteger(json.getAsString());
        }
    }

    private class BigDecimalSerializer implements JsonSerializer<BigDecimal> {

        @Override
        public JsonElement serialize(BigDecimal number, Type type, JsonSerializationContext context) {
            return new JsonPrimitive(number.toString());
        }
    }

    private class BigDecimalDeserializer implements JsonDeserializer<BigDecimal> {

        @Override
        public BigDecimal deserialize(JsonElement json, Type type, JsonDeserializationContext context) throws JsonParseException {
            return new BigDecimal(json.getAsString());
        }
    }

    private class MapDeserializer implements JsonDeserializer<Map> {

        @Override
        public Map deserialize(JsonElement json, Type type, JsonDeserializationContext context) throws JsonParseException {
            if (json.isJsonObject()) {
                return scan(json.getAsJsonObject());
            } else {
                return new HashMap();
            }
        }
    }

    private class ListDeserializer implements JsonDeserializer<List> {

        @Override
        public List deserialize(JsonElement json, Type type, JsonDeserializationContext context) throws JsonParseException {
            if (json.isJsonArray()) {
                return scan(json.getAsJsonArray());
            } else {
                return new ArrayList();
            }
        }
    }

    private Map scan(JsonObject o) {
        Map<String, Object> result = new HashMap<>();
        for (String k: o.keySet()) {
            if (!o.get(k).isJsonNull()) {
                if (o.get(k).isJsonObject()) {
                    result.put(k, scan(o.get(k).getAsJsonObject()));
                } else if (o.get(k).isJsonArray()) {
                    result.put(k, scan(o.get(k).getAsJsonArray()));
                } else if (o.get(k).isJsonPrimitive()) {
                    JsonPrimitive p = o.get(k).getAsJsonPrimitive();
                    if (p.isBoolean()) {
                        result.put(k, p.getAsBoolean());
                    }
                    if (p.isString()) {
                        result.put(k, p.getAsString());
                    }
                    if (p.isNumber()) {
                        String number = p.getAsString();
                        if (number.contains(".")) {
                            result.put(k, floatOrDouble(p.getAsDouble()));
                        } else {
                            result.put(k, intOrLong(p.getAsLong()));
                        }
                    }
                }
            }
        }
        return result;
    }

    @SuppressWarnings("unchecked")
    private List scan(JsonArray array) {
        List result = new ArrayList();
        for (JsonElement o: array) {
            if (o.isJsonNull()) {
                result.add(null);
            } else if (o.isJsonObject()) {
                result.add(scan(o.getAsJsonObject()));
            } else if (o.isJsonArray()) {
                result.add(scan(o.getAsJsonArray()));
            } else if (o.isJsonPrimitive()) {
                JsonPrimitive p = o.getAsJsonPrimitive();
                if (p.isBoolean()) {
                    result.add(p.getAsBoolean());
                }
                if (p.isString()) {
                    result.add(p.getAsString());
                }
                if (p.isNumber()) {
                    String number = p.getAsString();
                    if (number.contains(".")) {
                        result.add(floatOrDouble(p.getAsDouble()));
                    } else {
                        result.add(intOrLong(p.getAsLong()));
                    }
                }
            }
        }
        return result;
    }

    private Object intOrLong(long number) {
        if (number < Integer.MIN_VALUE || number > Integer.MAX_VALUE) {
            return number;
        } else {
            return (int) number;
        }
    }

    private Object floatOrDouble(double number) {
        if (number < Float.MIN_VALUE || number > Float.MAX_VALUE) {
            return number;
        } else {
            return (float) number;
        }
    }


}
