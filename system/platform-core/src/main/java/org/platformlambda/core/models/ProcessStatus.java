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

import java.util.HashMap;
import java.util.Map;

public class ProcessStatus {

    private boolean success = true;
    private boolean delivered = true;
    private float executionTime;
    private int status = 0;
    private String exception;
    private String deliveryError;
    private Map<String, Object> inputOutput = new HashMap<>();

    public ProcessStatus setUnDelivery(String error) {
        this.delivered = false;
        this.deliveryError = error;
        return this;
    }

    public boolean isDelivered() {
        return delivered;
    }

    public String getDeliveryError() {
        return deliveryError;
    }

    public ProcessStatus setExecutionTime(float executionTime) {
        this.executionTime = executionTime;
        this.success = true;
        return this;
    }

    public float getExecutionTime() {
        return executionTime;
    }

    public boolean isSuccess() {
        return success;
    }

    public ProcessStatus setInputOutput(Map<String, Object> inputOutput) {
        this.inputOutput = inputOutput;
        return this;
    }

    public Map<String, Object> getInputOutput() {
        return inputOutput;
    }

    public ProcessStatus setException(int status, String exception) {
        this.status = status;
        this.exception = exception;
        this.success = false;
        return this;
    }

    public int getStatus() {
        return status;
    }

    public String getException() {
        return exception;
    }


}
