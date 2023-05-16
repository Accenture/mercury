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
package org.platformlambda.core.system

import io.vertx.core.Handler
import io.vertx.core.eventbus.Message
import io.vertx.kotlin.coroutines.dispatcher
import kotlinx.coroutines.DelicateCoroutinesApi
import kotlinx.coroutines.GlobalScope
import kotlinx.coroutines.launch
import org.platformlambda.core.exception.AppException
import org.platformlambda.core.models.AsyncHttpRequest
import org.platformlambda.core.models.EventEnvelope
import org.platformlambda.core.models.PoJoMappingExceptionHandler
import org.platformlambda.core.models.ProcessStatus
import org.platformlambda.core.util.Utility
import org.slf4j.LoggerFactory
import java.io.IOException
import java.util.*
import java.util.concurrent.TimeoutException

class WorkerQueue(def: ServiceDef, route: String, private val instance: Int) : WorkerQueues(def, route) {
    private val myOrigin: String
    private val useEnvelope: Boolean
    private val async: Boolean
    private var coroutine = false
    private var interceptor = false
    private var tracing = false

    init {
        val system = Platform.getInstance().eventSystem
        consumer = system.localConsumer(route, WorkerHandler())
        myOrigin = Platform.getInstance().origin
        useEnvelope = def.inputIsEnvelope()
        async = def.isKotlin
        coroutine = def.isCoroutine
        interceptor = def.isInterceptor
        tracing = def.isTrackable
        // tell manager that this worker is ready to process a new event
        system.send(def.route, READY + route)
        started()
    }

    private inner class WorkerHandler : Handler<Message<ByteArray?>> {
        @OptIn(DelicateCoroutinesApi::class)
        override fun handle(message: Message<ByteArray?>) {
            if (!stopped) {
                val event = EventEnvelope()
                try {
                    event.load(message.body())
                    event.headers.remove(MY_ROUTE)
                    event.headers.remove(MY_TRACE_ID)
                    event.headers.remove(MY_TRACE_PATH)
                } catch (e: IOException) {
                    log.error("Unable to decode event - {}", e.message)
                    return
                }
                if (coroutine || async) {
                    // execute function as a coroutine
                    GlobalScope.launch(Platform.getInstance().vertx.dispatcher()) {
                        executeFunction(event)
                    }
                } else {
                    // execute function as a runnable
                    executor.submit {
                        val worker = WorkerHandler(def, route, instance, tracing, interceptor, useEnvelope)
                        worker.executeFunction(event)
                    }
                }
            }
        }

        private suspend fun executeFunction(event: EventEnvelope) {
            val rpc = event.getTag(EventEmitter.RPC)
            val po = EventEmitter.getInstance()
            val ref = if (tracing) po.startTracing(parentRoute, event.traceId, event.tracePath, instance) else "?"
            val ps = processEvent(event)
            val trace = po.stopTracing(ref)
            if (tracing && trace != null && trace.id != null && trace.path != null) {
                try {
                    val journaled = po.isJournaled(def.route)
                    if (journaled || rpc == null || !ps.isDelivered) {
                        // Send tracing information to distributed trace logger
                        val dt = EventEnvelope().setTo(EventEmitter.DISTRIBUTED_TRACING)
                        val payload: MutableMap<String, Any> = HashMap()
                        payload[ANNOTATIONS] = trace.annotations
                        // send input/output dataset to journal if configured in journal.yaml
                        if (journaled) {
                            payload[JOURNAL] = ps.inputOutput
                        }
                        val metrics: MutableMap<String, Any> = HashMap()
                        metrics[ORIGIN] = myOrigin
                        metrics[ID] = trace.id
                        metrics[PATH] = trace.path
                        metrics[SERVICE] = def.route
                        metrics[START] = trace.startTime
                        metrics[SUCCESS] = ps.isSuccess
                        metrics[FROM] = if (event.from == null) UNKNOWN else event.from
                        metrics[EXEC_TIME] = ps.executionTime
                        if (!ps.isSuccess) {
                            metrics[STATUS] = ps.status
                            metrics[EXCEPTION] = ps.exception
                        }
                        if (!ps.isDelivered) {
                            metrics[REMARK] = "Response not delivered - " + ps.deliveryError
                        }
                        payload[TRACE] = metrics
                        dt.setHeader(DELIVERED, ps.isDelivered)
                        dt.setHeader(RPC, rpc != null)
                        dt.setHeader(JOURNAL, journaled)
                        po.send(dt.setBody(payload))
                    }
                } catch (e: Exception) {
                    log.error("Unable to send to " + EventEmitter.DISTRIBUTED_TRACING, e)
                }
            } else {
                // print delivery warning if tracing is not enabled
                if (!ps.isDelivered) {
                    log.warn(
                        "Event not delivered - {}, from={}, to={}, type={}, exec_time={}",
                        ps.deliveryError,
                        if (event.from == null) "unknown" else event.from, event.to,
                        if (ps.isSuccess) "response" else "exception(" + ps.status + ", " + ps.exception + ")",
                        ps.executionTime
                    )
                }
            }
            /*
             * Send a ready signal to inform the system this worker is ready for next event.
             * This guarantees that incoming events are processed orderly by available workers.
             */
            Platform.getInstance().eventSystem.send(def.route, READY + route)
        }

        private suspend fun processEvent(event: EventEnvelope): ProcessStatus {
            val f: Any = if (def.isKotlin) def.suspendFunction else def.function
            val ps = ProcessStatus()
            val po = EventEmitter.getInstance()
            val inputOutput: MutableMap<String, Any> = HashMap()
            val input: MutableMap<String, Any> = HashMap()
            input[HEADERS] = event.headers
            if (event.rawBody != null) {
                input[BODY] = event.rawBody
            }
            inputOutput[INPUT] = input
            val begin = System.nanoTime()
            return try {
                /*
                 * Interceptor can read any input (i.e. including case for empty headers and null body).
                 * The system therefore disables ping when the target function is an interceptor.
                 */
                val ping = !interceptor && !event.isOptional && event.rawBody == null && event.headers.isEmpty()
                /*
                 * If the service is an interceptor or the input argument is EventEnvelope,
                 * we will pass the original event envelope instead of the message body.
                 */
                val inputBody: Any?
                if (useEnvelope || (interceptor && def.inputClass == null)) {
                    inputBody = event
                } else {
                    if (event.rawBody is Map<*, *> && def.inputClass != null) {
                        if (def.inputClass == AsyncHttpRequest::class.java) {
                            // handle special case
                            event.type = null
                            inputBody = AsyncHttpRequest(event.rawBody)
                        } else {
                            // automatically convert Map to PoJo
                            event.type = def.inputClass.name
                            inputBody = event.body
                        }
                    } else {
                        inputBody = event.body
                    }
                }
                // Insert READ only metadata into function input headers
                val parameters: MutableMap<String, String> = HashMap(event.headers)
                parameters[MY_ROUTE] = parentRoute
                if (event.traceId != null) {
                    parameters[MY_TRACE_ID] = event.traceId
                }
                if (event.tracePath != null) {
                    parameters[MY_TRACE_PATH] = event.tracePath
                }
                var result: Any? = null
                if (!ping) {
                    result = if (def.isKotlin) {
                        def.suspendFunction.handleEvent(parameters, inputBody, instance)
                    } else {
                        def.function.handleEvent(parameters, inputBody, instance)
                    }
                }
                val delta: Float = if (ping) 0f else (System.nanoTime() - begin).toFloat() / EventEmitter.ONE_MILLISECOND
                // adjust precision to 3 decimal points
                val diff = String.format("%.3f", 0.0f.coerceAtLeast(delta)).toFloat()
                val output: MutableMap<String, Any> = HashMap()
                val replyTo = event.replyTo
                if (replyTo != null) {
                    var serviceTimeout = false
                    val response = EventEnvelope()
                    response.to = replyTo
                    response.from = def.route
                    /*
                     * Preserve correlation ID and notes
                     *
                     * "Notes" is usually used by event interceptors. The system does not restrict the content of the notes.
                     * For example, to save some metadata from the original sender.
                     */if (event.correlationId != null) {
                        response.correlationId = event.correlationId
                    }
                    if (event.extra != null) {
                        response.extra = event.extra
                    }
                    // propagate the trace to the next service if any
                    if (event.traceId != null) {
                        response.setTrace(event.traceId, event.tracePath)
                    }
                    if (result is EventEnvelope) {
                        val headers = result.headers
                        if (headers.isEmpty() && result.status == 408 && result.rawBody == null) {
                            /*
                             * An empty event envelope with timeout status
                             * is used by the ObjectStreamService to simulate a READ timeout.
                             */
                            serviceTimeout = true
                        } else {
                            /*
                             * When EventEnvelope is used as a return type, the system will transport
                             * 1. payload
                             * 2. key-values (as headers)
                             * 3. optional parametric types for Java class that uses generic types
                             */
                            response.body = result.rawBody
                            response.type = result.type;
                            if (result.parametricType != null) {
                                response.parametricType = result.parametricType
                            }
                            for ((key, value) in headers) {
                                if (key != MY_ROUTE && key != MY_TRACE_ID && key != MY_TRACE_PATH) {
                                    response.setHeader(key, value)
                                }
                            }
                            response.status = result.status
                        }
                        if (response.headers.isNotEmpty()) {
                            output[HEADERS] = response.headers
                        }
                    } else {
                        response.body = result
                    }
                    output[BODY] = if (response.rawBody == null) "null" else response.rawBody
                    output[STATUS] = response.status
                    inputOutput[OUTPUT] = output
                    try {
                        if (ping) {
                            val parent =
                                if (route.contains(HASH)) route.substring(0, route.lastIndexOf(HASH)) else route
                            val platform = Platform.getInstance()
                            // execution time is not set because there is no need to execute the lambda function
                            val pong: MutableMap<String, Any> = HashMap()
                            pong[TYPE] = PONG
                            pong[TIME] = Date()
                            pong[APP] = platform.name
                            pong[ORIGIN] = platform.origin
                            pong[SERVICE] = parent
                            pong[REASON] = "This response is generated when you send an event without headers and body"
                            pong[MESSAGE] = "you have reached $parent"
                            response.body = pong
                            po.send(response)
                        } else {
                            if (!interceptor && !serviceTimeout) {
                                response.executionTime = diff
                                encodeTraceAnnotations(response)
                                po.send(response)
                            }
                        }
                    } catch (e2: Exception) {
                        ps.setUnDelivery(e2.message)
                    }
                } else {
                    val response = EventEnvelope().setBody(result)
                    output[BODY] = if (response.rawBody == null) "null" else response.rawBody
                    output[STATUS] = response.status
                    output[ASYNC] = true
                    inputOutput[OUTPUT] = output
                }
                ps.inputOutput = inputOutput
                ps.executionTime = diff
                return ps
            } catch (e: Exception) {
                val delta = (System.nanoTime() - begin).toFloat() / EventEmitter.ONE_MILLISECOND
                val diff = String.format("%.3f", 0.0f.coerceAtLeast(delta)).toFloat()
                ps.executionTime = diff
                val replyTo = event.replyTo
                val status: Int = when (e) {
                    is AppException -> {
                        e.status
                    }
                    is TimeoutException -> {
                        408
                    }
                    is IllegalArgumentException -> {
                        400
                    } else -> {
                        500
                    }
                }
                val ex = Utility.getInstance().getRootCause(e)
                if (f is PoJoMappingExceptionHandler) {
                    val error = simplifyCastError(ex.message)
                    try {
                        f.onError(parentRoute, AppException(status, error), event, instance)
                    } catch (e3: Exception) {
                        ps.setUnDelivery(e3.message)
                    }
                    val output: MutableMap<String, Any?> = HashMap()
                    output[STATUS] = status
                    output[EXCEPTION] = error
                    inputOutput[OUTPUT] = output
                    return ps.setException(status, error).setInputOutput(inputOutput)
                }
                val output: MutableMap<String, Any?> = HashMap()
                if (replyTo != null) {
                    val response = EventEnvelope()
                    response.setTo(replyTo).setStatus(status).body = ex.message
                    response.exception = e
                    response.executionTime = diff
                    response.from = def.route
                    if (event.correlationId != null) {
                        response.correlationId = event.correlationId
                    }
                    if (event.extra != null) {
                        response.extra = event.extra
                    }
                    // propagate the trace to the next service if any
                    if (event.traceId != null) {
                        response.setTrace(event.traceId, event.tracePath)
                    }
                    encodeTraceAnnotations(response)
                    try {
                        po.send(response)
                    } catch (e4: Exception) {
                        ps.setUnDelivery(e4.message)
                    }
                } else {
                    output[ASYNC] = true
                    if (status >= 500) {
                        log.error("Unhandled exception for $route", ex)
                    } else {
                        log.warn("Unhandled exception for {} - {}", route, ex.message)
                    }
                }
                output[STATUS] = status
                output[EXCEPTION] = ex.message
                inputOutput[OUTPUT] = output
                ps.setException(status, ex.message).setInputOutput(inputOutput)
            }
        }

        private fun simplifyCastError(error: String?): String {
            return if (error == null) {
                "null"
            } else {
                val sep = error.lastIndexOf(" (")
                if (sep > 0) error.substring(0, sep) else error
            }
        }

        private fun encodeTraceAnnotations(response: EventEnvelope) {
            val po = EventEmitter.getInstance()
            val headers = response.headers
            val trace = po.getTrace(parentRoute, instance)
            if (trace != null) {
                val annotations = trace.annotations
                if (annotations.isNotEmpty()) {
                    var n = 0
                    for ((key, value) in annotations) {
                        n++
                        headers["_$n"] = "$key=$value"
                    }
                    headers["_"] = n.toString()
                }
            }
        }
    }

    companion object {
        private val log = LoggerFactory.getLogger(WorkerQueue::class.java)
        private const val TYPE = "type"
        private const val ID = "id"
        private const val PATH = "path"
        private const val START = "start"
        private const val UNKNOWN = "unknown"
        private const val TRACE = "trace"
        private const val SUCCESS = "success"
        private const val FROM = "from"
        private const val EXEC_TIME = "exec_time"
        private const val TIME = "time"
        private const val APP = "app"
        private const val PONG = "pong"
        private const val REASON = "reason"
        private const val MESSAGE = "message"
        private const val ORIGIN = "origin"
        private const val SERVICE = "service"
        private const val INPUT = "input"
        private const val OUTPUT = "output"
        private const val HEADERS = "headers"
        private const val BODY = "body"
        private const val STATUS = "status"
        private const val EXCEPTION = "exception"
        private const val ASYNC = "async"
        private const val ANNOTATIONS = "annotations"
        private const val REMARK = "remark"
        private const val JOURNAL = "journal"
        private const val RPC = "rpc"
        private const val DELIVERED = "delivered"
        private const val MY_ROUTE = "my_route"
        private const val MY_TRACE_ID = "my_trace_id"
        private const val MY_TRACE_PATH = "my_trace_path"
    }
}