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

package org.platformlambda.core.util;

import org.junit.Test;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;

import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.TimeoutException;

import static org.junit.Assert.*;

public class RbacTest {
    
    private static final String ROLES = "roles";

    @Test
    @SuppressWarnings("unchecked")
    public void simpleRbac() throws IOException, TimeoutException, AppException {
        Utility util = Utility.getInstance();
        PostOffice po = PostOffice.getInstance();
        // validate RBAC inside a service function
        LambdaFunction f = (headers, body, instance) -> {
            List<String> result = new ArrayList<>();
            result.add(po.getRoute());
            /*
             * Simple RBAC to validate if the user roles are allowed to access this function.
             * Note that route name of this function is passed using po.getRoute()
             *
             * For production code, the user roles should come from the user profile after authentication.
             */
            SimpleRBAC rbac = SimpleRBAC.getInstance();
            if (headers.containsKey(ROLES)) {
                List<String> passRoles = util.split(headers.get(ROLES), ", ");
                // check if the roles are allowed
                boolean status = rbac.permitted(po.getRoute(), fromList(passRoles));
                result.add(String.valueOf(status));
            }
            return result;
        };

        Platform platform = Platform.getInstance();
        platform.registerPrivate("v1.data.read", f, 1);
        platform.registerPrivate("v1.data.update", f, 1);
        platform.registerPrivate("v1.data.export", f, 1);
        
        EventEnvelope res = po.request("v1.data.read", 5000, new Kv(ROLES, "MEMBER"));
        assertTrue(res.getBody() instanceof List);
        List<String> result = (List<String>) res.getBody();
        assertEquals(2, result.size());
        assertEquals("v1.data.read", result.get(0));
        assertEquals("true", result.get(1));

        res = po.request("v1.data.update", 5000, new Kv(ROLES, "DEVOPS, manager"));
        assertTrue(res.getBody() instanceof List);
        result = (List<String>) res.getBody();
        assertEquals(2, result.size());
        assertEquals("v1.data.update", result.get(0));
        assertEquals("true", result.get(1));


        res = po.request("v1.data.export", 5000, new Kv(ROLES, "admin, manager"));
        assertTrue(res.getBody() instanceof List);
        result = (List<String>) res.getBody();
        assertEquals(2, result.size());
        assertEquals("v1.data.export", result.get(0));
        assertEquals("false", result.get(1));

        res = po.request("v1.data.export", 5000, new Kv(ROLES, "member, devops"));
        assertTrue(res.getBody() instanceof List);
        result = (List<String>) res.getBody();
        assertEquals(2, result.size());
        assertEquals("v1.data.export", result.get(0));
        assertEquals("true", result.get(1));
    }

    private String[] fromList(List<String> list) {
        String[] result = new String[list.size()];
        for (int i=0; i < list.size(); i++) {
            result[i] = list.get(i);
        }
        return result;
    }
}
