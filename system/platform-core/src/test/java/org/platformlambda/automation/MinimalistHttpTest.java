package org.platformlambda.automation;

import org.junit.Assert;
import org.junit.Test;
import org.platformlambda.automation.util.SimpleHttpRequests;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.mock.TestBase;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.util.MultiLevelMap;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class MinimalistHttpTest extends TestBase {
    private static final Logger log = LoggerFactory.getLogger(MinimalistHttpTest.class);

    private static final int HTTP_PORT = MINIMALIST_HTTP_PORT;

    @SuppressWarnings("unchecked")
    @Test
    public void infoEndpointTest() throws AppException, IOException {
        String response = SimpleHttpRequests.get("http://127.0.0.1:"+ HTTP_PORT +"/info");
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        MultiLevelMap multi = new MultiLevelMap(result);
        Assert.assertEquals("platform-core", multi.getElement("app.name"));
        Assert.assertEquals("REST", multi.getElement("personality"));
        String origin = Platform.getInstance().getOrigin();
        Assert.assertEquals(origin, multi.getElement("origin"));
    }


    @Test(expected = AppException.class)
    public void nonExistRemoteInfoEndpointTest() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        SimpleHttpRequests.get("http://127.0.0.1:"+ HTTP_PORT +"/info", headers);
    }

    @SuppressWarnings("unchecked")
    @Test
    public void libEndpointTest() throws AppException, IOException {
        String response = SimpleHttpRequests.get("http://127.0.0.1:"+ HTTP_PORT +"/info/lib");
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        MultiLevelMap multi = new MultiLevelMap(result);
        Assert.assertEquals("platform-core", multi.getElement("app.name"));
        Assert.assertTrue(result.containsKey("library"));
    }

    @Test(expected = AppException.class)
    public void remoteLibEndpointTest() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        SimpleHttpRequests.get("http://127.0.0.1:"+HTTP_PORT+"/info/lib", headers);
    }

    @SuppressWarnings("unchecked")
    @Test
    public void routeEndpointTest() throws AppException, IOException {
        String response = SimpleHttpRequests.get("http://127.0.0.1:"+HTTP_PORT+"/info/routes");
        Map<String, Object> data = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        Assert.assertTrue(data.get("routing") instanceof Map);
        Map<String, Object> routing = (Map<String, Object>) data.get("routing");
        Assert.assertEquals(new HashMap<>(), routing.get("routes"));
    }

    @Test(expected = AppException.class)
    public void remoteRouteEndpointTest() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        SimpleHttpRequests.get("http://127.0.0.1:"+HTTP_PORT+"/info/routes", headers);
    }

    @SuppressWarnings("unchecked")
    @Test
    public void healthEndpointTest() throws AppException, IOException {
        String response = SimpleHttpRequests.get("http://127.0.0.1:"+HTTP_PORT+"/health");
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        Assert.assertEquals("UP", result.get("status"));
    }

    @Test(expected = AppException.class)
    public void remoteHealthEndpointTest() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        SimpleHttpRequests.get("http://127.0.0.1:"+HTTP_PORT+"/health", headers);
    }

    @Test
    public void livenessEndpointTest() throws AppException, IOException {
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+HTTP_PORT+"/livenessprobe", "text/plain");
        Assert.assertEquals("OK", response);
    }

    @Test(expected = AppException.class)
    public void remoteLivenessEndpointTest() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        SimpleHttpRequests.get("http://127.0.0.1:"+HTTP_PORT+"/livenessprobe", headers);
    }

    @SuppressWarnings("unchecked")
    @Test
    public void envEndpointTest() throws AppException, IOException {
        String response = SimpleHttpRequests.get("http://127.0.0.1:"+HTTP_PORT+"/env");
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        MultiLevelMap multi = new MultiLevelMap(result);
        Assert.assertEquals("platform-core", multi.getElement("app.name"));
        Assert.assertTrue(multi.getElement("env") instanceof Map);
        Assert.assertTrue(multi.getElement("routing.private") instanceof List);
        Assert.assertTrue(multi.getElement("routing.public") instanceof List);
    }

    @Test(expected = AppException.class)
    public void remoteEnvEndpointTest() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        SimpleHttpRequests.get("http://127.0.0.1:"+HTTP_PORT+"/env", headers);
    }

    @Test
    public void shutdownUsingGetWillFail() {
        try {
            SimpleHttpRequests.get("http://127.0.0.1:" + HTTP_PORT + "/shutdown/now");
        } catch (Exception e) {
            Assert.assertTrue(e instanceof AppException);
            AppException ex = (AppException) e;
            Assert.assertEquals(404, ex.getStatus());
        }
    }

    @Test
    public void suspendUsingGetWillFail() {
        try {
            SimpleHttpRequests.get("http://127.0.0.1:"+HTTP_PORT+"/suspend/now");
        } catch (Exception e) {
            Assert.assertTrue(e instanceof AppException);
            AppException ex = (AppException) e;
            Assert.assertEquals(404, ex.getStatus());
        }
    }

    @Test
    public void resumeUsingGetWillFail() {
        try {
            SimpleHttpRequests.get("http://127.0.0.1:"+HTTP_PORT+"/resume/now");
        } catch (Exception e) {
            Assert.assertTrue(e instanceof AppException);
            AppException ex = (AppException) e;
            Assert.assertEquals(404, ex.getStatus());
        }
    }

    @Test
    public void shutdownWithoutAppInstanceWillFail() {
        try {
            SimpleHttpRequests.post("http://127.0.0.1:"+HTTP_PORT+"/shutdown", new HashMap<>(), new HashMap<>());
        } catch (Exception e) {
            Assert.assertTrue(e instanceof AppException);
            AppException ex = (AppException) e;
            Assert.assertEquals(400, ex.getStatus());
        }
    }

    @Test(expected = AppException.class)
    public void shutdownWithIncorrectAppInstanceWillFail() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", "does-not-exist");
        SimpleHttpRequests.post("http://127.0.0.1:"+HTTP_PORT+"/shutdown", headers, new HashMap<>());
    }

    @Test(expected = AppException.class)
    public void suspendWithoutAppInstanceWillFail() throws AppException, IOException {
        SimpleHttpRequests.post("http://127.0.0.1:"+HTTP_PORT+"/suspend/now", new HashMap<>(), new HashMap<>());
    }

    @Test(expected = AppException.class)
    public void suspendWithIncorrectAppInstanceWillFail() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", "does-not-exist");
        SimpleHttpRequests.post("http://127.0.0.1:"+HTTP_PORT+"/suspend/now", headers, new HashMap<>());
    }

    @Test(expected = AppException.class)
    public void suspendWithOutAppInstanceWillFail() throws AppException, IOException {
        SimpleHttpRequests.post("http://127.0.0.1:"+HTTP_PORT+"/suspend/now", new HashMap<>(), new HashMap<>());
    }

    @Test(expected = AppException.class)
    public void resumeWithoutAppInstanceWillFail() throws AppException, IOException {
        SimpleHttpRequests.post("http://127.0.0.1:"+HTTP_PORT+"/resume/now", new HashMap<>(), new HashMap<>());
    }

    @Test(expected = AppException.class)
    public void resumeWithIncorrectAppInstanceWillFail() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", "does-not-exist");
        SimpleHttpRequests.post("http://127.0.0.1:"+HTTP_PORT+"/resume/now", headers, new HashMap<>());
    }

    @SuppressWarnings("unchecked")
    @Test
    public void suspendTest() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", Platform.getInstance().getOrigin());
        String response = SimpleHttpRequests.post("http://127.0.0.1:"+HTTP_PORT+"/suspend/now", headers, new HashMap<>());
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        Assert.assertEquals("suspend", result.get("type"));
        Assert.assertEquals(200, result.get("status"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void resumeTest() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", Platform.getInstance().getOrigin());
        String response = SimpleHttpRequests.post("http://127.0.0.1:"+HTTP_PORT+"/resume/now", headers, new HashMap<>());
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        Assert.assertEquals("resume", result.get("type"));
        Assert.assertEquals(200, result.get("status"));
    }

    @Test(expected = AppException.class)
    public void pageNotExists() throws AppException, IOException {
        SimpleHttpRequests.get("http://127.0.0.1:"+HTTP_PORT+"/no_such_page");
    }
}
