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

import org.jetbrains.annotations.NotNull;
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

public class HttpConverterHtml implements HttpMessageConverter<Object> {

    private static final Utility util = Utility.getInstance();
    private static final MediaType HTML_CONTENT = new MediaType("text", "html", StandardCharsets.UTF_8);
    private static final List<MediaType> types = Collections.singletonList(HTML_CONTENT);

    @Override
    public boolean canRead(@NotNull Class<?> clazz, @Nullable MediaType mediaType) {
        return mediaType != null && HTML_CONTENT.getType().equals(mediaType.getType())
                && (HTML_CONTENT.getSubtype().equals(mediaType.getSubtype()));
    }

    @Override
    public boolean canWrite(@NotNull Class<?> clazz, @Nullable MediaType mediaType) {
        return mediaType != null && HTML_CONTENT.getType().equals(mediaType.getType())
                && (HTML_CONTENT.getSubtype().equals(mediaType.getSubtype()));
    }

    @NotNull
    @Override
    public List<MediaType> getSupportedMediaTypes() {
        return types;
    }

    @NotNull
    @Override
    public Object read(@NotNull Class<?> clazz, HttpInputMessage inputMessage)
            throws HttpMessageNotReadableException, IOException {
        return util.getUTF(util.stream2bytes(inputMessage.getBody(), false));
    }

    @Override
    public void write(Object o, MediaType contentType, HttpOutputMessage outputMessage)
            throws HttpMessageNotWritableException, IOException {
        outputMessage.getHeaders().setContentType(HTML_CONTENT);
        // this may be too late to validate because Spring RestController has already got the object
        SimpleObjectMapper mapper = SimpleMapper.getInstance().getSafeMapper(o.getClass().getTypeName());
        OutputStream out = outputMessage.getBody();
        if (o instanceof String text) {
            out.write(util.getUTF(text));
        } else if (o instanceof byte[] bytes) {
            out.write(bytes);
        } else {
            out.write(util.getUTF("<html><body><pre>\n"));
            out.write(mapper.writeValueAsBytes(o));
    		out.write(util.getUTF("\n</pre></body></html>"));
        }
    }

}
