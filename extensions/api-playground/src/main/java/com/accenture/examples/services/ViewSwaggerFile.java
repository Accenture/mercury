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
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.AsyncHttpRequest;
import org.platformlambda.core.models.TypedLambdaFunction;
import org.platformlambda.core.util.Utility;

import java.io.InputStream;
import java.util.Map;

@PreLoad(route = "v1.view.swagger.file", instances=10)
public class ViewSwaggerFile implements TypedLambdaFunction<AsyncHttpRequest, String> {

    private static final String SWAGGER_LOCATION = "/sample/yaml";

    @Override
    public String handleEvent(Map<String, String> headers, AsyncHttpRequest input, int instance) throws AppException {
        String filename = input.getPathParameter("filename");
        if (filename == null) {
            throw new IllegalArgumentException("Missing filename in path parameter");
        }
        Utility util = Utility.getInstance();
        InputStream in = this.getClass().getResourceAsStream(SWAGGER_LOCATION + "/" +filename);
        if (in == null) {
            throw new AppException(404, "File not found");
        }
        return util.stream2str(in);
    }
}
