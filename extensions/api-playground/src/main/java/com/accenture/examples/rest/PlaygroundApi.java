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

package com.accenture.examples.rest;

import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.ws.rs.GET;
import javax.ws.rs.Path;
import javax.ws.rs.PathParam;
import javax.ws.rs.Produces;
import javax.ws.rs.core.MediaType;
import java.io.File;
import java.util.*;

@Path("/specs")
public class PlaygroundApi {
    private static final Logger log = LoggerFactory.getLogger(PlaygroundApi.class);

    private static final String YAML = ".yaml";
    private static final String JSON = ".json";
    private static final String TOTAL = "total";
    private static final String LIST = "list";
    private static final String TIME = "time";
    private static final String FILE_PREFIX = "file:";
    private static final String CLASSPATH_PREFIX = "classpath:";
    private static final String API_PLAYGROUND_APPS = "api.playground.apps";
    private static boolean ready = false;
    private static File dir;

    public PlaygroundApi() {
        if (dir == null) {
            AppConfigReader config = AppConfigReader.getInstance();
            String location = config.getProperty(API_PLAYGROUND_APPS, "/tmp/api-playground");
            if (location.startsWith(FILE_PREFIX)) {
                location = location.substring(FILE_PREFIX.length());
            }
            if (location.startsWith(CLASSPATH_PREFIX)) {
                log.error("{} must be a folder in the local file system", API_PLAYGROUND_APPS);
            }
            File f = new File(location);
            if (f.exists() && f.isDirectory()) {
                dir = f;
                ready = true;
            } else {
                log.error("{} contains invalid file path {}", API_PLAYGROUND_APPS, f);
            }
        }
    }

    @GET
    @Path("/")
    @Produces({MediaType.TEXT_PLAIN, MediaType.APPLICATION_XML, MediaType.APPLICATION_JSON, MediaType.TEXT_HTML})
    public Map<String, Object> listFiles() throws AppException {
        if (!ready) {
            throw new AppException(503, "API playground not ready");
        }
        List<String> fileList = new ArrayList<>();
        File[] files = dir.listFiles();
        for (File f: files) {
            if (f.getName().endsWith(YAML) || f.getName().endsWith(JSON)) {
                fileList.add(f.getName());
            }
        }
        if (fileList.size() > 1) {
            Collections.sort(fileList);
        }
        Map<String, Object> result = new HashMap<>();
        result.put(TOTAL, fileList.size());
        result.put(TIME, new Date());
        result.put(LIST, fileList);
        return result;
    }

    @GET
    @Path("/{id}")
    @Produces({MediaType.TEXT_PLAIN})
    public String getSpecs(@PathParam("id") String id) throws AppException {
        if (!ready) {
            throw new AppException(503, "API playground not ready");
        }
        File f = new File(dir, id);
        if (!f.exists()) {
            throw new AppException(404, "File not found");
        }
        return Utility.getInstance().file2str(f);
    }

}
