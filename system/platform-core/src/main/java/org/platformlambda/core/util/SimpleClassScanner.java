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

package org.platformlambda.core.util;

import io.github.classgraph.ClassGraph;
import io.github.classgraph.ClassInfo;
import io.github.classgraph.ScanResult;

import java.lang.annotation.Annotation;
import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class SimpleClassScanner {

    private static final String[] COMPONENT_SCAN = {"web.component.scan"};
    private static final String PLATFORM_LAMBDA = "org.platformlambda";
    private static final String EX_START = "Invalid package path (";
    private static final String EX_END = "). A proper package should have at least one dot character.";
    private static final SimpleClassScanner INSTANCE = new SimpleClassScanner();

    private SimpleClassScanner() {
        // singleton
    }

    public static SimpleClassScanner getInstance() {
        return INSTANCE;
    }

    public List<ClassInfo> getAnnotatedClasses(Class<? extends Annotation> type, boolean includeBasePackage) {
        List<ClassInfo> result = new ArrayList<>();
        Set<String> packages = getPackages(includeBasePackage);
        for (String p : packages) {
            result.addAll(getAnnotatedClasses(p, type));
        }
        return result;
    }

    public List<ClassInfo> getAnnotatedClasses(String scanPath, Class<? extends Annotation> type) {
        if (!scanPath.contains(".")) {
            throw new IllegalArgumentException(EX_START + scanPath + EX_END);
        }
        try (ScanResult sr = new ClassGraph().enableAllInfo().acceptPackages(scanPath).scan()) {
            return new ArrayList<>(sr.getClassesWithAnnotation(type));
        }
    }

    public Set<String> getPackages(boolean includeBasePackage) {
        Set<String> result = new HashSet<>();
        if (includeBasePackage) {
            result.add(PLATFORM_LAMBDA);
        }
        // consolidate list from web.component.scan
        for (String pc: COMPONENT_SCAN) {
            result.addAll(getScanComponents(pc));
        }
        return result;
    }

    private Set<String> getScanComponents(String components) {
        Set<String> result = new HashSet<>();
        AppConfigReader reader = AppConfigReader.getInstance();
        String list = reader.getProperty(components);
        if (list != null) {
            List<String> packages = Utility.getInstance().split(list, ", []");
            for (String p : packages) {
                if (!p.startsWith(PLATFORM_LAMBDA)) {
                    if (!p.contains(".")) {
                        throw new IllegalArgumentException(EX_START + p + EX_END);
                    } else {
                        result.add(p);
                    }
                }
            }
        }
        return result;
    }

}
