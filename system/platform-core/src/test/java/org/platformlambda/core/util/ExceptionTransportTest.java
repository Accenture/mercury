package org.platformlambda.core.util;

import org.junit.Assert;
import org.junit.Test;
import org.platformlambda.core.exception.AppException;

import java.io.*;

public class ExceptionTransportTest {

    @Test
    public void transportTest() throws IOException, ClassNotFoundException {
        int STATUS = 500;
        String MESSAGE1 = "hello world 1";
        String MESSAGE2 = "hello world 2";
        IOException e = new IOException(MESSAGE1);
        AppException ae = new AppException(STATUS, MESSAGE2, e);
        ByteArrayOutputStream out = new ByteArrayOutputStream();
        try (ObjectOutputStream stream = new ObjectOutputStream(out)) {
            stream.writeObject(ae);
        }
        try (ObjectInputStream in = new ObjectInputStream(new ByteArrayInputStream(out.toByteArray()))) {
            AppException restored = (AppException) in.readObject();
            Assert.assertEquals(STATUS, restored.getStatus());
            Assert.assertEquals(MESSAGE2, restored.getMessage());
            Assert.assertTrue(restored.getCause() instanceof IOException);
            IOException ioe = (IOException) restored.getCause();
            Assert.assertEquals(MESSAGE1, ioe.getMessage());
        }
    }

}
