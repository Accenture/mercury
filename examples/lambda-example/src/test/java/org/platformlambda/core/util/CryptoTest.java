/*

    Copyright 2018 Accenture Technology

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
import java.security.GeneralSecurityException;
import java.security.KeyPair;

import static org.junit.Assert.assertEquals;

public class CryptoTest {

    private static final CryptoApi crypto = new CryptoApi();

    @Test
    public void aesEncryptionTest() throws IOException, GeneralSecurityException {
        String input = "hello world";
        byte[] key = crypto.generateAesKey(128);
        byte[] encrypted = crypto.aesEncrypt(input.getBytes(), key);
        byte[] decrypted = crypto.aesDecrypt(encrypted, key);
        assertEquals(input, new String(decrypted));
    }

    @Test
    public void rsaEncryptionTest() throws GeneralSecurityException {
        String input = "hello world ";
        for (int i=0; i < 10; i++) {
            input += "0123456789";
        }
        KeyPair kp = crypto.generateRsaKey();
        byte[] pub = kp.getPublic().getEncoded();
        byte[] pri = kp.getPrivate().getEncoded();
        // encrypt
        byte[] encrypted = crypto.rsaEncrypt(input.getBytes(), pub);
        // decrypt
        byte[] decrypted = crypto.rsaDecrypt(encrypted, pri);
        assertEquals(input, new String(decrypted));
    }

    @Test
    public void hashTest() {
        String input = "hello world";
        byte[] hashed = crypto.getSHA256(input.getBytes());
        assertEquals(32, hashed.length);
        hashed = crypto.getSHA1(input.getBytes());
        assertEquals(20, hashed.length);
        hashed = crypto.getMd5(input.getBytes());
        assertEquals(16, hashed.length);
    }
}
