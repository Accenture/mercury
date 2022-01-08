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

package org.platformlambda.servlets;

import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;

import javax.servlet.annotation.WebServlet;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import javax.ws.rs.core.MediaType;
import java.io.IOException;
import java.util.concurrent.TimeoutException;

@WebServlet("/env")
public class EnvServlet extends InfoServletBase {
	private static final long serialVersionUID = 3495394964619652075L;

	private static final String APP_INSTANCE = "X-App-Instance";
	private static final String TYPE = "type";
	private static final String ENV = "env";

	@Override
	protected void doGet(HttpServletRequest request, HttpServletResponse response) throws IOException {
		String myOrigin = Platform.getInstance().getOrigin();
		String origin = request.getHeader(APP_INSTANCE);
		if (origin == null) {
			if (protectEndpoint && !isIntranetAddress(request)) {
				response.sendError(404, "Resource not found");
				return;
			}
			origin = myOrigin;
		}
		PostOffice po = PostOffice.getInstance();
		EventEnvelope event = new EventEnvelope().setHeader(TYPE, ENV);
		if (origin.equals(myOrigin)) {
			event.setTo(PostOffice.ACTUATOR_SERVICES);
		} else {
			if (!po.exists(origin)) {
				response.sendError(400, origin+" is not reachable");
				return;
			}
			event.setTo(PostOffice.ACTUATOR_SERVICES+"@"+origin);
		}
		try {
			EventEnvelope result = po.request(event, 10000);
			response.setContentType(MediaType.APPLICATION_JSON);
			byte[] b = SimpleMapper.getInstance().getMapper().writeValueAsBytes(result.getBody());
			response.setContentLength(b.length);
			response.getOutputStream().write(b);
		} catch (TimeoutException e) {
			response.sendError(408, origin+" timeout");
		} catch (AppException e) {
			response.sendError(e.getStatus(), e.getMessage());
		}
	}

}
