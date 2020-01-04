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

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class MultiLevelMap {

    private static final String TRUE = Boolean.TRUE.toString();
    private static final String FALSE = Boolean.FALSE.toString();
    private static final String NULL = "null";

    private Map<String, Object> multiLevels;

    public MultiLevelMap(Map<String, Object> map) {
        this.multiLevels = map;
    }

    public Map<String, Object> getMap() {
        return multiLevels;
    }

    /**
     * Retrieve an element from a map using a composite path
     * e.g. "some.key", "some.array[3]"
     *
     * @param compositePath using dot-bracket convention
     * @return element
     */
    public Object getElement(String compositePath) {
        return getElement(compositePath, multiLevels);
    }

    public MultiLevelMap setElement(String compositePath, Object value) {
        setElement(compositePath, value, multiLevels);
        return this;
    }

    private Object getElement(String compositePath, Map<String, Object> map) {
        if (compositePath == null || map == null) return null;
        if (!compositePath.contains(".") && !compositePath.contains("/") && !compositePath.contains("[")) {
            return map.get(compositePath);
        }
        Utility util = Utility.getInstance();
        List<String> list = util.split(compositePath, "./");
        Map o = map;
        int len = list.size();
        int n = 0;
        for (String p: list) {
            n++;
            if (p.contains("[") && p.endsWith("]") && !p.startsWith("[")) {
                // does not support array of array
                int bracketStart = p.indexOf('[');
                int bracketEnd = p.lastIndexOf(']');
                if (bracketStart > bracketEnd) break;

                String key = p.substring(0, bracketStart);
                String index = p.substring(bracketStart+1, bracketEnd).trim();

                if (index.length() == 0) break;
                if (!util.isNumeric(index)) break;

                int i = util.str2int(index);
                if (i < 0) break;

                if (o.containsKey(key)) {
                    Object x = o.get(key);
                    if (x instanceof List) {
                        List y = (List) x;
                        if (i >= y.size()) {
                            break;
                        } else {
                            if (n == len) {
                                return y.get(i);
                            } else if (y.get(i) instanceof Map) {
                                o = (Map) y.get(i);
                                continue;
                            }
                        }
                    }
                }
            } else {
                if (o.containsKey(p)) {
                    Object x = o.get(p);
                    if (n == len) {
                        return x;
                    } else if (x instanceof Map) {
                        o = (Map) x;
                        continue;
                    }
                }
            }
            // item not found
            break;
        }
        return null;
    }

    @SuppressWarnings("unchecked")
    private void setElement(String compositePath, Object value, Map<String, Object> map) {
        // Recursively create node if not found. Otherwise update it
        List<String> path = Utility.getInstance().split(compositePath, "./");
        int size = path.size();
        String element = path.get(size-1);
        String parentPath = getParentPath(compositePath);

        Object current = getElement(compositePath, map);
        if (current != null) {
            if (isListElement(element)) {
                String parent = compositePath.substring(0, compositePath.lastIndexOf('['));
                int n = getIndex(element);
                Object cp = getElement(parent, map);
                if (cp instanceof List) {
                    // update element in list
                    ((List) cp).set(n, getPrimitiveOrObject(value));
                }
            } else {
                if (parentPath == null) {
                    // root level
                    map.put(element, getPrimitiveOrObject(value));
                } else {
                    // get parent and update map
                    Object o = getElement(parentPath, map);
                    if (o instanceof Map) {
                        ((Map) o).put(element, getPrimitiveOrObject(value));
                    }
                }

            }
        } else {
            if (size == 1) {
                if (isListElement(element)) {
                    String listElement = element.substring(0, element.lastIndexOf('['));
                    String parent = compositePath.substring(0, compositePath.lastIndexOf('['));
                    int n = getIndex(element);
                    Object cp = getElement(parent, map);
                    if (cp == null) {
                        List<Object> nlist = new ArrayList<>();
                        if (n > 0) {
                            for (int i=0; i < n; i++) {
                                nlist.add(null);
                            }
                        }
                        nlist.add(getPrimitiveOrObject(value));
                        map.put(listElement, nlist);
                    } else if (cp instanceof List) {
                        List<Object> olist = (List<Object>) cp;
                        if (olist.size() > n) {
                            // update element in list
                            ((List) cp).set(n, getPrimitiveOrObject(value));
                        } else {
                            // insert place-holders if necessary
                            while (n > olist.size()) {
                                ((List) cp).add(null);
                            }
                            // insert new element into list
                            ((List) cp).add(getPrimitiveOrObject(value));
                        }
                    }
                } else {
                    map.put(element, getPrimitiveOrObject(value));
                }

            } else {
                if (isListElement(element)) {
                    String listElement = element.substring(0, element.lastIndexOf('['));
                    String parent = compositePath.substring(0, compositePath.lastIndexOf('['));
                    int n = getIndex(element);
                    Object cp = getElement(parent, map);
                    if (cp == null) {
                        List<Object> nlist = new ArrayList<>();
                        if (n > 0) {
                            for (int i=0; i < n; i++) {
                                nlist.add(null);
                            }
                        }
                        nlist.add(getPrimitiveOrObject(value));
                        Object o = getElement(parentPath, map);
                        if (o instanceof Map) {
                            ((Map) o).put(listElement, nlist);
                        } else {
                            HashMap<String, Object> c = new HashMap<>();
                            c.put(listElement, nlist);
                            setElement(parentPath, c, map);
                        }

                    } else if (cp instanceof List) {
                        List<Object> olist = (List<Object>) cp;
                        if (olist.size() > n) {
                            // update element in list
                            ((List) cp).set(n, getPrimitiveOrObject(value));
                        } else {
                            // insert place-holders if necessary
                            while (n > olist.size()) {
                                ((List) cp).add(null);
                            }
                            // insert new element into list
                            ((List) cp).add(getPrimitiveOrObject(value));
                        }
                    }
                } else {
                    Object o = getElement(parentPath, map);
                    if (o instanceof Map) {
                        ((Map) o).put(element, getPrimitiveOrObject(value));
                    } else {
                        HashMap<String, Object> c = new HashMap<>();
                        c.put(element, getPrimitiveOrObject(value));
                        setElement(parentPath, c, map);
                    }
                }
            }
        }
    }

    private boolean isListElement(String item) {
        return (item.contains("[") && item.endsWith("]") && !item.startsWith("["));
    }

    private Object getPrimitiveOrObject(Object v) {
        if (v instanceof String) {
            Utility util = Utility.getInstance();
            String value = (String) v;
            if (value.length() > 0) {
                if (value.equalsIgnoreCase(TRUE)) return true;
                if (value.equalsIgnoreCase(FALSE)) return false;
                if (value.equalsIgnoreCase(NULL)) return null;
                if (util.isNumeric(value)) return util.str2long(value);
            }
        }
        return v;
    }

    private int getIndex(String item) {
        int bracketStart = item.lastIndexOf('[');
        int bracketEnd = item.lastIndexOf(']');
        return Utility.getInstance().str2int(item.substring(bracketStart+1, bracketEnd).trim());
    }

    private String getParentPath(String path) {
        return (path.contains(".")) ? path.substring(0, path.lastIndexOf('.')) : null;
    }

}
