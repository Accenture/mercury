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

package org.platformlambda.core.annotations;

import java.lang.annotation.*;

/**
 * This indicates the class is a service to be preloaded.
 * (for a class to be preloaded, it must use a default constructor without arguments)
 * <p>
 * envInstances overrides instances from multiple sources.
 * <p>
 * 1. To get value from an environment variable, use this format ${ENV_VAR_NAME:defaultValue}.
 * 2. To get value from application.properties or application.yml, just set it to the parameter name.
 * Note that System property can override the application.properties/application.yml config.
 */
@Target({ElementType.TYPE})
@Retention(RetentionPolicy.RUNTIME)
@Documented
public @interface PreLoad {

    String route();
    int instances() default 1;
    String envInstances() default "";
    boolean isPrivate() default true;

}
