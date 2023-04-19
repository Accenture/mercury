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
package com.accenture.rest;

import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.Utility;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.RequestHeader;
import org.springframework.web.bind.annotation.RequestParam;
import org.springframework.web.bind.annotation.RestController;
import reactor.core.publisher.Mono;

import java.io.IOException;
import java.util.Map;
import java.util.concurrent.atomic.AtomicLong;

/**
 * Controller skeleton demo code borrowed from https://github.com/spring-guides/gs-rest-service
 */
@RestController
public class GreetingController {
	private final AtomicLong counter = new AtomicLong();

	/**
	 * This demo endpoint returns a Mono publisher.
	 * When the publisher is subscribed by the underlying Spring WebFlux dispatcher,
	 * it makes an asynchronous RPC to the "hello.world" demo service and publishes
	 * the result through the Mono's "success" callback.
	 * <p>
	 * This demo also illustrates distributed trace. Please check application log to see performance metrics.
	 *
	 * @param name value of the query parameter "name"
	 * @return ResponseEntity so we can set HTTP response status code
	 */
	@GetMapping("/greeting")
	public Mono<ResponseEntity<Greeting>> greeting(@RequestParam(value = "name", defaultValue = "") String name,
												   @RequestHeader(value = "x-trace-id") String traceId) {
		return Mono.create(callback -> {
			// propagate the traceId if provided in the HTTP request header
			String id = traceId == null? Utility.getInstance().getUuid() : traceId;
			// set default values
			String value = "Hello, "+(name.isEmpty()? "World" : name);
			String query = name.isEmpty()? "" : "?name="+name;
			// make async RPC call to the "hello.world" function
			PostOffice po = new PostOffice("greeting.controller", id, "GET /greeting"+query);
			EventEnvelope req = new EventEnvelope().setTo("hello.world").setBody(value);
			try {
				po.asyncRequest(req, 5000)
					.onSuccess(event -> {
						if (event.getBody() instanceof Map map) {
							Greeting data = new Greeting(counter.incrementAndGet(), map);
							callback.success(ResponseEntity.status(event.getStatus()).body(data));
						}
					})
					.onFailure(ex -> callback.error(new AppException(408, ex.getMessage())));
			} catch (IOException e) {
				callback.error(e);
			}
		});
	}

}
