package org.platformlambda.automation.models;

import org.platformlambda.core.system.ObjectStreamIO;
import org.platformlambda.core.system.ObjectStreamWriter;

import java.io.IOException;

public class ObjectStreamWrapper {

    private final int expirySeconds;
    private ObjectStreamIO stream = null;
    private ObjectStreamWriter out = null;

    public ObjectStreamWrapper(int expirySeconds) {
        this.expirySeconds = expirySeconds;
    }

    public void write(byte[] b, int start, int end) throws IOException {
        if (out == null) {
            stream = new ObjectStreamIO(expirySeconds);
            out = stream.getOutputStream();
        }
        out.write(b, start, end);
    }

    public void write(byte[] b) throws IOException {
        if (out == null) {
            stream = new ObjectStreamIO(expirySeconds);
            out = stream.getOutputStream();
        }
        out.write(b);
    }

    public String getId() {
        return stream == null? null : stream.getRoute();
    }

    public void close() throws IOException {
        if (out != null) {
            out.close();
        }
    }

}
