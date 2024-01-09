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

package com.accenture.examples.rest;

import io.vertx.core.Future;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.Utility;
import org.platformlambda.models.SamplePoJo;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PathVariable;
import org.springframework.web.bind.annotation.RestController;
import reactor.core.publisher.Mono;

import java.io.IOException;

@RestController
public class HelloPoJoServiceMesh {

    @GetMapping("/api/pojo/mesh/{id}")
    public Mono<SamplePoJo> getPoJo(@PathVariable("id") Integer id) {
        String traceId = Utility.getInstance().getUuid();
        PostOffice po = new PostOffice("hello.world.endpoint", traceId, "GET /api/pojo/mesh");
        EventEnvelope req = new EventEnvelope().setTo("hello.pojo").setHeader("id", id);
        return Mono.create(callback -> {
            try {
                po.asyncRequest(req, 5000)
                    .onSuccess(event -> {
                        // confirm that the PoJo object is transported correctly over the event stream system
                        if (event.getBody() instanceof SamplePoJo result) {
                            callback.success(result);
                        } else {
                            callback.error(new AppException(event.getStatus(), event.getError()));
                        }
                    })
                    .onFailure(ex -> callback.error(new AppException(408, ex.getMessage())));
            } catch (IOException e) {
                callback.error(e);
            }
        });
    }
}
