package org.platformlambda.core.serializers;

import com.google.gson.Gson;
import com.google.gson.reflect.TypeToken;
import org.platformlambda.core.util.Utility;

import java.io.InputStream;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class SimpleObjectMapper {

    private Gson regularEngine, jsonReader, jsonWriter;
    private Utility util;

    public SimpleObjectMapper(Gson regularEngine, Gson jsonReader, Gson jsonWriter) {
        this.regularEngine = regularEngine;
        this.jsonReader = jsonReader;
        this.jsonWriter = jsonWriter;
        this.util = Utility.getInstance();
    }

    public String writeValueAsString(Object value) {
        return jsonWriter.toJson(value);
    }

    public byte[] writeValueAsBytes(Object value) {
        return Utility.getInstance().getUTF(writeValueAsString(value));
    }

    @SuppressWarnings("unchecked")
    public <T> T readValue(Object fromValue, Class<T> toValueType) {
        // return original map
        if (fromValue instanceof Map && isMap(toValueType)) {
            return (T) fromValue;
        }
        // return original list
        if (fromValue instanceof List && isList(toValueType)) {
            return (T) fromValue;
        }
        if (fromValue instanceof InputStream) {
            // input stream is a JSON string
            return readJsonString(util.stream2str((InputStream) fromValue), toValueType);
        } else if (fromValue instanceof String) {
            // input is a JSON string
            return readJsonString((String) fromValue, toValueType);
        } else if (fromValue instanceof byte[]) {
            // input is a byte array of JSON
            return readJsonString(util.getUTF((byte[]) fromValue), toValueType);
        } else {
            // input is an object
            return regularEngine.fromJson(regularEngine.toJsonTree(fromValue), toValueType);
        }
    }

    private <T> T readJsonString(String fromValue, Class<T> toValueType) {
        if (isMap(toValueType) || isList(toValueType)) {
            return jsonReader.fromJson(fromValue, toValueType);
        } else {
            return regularEngine.fromJson(fromValue, toValueType);
        }
    }

    private boolean isMap(Class<?> type) {
        return type.equals(HashMap.class) || type.equals(Map.class);
    }

    private boolean isList(Class<?> type) {
        return type.equals(ArrayList.class) || type.equals(List.class);
    }



    public <T> T restoreGeneric(Object fromValue, Class<T> toValueType, Class<?>... args) {
        if (fromValue instanceof Map) {
            return regularEngine.fromJson(regularEngine.toJsonTree(fromValue), TypeToken.getParameterized(toValueType, args).getType());
        } else if (fromValue instanceof byte[]) {
            return regularEngine.fromJson(util.getUTF((byte[]) fromValue), TypeToken.getParameterized(toValueType, args).getType());
        } else {
            throw new IllegalArgumentException("Unable to restore to "+fromValue.getClass().getName()+" because payload is not byte array or map");
        }
    }

}
