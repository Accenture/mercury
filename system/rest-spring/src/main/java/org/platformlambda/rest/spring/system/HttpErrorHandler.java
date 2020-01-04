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

package org.platformlambda.rest.spring.system;

import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.serializers.SimpleXmlWriter;
import org.platformlambda.core.util.Utility;
import org.springframework.boot.web.servlet.error.ErrorController;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import javax.ws.rs.core.MediaType;
import java.io.IOException;
import java.util.HashMap;

@RestController
public class HttpErrorHandler implements ErrorController {
    private static final Utility util = Utility.getInstance();
    private static final SimpleXmlWriter xmlWriter = new SimpleXmlWriter();
    private static final String ERROR_PATH = "/error";
    private static final String UTF8 = "utf-8";
    private static final String REQUEST_URI = "javax.servlet.error.request_uri";
    private static final String FORWARD_URI = "javax.servlet.forward.request_uri";
    private static final String ERROR_MESSAGE = "javax.servlet.error.message";
    private static final String ERROR_EXCEPTION = "javax.servlet.error.exception";
    private static final String STATUS_CODE = "javax.servlet.error.status_code";
    private static final String NOT_FOUND = "Not Found";

    private static final String TEMPLATE = "/errorPage.html";
    private static final String HTTP_UNKNOWN_WARNING = "There may be a problem in processing your request";
    private static final String HTTP_400_WARNING = "The system is unable to process your request";
    private static final String HTTP_500_WARNING = "Something may be broken";
    private static final String TYPE = "type";
    private static final String ERROR = "error";
    private static final String PATH = "path";
    private static final String ACCEPT = "accept";
    private static final String ACCEPT_ANY = "*/*";
    private static final String MESSAGE = "message";
    private static final String STATUS = "status";
    private static final String SET_MESSAGE = "${message}";
    private static final String SET_PATH = "${path}";
    private static final String SET_STATUS = "${status}";
    private static final String SET_WARNING = "${warning}";
    private static String template;

    @Override
    public String getErrorPath() {
        return ERROR_PATH;
    }

    @RequestMapping(ERROR_PATH)
    public void handlerError(HttpServletRequest request, HttpServletResponse response) throws IOException {

        String path = (String) request.getAttribute(REQUEST_URI);
        if (path == null) {
            path = (String) request.getAttribute(FORWARD_URI);
        }
        if (path == null) {
            path = ERROR_PATH;
        }
        String message = getError(request);
        Integer status = (Integer) request.getAttribute(STATUS_CODE);
        if (status == null) {
            status = 404;
        }
        if (status == 404 && message.length() == 0) {
            message = NOT_FOUND;
        }
        HttpErrorHandler.sendResponse(response, status, message, path, request.getHeader(ACCEPT));
    }

    private String getError(HttpServletRequest request) {
        String message = (String) request.getAttribute(ERROR_MESSAGE);
        if (message != null && !message.isEmpty()) {
            return message;
        }
        Object exception = request.getAttribute(ERROR_EXCEPTION);
        if (exception instanceof Throwable) {
            return ((Throwable) exception).getMessage();
        }
        return "";
    }

    public static void sendResponse(HttpServletResponse response, int status, String message, String path, String accept) throws IOException {
        if (template == null) {
            template = util.stream2str(HttpErrorHandler.class.getResourceAsStream(TEMPLATE));
        }
        HashMap<String, Object> error = new HashMap<>();
        error.put(TYPE, ERROR);
        error.put(MESSAGE, message);
        error.put(PATH, path);
        error.put(STATUS, status);

        String contentType;
        if (accept == null) {
            contentType = MediaType.APPLICATION_JSON;
        } else if (accept.contains(MediaType.TEXT_HTML)) {
            contentType = MediaType.TEXT_HTML;
        } else if (accept.contains(MediaType.APPLICATION_XML)) {
            contentType = MediaType.APPLICATION_XML;
        } else if (accept.contains(MediaType.APPLICATION_JSON) || accept.contains(ACCEPT_ANY)) {
            contentType = MediaType.APPLICATION_JSON;
        } else {
            contentType = MediaType.TEXT_PLAIN;
        }
        response.setCharacterEncoding(UTF8);
        response.setContentType(contentType);
        if (contentType.equals(MediaType.TEXT_HTML)) {
            String errorPage = template.replace(SET_STATUS, String.valueOf(status))
                    .replace(SET_PATH, path)
                    .replace(SET_MESSAGE, message);
            if (status >= 500) {
                errorPage = errorPage.replace(SET_WARNING, HTTP_500_WARNING);
            } else if (status >= 400) {
                errorPage = errorPage.replace(SET_WARNING, HTTP_400_WARNING);
            } else {
                errorPage = errorPage.replace(SET_WARNING, HTTP_UNKNOWN_WARNING);
            }
            response.getOutputStream().write(util.getUTF(errorPage));
        } else if (contentType.equals(MediaType.APPLICATION_JSON) || contentType.equals(MediaType.TEXT_PLAIN)) {
            response.getOutputStream().write(util.getUTF(SimpleMapper.getInstance().getMapper().writeValueAsString(error)));
        } else {
            response.getOutputStream().write(util.getUTF(xmlWriter.write(ERROR, error)));
        }
    }

}
