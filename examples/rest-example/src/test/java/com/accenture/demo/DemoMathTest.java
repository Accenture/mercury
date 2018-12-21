package com.accenture.demo;

import org.junit.BeforeClass;
import org.junit.Test;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.Kv;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import com.accenture.examples.services.DemoMath;

import java.io.IOException;
import java.util.concurrent.TimeoutException;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertFalse;

public class DemoMathTest {

    @BeforeClass
    public static void setup() throws IOException {
        Platform.getInstance().register("math.addition", new DemoMath(), 5);
    }

    @Test
    public void demo() throws IOException, TimeoutException, AppException {

        int a = 100;
        int b = 200;

        PostOffice po = PostOffice.getInstance();
        EventEnvelope response = po.request("math.addition", 5000, new Kv("a", a), new Kv("b", b));
        assertFalse(response.hasError());
        assertEquals(100 + 200, response.getBody());

    }


}
