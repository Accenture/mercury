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

import org.platformlambda.core.util.Utility;

import java.util.Date;

public class FutureEvent {

    public Date time;
    public String to;
    public long taskId;

    public FutureEvent(String to, long taskId, Date time) {
        this.to = to;
        this.taskId = taskId;
        this.time = time;
    }

    public String getTime() {
        return Utility.getInstance().date2str(time, true);
    }

}
