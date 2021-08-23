/*

    Copyright 2018-2021 Accenture Technology

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

import org.platformlambda.core.models.PoJoList;
import org.platformlambda.core.models.TypedPayload;
import org.platformlambda.core.util.ManagedCache;
import org.platformlambda.core.util.Utility;

import java.util.*;

public class PayloadMapper {
    public static final String MAP = "M";
    public static final String LIST = "L";
    public static final String PRIMITIVE = "P";
    public static final String NOTHING = "N";
    public static final String JAVA_CLASS_CACHE = "java.class.cache";
    private static final long FIVE_MINUTE = 5 * 60 * 1000;

    private static final ManagedCache cache = ManagedCache.createCache(JAVA_CLASS_CACHE, FIVE_MINUTE);
    private static final PayloadMapper instance = new PayloadMapper();

    public static PayloadMapper getInstance() {
        return instance;
    }

    @SuppressWarnings("unchecked")
    public TypedPayload encode(Object obj, boolean binary) {
        if (obj == null) {
            return new TypedPayload(NOTHING, null);
        } else if (obj instanceof Map) {
            return new TypedPayload(MAP, obj);
        } else if (obj instanceof Object[]) {
            return encodeList(Arrays.asList((Object[]) obj), binary);
        } else if (obj instanceof List) {
            return encodeList((List<Object>) obj, binary);
        } else if (obj instanceof Date) {
            return new TypedPayload(PRIMITIVE, Utility.getInstance().date2str((Date) obj));
        } else if (isPrimitive(obj)) {
            return new TypedPayload(PRIMITIVE, obj);
        } else {
            return getTypedPayload(obj, binary);
        }
    }

    private TypedPayload encodeList(List<Object> objects, boolean binary) {
        boolean primitive = false;
        String pojoInside = null;
        List<Object> list = new ArrayList<>();
        for (Object o: objects) {
            if (o == null) {
                list.add(null);
            } else if (isPrimitive(o) || o instanceof Map) {
                list.add(o);
                primitive = true;
            } else {
                if (pojoInside == null) {
                    pojoInside = o.getClass().getName();
                }
                if (pojoInside.equals(o.getClass().getName())) {
                    list.add(o);
                } else {
                    throw new IllegalArgumentException("Unable to serialize because it is a list of mixed types");
                }
            }
        }
        if (pojoInside != null && primitive) {
            throw new IllegalArgumentException("Unable to serialize because it is a list of mixed types");
        }
        return pojoInside != null? getTypedPayloadFromList(list, pojoInside, binary) : new TypedPayload(LIST, list);
    }

    private TypedPayload getTypedPayloadFromList(List<Object> list, String pojoInside, boolean binary) {
        PoJoList<Object> pojoList = new PoJoList<>();
        for (Object pojo: list) {
            pojoList.add(pojo);
        }
        TypedPayload result = getTypedPayload(pojoList, binary);
        result.setParametricType(pojoInside);
        return result;
    }

    private TypedPayload getTypedPayload(Object obj, boolean binary) {
        SimpleObjectMapper mapper = SimpleMapper.getInstance().getMapper();
        if (binary) {
            return new TypedPayload(obj.getClass().getName(), mapper.readValue(obj, Map.class));
        } else {
            return new TypedPayload(obj.getClass().getName(), mapper.writeValueAsBytes(obj));
        }
    }

    public Object decode(TypedPayload typed) throws ClassNotFoundException {
        String type = typed.getType();
        if (NOTHING.equals(type)) {
            return null;
        }
        if (PRIMITIVE.equals(type) || LIST.equals(type) || MAP.equals(type)) {
            return typed.getPayload();
        }
        if (type.contains(".")) {
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
            return typed.getPayload();
        }
    }

    public Class<?> getClassByName(String name) {
        Object cached = cache.get(name);
        if (cached instanceof Class) {
            return (Class<?>) cached;
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

    public boolean isPrimitive(Object obj) {
        return (obj instanceof String || obj instanceof byte[] || obj instanceof Number || obj instanceof Boolean);
    }

}
