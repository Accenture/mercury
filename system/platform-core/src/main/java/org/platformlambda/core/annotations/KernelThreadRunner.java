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
 * The default execution mode for a function is virtual thread.
 * To execute your function using kernel thread, add this annotation to your function class.
 * <p>
 * (Kernel threads are limited resources, and they should be used only when absolutely necessary.
 * For example, you can use kernel threads for computational intensive tasks.
 * <p>
 * Note that when your function running in a kernel thread blocks while waiting for I/O, database,
 * or response from an external resource, it is consuming CPU cycle. As a result, overall
 * application performance may be reduced.)
 */
@Target({ElementType.TYPE})
@Retention(RetentionPolicy.RUNTIME)
@Documented
public @interface KernelThreadRunner { }