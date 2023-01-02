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

package org.platformlambda.spring.serializers;

import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.serializers.SimpleObjectMapper;
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
import java.util.List;

public class HttpConverterJson implements HttpMessageConverter<Object> {

    private static final Utility util = Utility.getInstance();
    private static final MediaType JSON = new MediaType("application", "json", StandardCharsets.UTF_8);
    private static final List<MediaType> types = Collections.singletonList(JSON);

    @Override
    public boolean canRead(Class<?> clazz, @Nullable MediaType mediaType) {
        return mediaType != null && JSON.getType().equals(mediaType.getType())
                && JSON.getSubtype().equals(mediaType.getSubtype());
    }

    @Override
    public boolean canWrite(Class<?> clazz, @Nullable MediaType mediaType) {
        return mediaType != null && JSON.getType().equals(mediaType.getType())
                && JSON.getSubtype().equals(mediaType.getSubtype());
    }

    @Override
    public List<MediaType> getSupportedMediaTypes() {
        return types;
    }

    @Override
    public Object read(Class<?> clazz, HttpInputMessage inputMessage) throws HttpMessageNotReadableException {
        try {
            // validate class with white list before loading the input stream
            SimpleObjectMapper mapper = SimpleMapper.getInstance().getSafeMapper(clazz);
            String input = util.stream2str(inputMessage.getBody());
            return mapper.readValue(input, clazz);
        } catch (IOException e) {
            throw new IllegalArgumentException(e.getMessage());
        }
    }

    @Override
    public void write(Object o, @Nullable MediaType contentType, HttpOutputMessage outputMessage)
            throws HttpMessageNotWritableException, IOException {
        outputMessage.getHeaders().setContentType(JSON);
        // this may be too late to validate because Spring RestController has already got the object
        SimpleObjectMapper mapper = SimpleMapper.getInstance().getSafeMapper(o.getClass().getTypeName());
        OutputStream out = outputMessage.getBody();
        if (o instanceof String) {
            out.write(util.getUTF((String) o));
        } else if (o instanceof byte[]) {
            out.write((byte[]) o);
        } else {
            out.write(mapper.writeValueAsBytes(o));
        }
    }

}
