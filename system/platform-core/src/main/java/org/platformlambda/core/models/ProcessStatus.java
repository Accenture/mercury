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

package org.platformlambda.core.models;

public class ProcessStatus {

    public boolean success;
    public float executionTime;
    public int status;
    public String exception;

    /**
     * Positive result
     *
     * @param executionTime in milliseconds
     */
    public ProcessStatus(float executionTime) {
        this.executionTime = executionTime;
        this.success = true;
    }

    /**
     * Negative result
     *
     * @param status code
     * @param exception message
     */
    public ProcessStatus(int status, String exception) {
        this.status = status;
        this.exception = exception;
        this.success = false;
    }

}
