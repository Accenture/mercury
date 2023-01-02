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

package org.platformlambda.rest.exception;

import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.util.Utility;

import javax.servlet.http.HttpServletRequest;
import javax.ws.rs.core.Context;
import javax.ws.rs.core.MediaType;
import javax.ws.rs.core.Response;
import javax.ws.rs.ext.ExceptionMapper;
import javax.ws.rs.ext.Provider;
import java.util.HashMap;
import java.util.Map;

@Provider
public class RestExceptionHandler implements ExceptionMapper<Throwable> {

    private static final Utility util = Utility.getInstance();

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

    private static final String RESTEASY_PREFIX = "RESTEASY";
    private static final String CHARSET_UTF = ";charset=utf-8";
    private static final String CONTENT_TYPE = "content-type";
    private static final String UNKNOWN = "Unknown error";
    private static final String NOT_FOUND_EXCEPTION = "NotFoundException";
    private static final String NOT_SUPPORTED_EXCEPTION = "NotSupportedException";
    private static final String NOT_ALLOWED_EXCEPTION = "NotAllowedException";
    private static final String ILLEGAL_ARG_EXCEPTION = "IllegalArgumentException";
    private static final String BAD_REQUEST_EXCEPTION = "BadRequestException";
    private static final String FORBIDDEN_EXCEPTION = "ForbiddenException";
    private static final String NOT_ACCEPTABLE_EXCEPTION = "NotAcceptableException";
    private static final String NOT_AUTHORIZED_EXCEPTION = "NotAuthorizedException";
    private static final String NOT_AVAILABLE_EXCEPTION = "ServiceUnavailableException";

    private static String template;

    @Context
    private HttpServletRequest request;

    @Override
    public Response toResponse(Throwable exception) {
        if (template == null) {
            template = util.stream2str(RestExceptionHandler.class.getResourceAsStream(TEMPLATE));
        }
        String path = Utility.getInstance().getUrlDecodedPath(request.getRequestURI());
        // for security, drop unsafe portion of URI path
        if (path.contains("://")) {
            path = path.substring(0, path.indexOf("://")) + "...";
        }
        if (path.contains(" ")) {
            path = path.substring(0, path.indexOf(' ')) + "...";
        }
        // this avoids double URL encoding security vulnerability
        if (path.contains("%")) {
            path = path.substring(0, path.indexOf('%')) + "...";
        }
        Throwable ex = util.getRootCause(exception);
        String cls = ex.getClass().getSimpleName();
        Map<String, Object> result = new HashMap<>();
        result.put(TYPE, ERROR);
        result.put(PATH, path);
        String accept = request.getHeader(ACCEPT);
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
        String errorMessage;
        int status;
        if (ex instanceof AppException) {
            status = ((AppException) ex).getStatus();
            errorMessage = ex.getMessage();
        } else {
            errorMessage = filterOutRestEasyError(ex.getMessage());
            switch(cls) {
                case NOT_FOUND_EXCEPTION:
                    status = Response.Status.NOT_FOUND.getStatusCode();
                    if (errorMessage == null) {
                        errorMessage = Response.Status.NOT_FOUND.getReasonPhrase();
                    }
                    break;
                case NOT_SUPPORTED_EXCEPTION:
                    status = Response.Status.UNSUPPORTED_MEDIA_TYPE.getStatusCode();
                    if (errorMessage == null) {
                        errorMessage = Response.Status.UNSUPPORTED_MEDIA_TYPE.getReasonPhrase();
                    }
                    break;
                case NOT_ALLOWED_EXCEPTION:
                    status = Response.Status.METHOD_NOT_ALLOWED.getStatusCode();
                    if (errorMessage == null) {
                        errorMessage = Response.Status.METHOD_NOT_ALLOWED.getReasonPhrase();
                    }
                    break;
                case ILLEGAL_ARG_EXCEPTION:
                case BAD_REQUEST_EXCEPTION:
                    status = Response.Status.BAD_REQUEST.getStatusCode();
                    if (errorMessage == null) {
                        errorMessage = Response.Status.BAD_REQUEST.getReasonPhrase();
                    }
                    break;
                case FORBIDDEN_EXCEPTION:
                    status = Response.Status.FORBIDDEN.getStatusCode();
                    if (errorMessage == null) {
                        errorMessage = Response.Status.FORBIDDEN.getReasonPhrase();
                    }
                    break;
                case NOT_ACCEPTABLE_EXCEPTION:
                    status = Response.Status.NOT_ACCEPTABLE.getStatusCode();
                    if (errorMessage == null) {
                        errorMessage = Response.Status.NOT_ACCEPTABLE.getReasonPhrase();
                    }
                    break;
                case NOT_AUTHORIZED_EXCEPTION:
                    status = Response.Status.UNAUTHORIZED.getStatusCode();
                    if (errorMessage == null) {
                        errorMessage = Response.Status.UNAUTHORIZED.getReasonPhrase();
                    }
                    break;
                case NOT_AVAILABLE_EXCEPTION:
                    status = Response.Status.SERVICE_UNAVAILABLE.getStatusCode();
                    if (errorMessage == null) {
                        errorMessage = Response.Status.SERVICE_UNAVAILABLE.getReasonPhrase();
                    }
                    break;
                default:
                    status = 500;
                    break;
            }
        }
        // if error message cannot be resolved
        if (errorMessage == null) {
            errorMessage = UNKNOWN;
        }
        if (contentType.equals(MediaType.TEXT_HTML)) {
            Response.ResponseBuilder htmlError = Response.status(status);
            htmlError.header(CONTENT_TYPE, MediaType.TEXT_HTML + CHARSET_UTF);
            String errorPage = template.replace(SET_STATUS, String.valueOf(status))
                    .replace(SET_PATH, path)
                    .replace(SET_MESSAGE, errorMessage);
            if (status >= 500) {
                errorPage = errorPage.replace(SET_WARNING, HTTP_500_WARNING);
            } else if (status >= 400) {
                errorPage = errorPage.replace(SET_WARNING, HTTP_400_WARNING);
            } else {
                errorPage = errorPage.replace(SET_WARNING, HTTP_UNKNOWN_WARNING);
            }
            return htmlError.entity(errorPage).build();
        } else {
            result.put(STATUS, status);
            result.put(MESSAGE, errorMessage);
            if (contentType.equals(MediaType.TEXT_PLAIN)) {
                try {
                    String text = SimpleMapper.getInstance().getMapper().writeValueAsString(result);
                    return Response.status(status)
                            .header(CONTENT_TYPE, MediaType.TEXT_PLAIN + CHARSET_UTF)
                            .entity(text).build();
                } catch (Exception e) {
                    return Response.status(status)
                            .header(CONTENT_TYPE, MediaType.APPLICATION_JSON + CHARSET_UTF)
                            .entity(result).build();
                }
            } else {
                return Response.status(status).header(CONTENT_TYPE, contentType + CHARSET_UTF).entity(result).build();
            }
        }
    }

    private String filterOutRestEasyError(String error) {
        // check for RestEasy error message signature
        if (error != null && error.startsWith(RESTEASY_PREFIX) && error.contains(":")) {
            return null;
        } else {
            return error;
        }
    }

}
