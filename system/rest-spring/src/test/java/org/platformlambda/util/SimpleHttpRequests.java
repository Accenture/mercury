package org.platformlambda.util;

import com.google.api.client.http.*;
import com.google.api.client.http.javanet.NetHttpTransport;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.util.Utility;

import java.io.IOException;
import java.io.InputStream;
import java.util.Map;

public class SimpleHttpRequests {

    private static final HttpTransport HTTP_TRANSPORT = new NetHttpTransport();
    private static final HttpRequestFactory factory = HTTP_TRANSPORT.createRequestFactory();
    private static final String APPLICATION_JSON = "application/json";

    public static Object put(String url, Map<String, Object> data) throws IOException, AppException {

        SimpleMapper mapper = SimpleMapper.getInstance();
        String json = mapper.getMapper().writeValueAsString(data);

        ByteArrayContent content = ByteArrayContent.fromString(APPLICATION_JSON, json);
        GenericUrl target = new GenericUrl(url);
        HttpHeaders headers = new HttpHeaders();
        headers.setAccept(APPLICATION_JSON);
        HttpRequest request = factory.buildPutRequest(target, content).setHeaders(headers);
        try {
            HttpResponse response = request.execute();
            InputStream in = response.getContent();
            return Utility.getInstance().stream2str(in);
        } catch (HttpResponseException e) {
            throw new AppException(e.getStatusCode(), e.getContent());
        }
    }

}
