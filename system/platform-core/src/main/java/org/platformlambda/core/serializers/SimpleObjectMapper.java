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

package org.platformlambda.core.serializers;

import com.google.gson.Gson;
import com.google.gson.JsonElement;
import com.google.gson.reflect.TypeToken;
import org.platformlambda.core.util.Utility;

import java.io.InputStream;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class SimpleObjectMapper {

    private Gson mapGson, objGson;
    private Utility util;

    public SimpleObjectMapper(Gson mapGson, Gson objGson) {
        this.mapGson = mapGson;
        this.objGson = objGson;
        this.util = Utility.getInstance();
    }

    public String writeValueAsString(Object value) {
        return mapGson.toJson(value);
    }

    public byte[] writeValueAsBytes(Object value) {
        return Utility.getInstance().getUTF(writeValueAsString(value));
    }

    @SuppressWarnings("unchecked")
    public <T> T readValue(Object fromValue, Class<T> toValueType) {
        // return original map
        boolean outputIsMap = isMap(toValueType);
        if (fromValue instanceof Map && outputIsMap) {
            return (T) fromValue;
        }
        // return original list
        boolean outputIsList = isList(toValueType);
        if (fromValue instanceof List && outputIsList) {
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
            if (outputIsList || outputIsMap) {
                return mapGson.fromJson(mapGson.toJsonTree(fromValue), toValueType);
            } else {
                return objGson.fromJson(objGson.toJsonTree(fromValue), toValueType);
            }
        }
    }

    private <T> T readJsonString(String fromValue, Class<T> toValueType) {
        if (isMap(toValueType) || isList(toValueType)) {
            return mapGson.fromJson(fromValue, toValueType);
        } else {
            return objGson.fromJson(fromValue, toValueType);
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
            return objGson.fromJson(objGson.toJsonTree(fromValue), TypeToken.getParameterized(toValueType, args).getType());
        } else if (fromValue instanceof byte[]) {
            return objGson.fromJson(util.getUTF((byte[]) fromValue), TypeToken.getParameterized(toValueType, args).getType());
        } else {
            throw new IllegalArgumentException("Unable to restore to "+fromValue.getClass().getName()+" because payload is not byte array or map");
        }
    }

}
