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

import org.platformlambda.core.exception.AppException;

/**
 * You can add this interface to your TypedLambdaFunction or KotlinLambdaFunction
 * to catch object casting exception.
 * <p>
 * For example, if the calling function sends a string to the input that is defined as PoJo,
 * object casting exception will happen. Without this interface, the casting error will be
 * treated as "unhandled exception" and logged.
 * <p>
 * With this interface, your application can catch this and handle properly.
 */
public interface PoJoMappingExceptionHandler {

    void onError(String route, AppException error, EventEnvelope event, int instance);
}
