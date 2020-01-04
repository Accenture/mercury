package org.platformlambda.core.config;

import com.google.api.client.http.*;
import com.google.api.client.http.javanet.NetHttpTransport;
import org.platformlambda.core.annotations.BeforeApplication;
import org.platformlambda.core.annotations.OptionalService;
import org.platformlambda.core.models.EntryPoint;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.net.MalformedURLException;
import java.net.URL;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

@BeforeApplication(sequence = 0)
@OptionalService("app.config.client")
public class SetupConfig implements EntryPoint {
    private static final Logger log = LoggerFactory.getLogger(SetupConfig.class);

    private static final HttpTransport HTTP_TRANSPORT = new NetHttpTransport();
    private static final HttpRequestFactory factory = HTTP_TRANSPORT.createRequestFactory();

    private static final String APPLICATION_JSON = "application/json";
    private static final String SPRING_APPNAME = "spring.application.name";
    private static final String APPNAME = "application.name";
    private static final String MESSAGE = "message";
    private static final String CONFIG = "config";
    private static final String X_API_KEY = "x-api-key";
    private static final String HTTP = "http://";
    private static final String HTTPS = "https://";
    private static final long INTERVAL = 5000;

    @Override
    @SuppressWarnings("unchecked")
    public void start(String[] args) throws IOException {
        Utility util = Utility.getInstance();
        AppConfigReader reader = AppConfigReader.getInstance();
        String name = util.filteredServiceName(reader.getProperty(SPRING_APPNAME, reader.getProperty(APPNAME, "")));
        if (name.length() == 0) {
            log.error(SPRING_APPNAME+" or "+APPNAME+" is missing from application.properties");
            System.exit(-1);
        }
        String manager = reader.getProperty("app.config.endpoint");
        if (manager == null) {
            log.error("app.config.endpoint is not configured");
            System.exit(-1);
        }
        String apiKey = getApiKey();
        if (apiKey == null) {
            log.error("app.config.key is not configured");
            System.exit(-1);
        }
        URL url = null;
        if (manager.startsWith(HTTP) || manager.startsWith(HTTPS)) {
            try {
                url = new URL(manager);
            } catch (MalformedURLException e) {
                log.error("app.config.endpoint is not a valid URL - {}", e.getMessage());
                System.exit(-1);
            }
        } else {
            log.error("app.config.endpoint is not a valid http or https URL");
        }
        int n = 0;
        long t0 = 0;
        while (true) {
            long now = System.currentTimeMillis();
            if (now - t0 > INTERVAL) {
                n++;
                t0 = now;
                String host = url.getHost();
                int port = url.getPort();
                if (util.portReady(host, port, 5000)) {
                    break;
                }
                log.info("Finding config manager at {} ...{}", url, n);
            }
            try {
                Thread.sleep(1000);
            } catch (InterruptedException e) {
                // ok to ignore
            }
        }
        String appPropUrl = manager+"/application.properties/"+name;
        Map<String, Object> appParam = downloadParameters(appPropUrl, apiKey);
        processAppProperties(appParam);
    }

    private String getApiKey() {
        Utility util = Utility.getInstance();
        AppConfigReader reader = AppConfigReader.getInstance();
        List<String> keyList = util.split(reader.getProperty("app.config.key", "no-keys"), ", ");
        if (!keyList.isEmpty()) {
            String k = keyList.get(0);
            int colon = k.indexOf(':');
            if (colon > 0) {
                String id = k.substring(0, colon);
                String value = k.substring(colon + 1);
                if (value.length() > 0) {
                    log.info("Using api.config.key {}", id);
                    return value;
                }
            }
        }
        return null;
    }

    @SuppressWarnings("unchecked")
    private void processAppProperties(Map<String, Object> map) {
        if (map.containsKey(CONFIG)) {
            Map<String, Object> config = (Map<String, Object>) map.get(CONFIG);
            for (String k: config.keySet()) {
                log.info("Override {}", k);
                System.setProperty(k, config.get(k).toString());
            }
        }
    }

    @SuppressWarnings("unchecked")
    private Map<String, Object> downloadParameters(String url, String apiKey) throws IOException {
        Utility util = Utility.getInstance();
        GenericUrl target = new GenericUrl(url);
        HttpHeaders httpHeaders = new HttpHeaders().setAccept(APPLICATION_JSON);
        httpHeaders.set(X_API_KEY, apiKey);
        HttpRequest request = factory.buildGetRequest(target).setHeaders(httpHeaders);
        try {
            HttpResponse response = request.execute();
            int rc = response.getStatusCode();
            String resBody = util.stream2str(response.getContent());
            response.disconnect();
            if (rc == 200) {
                return SimpleMapper.getInstance().getMapper().readValue(resBody, Map.class);
            }
        } catch (HttpResponseException e) {
            log.error("Unable to download config parameters. HTTP-{}, {}", e.getStatusCode(), getErrorMessage(e));
        }
        return new HashMap<>();
    }

    @SuppressWarnings("unchecked")
    private String getErrorMessage(HttpResponseException e) {
        String error = e.getContent();
        Map<String, Object> map = SimpleMapper.getInstance().getMapper().readValue(error, Map.class);
        if (map.containsKey(MESSAGE)) {
            return (String) map.get(MESSAGE);
        } else {
            return error;
        }
    }

}
