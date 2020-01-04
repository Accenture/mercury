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

import java.io.IOException;

import static org.junit.Assert.assertEquals;

public class StringConversionTest {
    private static final Utility util = Utility.getInstance();

    @Test
    public void base64() {
        String input = "hello world";
        String base64 = util.bytesToBase64(input.getBytes());
        byte[] b = util.base64ToBytes(base64);
        assertEquals(input, new String(b));
    }

    @Test
    public void hex() throws IOException {
        String input = "hello world";
        String hexString = util.bytes2hex(input.getBytes());
        byte[] b = util.hex2bytes(hexString);
        assertEquals(input, new String(b));
    }

}
