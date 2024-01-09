/*

    Copyright 2018-2024 Accenture Technology

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
package org.platformlambda.core.models;

import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.util.*;

public class EventBlocks {
    private final Map<Integer, byte[]> buffer = new HashMap<>();
    private final String id;

    public EventBlocks(String id) {
        this.id = id;
    }

    public String getId() {
        return id;
    }

    public void put(int n, byte[] block) {
        buffer.put(n, block);
    }

    public boolean exists(Integer n) {
        return buffer.containsKey(n);
    }

    public int size() {
        return buffer.size();
    }

    public byte[] toBytes() throws IOException {
        List<Integer> indexes = new ArrayList<>(buffer.keySet());
        Collections.sort(indexes);
        ByteArrayOutputStream out = new ByteArrayOutputStream();
        for (Integer i: indexes) {
            out.write(buffer.get(i));
        }
        buffer.clear();
        return out.toByteArray();
    }

}
