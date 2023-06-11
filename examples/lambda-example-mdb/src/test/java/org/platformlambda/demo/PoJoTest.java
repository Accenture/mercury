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

package org.platformlambda.demo;

import org.junit.Assert;
import org.junit.Test;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.demo.common.TestBase;
import org.platformlambda.models.SamplePoJo;

import java.io.IOException;
import java.util.concurrent.ArrayBlockingQueue;
import java.util.concurrent.BlockingQueue;
import java.util.concurrent.TimeUnit;

public class PoJoTest extends TestBase {

    @Test
    public void pojoRpcTest() throws IOException, InterruptedException {
        Integer ID = 1;
        String NAME = "Simple PoJo class";
        String ADDRESS = "100 World Blvd, Planet Earth";
        BlockingQueue<EventEnvelope> bench = new ArrayBlockingQueue<>(1);
        PostOffice po = new PostOffice("unit.test", "20001", "GET /api/hello/pojo");
        EventEnvelope request = new EventEnvelope().setTo("hello.pojo").setHeader("id", "1");
        po.asyncRequest(request, 800).onSuccess(bench::offer);
        EventEnvelope response = bench.poll(10, TimeUnit.SECONDS);
        assert response != null;
        Assert.assertEquals(SamplePoJo.class, response.getBody().getClass());
        SamplePoJo pojo = response.getBody(SamplePoJo.class);
        Assert.assertEquals(ID, pojo.getId());
        Assert.assertEquals(NAME, pojo.getName());
        Assert.assertEquals(ADDRESS, pojo.getAddress());
    }

}
