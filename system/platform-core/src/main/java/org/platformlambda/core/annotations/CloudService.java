/*

    Copyright 2018-2019 Accenture Technology

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
 * This indicates the annotated class is a cloud service "plug-in" module.
 * The selection of cloud service modules is defined at run-time using application.properties "cloud.services"
 */
@Target({ElementType.TYPE})
@Retention(RetentionPolicy.RUNTIME)
@Documented
public @interface CloudService {
    /*
     * name of the cloud service
     */
    String value();
    /*
     * You may create a cloud service as a wrapper of another cloud service.
     * In this case, the "original" cloud service will be executed
     * after the initialize() method of the wrapper is completed.
     */
    String original() default "none";
}
