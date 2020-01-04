/*

    Copyright 2018-2020 Accenture Technology

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

import org.platformlambda.MainApp;
import org.platformlambda.core.annotations.OptionalService;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.ws.rs.*;
import javax.ws.rs.core.MediaType;
import java.io.IOException;
import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.TimeoutException;

@Path("/config")
@OptionalService("app.config.manager")
public class ConfigManagerEndpoint {
    private static final Logger log = LoggerFactory.getLogger(ConfigManagerEndpoint.class);

    private static final String TYPE = "type";
    private static final String ENTITY = "entity";
    private static final String GET = "get";
    private static final String CREATE_APP = "create_app";
    private static final String DELETE_APP = "delete_app";
    private static final String UPDATE = "update";
    private static final String DELETE = "delete";
    private static final String APPLICATION = "application";
    private static final String X_API_KEY = "x-api-key";
    private static final String KEYS = "keys";

    private static String apiKey;

    public ConfigManagerEndpoint() {
        getApiKey();
    }

    public static String getApiKey() {
        if (apiKey == null) {
            AppConfigReader reader = AppConfigReader.getInstance();
            apiKey = reader.getProperty("app.config.key", Utility.getInstance().getUuid());
            log.info("Loaded app.config.key");
        }
        return apiKey;
    }

    @GET
    @Path("/apps")
    @Produces({MediaType.APPLICATION_JSON, MediaType.APPLICATION_XML, MediaType.TEXT_HTML})
    public Map<String, Object> getApps(@HeaderParam(X_API_KEY) String key)
            throws IOException, TimeoutException, AppException {
        if (key == null) {
            throw new IllegalArgumentException("Missing "+X_API_KEY);
        }
        if (!apiKey.equals(key)) {
            throw new AppException(401, "Invalid "+X_API_KEY);
        }
        PostOffice po = PostOffice.getInstance();
        EventEnvelope res = po.request(MainApp.CONFIG_MANAGER, 5000,
                new Kv(TYPE, GET));
        if (res.getBody() instanceof Map) {
            Map<String, Object> result = new HashMap<>();
            result.put("type", "apps");
            result.put("config", res.getBody());
            return result;

        } else {
            throw new AppException(500, "Unexpected server error - result set is not a map");
        }
    }

    @GET
    @Path("/{type}/{application}")
    @Produces({MediaType.APPLICATION_JSON, MediaType.APPLICATION_XML, MediaType.TEXT_HTML})
    public Map<String, Object> getConfig(@PathParam(APPLICATION) String app,
                                         @PathParam(TYPE) String entity,
                                         @HeaderParam(X_API_KEY) String key)
            throws IOException, TimeoutException, AppException {

        if (key == null) {
            throw new IllegalArgumentException("Missing "+X_API_KEY);
        }
        if (!apiKey.equals(key)) {
            throw new AppException(401, "Invalid "+X_API_KEY);
        }
        PostOffice po = PostOffice.getInstance();
        EventEnvelope res = po.request(MainApp.CONFIG_MANAGER, 5000,
                new Kv(TYPE, GET),
                new Kv(APPLICATION, app), new Kv(ENTITY, entity));
        if (res.getBody() instanceof Map) {
            Map<String, Object> result = new HashMap<>();
            result.put("application", app);
            result.put("type", entity);
            result.put("config", res.getBody());
            return result;

        } else {
            throw new AppException(500, "Unexpected server error - result set is not a map");
        }
    }

    @PUT
    @Path("/{type}/{application}")
    @Produces({MediaType.APPLICATION_JSON, MediaType.APPLICATION_XML, MediaType.TEXT_HTML})
    public Object saveConfig(@PathParam(APPLICATION) String app,
                             @PathParam(TYPE) String entity,
                             @HeaderParam(X_API_KEY) String key, Map<String, Object> kv)
            throws IOException, TimeoutException, AppException {

        if (key == null) {
            throw new IllegalArgumentException("Missing "+X_API_KEY);
        }
        if (!apiKey.equals(key)) {
            throw new AppException(401, "Invalid "+X_API_KEY);
        }
        if (kv == null || kv.isEmpty()) {
            throw new IllegalArgumentException("Input is empty");
        }
        PostOffice po = PostOffice.getInstance();
        EventEnvelope res = po.request(MainApp.CONFIG_MANAGER, 5000, kv,
                new Kv(TYPE, UPDATE),
                new Kv(APPLICATION, app), new Kv(ENTITY, entity));
        return res.getBody();
    }

    @DELETE
    @Path("/{type}/{application}")
    @Produces({MediaType.APPLICATION_JSON, MediaType.APPLICATION_XML, MediaType.TEXT_HTML})
    public Object delConfig(@PathParam(APPLICATION) String app,
                            @PathParam(TYPE) String entity,
                            @HeaderParam(X_API_KEY) String key, Map<String, Object> kv)
            throws IOException, TimeoutException, AppException {

        if (key == null) {
            throw new IllegalArgumentException("Missing "+X_API_KEY);
        }
        if (!apiKey.equals(key)) {
            throw new AppException(401, "Invalid "+X_API_KEY);
        }
        if (kv == null || kv.isEmpty()) {
            throw new IllegalArgumentException("Input is empty");
        }
        if (!kv.containsKey(KEYS)) {
            throw new IllegalArgumentException("Missing "+KEYS);
        }
        PostOffice po = PostOffice.getInstance();
        EventEnvelope res = po.request(MainApp.CONFIG_MANAGER, 5000, kv,
                new Kv(TYPE, DELETE),
                new Kv(APPLICATION, app), new Kv(ENTITY, entity));
        return res.getBody();
    }

    @POST
    @Path("/{application}")
    @Produces({MediaType.APPLICATION_JSON, MediaType.APPLICATION_XML, MediaType.TEXT_HTML})
    public Object createApp(@PathParam(APPLICATION) String app,
                            @HeaderParam(X_API_KEY) String key)
            throws IOException, TimeoutException, AppException {

        if (key == null) {
            throw new IllegalArgumentException("Missing "+X_API_KEY);
        }
        if (!apiKey.equals(key)) {
            throw new AppException(401, "Invalid "+X_API_KEY);
        }
        PostOffice po = PostOffice.getInstance();
        EventEnvelope res = po.request(MainApp.CONFIG_MANAGER, 5000,
                new Kv(TYPE, CREATE_APP), new Kv(APPLICATION, app));
        return res.getBody();
    }

    @DELETE
    @Path("/{application}")
    @Produces({MediaType.APPLICATION_JSON, MediaType.APPLICATION_XML, MediaType.TEXT_HTML})
    public Object removeApp(@PathParam(APPLICATION) String app,
                            @HeaderParam(X_API_KEY) String key)
            throws IOException, TimeoutException, AppException {

        if (key == null) {
            throw new IllegalArgumentException("Missing "+X_API_KEY);
        }
        if (!apiKey.equals(key)) {
            throw new AppException(401, "Invalid "+X_API_KEY);
        }
        PostOffice po = PostOffice.getInstance();
        EventEnvelope res = po.request(MainApp.CONFIG_MANAGER, 5000,
                new Kv(TYPE, DELETE_APP), new Kv(APPLICATION, app));
        return res.getBody();
    }

}
