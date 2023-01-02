/*

    Copyright 2018-2023 Accenture Technology

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

import org.platformlambda.core.system.Platform;

import java.util.Map;

public interface StreamFunction {
    /**
     * This will be set when your function starts
     *
     * @param manager name - this is the manager's route name of this function
     */
    void init(String manager);

    /**
     * You must implement this to return the manager name from the init method
     *
     * @return manager
     */
    String getManager();

    /**
     * Implement this handler to receive events
     *
     * @param headers of an event
     * @param body of an event
     * @throws Exception in case of error
     */
    void handleEvent(Map<String, String> headers, Object body) throws Exception;

    /**
     * Send a ready signal to the manager to fetch the next event.
     */
    default void get() {
        Platform.getInstance().getEventSystem().send(getManager(), "ready");
    }

}
