package org.platformlambda.automation;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;
import org.platformlambda.automation.util.SimpleHttpRequests;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.mock.MockCloud;
import org.platformlambda.core.mock.TestBase;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.util.MultiLevelMap;
import org.platformlambda.core.websocket.server.MinimalistHttpHandler;

import java.io.IOException;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class MinimalistHttpTest extends TestBase {

    private static final int HTTP_PORT = MINIMALIST_HTTP_PORT;
    private static final String[][] ADMIN_ENDPOINTS = MinimalistHttpHandler.ADMIN_ENDPOINTS;

    @SuppressWarnings("unchecked")
    @Test
    public void homePageTest() throws AppException, IOException {
        String response = SimpleHttpRequests.get("http://127.0.0.1:"+ HTTP_PORT);
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        MultiLevelMap multi = new MultiLevelMap(result);
        Assertions.assertEquals("Minimalist HTTP server supports these admin endpoints",
                            multi.getElement("message"));
        int n = 0;
        for (String[] service: ADMIN_ENDPOINTS) {
            Assertions.assertEquals(service[0], multi.getElement("endpoints["+n+"]"));
            n++;
        }
        Assertions.assertTrue(multi.exists("time"));
        Assertions.assertEquals("platform-core", multi.getElement("name"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void infoEndpointTest() throws AppException, IOException {
        String response = SimpleHttpRequests.get("http://127.0.0.1:"+ HTTP_PORT +"/info");
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        MultiLevelMap multi = new MultiLevelMap(result);
        Assertions.assertEquals("platform-core", multi.getElement("app.name"));
        Assertions.assertEquals("REST", multi.getElement("personality"));
        String origin = Platform.getInstance().getOrigin();
        Assertions.assertEquals(origin, multi.getElement("origin"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void nonExistRemoteInfoEndpointTest() {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        AppException ex = Assertions.assertThrows(AppException.class, () ->
                                SimpleHttpRequests.get("http://127.0.0.1:"+ HTTP_PORT +"/info", headers));
        String error = ex.getMessage();
        Assertions.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assertions.assertEquals(404, result.get("status"));
        Assertions.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void libEndpointTest() throws AppException, IOException {
        String response = SimpleHttpRequests.get("http://127.0.0.1:"+ HTTP_PORT +"/info/lib");
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        MultiLevelMap multi = new MultiLevelMap(result);
        Assertions.assertEquals("platform-core", multi.getElement("app.name"));
        Assertions.assertTrue(result.containsKey("library"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void remoteLibEndpointTest() {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        AppException ex = Assertions.assertThrows(AppException.class, () ->
                SimpleHttpRequests.get("http://127.0.0.1:"+ HTTP_PORT +"/info/lib", headers));
        String error = ex.getMessage();
        Assertions.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assertions.assertEquals(404, result.get("status"));
        Assertions.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void routeEndpointTest() throws AppException, IOException {
        String response = SimpleHttpRequests.get("http://127.0.0.1:"+HTTP_PORT+"/info/routes");
        Map<String, Object> data = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        Assertions.assertTrue(data.get("routing") instanceof Map);
        Map<String, Object> routing = (Map<String, Object>) data.get("routing");
        Assertions.assertEquals(new HashMap<>(), routing.get("routes"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void remoteRouteEndpointTest() {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        AppException ex = Assertions.assertThrows(AppException.class, () ->
                                SimpleHttpRequests.get("http://127.0.0.1:"+ HTTP_PORT +"/info/routes", headers));
        String error = ex.getMessage();
        Assertions.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assertions.assertEquals(404, result.get("status"));
        Assertions.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void healthEndpointTest() throws AppException, IOException {
        MockCloud.setSimulateException(false);
        String response = SimpleHttpRequests.get("http://127.0.0.1:"+HTTP_PORT+"/health");
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        MultiLevelMap map = new MultiLevelMap(result);
        Assertions.assertEquals("UP", map.getElement("status"));
        Assertions.assertEquals("fine", map.getElement("upstream[0].message"));
        Assertions.assertEquals(200, map.getElement("upstream[0].status_code"));
        Assertions.assertEquals("mock.connector", map.getElement("upstream[0].service"));
        // livenessProbe is linked to health check
        String live = SimpleHttpRequests.get("http://127.0.0.1:"+HTTP_PORT+"/livenessprobe", "text/plain");
        Assertions.assertEquals("OK", live);
    }

    @SuppressWarnings("unchecked")
    @Test
    public void simulateHealthCheckFailureTest() throws AppException, IOException {
        MockCloud.setSimulateException(true);
        AppException ex = Assertions.assertThrows(AppException.class, () -> {
            SimpleHttpRequests.get("http://127.0.0.1:"+HTTP_PORT+"/health");
        });
        // failed health check is returned as HTTP-400
        Assertions.assertEquals(400, ex.getStatus());
        String response = ex.getMessage();
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        MultiLevelMap map = new MultiLevelMap(result);
        Assertions.assertEquals("DOWN", map.getElement("status"));
        Assertions.assertEquals("just a test", map.getElement("upstream[0].message"));
        // original status code from upstream service is preserved
        Assertions.assertEquals(500, map.getElement("upstream[0].status_code"));
        Assertions.assertEquals("mock.connector", map.getElement("upstream[0].service"));
        // livenessProbe is linked to health check
        AppException live = Assertions.assertThrows(AppException.class, () -> {
            SimpleHttpRequests.get("http://127.0.0.1:"+HTTP_PORT+"/livenessprobe", "text/plain");
        });
        Assertions.assertEquals(400, live.getStatus());
        Assertions.assertEquals("Unhealthy. Please check '/health' endpoint.", live.getMessage());
        MockCloud.setSimulateException(false);
        // try it again
        SimpleHttpRequests.get("http://127.0.0.1:"+HTTP_PORT+"/health");
        String liveAgain = SimpleHttpRequests.get("http://127.0.0.1:"+HTTP_PORT+"/livenessprobe", "text/plain");
        Assertions.assertEquals("OK", liveAgain);

    }

    @SuppressWarnings("unchecked")
    @Test
    public void remoteHealthEndpointTest() {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        AppException ex = Assertions.assertThrows(AppException.class, () ->
                                    SimpleHttpRequests.get("http://127.0.0.1:"+ HTTP_PORT +"/health", headers));
        String error = ex.getMessage();
        Assertions.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assertions.assertEquals(404, result.get("status"));
        Assertions.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void remoteLivenessEndpointTest() {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        AppException ex = Assertions.assertThrows(AppException.class, () ->
                                SimpleHttpRequests.get("http://127.0.0.1:"+ HTTP_PORT +"/livenessprobe", headers));
        String error = ex.getMessage();
        Assertions.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assertions.assertEquals(404, result.get("status"));
        Assertions.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void envEndpointTest() throws AppException, IOException {
        String response = SimpleHttpRequests.get("http://127.0.0.1:"+HTTP_PORT+"/env");
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        MultiLevelMap multi = new MultiLevelMap(result);
        Assertions.assertEquals("platform-core", multi.getElement("app.name"));
        Assertions.assertTrue(multi.getElement("env") instanceof Map);
        Assertions.assertTrue(multi.getElement("routing.private") instanceof List);
        Assertions.assertTrue(multi.getElement("routing.public") instanceof List);
    }

    @SuppressWarnings("unchecked")
    @Test
    public void remoteEnvEndpointTest() {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        AppException ex = Assertions.assertThrows(AppException.class, () ->
                                    SimpleHttpRequests.get("http://127.0.0.1:"+ HTTP_PORT +"/env", headers));
        String error = ex.getMessage();
        Assertions.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assertions.assertEquals(404, result.get("status"));
        Assertions.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void shutdownUsingGetWillFail() {
        AppException ex = Assertions.assertThrows(AppException.class, () ->
                                    SimpleHttpRequests.get("http://127.0.0.1:"+ HTTP_PORT +"/shutdown"));
        String error = ex.getMessage();
        Assertions.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assertions.assertEquals(404, result.get("status"));
        Assertions.assertEquals("Resource not found", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void suspendUsingGetWillFail() {
        AppException ex = Assertions.assertThrows(AppException.class, () ->
                                    SimpleHttpRequests.get("http://127.0.0.1:"+ HTTP_PORT +"/suspend/now"));
        String error = ex.getMessage();
        Assertions.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assertions.assertEquals(404, result.get("status"));
        Assertions.assertEquals("Resource not found", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void resumeUsingGetWillFail() {
        AppException ex = Assertions.assertThrows(AppException.class, () ->
                                SimpleHttpRequests.get("http://127.0.0.1:"+ HTTP_PORT +"/resume/now"));
        String error = ex.getMessage();
        Assertions.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assertions.assertEquals(404, result.get("status"));
        Assertions.assertEquals("Resource not found", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void shutdownWithoutAppInstanceWillFail() {
        AppException ex = Assertions.assertThrows(AppException.class, () ->
                                    SimpleHttpRequests.post("http://127.0.0.1:"+ HTTP_PORT +"/shutdown",
                                        new HashMap<>(), new HashMap<>()));
        String error = ex.getMessage();
        Assertions.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assertions.assertEquals(400, result.get("status"));
        Assertions.assertEquals("Missing X-App-Instance in request header", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void shutdownWithIncorrectAppInstanceWillFail() {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", "does-not-exist");
        AppException ex = Assertions.assertThrows(AppException.class, () ->
                                    SimpleHttpRequests.post("http://127.0.0.1:"+ HTTP_PORT +"/shutdown",
                                            headers, new HashMap<>()));
        String error = ex.getMessage();
        Assertions.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assertions.assertEquals(404, result.get("status"));
        Assertions.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void suspendWithoutAppInstanceWillFail() {
        AppException ex = Assertions.assertThrows(AppException.class, () ->
                                    SimpleHttpRequests.post("http://127.0.0.1:"+ HTTP_PORT +"/suspend/now",
                                            new HashMap<>(), new HashMap<>()));
        String error = ex.getMessage();
        Assertions.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assertions.assertEquals(400, result.get("status"));
        Assertions.assertEquals("Missing X-App-Instance in request header", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void suspendWithIncorrectAppInstanceWillFail() {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", "does-not-exist");
        AppException ex = Assertions.assertThrows(AppException.class, () ->
                                    SimpleHttpRequests.post("http://127.0.0.1:"+ HTTP_PORT +"/suspend/now",
                                            headers, new HashMap<>()));
        String error = ex.getMessage();
        Assertions.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assertions.assertEquals(404, result.get("status"));
        Assertions.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void resumeWithoutAppInstanceWillFail() {
        AppException ex = Assertions.assertThrows(AppException.class, () ->
                                    SimpleHttpRequests.post("http://127.0.0.1:"+ HTTP_PORT +"/resume/now",
                                            new HashMap<>(), new HashMap<>()));
        String error = ex.getMessage();
        Assertions.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assertions.assertEquals(400, result.get("status"));
        Assertions.assertEquals("Missing X-App-Instance in request header", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void resumeWithIncorrectAppInstanceWillFail() {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", "does-not-exist");
        AppException ex = Assertions.assertThrows(AppException.class, () ->
                                    SimpleHttpRequests.post("http://127.0.0.1:"+ HTTP_PORT +"/resume/now",
                                            headers, new HashMap<>()));
        String error = ex.getMessage();
        Assertions.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assertions.assertEquals(404, result.get("status"));
        Assertions.assertEquals("does-not-exist is not reachable", result.get("message"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void suspendTest() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", Platform.getInstance().getOrigin());
        String response = SimpleHttpRequests.post("http://127.0.0.1:"+HTTP_PORT+"/suspend/now", headers, new HashMap<>());
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        Assertions.assertEquals("suspend", result.get("type"));
        Assertions.assertEquals(200, result.get("status"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void resumeTest() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", Platform.getInstance().getOrigin());
        String response = SimpleHttpRequests.post("http://127.0.0.1:"+HTTP_PORT+"/resume/now", headers, new HashMap<>());
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        Assertions.assertEquals("resume", result.get("type"));
        Assertions.assertEquals(200, result.get("status"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void pageNotExists() {
        AppException ex = Assertions.assertThrows(AppException.class, () ->
                                    SimpleHttpRequests.get("http://127.0.0.1:"+ HTTP_PORT +"/no_such_page"));
        String error = ex.getMessage();
        Assertions.assertTrue(error.startsWith("{") && error.endsWith("}"));
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        Assertions.assertEquals(404, result.get("status"));
        Assertions.assertEquals("Resource not found", result.get("message"));
    }
}
