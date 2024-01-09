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
package org.platformlambda.core.actuator

import org.platformlambda.core.exception.AppException
import org.platformlambda.core.models.EventEnvelope
import org.platformlambda.core.models.KotlinLambdaFunction
import org.platformlambda.core.models.Kv
import org.platformlambda.core.system.FastRPC
import org.platformlambda.core.system.Platform
import org.platformlambda.core.system.EventEmitter
import org.platformlambda.core.util.AppConfigReader
import org.platformlambda.core.util.SimpleCache
import org.platformlambda.core.util.Utility
import org.slf4j.LoggerFactory
import java.io.IOException

class HealthService : KotlinLambdaFunction<EventEnvelope, Any> {
    private val requiredServices: List<String>
    private val optionalServices: List<String>

    init {
        val reader = AppConfigReader.getInstance()
        requiredServices = Utility.getInstance().split(reader.getProperty(REQUIRED_SERVICES, ""), ", ")
        if (requiredServices.isNotEmpty()) {
            log.info("Mandatory service dependencies - {}", requiredServices)
        }
        optionalServices = Utility.getInstance().split(reader.getProperty(OPTIONAL_SERVICES, ""), ", ")
        if (optionalServices.isNotEmpty()) {
            log.info("Optional services dependencies - {}", optionalServices)
        }
    }

    override suspend fun handleEvent(headers: Map<String, String>, input: EventEnvelope, instance: Int): Any {
        val fastRPC = FastRPC(headers)
        val platform = Platform.getInstance()
        val po = EventEmitter.getInstance();
        var up = true
        val result: MutableMap<String, Any> = HashMap()
        /*
         * Checking dependencies
         */
        val upstream: MutableList<Map<String, Any?>> = ArrayList()
        checkServices(fastRPC, upstream, optionalServices, false)
        if (!checkServices(fastRPC, upstream, requiredServices, true)) {
            up = false
        }
        // save the current health status
        po.send(EventEmitter.ACTUATOR_SERVICES, up, Kv(TYPE, HEALTH_STATUS))
        // checkServices will update the "upstream" service list
        result[UPSTREAM] = upstream
        if (upstream.isEmpty()) {
            result[MESSAGE] = "Did you forget to define $REQUIRED_SERVICES or $OPTIONAL_SERVICES"
        }
        result[STATUS] = if (up) "UP" else "DOWN"
        result[ORIGIN] = platform.origin
        result[NAME] = platform.name
        val response = EventEnvelope()
                .setHeader(CONTENT_TYPE, APPLICATION_JSON).setBody(result).setStatus(if (up) 200 else 400)
        val accept = headers[ACCEPT] ?: "?"
        if (accept.startsWith(APPLICATION_XML)) {
            response.setHeader(CONTENT_TYPE, APPLICATION_XML)
        } else {
            response.setHeader(CONTENT_TYPE, APPLICATION_JSON)
        }
        return response
    }

    private suspend fun checkServices(fastRPC: FastRPC, upstream: MutableList<Map<String, Any?>>,
                                      services: List<String>, required: Boolean): Boolean {
        var up = true
        for (route in services) {
            val m: MutableMap<String, Any?> = HashMap()
            m[ROUTE] = route
            m[REQUIRED] = required
            upstream.add(m)
            try {
                val key = "$INFO/$route"
                if (!cache.exists(key)) {
                    val req = EventEnvelope().setTo(route).setHeader(TYPE, INFO)
                    val res = fastRPC.awaitRequest(req, 3000)
                    if (res.body is Map<*, *>) {
                        cache.put(key, res.body)
                    }
                }
                val info = cache[key]
                if (info is Map<*, *>) {
                    for (kv in info) {
                        if (kv.key != null) {
                            m[kv.key.toString()] = kv.value
                        }
                    }
                }
                val req = EventEnvelope().setTo(route).setHeader(TYPE, HEALTH)
                val res = fastRPC.awaitRequest(req, 10000)
                if (res.body is String) {
                    m[STATUS_CODE] = res.status
                    m[MESSAGE] = res.body
                    if (res.status != 200) {
                        up = false
                    }
                }
            } catch (e: IOException) {
                up = false
                if (e.message!!.contains(NOT_FOUND)) {
                    m[STATUS_CODE] = 404
                    m[MESSAGE] = PLEASE_CHECK + e.message
                } else {
                    m[STATUS_CODE] = 500
                    m[MESSAGE] = e.message
                }
            } catch (e: AppException) {
                up = false
                m[STATUS_CODE] = e.status
                m[MESSAGE] = e.message
            }
        }
        return up
    }

    companion object {
        private val log = LoggerFactory.getLogger(HealthService::class.java)
        private val cache = SimpleCache.createCache("health.info", 5000)
        private const val TYPE = "type"
        private const val ACCEPT = "accept"
        private const val INFO = "info"
        private const val HEALTH = "health"
        private const val HEALTH_STATUS = "health_status"
        private const val REQUIRED_SERVICES = "mandatory.health.dependencies"
        private const val OPTIONAL_SERVICES = "optional.health.dependencies"
        private const val ROUTE = "route"
        private const val MESSAGE = "message"
        private const val STATUS = "status"
        private const val ORIGIN = "origin"
        private const val NAME = "name"
        private const val STATUS_CODE = "status_code"
        private const val REQUIRED = "required"
        private const val UPSTREAM = "upstream"
        private const val NOT_FOUND = "not found"
        private const val PLEASE_CHECK = "Please check - "
        private const val CONTENT_TYPE = "content-type"
        private const val APPLICATION_JSON = "application/json"
        private const val APPLICATION_XML = "application/xml"
    }
}