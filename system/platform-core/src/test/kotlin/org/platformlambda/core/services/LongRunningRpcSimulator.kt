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

package org.platformlambda.core.services

import org.platformlambda.core.models.EventEnvelope
import org.platformlambda.core.models.KotlinLambdaFunction
import org.platformlambda.core.system.FastRPC
import org.platformlambda.core.system.PostOffice
import org.platformlambda.core.util.Utility
import java.util.*


/**
 * Demonstrate non-blocking RPC for single request and fork-n-join
 *
 * Since this is a "suspend" function, the awaitRequest will suspend this coroutine until a response arrives.
 */
class LongRunningRpcSimulator : KotlinLambdaFunction<EventEnvelope, Any> {

    override suspend fun handleEvent(headers: Map<String, String>, input: EventEnvelope, instance: Int): Any {
        val po = PostOffice(headers, instance)
        val fastRPC = FastRPC(headers)
        val util = Utility.getInstance()
        po.annotateTrace("demo", "long-running")
        po.annotateTrace("time", util.date2str(Date()))
        val timeout = Utility.getInstance().str2long(headers.getOrDefault(TIMEOUT, "5000"))
        if (headers.containsKey(FORK_N_JOIN)) {
            val forward = EventEnvelope().setTo(HELLO_WORLD).setHeaders(headers).setHeader(BODY, input.body)
            val requests  = ArrayList<EventEnvelope>()
            // create a list of 4 request events
            for (i in 0..3) {
                requests.add(EventEnvelope(forward.toBytes()).setBody(i).setCorrelationId("cid-$i"))
            }
            val consolidated = HashMap<String, Any>()
            val response = fastRPC.awaitRequest(requests, timeout)
            for (res in response) {
                if (res.status == 200) {
                    consolidated[res.correlationId] = res.body
                }
            }
            return EventEnvelope().setBody(consolidated)
        } else {
            // simulate delay of one second by setting body to even number
            val forward = EventEnvelope().setTo(HELLO_WORLD).setBody(2)
                .setHeaders(headers).setHeader(BODY, input.body)
            return fastRPC.awaitRequest(forward, timeout)
        }
    }

    companion object {
        private const val HELLO_WORLD = "hello.world"
        private const val FORK_N_JOIN = "fork-n-join"
        private const val TIMEOUT = "timeout"
        private const val BODY = "body"
    }

}