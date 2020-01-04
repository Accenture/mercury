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

package org.platformlambda.rest.spring.servlet;

import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;

import javax.servlet.annotation.WebServlet;
import javax.servlet.http.HttpServlet;
import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;
import java.io.IOException;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

@WebServlet("/env")
public class EnvServlet extends HttpServlet {

	private static final long serialVersionUID = 3495394964619652075L;
	private static final String SHOW_ENV = "show.env.variables";
	private static final String SHOW_PROPERTIES = "show.application.properties";
	private static final String SYSTEM_ENV = "systemEnvironment";
	private static final String APP_PROPS = "applicationProperties";
	private static final String MISSING = "missing";

	@Override
	protected void doGet(HttpServletRequest request, HttpServletResponse response) throws IOException {

		Map<String, Object> result = new HashMap<>(HealthServlet.getBasicInfo());

		Utility util = Utility.getInstance();
		AppConfigReader reader = AppConfigReader.getInstance();

		List<String> envVars = util.split(reader.getProperty(SHOW_ENV, ""), ", ");
		List<String> properties = util.split(reader.getProperty(SHOW_PROPERTIES, ""), ", ");

		List<String> missingVars = new ArrayList<>();
		Map<String, Object> eMap = new HashMap<>();
		if (!envVars.isEmpty()) {
			for (String var: envVars) {
				String v = System.getenv(var);
				if (v == null) {
					missingVars.add(var);
				} else {
					eMap.put(var, v);
				}
			}
		}
		result.put(SYSTEM_ENV, eMap);

		List<String> missingProp = new ArrayList<>();
		Map<String, Object> pMap = new HashMap<>();
		if (!properties.isEmpty()) {
			for (String var: properties) {
				String v = reader.getProperty(var);
				if (v == null) {
					missingProp.add(var);
				} else {
					pMap.put(var, v);
				}
			}
		}
		result.put(APP_PROPS, pMap);
		// any missing keys?
		Map<String, Object> missingKeys = new HashMap<>();
		if (!missingVars.isEmpty()) {
			missingKeys.put(SYSTEM_ENV, missingVars);
		}
		if (!missingProp.isEmpty()) {
			missingKeys.put(APP_PROPS, missingProp);
		}
		if (!missingKeys.isEmpty()) {
			result.put(MISSING, missingKeys);
		}
		response.setContentType("application/json");
		response.setCharacterEncoding("utf-8");
		response.getWriter().write(SimpleMapper.getInstance().getMapper().writeValueAsString(result));
	}

}
