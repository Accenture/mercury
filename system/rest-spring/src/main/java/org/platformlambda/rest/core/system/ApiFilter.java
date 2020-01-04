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

package org.platformlambda.rest.core.system;

import javax.annotation.Priority;
import javax.ws.rs.container.ContainerRequestContext;
import javax.ws.rs.container.ContainerResponseContext;
import javax.ws.rs.container.ContainerResponseFilter;
import javax.ws.rs.container.PreMatching;
import javax.ws.rs.ext.Provider;
import java.io.IOException;

@PreMatching
@Provider
@Priority(value = 2)
public class ApiFilter implements ContainerResponseFilter {

    private static final String PROTOCOL = "x-forwarded-proto";
    private static final String HTTPS = "https";
    private static final String UPGRADE = "upgrade";
    private static final String TRANSPORT_SECURITY_KEY = "Strict-Transport-Security";
    private static final String TRANSPORT_SECURITY_VALUE = "max-age=31536000; includeSubDomains";

    @Override
    public void filter(ContainerRequestContext requestContext, ContainerResponseContext responseContext) throws IOException {
        String upgrade = requestContext.getHeaderString(UPGRADE);
        String protocol = requestContext.getHeaderString(PROTOCOL);
        /*
         * HTTP Strict Transport Security (HSTS)
         * https://tools.ietf.org/html/rfc6797
         *
         * If HTTPS, add "Strict Transport Security" header.
         */
        if (upgrade == null && HTTPS.equals(protocol)) {
            responseContext.getHeaders().add(TRANSPORT_SECURITY_KEY, TRANSPORT_SECURITY_VALUE);
        }
    }

}
