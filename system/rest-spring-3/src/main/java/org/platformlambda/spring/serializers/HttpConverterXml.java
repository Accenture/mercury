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

package org.platformlambda.spring.serializers;

import org.jetbrains.annotations.NotNull;
import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.serializers.SimpleObjectMapper;
import org.platformlambda.core.serializers.SimpleXmlParser;
import org.platformlambda.core.serializers.SimpleXmlWriter;
import org.platformlambda.core.util.Utility;
import org.springframework.http.HttpInputMessage;
import org.springframework.http.HttpOutputMessage;
import org.springframework.http.MediaType;
import org.springframework.http.converter.HttpMessageConverter;
import org.springframework.http.converter.HttpMessageNotReadableException;
import org.springframework.http.converter.HttpMessageNotWritableException;
import org.springframework.lang.Nullable;

import java.io.IOException;
import java.io.OutputStream;
import java.nio.charset.StandardCharsets;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class HttpConverterXml implements HttpMessageConverter<Object> {
    private static final Utility util = Utility.getInstance();
    private static final SimpleXmlWriter map2xml = new SimpleXmlWriter();
    private static final SimpleXmlParser xml = new SimpleXmlParser();
    private static final MediaType XML_TYPE = new MediaType("application", "xml", StandardCharsets.UTF_8);
    private static final List<MediaType> types = Collections.singletonList(XML_TYPE);

    @Override
    public boolean canRead(@NotNull Class<?> clazz, @Nullable MediaType mediaType) {
        return mediaType != null && XML_TYPE.getType().equals(mediaType.getType())
                && XML_TYPE.getSubtype().equals(mediaType.getSubtype());
    }

    @Override
    public boolean canWrite(@NotNull Class<?> clazz, MediaType mediaType) {
        return mediaType != null && XML_TYPE.getType().equals(mediaType.getType())
                && XML_TYPE.getSubtype().equals(mediaType.getSubtype());
    }

    @NotNull
    @Override
    public List<MediaType> getSupportedMediaTypes() {
        return types;
    }

    @NotNull
    @Override
    public Object read(@NotNull Class<?> clazz, HttpInputMessage inputMessage) throws HttpMessageNotReadableException {
        // validate class with white list before loading the input stream
        SimpleMapper.getInstance().getSafeMapper(clazz);
        try {
            return xml.parse(inputMessage.getBody());
        } catch (IOException e) {
            throw new IllegalArgumentException(e.getMessage());
        }
    }

    @Override
    @SuppressWarnings("unchecked")
    public void write(Object o, MediaType contentType, HttpOutputMessage outputMessage)
            throws HttpMessageNotWritableException, IOException {
        outputMessage.getHeaders().setContentType(XML_TYPE);
        // this may be too late to validate because Spring RestController has already got the object
        SimpleObjectMapper mapper = SimpleMapper.getInstance().getSafeMapper(o.getClass().getTypeName());
        OutputStream out = outputMessage.getBody();
        if (o instanceof String text) {
            out.write(util.getUTF(text));
        } else if (o instanceof byte[] bytes) {
            out.write(bytes);
        } else {
            final String root;
            final Map<String, Object> map;
            if (o instanceof List) {
                root = "result";
                map = new HashMap<>();
                map.put("item", mapper.readValue(o, List.class));
            } else if (o instanceof Map) {
                root = "result";
                map = (Map<String, Object>) o;
            } else {
                root = o.getClass().getSimpleName().toLowerCase();
                map = mapper.readValue(o, Map.class);
            }
            String result = map2xml.write(root, map);
            out.write(util.getUTF(result));
        }
    }

}
