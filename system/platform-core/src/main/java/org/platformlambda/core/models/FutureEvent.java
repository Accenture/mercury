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

import akka.actor.Cancellable;

import java.util.Date;

public class FutureEvent {

    public Cancellable task;
    public Date time;
    public String to;

    public FutureEvent(String to, Cancellable task, Date time) {
        this.to = to;
        this.task = task;
        this.time = time;
    }

}
