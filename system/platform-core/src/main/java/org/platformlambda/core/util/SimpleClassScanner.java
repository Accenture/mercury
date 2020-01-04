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

import org.springframework.beans.factory.config.BeanDefinition;
import org.springframework.context.annotation.ClassPathScanningCandidateComponentProvider;
import org.springframework.core.type.filter.AnnotationTypeFilter;

import java.lang.annotation.Annotation;
import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class SimpleClassScanner {

    private static final String[] COMPONENT_SCAN = {"web.component.scan", "cloud.component.scan"};
    private static final String PLATFORM_LAMBDA = "org.platformlambda";
    private static String EX_START = "Invalid package path (";
    private static String EX_END = "). A proper package should have at least one dot character.";
    private static final SimpleClassScanner instance = new SimpleClassScanner();

    private SimpleClassScanner() {
        // singleton
    }

    public static SimpleClassScanner getInstance() {
        return instance;
    }

    public List<Class<?>> getAnnotatedClasses(Class<? extends Annotation> type, boolean includeBasePackage) {
        List<Class<?>> result = new ArrayList<>();
        Set<String> packages = getPackages(includeBasePackage);
        for (String p : packages) {
            List<Class<?>> services = getAnnotatedClasses(p, type);
            for (Class<?> c: services) {
                result.add(c);
            }
        }
        return result;
    }

    public List<Class<?>> getAnnotatedClasses(String scanPath, Class<? extends Annotation> type) {
        if (!scanPath.contains(".")) {
            throw new IllegalArgumentException(EX_START + scanPath + EX_END);
        }
        List<Class<?>> result = new ArrayList<>();
        ClassPathScanningCandidateComponentProvider provider = new ClassPathScanningCandidateComponentProvider(false);
        provider.addIncludeFilter(new AnnotationTypeFilter(type));
        for (BeanDefinition beanDef : provider.findCandidateComponents(scanPath)) {
            try {
                result.add(Class.forName(beanDef.getBeanClassName()));
            } catch (ClassNotFoundException e) {
                // ok to ignore
            }
        }
        return result;
    }

    public Set<String> getPackages(boolean includeBasePackage) {
        Set<String> result = new HashSet<>();
        if (includeBasePackage) {
            result.add(PLATFORM_LAMBDA);
        }
        // consolidate list from web.component.scan and cloud.component.scan
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
