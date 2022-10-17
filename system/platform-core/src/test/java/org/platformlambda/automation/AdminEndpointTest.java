package org.platformlambda.automation;

import org.junit.Assert;
import org.junit.Test;
import org.platformlambda.automation.util.SimpleHttpRequests;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.mock.TestBase;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.serializers.SimpleXmlParser;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.util.MultiLevelMap;
import org.platformlambda.core.util.Utility;

import java.io.IOException;
import java.io.InputStream;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class AdminEndpointTest extends TestBase {

    private static final SimpleXmlParser xmlParser = new SimpleXmlParser();

    @SuppressWarnings("unchecked")
    @Test
    public void infoEndpointTest() throws AppException, IOException {
        String response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/info");
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        MultiLevelMap multi = new MultiLevelMap(result);
        Assert.assertEquals("platform-core", multi.getElement("app.name"));
        Assert.assertEquals("REST", multi.getElement("personality"));
        String origin = Platform.getInstance().getOrigin();
        Assert.assertEquals(origin, multi.getElement("origin"));
    }

    @Test
    public void infoEndpointXmlTest() throws AppException, IOException {
        String response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/info", "application/xml");
        Map<String, Object> result = xmlParser.parse(response);
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
        SimpleHttpRequests.get("http://127.0.0.1:"+port+"/info", headers);
    }

    @SuppressWarnings("unchecked")
    @Test
    public void libEndpointTest() throws AppException, IOException {
        String response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/info/lib");
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        MultiLevelMap multi = new MultiLevelMap(result);
        Assert.assertEquals("platform-core", multi.getElement("app.name"));
        Assert.assertTrue(result.containsKey("library"));
    }

    @Test(expected = AppException.class)
    public void remoteLibEndpointTest() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        SimpleHttpRequests.get("http://127.0.0.1:"+port+"/info/lib", headers);
    }

    @SuppressWarnings("unchecked")
    @Test
    public void routeEndpointTest() throws AppException, IOException {
        String response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/info/routes");
        Map<String, Object> data = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        Assert.assertTrue(data.get("routing") instanceof Map);
        Map<String, Object> routing = (Map<String, Object>) data.get("routing");
        Assert.assertEquals(new HashMap<>(), routing.get("routes"));
    }

    @Test(expected = AppException.class)
    public void remoteRouteEndpointTest() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        SimpleHttpRequests.get("http://127.0.0.1:"+port+"/info/routes", headers);
    }

    @SuppressWarnings("unchecked")
    @Test
    public void healthEndpointTest() throws AppException, IOException {
        String response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/health");
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        Assert.assertEquals("UP", result.get("status"));
    }

    @Test(expected = AppException.class)
    public void remoteHealthEndpointTest() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        SimpleHttpRequests.get("http://127.0.0.1:"+port+"/health", headers);
    }

    @Test
    public void livenessEndpointTest() throws AppException, IOException {
        Object response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/livenessprobe", "text/plain");
        Assert.assertEquals("OK", response);
    }

    @Test(expected = AppException.class)
    public void remoteLivenessEndpointTest() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("x-app-instance", "does-not-exist");
        SimpleHttpRequests.get("http://127.0.0.1:"+port+"/livenessprobe", headers);
    }

    @SuppressWarnings("unchecked")
    @Test
    public void envEndpointTest() throws AppException, IOException {
        String response = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/env");
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
        SimpleHttpRequests.get("http://127.0.0.1:"+port+"/env", headers);
    }

    @Test
    public void shutdownUsingGetWillFail() {
        try {
            SimpleHttpRequests.get("http://127.0.0.1:" + port + "/shutdown/now");
        } catch (Exception e) {
            Assert.assertTrue(e instanceof AppException);
            AppException ex = (AppException) e;
            Assert.assertEquals(404, ex.getStatus());
        }
    }

    @Test
    public void suspendUsingGetWillFail() {
        try {
            SimpleHttpRequests.get("http://127.0.0.1:"+port+"/suspend/now");
        } catch (Exception e) {
            Assert.assertTrue(e instanceof AppException);
            AppException ex = (AppException) e;
            Assert.assertEquals(404, ex.getStatus());
        }
    }

    @Test
    public void resumeUsingGetWillFail() {
        try {
            SimpleHttpRequests.get("http://127.0.0.1:"+port+"/resume/now");
        } catch (Exception e) {
            Assert.assertTrue(e instanceof AppException);
            AppException ex = (AppException) e;
            Assert.assertEquals(404, ex.getStatus());
        }
    }

    @Test
    public void shutdownWithoutAppInstanceWillFail() {
        try {
            SimpleHttpRequests.post("http://127.0.0.1:"+port+"/shutdown", new HashMap<>(), new HashMap<>());
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
        SimpleHttpRequests.post("http://127.0.0.1:"+port+"/shutdown", headers, new HashMap<>());
    }

    @Test(expected = AppException.class)
    public void suspendWithoutAppInstanceWillFail() throws AppException, IOException {
        SimpleHttpRequests.post("http://127.0.0.1:"+port+"/suspend/now", new HashMap<>(), new HashMap<>());
    }

    @Test(expected = AppException.class)
    public void suspendWithIncorrectAppInstanceWillFail() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", "does-not-exist");
        SimpleHttpRequests.post("http://127.0.0.1:"+port+"/suspend/now", headers, new HashMap<>());
    }

    @Test(expected = AppException.class)
    public void suspendWithOutAppInstanceWillFail() throws AppException, IOException {
        SimpleHttpRequests.post("http://127.0.0.1:"+port+"/suspend/now", new HashMap<>(), new HashMap<>());
    }

    @Test(expected = AppException.class)
    public void resumeWithoutAppInstanceWillFail() throws AppException, IOException {
        SimpleHttpRequests.post("http://127.0.0.1:"+port+"/resume/now", new HashMap<>(), new HashMap<>());
    }

    @Test(expected = AppException.class)
    public void resumeWithIncorrectAppInstanceWillFail() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", "does-not-exist");
        SimpleHttpRequests.post("http://127.0.0.1:"+port+"/resume/now", headers, new HashMap<>());
    }

    @SuppressWarnings("unchecked")
    @Test
    public void suspendTest() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", Platform.getInstance().getOrigin());
        String response = SimpleHttpRequests.post("http://127.0.0.1:"+port+"/suspend/now", headers, new HashMap<>());
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        Assert.assertEquals("suspend", result.get("type"));
        Assert.assertEquals(200, result.get("status"));
    }

    @SuppressWarnings("unchecked")
    @Test
    public void resumeTest() throws AppException, IOException {
        Map<String, String> headers = new HashMap<>();
        headers.put("X-App-Instance", Platform.getInstance().getOrigin());
        String response = SimpleHttpRequests.post("http://127.0.0.1:"+port+"/resume/now", headers, new HashMap<>());
        Map<String, Object> result = SimpleMapper.getInstance().getMapper().readValue(response, Map.class);
        Assert.assertEquals("resume", result.get("type"));
        Assert.assertEquals(200, result.get("status"));
    }

    @Test
    public void getIndexPage() throws AppException, IOException {
        String text = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/");
        InputStream in = this.getClass().getResourceAsStream("/public/index.html");
        String css = Utility.getInstance().stream2str(in);
        Assert.assertEquals(css, text);
    }

    @Test
    public void getCssPage() throws AppException, IOException {
        String text = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/sample.css");
        InputStream in = this.getClass().getResourceAsStream("/public/sample.css");
        String css = Utility.getInstance().stream2str(in);
        Assert.assertEquals(css, text);
    }

    @Test
    public void getTextPage() throws AppException, IOException {
        String text = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/sample.txt");
        InputStream in = this.getClass().getResourceAsStream("/public/sample.txt");
        String txt = Utility.getInstance().stream2str(in);
        Assert.assertEquals(txt, text);
    }

    @Test
    public void getJsPage() throws AppException, IOException {
        String text = SimpleHttpRequests.get("http://127.0.0.1:"+port+"/sample.js");
        InputStream in = this.getClass().getResourceAsStream("/public/sample.js");
        String js = Utility.getInstance().stream2str(in);
        Assert.assertEquals(js, text);
    }

    @Test(expected = AppException.class)
    public void pageNotExists() throws AppException, IOException {
        SimpleHttpRequests.get("http://127.0.0.1:"+port+"/no_such_page");
    }
}
