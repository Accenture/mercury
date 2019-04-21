package org.platformlambda.core.serializers;

import com.google.gson.Gson;
import com.google.gson.reflect.TypeToken;
import org.platformlambda.core.util.Utility;

import java.io.InputStream;
import java.util.Map;

public class SimpleObjectMapper {

    private Gson regularEngine, objectEngine;
    private Utility util;

    public SimpleObjectMapper(Gson regularEngine, Gson objectEngine) {
        this.regularEngine = regularEngine;
        this.objectEngine = objectEngine;
        this.util = Utility.getInstance();
    }

    public String writeValueAsString(Object value) {
        return objectEngine.toJson(value);
    }

    public byte[] writeValueAsBytes(Object value) {
        return Utility.getInstance().getUTF(writeValueAsString(value));
    }

    public <T> T readValue(Object fromValue, Class<T> toValueType) {
        if (fromValue instanceof InputStream) {
            // convert input stream to a JSON string first
            return regularEngine.fromJson(util.stream2str((InputStream) fromValue), toValueType);
        } else if (fromValue instanceof String) {
            // input is a JSON string
            return regularEngine.fromJson((String) fromValue, toValueType);
        } else if (fromValue instanceof byte[]) {
            // input is a byte array of JSON
            return regularEngine.fromJson(util.getUTF((byte[]) fromValue), toValueType);
        } else {
            // input is an object
            return regularEngine.fromJson(regularEngine.toJsonTree(fromValue), toValueType);
        }
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
