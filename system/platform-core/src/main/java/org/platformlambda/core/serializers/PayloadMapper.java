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

import org.platformlambda.core.models.TypedPayload;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.ManagedCache;
import org.platformlambda.core.util.Utility;

import java.util.ArrayList;
import java.util.Date;
import java.util.List;
import java.util.Map;

public class PayloadMapper {
    public static final String MAP = "M";
    public static final String ARRAY = "A";
    public static final String LIST = "L";
    public static final String PRIMITIVE = "P";
    public static final String NOTHING = "N";
    public static final String JAVA_CLASS_CACHE = "java.class.cache";
    private static final long FIVE_MINUTE = 5 * 60 * 1000;

    private static final ManagedCache cache = ManagedCache.createCache(JAVA_CLASS_CACHE, FIVE_MINUTE);
    private static final PayloadMapper instance = new PayloadMapper();

    private static Boolean enablePoJo;

    private PayloadMapper() {
        if (enablePoJo == null) {
            // by default, pojo transport is enabled
            AppConfigReader config = AppConfigReader.getInstance();
            enablePoJo = "true".equals(config.getProperty("enable.pojo.transport", "true"));
        }
    }

    public static PayloadMapper getInstance() {
        return instance;
    }

    public TypedPayload encode(Object obj, boolean binary) {
        if (obj == null) {
            return new TypedPayload(NOTHING, null);
        } else if (obj instanceof Map) {
            return new TypedPayload(MAP, obj);
        } else if (obj instanceof Object[]) {
            List<Object> list = new ArrayList<>();
            Object[] objects = (Object[]) obj;
            for (Object o: objects) {
                if (isPrimitive(o) || o instanceof Map) {
                    list.add(o);
                } else {
                    throw new IllegalArgumentException("Unable to serialize because object is an array of non-primitive types");
                }
            }
            return new TypedPayload(ARRAY, list);
        } else if (obj instanceof List) {
            List<Object> list = new ArrayList<>();
            List objects = (List) obj;
            for (Object o: objects) {
                if (isPrimitive(o) || o instanceof Map) {
                    list.add(o);
                } else {
                    throw new IllegalArgumentException("Unable to serialize because object is a list of non-primitive types");
                }
            }
            return new TypedPayload(LIST, list);
        } else if (isPrimitive(obj)) {
            return new TypedPayload(PRIMITIVE, obj);
        } else {
            SimpleObjectMapper mapper = SimpleMapper.getInstance().getMapper();
            // convert PoJo to typed payload (type and encoded map)
            if (binary) {
                return new TypedPayload(obj.getClass().getName(), mapper.readValue(obj, Map.class));
            } else {
                return new TypedPayload(obj.getClass().getName(), mapper.writeValueAsBytes(obj));
            }
        }
    }

    @SuppressWarnings("unchecked")
    public Object decode(TypedPayload typed) throws ClassNotFoundException {
        String type = typed.getType();
        if (NOTHING.equals(type)) {
            return null;
        }
        if (PRIMITIVE.equals(type) || LIST.equals(type) || MAP.equals(type)) {
            return typed.getPayload();
        }
        if (ARRAY.equals(type)) {
            List<Object> objects = (List<Object>) typed.getPayload();
            Object[] result = new Object[objects.size()];
            for (int i = 0; i < result.length; i++) {
                result[i] = objects.get(i);
            }
            return result;
        }
        if (type.contains(".") && enablePoJo) {
            // best effort to convert to original class
            Class<?> cls = getClassByName(type);
            if (cls != null) {
                List<String> paraClass = Utility.getInstance().split(typed.getParametricType(), ", ");
                SimpleObjectMapper mapper = SimpleMapper.getInstance().getMapper();
                if (paraClass.isEmpty()) {
                    return mapper.readValue(typed.getPayload(), cls);
                } else {
                    Class<?>[] paraClsList = new Class<?>[paraClass.size()];
                    for (int i=0; i < paraClass.size(); i++) {
                        Class<?> pc = getClassByName(paraClass.get(i));
                        if (pc == null) {
                            throw new ClassNotFoundException(paraClass.get(i)+" not found");
                        }
                        paraClsList[i] = pc;
                    }
                    return mapper.restoreGeneric(typed.getPayload(), cls, paraClsList);
                }
            } else {
                throw new ClassNotFoundException(type+" not found");
            }

        } else {
            // return original payload because it is not a typed object
            return typed.getPayload();
        }
    }

    private Class<?> getClassByName(String name) {
        Object cached = cache.get(name);
        if (cached instanceof Class) {
            return (Class) cached;
        }
        if (cached instanceof Boolean) {
            return null;
        }
        try {
            Class<?> cls = Class.forName(name);
            cache.put(name, cls);
            return cls;

        } catch (ClassNotFoundException e) {
            cache.put(name, false);
            return null;
        }
    }

    private boolean isPrimitive(Object obj) {
        return (obj instanceof String || obj instanceof byte[] || obj instanceof Number ||
                obj instanceof Boolean || obj instanceof Date);
    }

}
