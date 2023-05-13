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

package org.platformlambda.core.actuator

import org.platformlambda.core.models.EventEnvelope
import org.platformlambda.core.models.KotlinLambdaFunction
import org.platformlambda.core.system.*
import org.platformlambda.core.util.AppConfigReader
import org.platformlambda.core.util.Utility
import java.text.NumberFormat
import java.util.*
import kotlin.collections.HashMap

class InfoService : KotlinLambdaFunction<EventEnvelope, Any> {
    private val appDesc: String
    private val isServiceMonitor: Boolean
    private val hasCloudConnector: Boolean

    init {
        val config = AppConfigReader.getInstance()
        appDesc = config.getProperty(APP_DESCRIPTION, Platform.getInstance().name)
        isServiceMonitor = "true" == config.getProperty("service.monitor", "false")
        hasCloudConnector = !"none".equals(config.getProperty(EventEmitter.CLOUD_CONNECTOR, "none"),
            ignoreCase = true)
    }

    override suspend fun handleEvent(headers: Map<String, String>, input: EventEnvelope, instance: Int): Any {
        val fastRPC = FastRPC(headers)
        var accept = headers[ACCEPT] ?: "?"
        accept = if (accept.startsWith(APPLICATION_XML)) {
            APPLICATION_XML
        } else {
            APPLICATION_JSON
        }
        val type = headers[TYPE] ?: INFO
        val platform = Platform.getInstance()
        val result: MutableMap<String, Any> = HashMap()
        val app: MutableMap<String, Any> = HashMap()
        val util = Utility.getInstance()
        val info = util.versionInfo
        result[APP] = app
        /*
         * When running inside IDE, there are no information about libraries.
         * It is therefore better to take the application name from the application.properties.
         */
        app[NAME] = info.artifactId
        app[VERSION] = info.version
        app[DESCRIPTION] = appDesc
        val appId = platform.appId
        if (appId != null) {
            app[INSTANCE] = appId
        }
        if (ROUTES == type) {
            if (isServiceMonitor) {
                result[ROUTING] = HashMap<String, String>()
                result[MESSAGE] = "Routing table is not visible from a presence monitor"
            } else {
                val po = EventEmitter.getInstance()
                val journaledRoutes = po.journaledRoutes
                if (journaledRoutes.size > 1) {
                    journaledRoutes.sort()
                }
                val more = getRoutingTable(fastRPC)
                if (more != null) {
                    result[ROUTING] = more
                }
                result[JOURNAL] = journaledRoutes
                // add route substitution list if any
                val substitutions = po.routeSubstitutionList
                if (substitutions.isNotEmpty()) {
                    result[ROUTE_SUBSTITUTION] = substitutions
                }
            }
        } else if (LIB == type) {
            result[LIBRARY] = util.libraryList
        } else if (ENV == type) {
            result[ENV] = env
            result[ROUTING] = registeredServices
        } else {
            // java VM information
            val jvm: MutableMap<String, Any> = HashMap()
            result[JVM] = jvm
            jvm["java_version"] = System.getProperty(JAVA_VERSION)
            jvm["java_vm_version"] = System.getProperty(JAVA_VM_VERSION)
            jvm["java_runtime_version"] = System.getProperty(JAVA_RUNTIME_VERSION)
            // memory usage
            val runtime = Runtime.getRuntime()
            val number = NumberFormat.getInstance()
            val maxMemory = runtime.maxMemory()
            val allocatedMemory = runtime.totalMemory()
            val freeMemory = runtime.freeMemory()
            val memory: MutableMap<String, Any> = HashMap()
            result[MEMORY] = memory
            memory[MAX] = number.format(maxMemory)
            memory[FREE] = number.format(freeMemory)
            memory[ALLOCATED] = number.format(allocatedMemory)
            memory[USED] = number.format(allocatedMemory - freeMemory)
            /*
             * check streams resources if any
             */result[STREAMS] = ObjectStreamIO.getStreamInfo()
            val more = getAdditionalInfo(fastRPC)
            if (more != null) {
                result["additional_info"] = more
            }
            result[ORIGIN] = platform.origin
            result[PERSONALITY] = ServerPersonality.getInstance().type.name
            val time: MutableMap<String, Any> = HashMap()
            val now = Date()
            time[START] = Date(platform.startTime)
            time[CURRENT] = now
            result[TIME] = time
            result[UP_TIME] = util.elapsedTime(now.time - platform.startTime)
        }
        return EventEnvelope().setHeader(CONTENT_TYPE, accept).setBody(result)
    }

    private suspend fun getAdditionalInfo(fastRPC: FastRPC): Any? {
        return if (Platform.getInstance().hasRoute(ADDITIONAL_INFO)) {
            try {
                val req = EventEnvelope().setTo(ADDITIONAL_INFO).setHeader(TYPE, QUERY)
                fastRPC.awaitRequest(req, 5000).body
            } catch (e: Exception) {
                ERROR_FETCHING_INFO + e.message
            }
        } else {
            null
        }
    }

    private suspend fun getRoutingTable(fastRPC: FastRPC): Any? {
        val platform = Platform.getInstance()
        return if (hasCloudConnector &&
            (platform.hasRoute(ServiceDiscovery.SERVICE_QUERY) || platform.hasRoute(CLOUD_CONNECTOR))) {
            try {
                val req = EventEnvelope().setTo(ServiceDiscovery.SERVICE_QUERY)
                    .setHeader(TYPE, DOWNLOAD).setHeader(ORIGIN, platform.origin)
                fastRPC.awaitRequest(req, 5000).body
            } catch (e: Exception) {
                localPublicRouting
            }
        } else {
            localPublicRouting
        }
    }

    private val localPublicRouting: Map<String, Any>
        get() {
            val result: MutableMap<String, Any> = HashMap()
            val map: Map<String, ServiceDef> = Platform.getInstance().localRoutingTable
            for (route in map.keys) {
                val service = map[route]
                if (!service!!.isPrivate) {
                    result[route] = service.created
                }
            }
            return result
        }
    private val registeredServices: Map<String, List<String>>
        get() {
            val result: MutableMap<String, List<String>> = HashMap()
            result["public"] = getLocalRoutingDetails(false)
            result["private"] = getLocalRoutingDetails(true)
            return result
        }

    private fun getLocalRoutingDetails(isPrivate: Boolean): List<String> {
        val result: MutableList<String> = ArrayList()
        val map: Map<String, ServiceDef> = Platform.getInstance().localRoutingTable
        for (route in map.keys) {
            val service = map[route]
            if (service!!.isPrivate == isPrivate) {
                val queue = service.manager
                val read = queue.readCounter
                val write = queue.writeCounter
                result.add(
                    route + " (" + queue.freeWorkers + "/" + service.concurrency + ") " +
                            " r/w=" + read + "/" + write
                )
            }
        }
        if (result.size > 1) {
            result.sort()
        }
        return result
    }

    private val env: Map<String, Any>
        get() {
            val result: MutableMap<String, Any> = HashMap()
            val util = Utility.getInstance()
            val reader = AppConfigReader.getInstance()
            val envVars = util.split(reader.getProperty(SHOW_ENV, ""), ", ")
            val properties = util.split(reader.getProperty(SHOW_PROPERTIES, ""), ", ")
            val missingVars: MutableList<String> = ArrayList()
            val eMap: MutableMap<String, Any> = HashMap()
            if (envVars.isNotEmpty()) {
                for (key in envVars) {
                    val v = System.getenv(key)
                    if (v == null) {
                        missingVars.add(key)
                    } else {
                        eMap[key] = v
                    }
                }
            }
            result[SYSTEM_ENV] = eMap
            val missingProp: MutableList<String> = ArrayList()
            val pMap: MutableMap<String, Any> = HashMap()
            if (properties.isNotEmpty()) {
                for (key in properties) {
                    val v = reader.getProperty(key)
                    if (v == null) {
                        missingProp.add(key)
                    } else {
                        pMap[key] = v
                    }
                }
            }
            result[APP_PROPS] = pMap
            // any missing keys?
            val missingKeys: MutableMap<String, Any> = HashMap()
            if (missingVars.isNotEmpty()) {
                missingKeys[SYSTEM_ENV] = missingVars
            }
            if (missingProp.isNotEmpty()) {
                missingKeys[APP_PROPS] = missingProp
            }
            if (missingKeys.isNotEmpty()) {
                result[MISSING] = missingKeys
            }
            return result
        }

    companion object {
        private const val ADDITIONAL_INFO = "additional.info"
        private const val ERROR_FETCHING_INFO = "Unable to check additional.info - "
        private const val STREAMS = "streams"
        private const val JAVA_VERSION = "java.version"
        private const val JAVA_VM_VERSION = "java.vm.version"
        private const val JAVA_RUNTIME_VERSION = "java.runtime.version"
        private const val TYPE = "type"
        private const val ACCEPT = "accept"
        private const val INFO = "info"
        private const val QUERY = "query"
        private const val APP_DESCRIPTION = "info.app.description"
        private const val APP = "app"
        private const val NAME = "name"
        private const val VERSION = "version"
        private const val DESCRIPTION = "description"
        private const val JVM = "vm"
        private const val MEMORY = "memory"
        private const val MAX = "max"
        private const val ALLOCATED = "allocated"
        private const val USED = "used"
        private const val FREE = "free"
        private const val ORIGIN = "origin"
        private const val INSTANCE = "instance"
        private const val PERSONALITY = "personality"
        private const val ROUTING = "routing"
        private const val MESSAGE = "message"
        private const val ROUTES = "routes"
        private const val LIB = "lib"
        private const val ENV = "env"
        private const val DOWNLOAD = "download"
        private const val CLOUD_CONNECTOR = "cloud.connector"
        private const val LIBRARY = "library"
        private const val ROUTE_SUBSTITUTION = "route_substitution"
        private const val TIME = "time"
        private const val START = "start"
        private const val CURRENT = "current"
        private const val UP_TIME = "uptime"
        private const val SHOW_ENV = "show.env.variables"
        private const val SHOW_PROPERTIES = "show.application.properties"
        private const val SYSTEM_ENV = "environment"
        private const val APP_PROPS = "properties"
        private const val MISSING = "missing"
        private const val JOURNAL = "journal"
        private const val CONTENT_TYPE = "content-type"
        private const val APPLICATION_JSON = "application/json"
        private const val APPLICATION_XML = "application/xml"
    }

}