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

import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.VersionInfo;
import org.platformlambda.core.models.WsEnvelope;
import org.platformlambda.core.services.WsTransmitter;
import org.platformlambda.core.system.PostOffice;
import org.springframework.core.io.Resource;
import org.springframework.core.io.support.PathMatchingResourcePatternResolver;

import javax.websocket.CloseReason;
import java.io.*;
import java.net.InetSocketAddress;
import java.net.Socket;
import java.nio.charset.StandardCharsets;
import java.time.Instant;
import java.time.ZoneId;
import java.time.ZonedDateTime;
import java.time.format.DateTimeFormatter;
import java.time.format.DateTimeParseException;
import java.util.*;

public class Utility {
    public static final String ISO_DATE_FORMAT = "yyyy-MM-dd'T'HH:mm:ssZ";
    public static final String ISO_MS_FORMAT = "yyyy-MM-dd'T'HH:mm:ss.SSSZ";

    private static final DateTimeFormatter ISO_DATE = DateTimeFormatter.ofPattern(ISO_DATE_FORMAT);
    private static final DateTimeFormatter ISO_DATE_MS = DateTimeFormatter.ofPattern(ISO_MS_FORMAT);
    private static final DateTimeFormatter DATE_ONLY = DateTimeFormatter.ofPattern("yyyyMMdd");
    private static final DateTimeFormatter DATE_TIME = DateTimeFormatter.ofPattern("yyyyMMdd-HHmmss");
    private static final DateTimeFormatter SQL_DATE = DateTimeFormatter.ofPattern("yyyy-MM-dd");
    private static final DateTimeFormatter HTML_DATE = DateTimeFormatter.ofPattern("EEE, dd MMM yyyy HH:mm:ss O");
    private static final String TIMESTAMP_FORMAT = "yyyyMMddHHmmssSSS";
    private static final DateTimeFormatter TIMESTAMP = DateTimeFormatter.ofPattern(TIMESTAMP_FORMAT);
    private static final String SQL_TIMESTAMP_FORMAT = "yyyy-MM-dd HH.mm.ss.SSS";
    private static final DateTimeFormatter SQL_TIMESTAMP = DateTimeFormatter.ofPattern(SQL_TIMESTAMP_FORMAT);
    private static final ZoneId UTC_TIME = ZoneId.of("UTC");
    private static final String NESTED_EXCEPTION_MARKER = "** BEGIN NESTED EXCEPTION **";
    private static final String NESTED_EXCEPTION_START = "MESSAGE:";
    private static final String NESTED_EXCEPTION_END = "STACKTRACE:";
    private static final String SPRING_BOOT_LIB_PATH = "/BOOT-INF/lib/*.jar";
    private static final String LIB_PATH = "/lib/*.jar";
    private static final String JAR = ".jar";
    private static final String POM_LOCATION = "pom.properties.location";
    private static final String POM_PROPERTIES = "/META-INF/maven/*/*/pom.properties";
    private static final String GROUP_ID = "groupId";
    private static final String ARTIFACT_ID = "artifactId";
    private static final String VERSION = "version";
    private static final String SPRING_APPNAME = "spring.application.name";
    private static final String APPNAME = "application.name";
    private static final String DEFAULT_APPNAME = "application";
    private static final String APP_VERSION = "info.app.version";
    private static final String DEFAULT_APP_VERSION = "0.0.0";
    private static final String WORK_FOLDER = "application.work.location";
    private static final String CLOUD_FOLDER = "cloud.work.location";
    private static final String IDENTITY_FOLDER = "application.identity.location";
    private static final String CREDENTIALS_FOLDER = "application.credentials.location";
    private static final String LAMBDA_FOLDER_NAME = "/tmp/lambda/apps";
    private static final String CLOUD_FOLDER_NAME = "/tmp/lambda/cloud";
    private static final String IDENTITY_FOLDER_NAME = "/tmp/lambda/identities";
    private static final String CREDENTIALS_FOLDER_NAME = "/tmp/lambda/credentials";
    private static final String NODES = "nodes";
    private static final String LAMBDAS = "lambdas";
    private static final String[] RESERVED_FILENAMES = {"thumbs.db"};
    private static final String[] RESERVED_EXTENSIONS = {
            ".con", ".prn", ".aux", ".nul", ".com", ".exe",
            ".com1", ".com2", ".com3", ".com4", ".com5", ".com6", ".com7", ".com8", ".com9",
            ".lpt1", ".lpt2", ".lpt3", ".lpt4", ".lpt5", ".lpt6", ".lpt7", ".lpt8", ".lpt9"};
    private static final String INVALID_HEX = "Invalid hex string";
    private static final char[] HEX_DIGITS = "0123456789abcdef".toCharArray();
    private static final String ZEROS = "0000000000000000";
    private static final String TOTAL = "Total";

    private static final VersionInfo versionInfo = new VersionInfo();
    private static final List<String> libs = new ArrayList<>();
    private static final Utility instance = new Utility();

    @SuppressWarnings("unchecked")
    private Utility() {
        // Get basic application info when the system starts
        List<String> list = new ArrayList<>();
        PathMatchingResourcePatternResolver resolver1 = new PathMatchingResourcePatternResolver();
        // Search Spring Boot packager lib path for dependencies
        Resource[] res1 = new Resource[0];
        try {
            res1 = resolver1.getResources(SPRING_BOOT_LIB_PATH);
        } catch (IOException e) {
            try {
                res1 = resolver1.getResources(LIB_PATH);
            } catch (IOException e1) {
                // nothing we can do
            }
        }
        for (Resource r: res1) {
            String filename = r.getFilename();
            if (filename != null) {
                list.add(filename.endsWith(JAR)? filename.substring(0, filename.length()-JAR.length()) : filename);
            }
        }
        /*
         * Sort the library names in ascending order.
         * Library listing is usually used by the application's Info admin endpoint.
         */
        if (list.size() > 1) {
            Collections.sort(list);
        }
        if (!list.isEmpty()) {
            int size = list.size();
            int n = 0;
            for (String f : list) {
                libs.add(zeroFill(++n, size) + ". " + f);
            }
            libs.add(TOTAL + ": " + list.size());
        }
        // Get default version info from application.properties
        AppConfigReader reader = AppConfigReader.getInstance();
        String name = filteredServiceName(reader.getProperty(SPRING_APPNAME, reader.getProperty(APPNAME, DEFAULT_APPNAME)));
        versionInfo.setGroupId("unknown");
        versionInfo.setArtifactId(name);
        versionInfo.setVersion(reader.getProperty(APP_VERSION, DEFAULT_APP_VERSION));
        // if not running in IDE, get version information from JAR
        if (!libs.isEmpty()) {
            // resolve it from /META-INF/maven/*/*/pom.properties
            PathMatchingResourcePatternResolver resolver2 = new PathMatchingResourcePatternResolver();
            try {
                Resource[] res2 = resolver2.getResources(reader.getProperty(POM_LOCATION, POM_PROPERTIES));
                if (res2.length == 1) {
                    Properties p = new Properties();
                    p.load(new ByteArrayInputStream(stream2bytes(res2[0].getInputStream())));
                    if (p.containsKey(GROUP_ID) && p.containsKey(ARTIFACT_ID) && p.containsKey(VERSION)) {
                        versionInfo.setArtifactId(filteredServiceName(p.getProperty(ARTIFACT_ID)))
                                   .setGroupId(p.getProperty(GROUP_ID))
                                   .setVersion(p.getProperty(VERSION));
                    }
                }

            } catch (IOException e) {
                // nothing we can do
            }
        }
    }

    public static Utility getInstance() {
        return instance;
    }

    public VersionInfo getVersionInfo() {
        return versionInfo;
    }

    public List<String> getLibraryList() {
        return libs;
    }

    public String getPackageName() {
        return versionInfo.getArtifactId();
    }

    public String zeroFill(int seq, int max) {
        int len = String.valueOf(max).length();
        String value = String.valueOf(seq);
        return value.length() < len? ZEROS.substring(0, len - value.length()) + value : value;
    }

    public String normalizeFolder(String folder) {
        String result = folder.startsWith("~") ? folder.replace("~", System.getProperty("user.home")) : folder;
        return result.replace("\\", "/");
    }

    public File getWorkFolder() {
        AppConfigReader reader = AppConfigReader.getInstance();
        return new File(new File(normalizeFolder(reader.getProperty(WORK_FOLDER, LAMBDA_FOLDER_NAME))), getPackageName());
    }

    public File getCloudFolder() {
        AppConfigReader reader = AppConfigReader.getInstance();
        return new File(normalizeFolder(reader.getProperty(CLOUD_FOLDER, CLOUD_FOLDER_NAME)));
    }

    public File getIdentityFolder() {
        AppConfigReader reader = AppConfigReader.getInstance();
        return new File(new File(normalizeFolder(reader.getProperty(IDENTITY_FOLDER, IDENTITY_FOLDER_NAME))), getPackageName());
    }

    private File getCredentialFolder() {
        AppConfigReader reader = AppConfigReader.getInstance();
        return new File(normalizeFolder(reader.getProperty(CREDENTIALS_FOLDER, CREDENTIALS_FOLDER_NAME)));
    }

    public File getEventNodeCredentials() {
        return new File(getCredentialFolder(), NODES);
    }

    public File getLambdaCredentials() {
        return new File(getCredentialFolder(), LAMBDAS);
    }

    public String getUuid() {
        return UUID.randomUUID().toString().replace("-", "");
    }

    public String getDateUuid() {
        return getDateOnly(new Date())+getUuid();
    }

    /////////////////////////////////////////////////////
    // Simple string splitter without overheads of regex
    /////////////////////////////////////////////////////

    /**
     * Split a string using a set of separators
     * Skips empty values.
     *
     * @param str the input string
     * @param chars the characters to be used as separators
     * @return list
     */
    public List<String> split(String str, String chars) {
        return split(str, chars, false);
    }

    /**
     * Split a string using a set of separators
     *
     * @param str the input string
     * @param chars the characters to be used as separators
     * @param empty if true, return empty elements
     * @return list
     */
    public List<String> split(String str, String chars, boolean empty) {
        List<String> rv = new ArrayList<>();
        if (str == null) return rv;
        StringBuilder sb = new StringBuilder();
        boolean found;
        for (int i=0; i < str.length(); i++) {
            found = false;
            for (int j=0; j < chars.length(); j++) {
                if (str.charAt(i) == chars.charAt(j)) {
                    if (sb.length() > 0) {
                        rv.add(sb.toString());
                    } else if (empty) {
                        rv.add("");
                    }
                    // reset buffer
                    sb.setLength(0);
                    found = true;
                    break;
                }
            }
            if (!found) sb.append(str.charAt(i));
        }
        if (sb.length() > 0) rv.add(sb.toString());
        return rv;
    }

    ////////////////////////////////////////
    // Convenient properties file utilities
    ////////////////////////////////////////

    /**
     * Hierarchical data of a map will be flatten into properties like "this.is.a.key"
     *
     * @param src map
     * @return flat map
     */
    public Map<String, Object> getFlatMap(Map<String, Object> src) {
        Map<String, Object> target = new HashMap<>();
        getFlatMap(null, src, target);
        return target;
    }

    @SuppressWarnings("unchecked")
    private void getFlatMap(String prefix, Map<String, Object> src, Map<String, Object> target) {
        for (String k: src.keySet()) {
            Object v = src.get(k);
            if (v instanceof Map) {
                String key = prefix == null? k : prefix+"."+k;
                getFlatMap(key, (Map) v, target);
            } else if (v instanceof List) {
                int n = 0;
                for (Object o: (List) v) {
                    String key = (prefix == null? k : prefix+"."+k) +"["+n+"]";
                    n++;
                    if (o instanceof Map) {
                        getFlatMap(key, (Map) o, target);
                    } else if (o instanceof List) {
                        getFlatList(key, (List) o, target);
                    } else if (o != null) {
                        target.put(key, o);
                    }
                }
            } else if (v != null){
                String key = prefix == null? k : prefix+"."+k;
                target.put(key, v);
            }
        }
    }

    @SuppressWarnings("unchecked")
    private void getFlatList(String prefix, List src, Map<String, Object> target) {
        int n = 0;
        for (Object v: src) {
            String key = prefix+"["+n+"]";
            n++;
            if (v instanceof Map) {
                getFlatMap(key, (Map) v, target);
            } else if (v instanceof List) {
                getFlatList(key, (List) v, target);
            } else {
                target.put(key, v);
            }
        }
    }

    //////////////////////////////////
    // Convenient timestamp utilities
    //////////////////////////////////

    public String getTimestamp() {
        return getTimestamp(System.currentTimeMillis());
    }

    public String getTimestamp(long time) {
        ZonedDateTime zdt = ZonedDateTime.ofInstant(Instant.ofEpochMilli(time), UTC_TIME);
        return zdt.format(TIMESTAMP);
    }

    public String getSqlDate(java.sql.Date date) {
        ZonedDateTime zdt = ZonedDateTime.ofInstant(Instant.ofEpochMilli(date.getTime()), UTC_TIME);
        return zdt.format(SQL_DATE);
    }

    public String getSqlTimestamp(java.sql.Timestamp time) {
        ZonedDateTime zdt = ZonedDateTime.ofInstant(Instant.ofEpochMilli(time.getTime()), UTC_TIME);
        return zdt.format(SQL_TIMESTAMP);
    }

    public String getDateOnly(Date date) {
        ZonedDateTime zdt = ZonedDateTime.ofInstant(Instant.ofEpochMilli(date.getTime()), UTC_TIME);
        return zdt.format(DATE_ONLY);
    }

    public String getAmazonDate(Date date) {
        ZonedDateTime zdt = ZonedDateTime.ofInstant(Instant.ofEpochMilli(date.getTime()), UTC_TIME);
        return zdt.format(DATE_TIME).replace('-', 'T')+"Z";
    }

    /**
     * RFC 7231 Date (aka HTML date)
     * @param date input
     * @return date string for use in a HTTP header
     */
    public String getHtmlDate(Date date) {
        ZonedDateTime zdt = ZonedDateTime.ofInstant(Instant.ofEpochMilli(date.getTime()), UTC_TIME);
        return zdt.format(HTML_DATE);
    }

    public long timestamp2ms(String timestamp) {
        // timestamp is formatted as yyyyMMddHHmmssSSS
        if (timestamp == null || timestamp.length() != TIMESTAMP_FORMAT.length()) {
            return 0;
        }
        int year = str2int(timestamp.substring(0, 4));
        int month = str2int(timestamp.substring(4, 6));
        int day = str2int(timestamp.substring(6, 8));
        int hour = str2int(timestamp.substring(8, 10));
        int minute = str2int(timestamp.substring(10, 12));
        int second = str2int(timestamp.substring(12, 14));
        int ms = str2int(timestamp.substring(14));
        ZonedDateTime zdt = ZonedDateTime.of(year, month, day, hour, minute, second, ms * 1000000, UTC_TIME);
        return zdt.toInstant().toEpochMilli();
    }

    /////////////////////////////
    // Convenient file utilities
    /////////////////////////////

    public byte[] stream2bytes(InputStream stream) {
        return stream2bytes(stream, true);
    }

    public byte[] stream2bytes(InputStream stream, boolean closeStream) {
        return stream2bytes(stream, closeStream, -1);
    }

    public byte[] stream2bytes(InputStream stream, boolean closeStream, int maxSize) {
        if (stream == null) {
            return null;
        }
        ByteArrayOutputStream out = new ByteArrayOutputStream();
        int len, total = 0;
        byte[] buffer = new byte[1024];
        if (stream instanceof ByteArrayInputStream) {
            ByteArrayInputStream in = (ByteArrayInputStream) stream;
            while ((len = in.read(buffer, 0, buffer.length)) != -1) {
                out.write(buffer, 0, len) ;
            }
        } else {
            BufferedInputStream bin = (stream instanceof BufferedInputStream) ?
                    (BufferedInputStream) stream : new BufferedInputStream(stream);
            try {
                while ((len = bin.read(buffer, 0, buffer.length)) != -1) {
                    total += len;
                    if (maxSize > 0 && total > maxSize) {
                        throw new IllegalArgumentException("input larger than max of "+maxSize+" bytes");
                    }
                    out.write(buffer, 0, len) ;
                }
                if (closeStream) bin.close();
                bin = null;
            } catch (IOException e) {
                // ignore
            } finally {
                if (bin != null && closeStream)
                    try {
                        bin.close();
                    } catch (IOException e) {
                        // does not matter
                    }
            }
        }
        return out.toByteArray();
    }

    public String stream2str(InputStream stream) {
        return getUTF(stream2bytes(stream));
    }

    public boolean str2file(File f, String s) {
        return bytes2file(f, getUTF(s));
    }

    public boolean bytes2file(File f, byte[] b) {
        FileOutputStream out = null;
        try {
            out = new FileOutputStream(f);
            out.write(b);
            out.close();
            out = null;
            return true;
        } catch (IOException e) {
            return false;
        } finally {
            if (out != null) {
                try {
                    out.close();
                } catch (IOException e) {
                    // does not matter
                }
            }
        }
    }

    public String file2str(File f) {
        return getUTF(file2bytes(f));
    }

    public byte[] file2bytes(File f) {
        if (f != null) {
            try {
                FileInputStream in = new FileInputStream(f);
                return stream2bytes(in);
            } catch (FileNotFoundException e) {
                // does not matter
            }
        }
        return new byte[0];
    }

    public void cleanupDir(File dir) {
        cleanupDir(dir, false);
    }

    public void cleanupDir(File dir, boolean keep) {
        if (dir != null && dir.exists() && dir.isDirectory()) {
            File[] files = dir.listFiles();
            if (files != null) {
                for (File f: files) {
                    if (f.isDirectory()) {
                        cleanupDir(f, false);
                    } else {
                        f.delete();
                    }
                }
            }
            if (!keep) dir.delete();
        }
    }

    ////////////////////////////////////////////////
    // Useful integer and long conversion utilities
    ////////////////////////////////////////////////

    public int bytes2int(byte[] b) {
        if (b == null || b.length != 4) return -1;
        int val = 0;
        for (int i=0 ; i < 4; ++i) {
            val *= 256L ;
            if ((b[i] & 0x80) == 0x80) {
                val += 128L ;
            }
            val += (b[i] & 0x7F) ;
        }
        return val;
    }

    public byte[] int2bytes(int val) {
        byte[] results = new byte[4];

        for (int idx=3; idx >=0; --idx) {
            results[idx] = (byte) (val & 0xFF);
            val = val >> 8;
        }
        return results;
    }

    public int str2int(String str) {
        if (str == null || str.length() == 0) {
            return -1;
        }
        try {
            // drop decimal point
            int dot = str.indexOf('.');
            return Integer.parseInt(dot > 0 ? str.substring(0, dot) : str);
        } catch (NumberFormatException e) {
            return -1;
        }
    }

    public byte[] long2bytes(long val) {
        byte[] results = new byte[8];
        for (int idx=7; idx >=0; --idx) {
            results[idx] = (byte) (val & 0xFF);
            val = val >> 8;
        }
        return results;
    }

    public long bytes2long(byte[] b) {
        if (b == null || b.length != 8) return -1;
        long val = 0;
        for (int i=0 ; i < 8; ++i) {
            val *= 256L ;
            if ((b[i] & 0x80) == 0x80) {
                val += 128L ;
            }
            val += (b[i] & 0x7F) ;
        }
        return val;
    }

    public long str2long(String str) {
        if (str == null || str.length() == 0) {
            return -1;
        }
        try {
            // drop decimal point
            int dot = str.indexOf('.');
            return Long.parseLong(dot > 0 ? str.substring(0, dot) : str);
        } catch (NumberFormatException e) {
            return -1;
        }
    }

    public float str2float(String str) {
        if (str == null || str.length() == 0) {
            return -1.0f;
        }
        try {
            return Float.parseFloat(str);
        } catch (NumberFormatException e) {
            return -1.0f;
        }
    }

    public double str2double(String str) {
        if (str == null || str.length() == 0) {
            return -1.0d;
        }
        try {
            return Double.parseDouble(str);
        } catch (NumberFormatException e) {
            return -1.0d;
        }
    }

    public boolean isNumeric(String str) {
        if (str == null) {
            return false;
        }
        if (str.length() > 1 && str.startsWith("-")) {
            return isDigits(str.substring(1));
        } else {
            return isDigits(str);
        }
    }

    ///////////////////////////////////////
    // Conversion between string and bytes
    ///////////////////////////////////////

    public byte[] getUTF(String str) {
        if (str == null || str.length() == 0) return new byte[0];
        return str.getBytes(StandardCharsets.UTF_8);
    }

    public String getUTF(byte[] b) {
        if (b == null || b.length == 0) return "";
        return new String(b, StandardCharsets.UTF_8);
    }

    /**
     * Valid service name (routing path and project ID)
     * must contain only 0-9, a-z, period, underscore and hyphen.
     *
     * @param str for routing path or project ID
     * @return true if valid
     */
    public boolean validServiceName(String str) {
        if (str == null || str.length() == 0) return false;
        if (str.startsWith(".") || str.startsWith("_") || str.startsWith("-")
                || str.contains("..")
                || str.endsWith(".") || str.endsWith("_") || str.endsWith("-")) return false;
        for (int i=0; i < str.length(); i++) {
            if (str.charAt(i) >= '0' && str.charAt(i) <= '9') continue;
            if (str.charAt(i) >= 'a' && str.charAt(i) <= 'z') continue;
            if (str.charAt(i) == '.' || str.charAt(i) == '_' || str.charAt(i) == '-') continue;
            return false;
        }
        return true;
    }

    public String filteredServiceName(String name) {
        String str = name.toLowerCase();
        boolean dot = true;
        StringBuilder sb = new StringBuilder();
        for (int i=0; i < str.length(); i++) {
            if (dot) {
                if (str.charAt(i) == '.') {
                    continue;
                } else {
                    dot = false;
                }
            }
            if (str.charAt(i) >= '0' && str.charAt(i) <= '9') {
                sb.append(str.charAt(i));
                continue;
            }
            if (str.charAt(i) >= 'a' && str.charAt(i) <= 'z') {
                sb.append(str.charAt(i));
                continue;
            }
            if (str.charAt(i) == '.' || str.charAt(i) == '_' || str.charAt(i) == '-') {
                sb.append(str.charAt(i));
            }
        }
        return sb.toString();
    }

    public boolean reservedFilename(String name) {
        for (String filename: RESERVED_FILENAMES) {
            if (name.equals(filename)) {
                return true;
            }
        }
        return false;
    }

    public boolean reservedExtension(String name) {
        for (String ext: RESERVED_EXTENSIONS) {
            if (name.endsWith(ext)) {
                return true;
            }
        }
        return false;
    }

    public String date2str(Date date) {
        return date2str(date, false);
    }

    public String date2str(Date date, boolean upToSeconds) {
        // just in case input is SQL date or time
        if (date instanceof java.sql.Date || date instanceof java.sql.Time) {
            return date.toString();
        }
        long time = upToSeconds? date.getTime() / 1000 * 1000 : date.getTime();
        ZonedDateTime zdt = ZonedDateTime.ofInstant(Instant.ofEpochMilli(time), UTC_TIME);
        return zdt.format(DateTimeFormatter.ISO_INSTANT);
    }

    public Date str2date(String str) {
        return str2date(str, false);
    }

    public Date str2date(String str, boolean throwException) {
        if (isDigits(str)) {
            return new Date(Long.parseLong(str));
        }
        /*
         * Support multiple variance of ISO-8601
         *
         * 1. 2015-01-06
         * 2. 2015-01-06 01:02:03
         * 3. 2015-01-06T01:02:03
         * 4. 2015-01-06T01:02:03Z
         * 5. 2015-01-06T01:02:03+0000
         * 6. 2015-01-06T01:02:03+00:00
         * 7. 2015-01-06T01:02:03.123Z
         * 8. 2015-01-06T01:02:03.123+0000
         * 9. 2015-01-06T01:02:03.123+00:00
         *
         * DO NOT CHANGE the string substitution sequence below
         */
        // zero fill time and skip fields
        if (str.length() == 10) {
            str += "T00:00:00+0000";
        }
        // change SPACE to T if needed
        if (str.length() > 11 && str.charAt(10) == ' ') {
            str = str.substring(0, 10) + "T" + str.substring(11);
        }
        // use UTC timezone if not specified
        if (str.length() == 19) {
            str += "+0000";
        }
        // normalize UTC "Z" indicator to +0000
        if (str.endsWith("Z")) {
            str = str.substring(0, str.length()-1)+"+0000";
        }
        // precision up to milliseconds only and drop microseconds if any
        int dot = str.indexOf('.');
        if (dot == 19) {
            int sep = str.indexOf('+', 19);
            if (sep == -1) {
                sep = str.indexOf('-', 19);
            }
            String ms = get3digitMs(sep > 0? str.substring(dot, sep) : str.substring(dot));
            // remove colon from timezone
            String timezone = sep == -1? "+0000" : str.substring(sep).replace(":", "");
            str = str.substring(0, dot) + ms + timezone;
        }
        // parse the normalized time string
        try {
            ZonedDateTime zdt = dot==19? ZonedDateTime.parse(str, ISO_DATE_MS) : ZonedDateTime.parse(str, ISO_DATE);
            return Date.from(zdt.toInstant());
        } catch (IllegalArgumentException | DateTimeParseException e) {
            if (throwException) {
                throw new IllegalArgumentException(e.getMessage());
            } else {
                return new Date(0);
            }
        }
    }

    public boolean isDigits(String str) {
        if (str == null) {
            return false;
        }
        for (int i=0; i < str.length(); i++) {
            if (str.charAt(i) >= '0' && str.charAt(i) <= '9') continue;
            return false;
        }
        return str.length() > 0;
    }

    private String get3digitMs(String s) {
        if (s.length() == 4) {
            return s;
        } else {
            String ms = String.format("%.03f", Float.parseFloat(s));
            return ms.substring(ms.indexOf('.'));
        }
    }

    //////////////////////////////////////
    // Resolve root cause of an exception
    //////////////////////////////////////

    public Throwable getRootCause(Throwable exception) {
        if (exception.getCause() != null) {
            return getRootCause(exception.getCause());
        }
        // just in case the exception is expressed as a string
        String message = exception.getMessage();
        if (message != null) {
            int nested = message.indexOf(NESTED_EXCEPTION_MARKER);
            if (nested > 0) {
                String subMessage = message.substring(nested+NESTED_EXCEPTION_MARKER.length()+1);
                int start = subMessage.indexOf(NESTED_EXCEPTION_START);
                int end = subMessage.indexOf(NESTED_EXCEPTION_END);
                if (start > 0 && end > 0 && end > start) {
                    return new RuntimeException(subMessage.substring(start+NESTED_EXCEPTION_START.length()+1, end));
                }
            }
        }
        return exception;
    }

    //////////////////////////////
    // Hex and base64 converters
    /////////////////////////////

    public String bytesToBase64(byte[] b) {
        return bytesToBase64(b, false, false);
    }

    public String bytesToUrlBase64(byte[] b) {
        return bytesToBase64(b, false, true);
    }

    public String bytesToBase64(byte[] b, boolean pretty, boolean isUrl) {
        byte[] b64 = isUrl? Base64.getUrlEncoder().encode(b) : Base64.getEncoder().encode(b);
        String result = getUTF(b64);
        if (pretty) {
            StringBuilder sb = new StringBuilder();
            int i = 0;
            for (char c: result.toCharArray()) {
                sb.append(c);
                if (++i == 64) {
                    sb.append("\r\n");
                    i = 0;
                }
            }
            String prettyStr = sb.toString();
            result = prettyStr.endsWith("\n") ? prettyStr : sb.toString()+"\r\n";
        }
        return result;
    }

    public byte[] base64ToBytes(String base64) {
        return base64ToBytes(base64, false);
    }

    public byte[] urlBase64ToBytes(String base64) {
        return base64ToBytes(base64, true);
    }

    public byte[] base64ToBytes(String base64, boolean isUrl) {
        String b64 = base64.contains("\n") ? base64.replace("\n", "").replace("\r", "") : base64;
        if (isUrl) {
            return Base64.getUrlDecoder().decode(b64);
        } else {
            return Base64.getDecoder().decode(b64);
        }
    }

    public byte[] hex2bytes(String hex) throws IOException {
        char[] data = hex.toLowerCase().toCharArray();
        int len = data.length;
        if (len % 2 != 0) {
            throw new IOException(INVALID_HEX);
        }
        byte[] out = new byte[len >> 1];
        for (int i = 0, j = 0; j < len; i++) {
            int f = getNumericValue(data[j]) << 4;
            j++;
            f = f | getNumericValue(data[j]);
            j++;
            out[i] = (byte) (f & 0xFF);
        }
        return out;
    }

    public String bytes2hex(byte[] bytes) {
        int l = bytes.length;
        char[] out = new char[l << 1];
        for (int i = 0, j = 0; i < l; i++) {
            out[j++] = HEX_DIGITS[(0xF0 & bytes[i]) >>> 4 ];
            out[j++] = HEX_DIGITS[ 0x0F & bytes[i] ];
        }
        return String.valueOf(out);
    }

    private int getNumericValue(char c) throws IOException {
        int result = Character.digit(c, 16);
        if (result == -1) {
            throw new IOException(INVALID_HEX);
        }
        return result;
    }

    //////////////////////////////////
    // Network connectivity utilities
    //////////////////////////////////

    public boolean portReady(String host, int port, int timeoutMs) {
        if (port < 1) {
            return false;
        }
        InetSocketAddress target = new InetSocketAddress(host, port);
        Socket s = null;
        try {
            s = new Socket();
            s.setReuseAddress(true);
            s.bind(null);
            s.connect(target, timeoutMs);
            s.close();
            s = null;
            return true;
        } catch (IOException te) {
            try {
                s.close();
                s = null;
            } catch (IOException e) {
                // does not matter
            }
        } finally {
            if (s != null) {
                try {
                    s.close();
                } catch (IOException e) {
                    // does not matter
                }
            }
        }
        return false;
    }

    public void closeConnection(String txPath, CloseReason.CloseCodes status, String message) throws IOException {
        if (txPath != null && status != null && message != null) {
            EventEnvelope error = new EventEnvelope();
            error.setTo(txPath);
            error.setHeader(WsTransmitter.STATUS, String.valueOf(status.getCode()));
            error.setHeader(WsTransmitter.MESSAGE, message);
            error.setHeader(WsEnvelope.TYPE, WsEnvelope.CLOSE);
            PostOffice.getInstance().send(error);
        }
    }

}
