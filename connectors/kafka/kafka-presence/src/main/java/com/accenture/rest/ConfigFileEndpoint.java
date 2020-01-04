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
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.system.PostOffice;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.servlet.ServletException;
import javax.servlet.annotation.MultipartConfig;
import javax.servlet.annotation.WebServlet;
import javax.servlet.http.HttpServlet;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import javax.servlet.http.Part;
import javax.ws.rs.core.MediaType;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.util.Date;
import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.TimeoutException;

@WebServlet(urlPatterns="/config/file")
@MultipartConfig
@OptionalService("app.config.manager")
public class ConfigFileEndpoint extends HttpServlet {
    private static final Logger log = LoggerFactory.getLogger(ConfigFileEndpoint.class);

    private static final String TYPE = "type";
    private static final String TIME = "time";
    private static final String DECRYPT = "decrypt";
    private static final String FILE_UPLOAD = "file_upload";
    private static final String FILE_DOWNLOAD = "file_download";
    private static final String MULTI_PART_LABEL = "file";
    private static final String MESSAGE = "message";
    private static final String TOTAL = "total";
    private static final String X_API_KEY = "x-api-key";
    private static final String CURRENT_APP_CONFIG = "current-config.json";
    private static final String OCTET_STREAM = MediaType.APPLICATION_OCTET_STREAM;
    private static final String CONTENT_DISPOSITION = "Content-Disposition";
    private static final String ATTACHMENT = "attachment; filename="+"\""+CURRENT_APP_CONFIG+"\"";
    private static final int BUFFER_SIZE = 2048;
    private static final int MAX_FILESIZE = 5 * 1000 * 1000;


    @Override
    public void doGet(HttpServletRequest request, HttpServletResponse response) throws IOException {
        String key = getApiKey(request);
        if (key == null) {
            response.sendError(401, "Missing "+X_API_KEY);
            return;
        }
        if (!key.equals(ConfigManagerEndpoint.getApiKey())) {
            response.sendError(401, "Invalid "+X_API_KEY);
            return;
        }
        PostOffice po = PostOffice.getInstance();
        // in case of failed upload, it will throw AppException
        try {
            EventEnvelope res = po.request(MainApp.CONFIG_MANAGER, 10000,
                    new Kv(DECRYPT, "true".equalsIgnoreCase(request.getParameter(DECRYPT))),
                    new Kv(TYPE, FILE_DOWNLOAD));
            if (res.getBody() instanceof Map) {
                byte[] b = SimpleMapper.getInstance().getMapper().writeValueAsBytes(res.getBody());
                response.setContentType(OCTET_STREAM);
                response.setHeader(CONTENT_DISPOSITION, ATTACHMENT);
                response.getOutputStream().write(b);

            } else {
                response.sendError(500, "Failed to download - unexpected server error");
            }
        } catch (TimeoutException e) {
            response.sendError(408, e.getMessage());
        } catch (AppException e) {
            response.sendError(e.getStatus(), e.getMessage());
        }
    }

    private String getApiKey(HttpServletRequest request) {
        String key = request.getHeader(X_API_KEY);
        return key == null? request.getParameter(X_API_KEY) : key;
    }

    @Override
    public void doPost(HttpServletRequest request, HttpServletResponse response) throws IOException {
        String key = request.getHeader(X_API_KEY);
        if (key == null) {
            response.sendError(401, "Missing "+X_API_KEY);
            return;
        }
        if (!key.equals(ConfigManagerEndpoint.getApiKey())) {
            response.sendError(401, "Invalid "+X_API_KEY);
            return;
        }
        String contentType = request.getContentType();
        if (!contentType.startsWith(MediaType.MULTIPART_FORM_DATA)) {
            response.sendError(400, "Invalid content-type. Expected: "+MediaType.MULTIPART_FORM_DATA+", Actual: "+contentType);
            return;
        }
        try {
            Part filePart = request.getPart(MULTI_PART_LABEL);
            if (filePart != null) {
                String fileName = getFileName(filePart);
                if (fileName != null) {
                    int len;
                    int total = 0;
                    byte[] buffer = new byte[BUFFER_SIZE];
                    InputStream in = filePart.getInputStream();
                    ByteArrayOutputStream out = new ByteArrayOutputStream();
                    while ((len = in.read(buffer, 0, buffer.length)) != -1) {
                        total += len;
                        if (total > MAX_FILESIZE) {
                            response.sendError(400, "File too large. Max "+MAX_FILESIZE+" bytes");
                            return;
                        }
                        out.write(buffer, 0, len);
                    }
                    if (total == 0) {
                        response.sendError(400, "Input file is empty");
                        return;
                    }
                    PostOffice po = PostOffice.getInstance();
                    // in case of failed upload, it will throw AppException
                    po.request(MainApp.CONFIG_MANAGER, 10000, out.toByteArray(), new Kv(TYPE, FILE_UPLOAD));
                    Map<String, Object> result = new HashMap<>();
                    result.put(TYPE, FILE_UPLOAD);
                    result.put(TOTAL, total);
                    result.put(MESSAGE, "Config file uploaded");
                    result.put(TIME, new Date());
                    response.setContentType(MediaType.APPLICATION_JSON);
                    byte[] json = SimpleMapper.getInstance().getMapper().writeValueAsBytes(result);
                    response.getOutputStream().write(json);
                    log.info("Uploaded config file with {} bytes", total);
                }
            } else {
                response.sendError(400, "Missing input '"+MULTI_PART_LABEL+"' in multi-part upload");
            }
        } catch (ServletException e) {
            response.sendError(500, e.getMessage());
        } catch (IllegalArgumentException e) {
            response.sendError(400, e.getMessage());
        } catch (TimeoutException e) {
            response.sendError(408, e.getMessage());
        } catch (AppException e) {
            response.sendError(e.getStatus(), e.getMessage());
        }
    }

    private String getFileName(final Part part) {
        for (String content : part.getHeader("content-disposition").split(";")) {
            if (content.trim().startsWith("filename")) {
                return content.substring(content.indexOf('=') + 1).trim().replace("\"", "");
            }
        }
        return null;
    }

}
