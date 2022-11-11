/*

    Copyright 2018-2022 Accenture Technology

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
    private final Map<String, Object> multiLevels;

    public MultiLevelMap() {
        this.multiLevels = new HashMap<>();
    }

    public MultiLevelMap(Map<String, Object> map) {
        this.multiLevels = map == null? new HashMap<>() : map;
    }

    public Map<String, Object> getMap() {
        return multiLevels;
    }

    public boolean isEmpty() {
        return multiLevels.isEmpty();
    }

    public boolean exists(String compositePath) {
        Object element = getElement(compositePath, multiLevels);
        return element != null && !(element instanceof NotFound);
    }

    public boolean keyExists(String compositePath) {
        Object element = getElement(compositePath, multiLevels);
        return !(element instanceof NotFound);
    }

    /**
     * Retrieve an element from a map using a composite path
     * (Nested array is supported)
     * e.g. "some.key", "some.array[3]", "hello.world[2][10][1]"
     *
     * @param compositePath using dot-bracket convention
     * @return element
     */
    public Object getElement(String compositePath) {
        Object element = getElement(compositePath, multiLevels);
        return element instanceof NotFound? null : element;
    }

    @SuppressWarnings("unchecked")
    private Object getListElement(List<Integer> indexes, List<Object> data) {
        List<Object> current = data;
        int n = 0;
        int len = indexes.size();
        for (Integer i: indexes) {
            n++;
            if (i < 0 || i >= current.size()) {
                break;
            }
            Object o = current.get(i);
            if (n == len) {
                return o;
            }
            if (o instanceof List) {
                current = (List<Object>) o;
            } else {
                break;
            }
        }
        return null;
    }

    @SuppressWarnings("unchecked")
    private Object getElement(String path, Map<String, Object> map) {
        if (path == null || map == null || map.isEmpty()) return null;
        if (map.containsKey(path)) {
            return map.get(path);
        }
        if (!isComposite(path)) {
            return null;
        }
        Utility util = Utility.getInstance();
        List<String> list = util.split(path, "./");
        Map<String, Object> current = map;
        int len = list.size();
        int n = 0;
        for (String p: list) {
            n++;
            if (isListElement(p)) {
                int start = p.indexOf('[');
                int end = p.indexOf(']', start);
                if (end == -1) break;
                String key = p.substring(0, start);
                String index = p.substring(start+1, end).trim();
                if (index.length() == 0 || !util.isDigits(index)) break;
                if (current.containsKey(key)) {
                    Object nextList = current.get(key);
                    if (nextList instanceof List) {
                        List<Integer> indexes = getIndexes(p.substring(start));
                        Object next = getListElement(indexes, (List<Object>) nextList);
                        if (n == len) {
                            return next;
                        }
                        if (next instanceof Map) {
                            current = (Map<String, Object>) next;
                            continue;
                        }
                    }
                }
            } else {
                if (current.containsKey(p)) {
                    Object next = current.get(p);
                    if (n == len) {
                        return next;
                    } else if (next instanceof Map) {
                        current = (Map<String, Object>) next;
                        continue;
                    }
                }
            }
            // item not found
            break;
        }
        return new NotFound();
    }

    public MultiLevelMap setElement(String compositePath, Object value) {
        validateCompositePathSyntax(compositePath);
        setElement(compositePath, value, multiLevels, false);
        return this;
    }

    public MultiLevelMap removeElement(String compositePath) {
        validateCompositePathSyntax(compositePath);
        setElement(compositePath, null, multiLevels, true);
        return this;
    }

    @SuppressWarnings("unchecked")
    private void setElement(String path, Object value, Map<String, Object> map, boolean delete) {
        Utility util = Utility.getInstance();
        List<String> segments = util.split(path, "./");
        if (segments.isEmpty()) {
            return;
        }
        Map<String, Object> current = map;
        int len = segments.size();
        int n = 0;
        // reconstruct the composite as we walk the path segments
        StringBuilder composite = new StringBuilder();
        for (String p : segments) {
            n++;
            if (isListElement(p)) {
                int sep = p.indexOf('[');
                List<Integer> indexes = getIndexes(p.substring(sep));
                String element = p.substring(0, sep);
                Object parent = getElement(composite+element, map);
                if (n == len) {
                    if (parent instanceof List) {
                        setListElement(indexes, (List<Object>) parent, value);
                    } else {
                        List<Object> newList = new ArrayList<>();
                        setListElement(indexes, newList, value);
                        current.put(element, newList);
                    }
                    break;
                } else {
                    if (parent instanceof List) {
                        Object next = getElement(composite+p, map);
                        if (next instanceof Map) {
                            current = (Map<String, Object>) next;
                        } else {
                            Map<String, Object> m = new HashMap<>();
                            setListElement(indexes, (List<Object>) parent, m);
                            current = m;
                        }
                    } else {
                        Map<String, Object> nextMap = new HashMap<>();
                        List<Object> newList = new ArrayList<>();
                        setListElement(indexes, newList, nextMap);
                        current.put(element, newList);
                        current = nextMap;
                    }
                }

            } else {
                if (n == len) {
                    if (value == null && delete) {
                        current.remove(p);
                    } else {
                        current.put(p, value);
                    }
                    break;
                } else {
                    Object next = current.get(p);
                    if (next instanceof Map) {
                        current = (Map<String, Object>) next;
                    } else {
                        Map<String, Object> nextMap = new HashMap<>();
                        current.put(p, nextMap);
                        current = nextMap;
                    }
                }
            }
            composite.append(p).append('.');
        }
    }

    @SuppressWarnings("unchecked")
    private void setListElement(List<Integer> indexes, List<Object> dataset, Object value) {
        List<Object> current = expandList(indexes, dataset);
        int len = indexes.size();
        for (int i=0; i < len; i++) {
            int idx = indexes.get(i);
            if (i == len - 1) {
                current.set(idx, value);
            } else {
                Object o = current.get(idx);
                if (o instanceof List) {
                    current = (List<Object>) o;
                }
            }
        }
    }

    @SuppressWarnings("unchecked")
    private List<Object> expandList(List<Integer> indexes, List<Object> dataset) {
        List<Object> current = dataset;
        int len = indexes.size();
        for (int i=0; i < len; i++) {
            int idx = indexes.get(i);
            if (idx >= current.size()) {
                int diff = idx - current.size();
                while (diff-- >= 0) {
                    current.add(null);
                }
            }
            if (i == len - 1) {
                break;
            }
            Object o = current.get(idx);
            if (o instanceof List) {
                current = (List<Object>) o;
            } else {
                List<Object> newList = new ArrayList<>();
                current.set(idx, newList);
                current = newList;
            }
        }
        return dataset;
    }

    private boolean isComposite(String item) {
        return item.contains(".") || item.contains("/") || item.contains("[") || item.contains("]");
    }

    private boolean isListElement(String item) {
        return (item.contains("[") && item.endsWith("]") && !item.startsWith("["));
    }

    private List<Integer> getIndexes(String indexSegment) {
        Utility util = Utility.getInstance();
        List<String> indexes = util.split(indexSegment, "[]");
        List<Integer> result = new ArrayList<>();
        for (String s: indexes) {
            result.add(util.str2int(s));
        }
        return result;
    }

    public void validateCompositePathSyntax(String path) {
        Utility util = Utility.getInstance();
        List<String> segments = util.split(path, "./");
        if (segments.isEmpty()) {
            throw new IllegalArgumentException("Missing composite path");
        }
        for (String s: segments) {
            if (s.contains("[") || s.contains("]")) {
                if (!s.contains("[")) {
                    throw new IllegalArgumentException("Invalid composite path - missing start bracket");
                }
                if (!s.endsWith("]")) {
                    throw new IllegalArgumentException("Invalid composite path - missing end bracket");
                }
                // check start-end pair
                int sep1 = s.indexOf('[');
                int sep2 = s.indexOf(']');
                if (sep2 < sep1) {
                    throw new IllegalArgumentException("Invalid composite path - missing start bracket");
                }
                boolean start = false;
                for (char c: s.substring(sep1).toCharArray()) {
                    if (c == '[') {
                        if (start) {
                            throw new IllegalArgumentException("Invalid composite path - missing end bracket");
                        } else {
                            start = true;
                        }
                    } else if (c == ']') {
                        if (!start) {
                            throw new IllegalArgumentException("Invalid composite path - duplicated end bracket");
                        } else {
                            start = false;
                        }
                    } else {
                        if (start) {
                            if (c < '0' || c > '9') {
                                throw new IllegalArgumentException("Invalid composite path - indexes must be digits");
                            }
                        } else {
                            throw new IllegalArgumentException("Invalid composite path - invalid indexes");
                        }
                    }
                }
            }
        }
    }

    private static class NotFound { }

}
