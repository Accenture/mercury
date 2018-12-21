/*

    Copyright 2018 Accenture Technology

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

public class Kv {

    public String key;
    public String value;

    /**
     * Key-Value Pair
     *
     * @param key in string
     * @param value object will be converted to string
     */
    public Kv(String key, Object value) {
        this.key = key;
        this.value = value == null? "" : (value instanceof String? (String) value : value.toString());
    }

}
