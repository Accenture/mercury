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

package org.platformlambda.core.system

import io.vertx.core.Vertx
import io.vertx.core.eventbus.EventBus
import io.vertx.core.eventbus.Message
import io.vertx.kotlin.coroutines.receiveChannelHandler
import org.platformlambda.core.models.*
import org.platformlambda.core.util.Utility
import org.platformlambda.core.websocket.common.MultipartPayload
import org.slf4j.LoggerFactory
import java.io.IOException
import java.net.URI
import java.util.*

class FastRPC(headers: Map<String, String>) {
    private val myRoute: String?
    private val myTraceId: String?
    private val myTracePath: String?

    init {
        myRoute = headers[MY_ROUTE]
        myTraceId = headers[MY_TRACE_ID]
        myTracePath = headers[MY_TRACE_PATH]
    }

    /**
     * Make a non-blocking RPC call to a service
     * <p>
     * When service timeout, the response will be an envelope with status "408" and a timeout error in body
     *
     * @param request envelope - you must set "TO" and put in key-value(s) in headers and/or body (payload)
     * @param timeout in milliseconds
     * @return response envelope
     * @throws IOException when target service is not available
     */
    @Throws(IOException::class)
    suspend fun awaitRequest(request: EventEnvelope, timeout: Long): EventEnvelope {
        val platform = Platform.getInstance()
        val po = EventEmitter.getInstance()
        val util = Utility.getInstance()
        val signature = util.uuid
        val start = util.date2str(Date())
        val begin = System.nanoTime()
        val dest = request.to ?: throw IllegalArgumentException(EventEmitter.MISSING_ROUTING_PATH)
        val to = po.substituteRouteIfAny(dest)
        request.to = to
        propagateTrace(request)
        val from = request.from
        val traceId = request.traceId
        val tracePath = request.tracePath
        if (request.correlationId == null) {
            request.correlationId = "1"
        }
        val target = po.discover(to, request.isEndOfRoute)
        val inbox = NonBlockingInbox()
        val returnPath = inbox.id
        request.replyTo = returnPath + "@" + platform.origin
        request.addTag(EventEmitter.RPC, timeout)
        // broadcast is not possible with RPC call
        request.broadcastLevel = 0
        val vertx: Vertx = platform.vertx
        val eventBus: EventBus = platform.eventSystem
        val adapter = vertx.receiveChannelHandler<Message<ByteArray>>()
        val consumer = eventBus.localConsumer<ByteArray>(returnPath).handler(adapter)
        val timer = platform.vertx.setTimer(timeout) {
            val timeoutEvent = EventEnvelope().setTo(returnPath).setStatus(408).setHeader(SYSTEM, signature)
                .setBody("Timeout for $timeout ms")
            platform.eventSystem.send(returnPath, timeoutEvent.toBytes())
        }
        if (target.isCloud) {
            MultipartPayload.getInstance().outgoing(target.manager, request)
        } else {
            platform.eventSystem.send(target.manager.route, request.toBytes())
        }
        val message = adapter.receive()
        if (consumer.isRegistered) {
            consumer.unregister()
        }
        vertx.cancelTimer(timer)
        inbox.close()
        val result = EventEnvelope(message.body())
        result.roundTrip = (System.nanoTime() - begin).toFloat() / EventEmitter.ONE_MILLISECOND
        // remove some metadata that are not relevant for a RPC response
        result.removeTag(RPC).setTo(null).setReplyTo(null).setTrace(null, null)
        if (platform.isTrackable(to) && traceId != null && tracePath != null && signature != result.headers[SYSTEM]) {
            sendTrace(result, start, from, to, traceId, tracePath)
        }
        return if (signature == result.headers[SYSTEM]) {
            EventEnvelope().setTo(returnPath).setStatus(408).setBody("Timeout for $timeout ms")
        } else {
            result
        }
    }

    /**
     * This method allows your app to send a non-blocking RPC request to another application instance
     *
     * @param request to be sent to a peer application instance
     * @param timeout to abort the request
     * @param headers optional security headers such as "Authorization"
     * @param eventEndpoint fully qualified URL such as http://domain:port/api/event
     * @param rpc if true, the target service will return a response.
     *            Otherwise, a response with status=202 will be returned to indicate that the event will be delivered.
     * @return response event
     * @throws IOException in case of routing error
     */
    @Throws(IOException::class)
    suspend fun awaitRequest(request: EventEnvelope, timeout: Long,
                             headers: Map<String, String>,
                             eventEndpoint: String, rpc: Boolean): EventEnvelope {
        requireNotNull(request.to) { EventEmitter.MISSING_ROUTING_PATH }
        val url = URI(eventEndpoint)
        val req = AsyncHttpRequest()
        req.setMethod(POST)
        req.setHeader(CONTENT_TYPE, "application/octet-stream")
        req.setHeader(ACCEPT, "*/*")
        req.setHeader(X_TIMEOUT, 100L.coerceAtLeast(timeout).toString())
        if (!rpc) {
            req.setHeader(X_ASYNC, "true")
        }
        // propagate trace-ID if any
        if (request.traceId != null) {
            req.setHeader(X_TRACE_ID, request.traceId)
        }
        // optional HTTP request headers
        for ((key, value) in headers) {
            req.setHeader(key, value)
        }
        req.setUrl(url.path)
        req.setTargetHost(getTargetFromUrl(url))
        propagateTrace(request)
        val b: ByteArray = request.toBytes()
        req.setBody(b)
        req.setContentLength(b.size)
        val remoteRequest = EventEnvelope().setTo(HTTP_REQUEST).setBody(req)
        // add 100 ms to make sure it does not time out earlier than the target service
        val response = awaitRequest(remoteRequest, 100L.coerceAtLeast(timeout) + 100L)
        return if (response.body is ByteArray) {
            try {
                EventEnvelope(response.body as ByteArray)
            } catch (e: IOException) {
                EventEnvelope().setStatus(400).setBody("Did you configure rest.yaml correctly? " +
                        "Invalid result set - " + e.message)
            }
        } else {
            if (response.status >= 400 && response.body is Map<*, *>) {
                val data = response.body as Map<*, *>
                if (ERROR == data[TYPE] && data.containsKey(MESSAGE) && data[MESSAGE] is String) {
                    EventEnvelope().setStatus(response.status).setBody(data[MESSAGE])
                } else {
                    response
                }
            } else {
                response
            }
        }
    }

    /**
     * Make a non-blocking RPC call to a service
     * <p>
     * When service timeout, the response will contain one or more envelopes with status "408" and a timeout error in body
     *
     * @param requests as a list of envelopes - you must set "TO" and put in key-value(s) in headers and/or body (payload)
     * @param timeout in milliseconds
     * @return response as a list of results (partial or complete)
     * @throws IOException when target service is not available
     */
    @Throws(IOException::class)
    suspend fun awaitRequest(requests: List<EventEnvelope>, timeout: Long): List<EventEnvelope> {
        require(requests.isNotEmpty()) { EventEmitter.MISSING_EVENT }
        val platform = Platform.getInstance()
        val po = EventEmitter.getInstance()
        val util = Utility.getInstance()
        val signature = util.uuid
        val start = util.date2str(Date())
        val begin = System.nanoTime()
        val first = requests[0]
        propagateTrace(first)
        val from = first.from
        val traceId = first.traceId
        val tracePath = first.tracePath
        val correlations: MutableMap<String, String> = HashMap()
        val destinations: MutableList<TargetRoute> = ArrayList()
        var seq = 1
        val inbox = NonBlockingInbox()
        val returnPath = inbox.id
        for (event in requests) {
            val dest = event.to ?: throw IllegalArgumentException(EventEmitter.MISSING_ROUTING_PATH)
            val to: String = po.substituteRouteIfAny(dest)
            event.to = to
            // propagate the same trace info to each request
            event.from = from
            event.traceId = traceId
            event.tracePath = tracePath
            // insert sequence number when correlation ID if not present
            // so that the caller can correlate the service responses
            if (event.correlationId == null) {
                event.correlationId = seq++.toString()
            }
            correlations[event.correlationId] = to
            destinations.add(po.discover(to, event.isEndOfRoute))
        }
        val results = ArrayList<EventEnvelope>()
        val vertx: Vertx = platform.vertx
        val eventBus: EventBus = platform.eventSystem
        val adapter = vertx.receiveChannelHandler<Message<ByteArray>>()
        val consumer = eventBus.localConsumer<ByteArray>(returnPath).handler(adapter)
        val timer = platform.vertx.setTimer(timeout) {
            val timeoutEvent = EventEnvelope().setTo(returnPath).setStatus(408).setHeader(SYSTEM, signature)
                .setBody("Timeout for $timeout ms")
            val b = timeoutEvent.toBytes()
            val outstanding = requests.size - results.size
            if (outstanding > 0) {
                for (i in 1..outstanding) {
                    platform.eventSystem.send(returnPath, b)
                }
            }
        }
        for ((n, event) in requests.withIndex()) {
            val target = destinations[n]
            event.replyTo = returnPath + "@" + platform.origin
            event.addTag(EventEmitter.RPC, timeout)
            // broadcast is not possible with RPC call
            event.broadcastLevel = 0
            if (target.isCloud) {
                MultipartPayload.getInstance().outgoing(target.manager, event)
            } else {
                platform.eventSystem.send(target.manager.route, event.toBytes())
            }
        }
        for (i in requests.indices) {
            val message = adapter.receive()
            val result = EventEnvelope(message.body())
            result.roundTrip = (System.nanoTime() - begin).toFloat() / EventEmitter.ONE_MILLISECOND
            // remove some metadata that are not relevant for a RPC response
            result.removeTag(RPC).setTo(null).setReplyTo(null).setTrace(null, null)
            if (traceId != null && tracePath != null && signature != result.headers[SYSTEM]) {
                val to = correlations[result.correlationId]
                if (to != null && platform.isTrackable(to)) {
                    sendTrace(result, start, from, to, traceId, tracePath)
                }
            }
            if (signature != result.headers[SYSTEM]) {
                results.add(result)
            }
        }
        if (consumer.isRegistered) {
            consumer.unregister()
        }
        vertx.cancelTimer(timer)
        inbox.close()
        return results
    }

    private fun propagateTrace(event: EventEnvelope) {
        if (event.from == null) {
            event.from = myRoute
        }
        if (event.traceId == null) {
            event.traceId = myTraceId
        }
        if (event.tracePath == null) {
            event.tracePath = myTracePath
        }
    }

    private fun sendTrace(result: EventEnvelope,
                          start: String, from: String, to: String, traceId: String, tracePath: String) {
        val annotations: MutableMap<String, Any> = HashMap()
        // decode trace annotations from reply event
        val headers: MutableMap<String, String> = result.headers
        if (headers.containsKey(UNDERSCORE)) {
            val count = Utility.getInstance().str2int(headers[UNDERSCORE])
            for (i in 1..count) {
                val kv = headers[UNDERSCORE + i]
                if (kv != null) {
                    val eq = kv.indexOf('=')
                    if (eq > 0) {
                        annotations[kv.substring(0, eq)] = kv.substring(eq + 1)
                    }
                }
            }
            headers.remove(UNDERSCORE)
            for (i in 1..count) {
                headers.remove(UNDERSCORE + i)
            }
        }
        try {
            val payload: MutableMap<String, Any> = HashMap()
            val metrics: MutableMap<String, Any> = HashMap()
            metrics["origin"] = Platform.getInstance().origin
            metrics["id"] = traceId
            metrics["service"] = to
            metrics["from"] = from
            metrics["exec_time"] = result.executionTime
            metrics["round_trip"] = result.roundTrip
            metrics["success"] = true
            metrics["status"] = result.status
            metrics["start"] = start
            metrics["path"] = tracePath
            payload["trace"] = metrics
            if (annotations.isNotEmpty()) {
                payload[ANNOTATIONS] = annotations
            }
            val dt = EventEnvelope().setTo(EventEmitter.DISTRIBUTED_TRACING)
            EventEmitter.getInstance().send(dt.setBody(payload))
        } catch (e: Exception) {
            log.error(FAIL_TO_SEND + EventEmitter.DISTRIBUTED_TRACING, e)
        }
    }

    private fun getTargetFromUrl(url: URI): String {
        val secure: Boolean
        val protocol = url.scheme
        secure = if (HTTP == protocol) {
            false
        } else if (HTTPS == protocol) {
            true
        } else {
            throw IllegalArgumentException(HTTP_OR_HTTPS)
        }
        val host = url.host.trim { it <= ' ' }
        require(host.isNotEmpty()) { "Unable to resolve target host as domain or IP address" }
        var port = url.port
        if (port < 0) {
            port = if (secure) 443 else 80
        }
        return "$protocol://$host:$port"
    }
    companion object {
        private val log = LoggerFactory.getLogger(FastRPC::class.java)
        private const val SYSTEM = "_sys_"
        private const val RPC = "rpc"
        private const val UNDERSCORE = "_"
        private const val ANNOTATIONS = "annotations"
        private const val FAIL_TO_SEND = "Unable to send to "
        private const val MY_ROUTE = "my_route"
        private const val MY_TRACE_ID = "my_trace_id"
        private const val MY_TRACE_PATH = "my_trace_path"
        private const val POST = "POST"
        private const val CONTENT_TYPE = "content-type"
        private const val ACCEPT = "accept"
        private const val X_TIMEOUT = "x-timeout"
        private const val X_ASYNC = "x-async"
        private const val X_TRACE_ID = "x-trace-id"
        private const val HTTP = "http"
        private const val HTTPS = "https"
        private const val HTTP_OR_HTTPS = "Protocol must be http or https"
        private const val HTTP_REQUEST = "async.http.request"
        private const val TYPE = "type"
        private const val ERROR = "error"
        private const val MESSAGE = "message"
    }
}