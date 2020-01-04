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

package org.platformlambda.core.util;

import org.junit.Test;

import static org.junit.Assert.*;

public class ServiceNameTest {

    private static final Utility util = Utility.getInstance();

    @Test
    public void filterName() {
        String valid = "hello.world";
        String invalid = "hello.wor?ld";
        String dotted = "..."+invalid;
        String filtered1 = util.filteredServiceName(invalid);
        assertEquals(valid, filtered1);
        String filtered2 = util.filteredServiceName(dotted);
        assertEquals(valid, filtered2);
    }

    @Test
    public void validName() {
        String windowsMetafile = "thumbs.db";
        String windowsExt = "hello.com";
        String valid = "com.hello";
        assertTrue(util.reservedFilename(windowsMetafile));
        assertTrue(util.reservedExtension(windowsExt));
        assertFalse(util.reservedExtension(valid));

    }

}
