package org.platformlambda.rest.core.serializers;

import org.platformlambda.core.serializers.SimpleMapper;
import org.platformlambda.core.util.Utility;

import javax.ws.rs.Consumes;
import javax.ws.rs.WebApplicationException;
import javax.ws.rs.core.MediaType;
import javax.ws.rs.core.MultivaluedMap;
import javax.ws.rs.ext.MessageBodyReader;
import javax.ws.rs.ext.Provider;
import java.io.IOException;
import java.io.InputStream;
import java.lang.annotation.Annotation;
import java.lang.reflect.Type;

@Provider
@Consumes(MediaType.APPLICATION_JSON)
public class MessageBodyReaderJson<T> implements MessageBodyReader<T> {

    @Override
    public boolean isReadable(Class<?> type, Type genericType, Annotation[] annotations, MediaType mediaType) {
        return true;
    }

    @Override
    public T readFrom(Class<T> type, Type genericType, Annotation[] annotations, MediaType mediaType, MultivaluedMap<String, String> httpHeaders, InputStream entityStream) throws WebApplicationException {
        if (entityStream != null) {
            byte[] json = Utility.getInstance().stream2bytes(entityStream, false);
            try {
                return SimpleMapper.getInstance().getMapper().readValue(json, type);
            } catch (IOException e) {
                throw new IllegalArgumentException(e.getMessage());
            }
        } else {
            return null;
        }
    }

}
