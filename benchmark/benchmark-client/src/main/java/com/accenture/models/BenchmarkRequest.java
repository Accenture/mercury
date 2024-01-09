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

package com.accenture.models;

import org.platformlambda.core.util.Utility;

import java.util.Date;

public class BenchmarkRequest {

    public String type;
    public int count;
    public int size;
    public Date start;
    public String cid;
    public long timeSpendPublishing = 0;
    public int received = 0;

    public BenchmarkRequest(String type, int count, int size) {
        this.type = type;
        this.count = Math.max(1, count);
        this.size = Math.max(10, (size / 10) * 10); // to be nearest 10
        this.start = new Date();
        this.cid = Utility.getInstance().getUuid();
    }
}
