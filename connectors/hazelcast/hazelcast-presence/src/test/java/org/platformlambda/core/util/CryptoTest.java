package org.platformlambda.core.util;

import org.junit.Test;

import java.io.IOException;

public class CryptoTest {

    @Test
    public void encKey() {
        CryptoApi crypto = new CryptoApi();

        AppConfigReader reader = AppConfigReader.getInstance();
        String apiKey = reader.getProperty("app.config.key", Utility.getInstance().getUuid());

        System.out.println("--------"+apiKey);

        byte[] b = crypto.generateAesKey(256);
        System.out.println("====="+b.length);

        byte[] x = crypto.getSHA256(Utility.getInstance().getUTF(apiKey));
        System.out.println("====="+x.length);

    }

    @Test
    public void configTest() {
        ConfigReader config = new ConfigReader();
        try {
            config.load("file:/tmp/config/store/current-config.json");
            System.out.println(config.getMap());
        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}
