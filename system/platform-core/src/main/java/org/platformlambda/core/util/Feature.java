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

import org.platformlambda.core.annotations.OptionalService;

public class Feature {

    private static final String TRUE = String.valueOf(Boolean.TRUE);

    public static boolean isRequired(Class<?> cls) {
        OptionalService condition = cls.getAnnotation(OptionalService.class);
        if (condition != null) {
            String value = condition.value();
            if (value.startsWith("!")) {
                return !matched(value.substring(1));
            } else {
                return matched(value);
            }

        } else {
            return true;
        }
    }

    private static boolean matched(String condition) {
        AppConfigReader reader = AppConfigReader.getInstance();
        if (condition.contains("=")) {
            int eq = condition.indexOf('=');
            String k = condition.substring(0, eq);
            String v = condition.substring(eq+1);
            if (k.length() > 0) {
                if (v.length() > 0) {
                    return v.equalsIgnoreCase(reader.getProperty(k));
                } else {
                    return TRUE.equalsIgnoreCase(reader.getProperty(k));
                }
            } else {
                return false;
            }

        } else {
            return TRUE.equalsIgnoreCase(reader.getProperty(condition));
        }
    }

}
