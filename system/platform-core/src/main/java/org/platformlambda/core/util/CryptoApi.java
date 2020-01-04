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

import javax.crypto.Cipher;
import javax.crypto.Mac;
import javax.crypto.spec.IvParameterSpec;
import javax.crypto.spec.SecretKeySpec;
import javax.net.ssl.SSLContext;
import javax.net.ssl.TrustManagerFactory;
import java.io.*;
import java.security.*;
import java.security.cert.CertificateFactory;
import java.security.cert.X509Certificate;
import java.security.spec.PKCS8EncodedKeySpec;
import java.security.spec.X509EncodedKeySpec;
import java.util.Arrays;
import java.util.List;
import java.util.concurrent.ThreadLocalRandom;

public class CryptoApi {

    public static final String PUBLIC = ".pub";
    public static final String PRIVATE = ".key";
    private static final int IV_LENGTH = 16;
    private static final SecureRandom random = new SecureRandom();
    private static final int BUFFER_SIZE = 1024;
    private static final String RSA = "RSA";
    private static final String DSA = "DSA";
    private static final String AES = "AES";
    private static final String MD5 = "MD5";
    private static final String SHA1 = "SHA1";
    private static final String SHA256 = "SHA-256";
    private static final String HMAC_SHA1 = "HmacSHA1";
    private static final String HMAC_SHA256 = "HmacSHA256";
    private static final int RSA_KEY_SIZE = 1024;   // or 2048
    private static final String SHA1_DSA = "SHA1withDSA"; // or SHA256withDSA
    private static final String SHA1_RSA = "SHA1withRSA"; // or SHA256withRSA
    private static final String AES_PADDING = "AES/CBC/PKCS5Padding";
    private static final String RSA_PADDING = "RSA/ECB/OAEPWithSHA1AndMGF1Padding";
    private static final String CORRUPTED_IV = "Corrupted IV";
    private static final String DASHES = "-----";
    private static final String BEGIN = "BEGIN";
    private static final String END = "END";

    /////////////////////////
    // RSA crypto utilities
    ////////////////////////

    /**
     * Generate RSA keypair
     * @return keypair
     */
    public KeyPair generateRsaKey() {
        try {
            KeyPairGenerator kg = KeyPairGenerator.getInstance(RSA);
            kg.initialize(RSA_KEY_SIZE);
            return KeyPairGenerator.getInstance(RSA).generateKeyPair();
        } catch (NoSuchAlgorithmException e) {
            // this does not happen
            return null;
        }
    }

    /**
     * Get private key bytes
     *
     * @param keyPair RSA-1024 or DSA-1024 key pair
     * @return PKCS-8 encoded byte array
     */
    public byte[] getEncodedPrivateKey(KeyPair keyPair) {
        return keyPair.getPrivate().getEncoded();
    }

    public PublicKey getPublic(byte[] publicKey) throws GeneralSecurityException {
        return getPublic(publicKey, true);
    }

    public PublicKey getPublic(byte[] publicKey, boolean rsa) throws GeneralSecurityException {
        return KeyFactory.getInstance(rsa? RSA : DSA).generatePublic(new X509EncodedKeySpec(publicKey));
    }

    public PrivateKey getPrivate(byte[] privateKey) throws GeneralSecurityException {
        return getPrivate(privateKey, true);
    }

    public PrivateKey getPrivate(byte[] privateKey, boolean rsa) throws GeneralSecurityException {
        return KeyFactory.getInstance(rsa? RSA : DSA).generatePrivate(new PKCS8EncodedKeySpec(privateKey));
    }

    /**
     * Get public key bytes
     *
     * @param keyPair RSA-1024 or DSA-1024 key pair
     * @return X509 encoded byte array
     */
    public byte[] getEncodedPublicKey(KeyPair keyPair) {
        return keyPair.getPublic().getEncoded();
    }

    public byte[] rsaEncrypt(byte[] clearText, byte[] publicKey) throws GeneralSecurityException {
        PublicKey key = getPublic(publicKey, true);
        Cipher cipher = Cipher.getInstance(RSA_PADDING);
        cipher.init(Cipher.ENCRYPT_MODE, key);
        return cipher.doFinal(clearText);
    }

    public byte[] rsaDecrypt(byte[] cipherText, byte[] privateKey) throws GeneralSecurityException {
        PrivateKey key = getPrivate(privateKey, true);
        Cipher cipher = Cipher.getInstance(RSA_PADDING);
        cipher.init(Cipher.DECRYPT_MODE, key);
        return cipher.doFinal(cipherText);
    }

    /**
     * Create a digital signature using a private key (RSA)
     * @param data bytes
     * @param privateKey encoded key
     * @return signature
     * @throws GeneralSecurityException in case of error
     */
    public byte[] rsaSign(byte[] data, byte[] privateKey) throws GeneralSecurityException {
        PrivateKey key = getPrivate(privateKey, true);
        Signature dsa = Signature.getInstance(SHA1_RSA);
        dsa.initSign(key);
        dsa.update(data);
        return dsa.sign();
    }

    /**
     * Verify a digital signature using a public key (RSA)
     * (Python PyCrypto compatibility tested
     *  example in https://www.dlitz.net/software/pycrypto/api/2.6/)
     *
     *  <code>
     *  Package Crypto :: Package Signature :: Module PKCS1_v1_5
     *
     *  from Crypto.Signature import PKCS1_v1_5 as pkcs
     *  from Crypto.Hash import SHA
     *  from Crypto.PublicKey import RSA
     *  message = 'To be signed'
     *  private_key = RSA.importKey(open('7c05b2b0353c4d40904204ecd274fdf2.pri').read())
     *  h = SHA.new(message)
     *  signer = pkcs.new(private_key)
     *  signature = signer.sign(h)
     *  </code>
     *
     * @param data bytes
     * @param signature bytes
     * @param publicKey encoded public key
     * @return true if signature is verified
     * @throws GeneralSecurityException when unable to generate key
     */
    public boolean rsaVerify(byte[] data, byte[] signature, byte[] publicKey) throws GeneralSecurityException {
        PublicKey key = getPublic(publicKey, true);
        Signature dsa = Signature.getInstance(SHA1_RSA);
        dsa.initVerify(key);
        dsa.update(data);
        return dsa.verify(signature);
    }

    /////////////////////////
    // DSA crypto utilities
    ////////////////////////

    public KeyPair generateDsaKey() {
        try {
            KeyPairGenerator kg = KeyPairGenerator.getInstance(DSA);
            // for compatibility with some scripting languages that do not support 2048 bit key length DSA
            kg.initialize(1024);
            return kg.generateKeyPair();
        } catch (NoSuchAlgorithmException e) {
            // this does not happen
            return null;
        }
    }

    /**
     * Create a digital signature using a private key (DSA)
     * @param data bytes
     * @param privateKey encoded key
     * @return signature
     * @throws GeneralSecurityException in case of error
     */
    public byte[] dsaSign(byte[] data, byte[] privateKey) throws GeneralSecurityException {
        PrivateKey key = getPrivate(privateKey, false);
        Signature dsa = Signature.getInstance(SHA1_DSA);
        dsa.initSign(key);
        dsa.update(data);
        return dsa.sign();
    }

    /**
     * Verify a digital signature using a public key (DSA)
     * @param data bytes
     * @param signature bytes
     * @param publicKey encoded public key
     * @return true if signature is verified
     * @throws GeneralSecurityException in case of error
     */
    public boolean dsaVerify(byte[] data, byte[] signature, byte[] publicKey) throws GeneralSecurityException {
        PublicKey key = getPublic(publicKey, false);
        Signature dsa = Signature.getInstance(SHA1_DSA);
        dsa.initVerify(key);
        dsa.update(data);
        return dsa.verify(signature);
    }

    /////////////////////////
    // AES crypto utilities
    ////////////////////////

    /**
     * Secure random for secret key generation
     * @param bits 128, 192 or 256
     * @return random bytes
     */
    public byte[] generateAesKey(int bits) {
        if (bits == 128 || bits == 192 || bits == 256) {
            byte[] key = new byte[bits / 8];
            random.nextBytes(key);
            return key;
        } else {
            throw new IllegalArgumentException("AES key length must be 128, 192 or 256 bits");
        }
    }

    public void aesEncrypt(InputStream clearIn, OutputStream encryptedOut, byte[] key) throws GeneralSecurityException, IOException {
        byte[] iv = getIv();
        SecretKeySpec secret = new SecretKeySpec(key, AES);
        Cipher cipher = Cipher.getInstance(AES_PADDING);
        cipher.init(Cipher.ENCRYPT_MODE, secret, new IvParameterSpec(iv));
        encryptedOut.write(iv);

        int len;
        byte[] buffer = new byte[BUFFER_SIZE];
        while ((len = clearIn.read(buffer, 0, buffer.length)) != -1) {
            byte[] encrypted = cipher.update(buffer, 0, len);
            encryptedOut.write(encrypted);
        }
        byte[] finalBlk = cipher.doFinal();
        if (finalBlk.length > 0) {
            encryptedOut.write(finalBlk);
        }
    }

    public void aesDecrypt(InputStream encryptedIn, OutputStream clearOut, byte[] key) throws GeneralSecurityException, IOException {
        byte[] iv = new byte[IV_LENGTH];
        int len = encryptedIn.read(iv);
        if (len < IV_LENGTH) {
            throw new IOException(CORRUPTED_IV);
        }
        SecretKeySpec secret = new SecretKeySpec(key, AES);
        Cipher cipher = Cipher.getInstance(AES_PADDING);
        cipher.init(Cipher.DECRYPT_MODE, secret, new IvParameterSpec(iv));

        byte[] buffer = new byte[BUFFER_SIZE];
        while ((len = encryptedIn.read(buffer, 0, buffer.length)) != -1) {
            byte[] decrypted = cipher.update(buffer, 0, len);
            clearOut.write(decrypted);
        }
        byte[] finalBlk = cipher.doFinal();
        if (finalBlk.length > 0) {
            clearOut.write(finalBlk);
        }
    }

    public byte[] aesEncrypt(byte[] clearText, byte[] key) throws GeneralSecurityException, IOException {
        byte[] iv = getIv();
        SecretKeySpec secret = new SecretKeySpec(key, AES);
        Cipher cipher = Cipher.getInstance(AES_PADDING);
        cipher.init(Cipher.ENCRYPT_MODE, secret, new IvParameterSpec(iv));
        byte[] encrypted = cipher.doFinal(clearText);
        // prepend random IV so encrypting the same text multiple times would generate different results
        ByteArrayOutputStream out = new ByteArrayOutputStream(encrypted.length);
        out.write(iv);
        out.write(encrypted);
        return out.toByteArray();
    }

    public byte[] aesDecrypt(byte[] encrypted, byte[] key) throws GeneralSecurityException, IOException {
        if (encrypted.length < IV_LENGTH) {
            throw new IOException(CORRUPTED_IV);
        }
        byte[] iv = Arrays.copyOfRange(encrypted, 0, IV_LENGTH);
        byte[] payload = Arrays.copyOfRange(encrypted, IV_LENGTH, encrypted.length);

        SecretKeySpec secret = new SecretKeySpec(key, AES);
        Cipher cipher = Cipher.getInstance(AES_PADDING);
        cipher.init(Cipher.DECRYPT_MODE, secret, new IvParameterSpec(iv));
        return cipher.doFinal(payload);
    }

    /**
     * IV does not need to be truly random
     * so we use the basic pseudo-random generator for performance reason.
     *
     * @return random value of 16 bytes
     */
    private byte[] getIv() {
        return getRandomBytes(IV_LENGTH);
    }

    /////////////////////
    // Hashing utilities
    /////////////////////

    public byte[] getMd5(byte[] data) {
        return getHash(data, MD5);
    }

    public byte[] getMd5(InputStream in) throws IOException {
        return getHash(in, MD5);
    }

    public byte[] getSHA1(byte[] data) {
        return getHash(data, SHA1);
    }

    public byte[] getSHA1(InputStream in) throws IOException {
        return getHash(in, SHA1);
    }

    public byte[] getSHA256(byte[] data) {
        return getHash(data, SHA256);
    }

    public byte[] getSHA256(InputStream in) throws IOException {
        return getHash(in, SHA256);
    }

    private byte[] getHash(byte[] data, String algorithm) {
        try {
            MessageDigest hash = MessageDigest.getInstance(algorithm);
            return hash.digest(data);
        } catch (NoSuchAlgorithmException e) {
            // this should not happen
            return null;
        }
    }

    private byte[] getHash(InputStream in, String algorithm) throws IOException {
        try {
            MessageDigest hash = MessageDigest.getInstance(algorithm);
            int len;
            byte[] buffer = new byte[BUFFER_SIZE];
            while ((len = in.read(buffer, 0, buffer.length)) != -1) {
                hash.update(buffer, 0, len);
            }
            return hash.digest();
        } catch (NoSuchAlgorithmException e) {
            // this should not happen
            return null;
        }
    }

    /////////////////////////////////////////////////////////////////
    // HMAC utilities - cross-verified with Python hmac and hashlib
    ////////////////////////////////////////////////////////////////

    public byte[] getHmacSha1(byte[] key, byte[] message) {
        return getHmac(key, message, HMAC_SHA1);
    }

    public byte[] getHmacSha256(byte[] key, byte[] message) {
        return getHmac(key, message, HMAC_SHA256);
    }

    private byte[] getHmac(byte[] key, byte[] message, String algorithm) {
        try {
            Mac mac = Mac.getInstance(algorithm);
            SecretKeySpec keySpec = new SecretKeySpec(key, algorithm);
            mac.init(keySpec);
            return mac.doFinal(message);

        } catch (NoSuchAlgorithmException | InvalidKeyException e) {
            // this should not happen
            return null;
        }
    }

    ///////////////////
    // Other utilities
    ///////////////////

    public String writePem(byte[] key, String type) {
        StringBuilder sb = new StringBuilder();
        sb.append(DASHES);
        sb.append(BEGIN);
        sb.append(' ');
        sb.append(type.toUpperCase());
        sb.append(DASHES);
        sb.append("\r\n");
        sb.append(Utility.getInstance().bytesToBase64(key, true, false));
        sb.append(DASHES);
        sb.append(END);
        sb.append(' ');
        sb.append(type.toUpperCase());
        sb.append(DASHES);
        sb.append("\r\n");
        return sb.toString();
    }

    public byte[] readPem(String pem) {
        List<String> lines = Utility.getInstance().split(pem, "\r\n");
        StringBuilder sb = new StringBuilder();
        boolean begin = false;
        for (String s: lines) {
            if (s.contains(DASHES) && s.contains(BEGIN)) {
                begin = true;
                continue;
            }
            if (s.contains(DASHES) && s.contains(END)) {
                break;
            }
            if (begin) {
                // Beginning and trailing white spaces are not allowed because it is a base64 string
                sb.append(s.trim());
            }
        }
        return Utility.getInstance().base64ToBytes(sb.toString());
    }

    public SSLContext getSslContext(String pem) throws GeneralSecurityException, IOException {
        byte[] der = readPem(pem);
        X509Certificate cert = (X509Certificate) CertificateFactory.getInstance("X.509").generateCertificate(new ByteArrayInputStream(der));
        KeyStore ks = KeyStore.getInstance(KeyStore.getDefaultType());
        // initialize keystore with a random password which is not relevant in keystore operation
        ks.load(null, Utility.getInstance().getUuid().toCharArray());
        ks.setCertificateEntry("certificate", cert);
        // initialize trust trust manager factory
        TrustManagerFactory trustManagerFactory = TrustManagerFactory.getInstance(TrustManagerFactory.getDefaultAlgorithm());
        trustManagerFactory.init(ks);
        // return configured ssl socket factory
        SSLContext context = SSLContext.getInstance("TLS");
        context.init(null, trustManagerFactory.getTrustManagers(), new SecureRandom());
        return context;
    }

    public boolean strongCryptoSupported() {
        byte[] data = getRandomBytes(32);
        byte[] key = generateAesKey(256);
        try {
            byte[] encrypted = aesEncrypt(data, key);
            byte[] decrypted = aesDecrypt(encrypted, key);
            return Arrays.equals(data, decrypted);
        } catch (GeneralSecurityException | IOException e) {
            return false;
        }
    }

    /**
     * Pseudo random generator. Note that this is not cryptographically secure.
     * Therefore, do not use this to generate AES encryption key.
     *
     * @param len of random bytes
     * @return random bytes
     */
    public byte[] getRandomBytes(int len) {
        byte[] key = new byte[len];
        ThreadLocalRandom.current().nextBytes(key);
        return key;
    }

    public int nextInt(int bound) {
        return ThreadLocalRandom.current().nextInt(bound);
    }

    public int nextInt(int start, int bound) {
        return ThreadLocalRandom.current().nextInt(start, bound);
    }

}
