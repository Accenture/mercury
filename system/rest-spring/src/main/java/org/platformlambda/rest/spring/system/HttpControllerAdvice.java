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

import org.platformlambda.core.exception.AppException;
import org.springframework.web.bind.annotation.ExceptionHandler;
import org.springframework.web.bind.annotation.RestControllerAdvice;

import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import java.io.IOException;

@RestControllerAdvice
public class HttpControllerAdvice {

    private static final String ACCEPT = "accept";

    @ExceptionHandler(value = { IllegalArgumentException.class })
    public void handleError(HttpServletRequest request, HttpServletResponse response, IllegalArgumentException e) throws IOException {
        HttpErrorHandler.sendResponse(response, 400, e.getMessage(), request.getRequestURI(), request.getHeader(ACCEPT));
    }

    @ExceptionHandler(value = { IOException.class })
    public void handleError(HttpServletRequest request, HttpServletResponse response, IOException e) throws IOException {
        HttpErrorHandler.sendResponse(response, 500, e.getMessage(), request.getRequestURI(), request.getHeader(ACCEPT));
    }

    @ExceptionHandler(value = { NullPointerException.class })
    public void handleError(HttpServletRequest request, HttpServletResponse response) throws IOException {
        HttpErrorHandler.sendResponse(response, 500, "Null pointer exception", request.getRequestURI(), request.getHeader(ACCEPT));
    }

    /////////////////////////////
    // for application exception
    /////////////////////////////

    @ExceptionHandler(value = { AppException.class })
    public void handleError(HttpServletRequest request, HttpServletResponse response, AppException e) throws IOException {
        HttpErrorHandler.sendResponse(response, e.getStatus(), e.getMessage(), request.getRequestURI(), request.getHeader(ACCEPT));
    }

}
