/*

    Copyright 2018-2024 Accenture Technology

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

package org.platformlambda.demo;

import org.junit.Assert;
import org.junit.Test;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.core.util.MultiLevelMap;
import org.platformlambda.core.util.Utility;
import org.platformlambda.demo.common.TestBase;

import java.io.IOException;
import java.util.Date;
import java.util.HashMap;
import java.util.Map;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.BlockingQueue;
import java.util.concurrent.TimeUnit;

public class HelloWorldTest extends TestBase {

    @SuppressWarnings("unchecked")
    @Test
    public void rpcTest() throws IOException, InterruptedException {
        Utility util = Utility.getInstance();
        BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        String NAME = "hello";
        String ADDRESS = "world";
        String TELEPHONE = "123-456-7890";
        DemoPoJo pojo = new DemoPoJo(NAME, ADDRESS, TELEPHONE);
        PostOffice po = new PostOffice("unit.test", "12345", "POST /api/hello/world");
        EventEnvelope request = new EventEnvelope().setTo("hello.world").setBody(pojo.toMap());
        po.asyncRequest(request, 800).onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertEquals(HashMap.class, response.getBody().getClass());
        MultiLevelMap map = new MultiLevelMap((Map<String, Object>) response.getBody());
        Assert.assertEquals(NAME, map.getElement("body.name"));
        Assert.assertEquals(ADDRESS, map.getElement("body.address"));
        Assert.assertEquals(TELEPHONE, map.getElement("body.telephone"));
        Assert.assertEquals(util.date2str(pojo.time), map.getElement("body.time"));
    }

    private class DemoPoJo {
        String name;
        String address;
        String telephone;
        Date time;

        private DemoPoJo(String name, String address, String telephone) {
            this.name = name;
            this.address = address;
            this.telephone = telephone;
            this.time = new Date();
        }

        private Map<String, Object> toMap() {
            Map<String, Object> result = new HashMap<>();
            result.put("name", name);
            result.put("address", address);
            result.put("telephone", telephone);
            result.put("time", time);
            return result;
        }
    }

}
