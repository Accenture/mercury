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

package org.platformlambda.automation.models;

import java.util.List;

public class SimpleHttpFilter {

    public List<String> pathList;
    public List<String> exclusionList;
    public String service;

    public SimpleHttpFilter(List<String> pathList, List<String> exclusionList, String service) {
        this.pathList = pathList;
        this.exclusionList = exclusionList;
        this.service = service;
    }
}
