package org.platformlambda.core.util;

import org.junit.Test;

import java.io.IOException;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;

public class ServiceNameTest {

    private static final Utility util = Utility.getInstance();

    @Test
    public void filterName() throws IOException {
        String valid = "hello.world";
        String invalid = "hello.wor?ld";
        String dotted = "..."+invalid;
        String filtered1 = util.filteredServiceName(invalid);
        assertEquals(valid, filtered1);
        String filtered2 = util.filteredServiceName(dotted);
        assertEquals(valid, filtered2);
    }

    @Test
    public void validName() throws IOException {
        String windowsMetafile = "thumbs.db";
        String windowsExt = "hello.com";
        String valid = "com.hello";
        assertTrue(util.reservedFilename(windowsMetafile));
        assertTrue(util.reservedExtension(windowsExt));
        assertEquals(false, util.reservedExtension(valid));

    }

}
