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
 * This indicates that the annotated class will be executed before the MainApplication runs.
 * <p>
 * Note that the "BeforeApplication" class should also implement the EntryPoint interface.
 * <p>
 * Smaller sequence will be executed first
 * <p>
 * normal sequence must be between 3 and 999
 * <p>
 * (Sequence 2 is reserved by the AsyncHttpClientLoader. If your startup code must run
 *  before this system module, you can set sequence to 1)
 */
@Target({ElementType.TYPE})
@Retention(RetentionPolicy.RUNTIME)
@Documented
public @interface BeforeApplication {

    int sequence() default 10;

}
