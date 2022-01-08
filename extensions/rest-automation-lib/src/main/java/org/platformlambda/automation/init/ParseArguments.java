/*

    Copyright 2018-2022 Accenture Technology

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

package org.platformlambda.automation.init;

import org.platformlambda.core.annotations.BeforeApplication;
import org.platformlambda.core.models.EntryPoint;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.File;

@BeforeApplication(sequence = 1)
public class ParseArguments implements EntryPoint {
    private static final Logger log = LoggerFactory.getLogger(ParseArguments.class);

    private static final String HTML_FOLDER = "spring.resources.static-locations";
    private static final String HTML_ARG = "-html";
    private static final String FILE_PATH = "file:/";

    @Override
    public void start(String[] args) {
        if (args.length > 0) {
            for (int i=0; i < args.length; i++) {
                if (args[i].equals(HTML_ARG)) {
                    if (i+1 < args.length) {
                        String path = normalizePath(args[i+1]);
                        if (folderExists(path)) {
                            System.setProperty(HTML_FOLDER, path);
                            log.info("Using HTML folder {}", path);
                        } else {
                            log.error("Invalid HTML folder path {}", path);
                        }

                    } else {
                        log.error("Missing html file path");
                    }
                }
            }
        }
    }

    private String normalizePath(String path) {
        if (path.startsWith(FILE_PATH)) {
            return path;
        } else {
            return FILE_PATH + (path.startsWith("/")? path.substring(1) : path);
        }
    }

    private boolean folderExists(String path) {
        File dir = new File(path.substring(FILE_PATH.length()-1));
        return dir.exists() && dir.isDirectory();
    }
    
}
