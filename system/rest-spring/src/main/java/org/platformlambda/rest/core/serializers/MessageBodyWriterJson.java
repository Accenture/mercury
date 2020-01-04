/*

    Copyright 2018-2020 Accenture Technology

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

package org.platformlambda.rest.core.serializers;

import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.util.Utility;

import javax.ws.rs.Produces;
import javax.ws.rs.WebApplicationException;
import javax.ws.rs.core.MediaType;
import javax.ws.rs.core.MultivaluedMap;
import javax.ws.rs.ext.MessageBodyWriter;
import javax.ws.rs.ext.Provider;
import java.io.IOException;
import java.io.OutputStream;
import java.lang.annotation.Annotation;
import java.lang.reflect.Type;

@Provider
@Produces(MediaType.APPLICATION_JSON)
public class MessageBodyWriterJson<T> implements MessageBodyWriter<T> {

    private static final Utility util = Utility.getInstance();

    @Override
    public boolean isWriteable(Class<?> cls, Type genericType, Annotation[] annotations, MediaType mediaType) {
        return true;
    }

    @Override
    public long getSize(T t, Class<?> cls, Type genericType, Annotation[] annotations, MediaType mediaType) {
        return 0;
    }

    @Override
    public void writeTo(T t, Class<?> cls, Type genericType, Annotation[] annotations, MediaType mediaType, MultivaluedMap<String, Object> httpHeaders, OutputStream entityStream) throws IOException, WebApplicationException {
        if (t instanceof String) {
            entityStream.write(util.getUTF((String) t));
        } else if (t instanceof byte[]) {
            entityStream.write((byte[]) t);
        } else {
            entityStream.write(util.getUTF(SimpleMapper.getInstance().getWhiteListMapper(cls).writeValueAsString(t)));
        }
    }
}
