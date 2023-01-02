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

package org.platformlambda.core;

import org.junit.Assert;
import org.junit.BeforeClass;
import org.junit.Test;
import org.platformlambda.core.util.CryptoApi;
import org.platformlambda.core.util.Utility;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.security.GeneralSecurityException;
import java.security.KeyPair;
import java.security.PrivateKey;
import java.security.PublicKey;

public class CryptoTest {
    private static final Logger log = LoggerFactory.getLogger(CryptoTest.class);

    private static final CryptoApi crypto = new CryptoApi();
    private static boolean strongCrypto;

    @BeforeClass
    public static void checkCrypto() {
        strongCrypto = crypto.strongCryptoSupported();
        if (!strongCrypto) {
            log.warn("Not using Java Cryptography Extension (JCE) Unlimited Strength Jurisdiction Policy");
            log.info("AES-128 supported");
        } else {
            log.info("Using Java Cryptography Extension (JCE) Unlimited Strength Jurisdiction Policy");
            log.info("AES-256 supported");
        }
    }

    @Test
    public void aesEncryptionTest() throws IOException, GeneralSecurityException {
        String input = "hello world";
        byte[] key = crypto.generateAesKey(strongCrypto? 256 : 128);
        byte[] encrypted = crypto.aesEncrypt(input.getBytes(), key);
        byte[] decrypted = crypto.aesDecrypt(encrypted, key);
        Assert.assertEquals(input, new String(decrypted));
        // streaming methods
        ByteArrayInputStream clearIn = new ByteArrayInputStream(input.getBytes());
        ByteArrayOutputStream encryptedOut = new ByteArrayOutputStream();
        crypto.aesEncrypt(clearIn, encryptedOut, key);
        encrypted = encryptedOut.toByteArray();
        ByteArrayInputStream encryptedIn = new ByteArrayInputStream(encrypted);
        ByteArrayOutputStream clearOut = new ByteArrayOutputStream();
        crypto.aesDecrypt(encryptedIn, clearOut, key);
        decrypted = clearOut.toByteArray();
        Assert.assertEquals(input, new String(decrypted));
    }

    @Test
    public void rsaEncryptionTest() throws GeneralSecurityException {
        // RSA encryption is usually used to transport symmetric encryption key
        byte[] input = crypto.generateAesKey(256);
        KeyPair kp = crypto.generateRsaKey();
        byte[] pub = kp.getPublic().getEncoded();
        byte[] pri = kp.getPrivate().getEncoded();
        // encrypt
        byte[] encrypted = crypto.rsaEncrypt(input, pub);
        // decrypt
        byte[] decrypted = crypto.rsaDecrypt(encrypted, pri);
        // do a byte-by-byte comparison
        Assert.assertArrayEquals(input, decrypted);
    }

    @Test
    public void invalidRsaKeyLength() {
        IllegalArgumentException ex = Assert.assertThrows(IllegalArgumentException.class,
                                            () -> crypto.generateRsaKey(1000));
        Assert.assertEquals("Key size must be one of [1024, 2048, 3072, 4096]", ex.getMessage());
    }

    @Test
    public void pemReadWriteTest() {
        KeyPair kp = crypto.generateRsaKey();
        byte[] pub = kp.getPublic().getEncoded();
        String pem = crypto.writePem(pub, "PUBLIC KEY");
        byte[] restored = crypto.readPem(pem);
        String pemRestored = crypto.writePem(restored, "PUBLIC KEY");
        Assert.assertEquals(pem, pemRestored);
    }

    @Test
    public void randomIntegerTest() {
        int n1 = crypto.nextInt(10000);
        int n2 = crypto.nextInt(10000);
        Assert.assertNotEquals(n1, n2);
    }

    @Test
    public void publicPrivateKeyEncodingTest() throws GeneralSecurityException {
        KeyPair kp = crypto.generateRsaKey();
        byte[] pub = kp.getPublic().getEncoded();
        byte[] pri = kp.getPrivate().getEncoded();
        PublicKey publicKey = crypto.getPublic(pub);
        PrivateKey privateKey = crypto.getPrivate(pri);
        Assert.assertArrayEquals(pub, publicKey.getEncoded());
        Assert.assertArrayEquals(pri, privateKey.getEncoded());
        byte[] pubAgain = crypto.getEncodedPublicKey(kp);
        byte[] priAgain = crypto.getEncodedPrivateKey(kp);
        Assert.assertArrayEquals(pubAgain, publicKey.getEncoded());
        Assert.assertArrayEquals(priAgain, privateKey.getEncoded());
    }

    @Test
    public void dsaSignatureTest() throws GeneralSecurityException {
        KeyPair kp = crypto.generateDsaKey();
        byte[] pub = kp.getPublic().getEncoded();
        byte[] pri = kp.getPrivate().getEncoded();
        byte[] data = "hello world".getBytes();
        byte[] signature = crypto.dsaSign(data, pri);
        boolean result = crypto.dsaVerify(data, signature, pub);
        Assert.assertTrue(result);
    }

    @Test
    public void rsaSignatureTest() throws GeneralSecurityException {
        KeyPair kp = crypto.generateRsaKey();
        byte[] pub = kp.getPublic().getEncoded();
        byte[] pri = kp.getPrivate().getEncoded();
        byte[] data = "hello world".getBytes();
        byte[] signature = crypto.rsaSign(data, pri);
        boolean result = crypto.rsaVerify(data, signature, pub);
        Assert.assertTrue(result);
    }

    @Test
    public void hashTest() throws IOException {
        String input = "hello world";
        byte[] hashed = crypto.getSHA256(input.getBytes());
        Assert.assertEquals(32, hashed.length);
        byte[] hashedFromStream = crypto.getSHA256(new ByteArrayInputStream(input.getBytes()));
        Assert.assertArrayEquals(hashed, hashedFromStream);
        hashed = crypto.getSHA1(input.getBytes());
        Assert.assertEquals(20, hashed.length);
        hashedFromStream = crypto.getSHA1(new ByteArrayInputStream(input.getBytes()));
        Assert.assertArrayEquals(hashed, hashedFromStream);
        hashed = crypto.getMd5(input.getBytes());
        Assert.assertEquals(16, hashed.length);
        hashedFromStream = crypto.getMd5(new ByteArrayInputStream(input.getBytes()));
        Assert.assertArrayEquals(hashed, hashedFromStream);
    }

    @Test
    public void hmac1Test() {
        String expected = "8a3a84bcd0d0065e97f175d370447c7d02e00973";
        byte[] key = "hello".getBytes();
        byte[] message = "world".getBytes();
        byte[] b = crypto.getHmacSha1(key, message);
        Assert.assertEquals(expected, Utility.getInstance().bytes2hex(b));
    }

    @Test
    public void hmac256Test() {
        String expected = "f1ac9702eb5faf23ca291a4dc46deddeee2a78ccdaf0a412bed7714cfffb1cc4";
        byte[] key = "hello".getBytes();
        byte[] message = "world".getBytes();
        byte[] b = crypto.getHmacSha256(key, message);
        Assert.assertEquals(expected, Utility.getInstance().bytes2hex(b));
    }

}
