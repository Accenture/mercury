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
 * customSerializer is optional. It allows you to apply a pre-configured ObjectMapper suitable for
 * your user function. It should point to a fully qualified classpath to a class implementing
 * the CustomSerializer interface.
 * <p>
 * envInstances is optional. If present, it must be a parameter in application.properties (or application.yml).
 * The parameter may fetch value from an environment variable using "${ENV_VAR:default_value}" format.
 * If the parameter does not exist, or it does not resolve to a numeric value, the "instances" value in this
 * annotation will be used instead.
 */
@Target({ElementType.TYPE})
@Retention(RetentionPolicy.RUNTIME)
@Documented
public @interface PreLoad {

    String route();
    String customSerializer() default "";
    int instances() default 1;
    String envInstances() default "";
    boolean isPrivate() default true;

}
