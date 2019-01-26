/*

    Copyright 2018-2019 Accenture Technology

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

package org.platformlambda.rest.spring.serializers;

import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.serializers.SimpleXmlParser;
import org.platformlambda.core.serializers.SimpleXmlWriter;
import org.platformlambda.core.util.Utility;
import org.springframework.http.HttpInputMessage;
import org.springframework.http.HttpOutputMessage;
import org.springframework.http.MediaType;
import org.springframework.http.converter.HttpMessageConverter;
import org.springframework.http.converter.HttpMessageNotReadableException;
import org.springframework.http.converter.HttpMessageNotWritableException;

import java.io.IOException;
import java.io.OutputStream;
import java.nio.charset.Charset;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class HttpConverterXml implements HttpMessageConverter<Object> {
    private static final Utility util = Utility.getInstance();
    private static final SimpleXmlWriter map2xml = new SimpleXmlWriter();
    private static final MediaType XML = new MediaType("application", "xml", Charset.forName("UTF-8"));
    private static List<MediaType> types = new ArrayList<>();

    @Override
    public boolean canRead(Class<?> clazz, MediaType mediaType) {
        return mediaType != null && XML.getType().equals(mediaType.getType())
                && XML.getSubtype().equals(mediaType.getSubtype());
    }

    @Override
    public boolean canWrite(Class<?> clazz, MediaType mediaType) {
        return mediaType != null && XML.getType().equals(mediaType.getType())
                && XML.getSubtype().equals(mediaType.getSubtype());
    }

    @Override
    public List<MediaType> getSupportedMediaTypes() {
        if (types.isEmpty()) {
            types.add(XML);
        }
        return types;
    }

    @Override
    public Object read(Class<?> clazz, HttpInputMessage inputMessage) throws HttpMessageNotReadableException {
        if (inputMessage != null) {
            try {
                SimpleXmlParser xml = new SimpleXmlParser();
                Map<String, Object> map = xml.parse(inputMessage.getBody());
                for (String key : map.keySet()) {
                    Object o = map.get(key);
                    if (o instanceof Map) {
                        return o;
                    }
                }
                return new HashMap<String, Object>();
            } catch (IOException e) {
                throw new IllegalArgumentException(e.getMessage());
            }
        } else {
            return null;
        }
    }

    @Override
    @SuppressWarnings("unchecked")
    public void write(Object o, MediaType contentType, HttpOutputMessage outputMessage) throws HttpMessageNotWritableException, IOException {
        outputMessage.getHeaders().setContentType(XML);
        OutputStream out = outputMessage.getBody();
        if (o instanceof String) {
            out.write(util.getUTF((String) o));
        } else if (o instanceof byte[]) {
            out.write((byte[]) o);
        } else {
            Map<String, Object> map;
            if (o instanceof List) {
                map = new HashMap<>();
                map.put("item", SimpleMapper.getInstance().getMapper().convertValue(o, List.class));
            } else if (o instanceof Map) {
                map = (Map<String, Object>) o;
            } else {
                map = SimpleMapper.getInstance().getMapper().convertValue(o, Map.class);
            }
            String root = o.getClass().getSimpleName().toLowerCase();
            String result = map2xml.write(root, map);
            out.write(util.getUTF(result));
        }
    }

}
