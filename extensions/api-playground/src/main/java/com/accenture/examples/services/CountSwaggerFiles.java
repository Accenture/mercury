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

package com.accenture.examples.services;

import org.platformlambda.core.annotations.PreLoad;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.util.Utility;

import java.io.InputStream;
import java.util.Date;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

@PreLoad(route = "v1.list.swagger.files", instances=10)
public class CountSwaggerFiles implements LambdaFunction {

    private static final String SWAGGER_LOCATION = "/sample/yaml";

    @Override
    public Object handleEvent(Map<String, String> headers, Object input, int instance) {

        Utility util = Utility.getInstance();
        InputStream in = this.getClass().getResourceAsStream(SWAGGER_LOCATION);
        if (in == null) {
            throw new IllegalArgumentException("Missing swagger files in resources/sample/yaml");
        }
        List<String> fileList = util.split(util.stream2str(in), "\r\n");
        Map<String, Object> result = new HashMap<>();
        result.put("time", new Date());
        result.put("total", fileList.size());
        result.put("list", fileList);
        return result;
    }
}
