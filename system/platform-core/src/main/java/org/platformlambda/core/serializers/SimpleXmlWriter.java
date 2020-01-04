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

package org.platformlambda.core.serializers;

import java.time.Instant;
import java.time.ZoneId;
import java.time.ZonedDateTime;
import java.time.format.DateTimeFormatter;
import java.util.*;


public class SimpleXmlWriter {

    private final static String SPACES = "  ";
    private static final DateTimeFormatter isoDate = DateTimeFormatter.ISO_INSTANT;

    private enum TagType {
        START, BODY, END
    }

    public String write(Object map) {
        String className = map.getClass().getSimpleName();
        // className hierarchy filtering: dot for subclass and dollar-sign for nested class
        String root = className.equals("HashMap") ? "root" : className;
        return write(root.toLowerCase(), map);
    }

    @SuppressWarnings("unchecked")
    public String write(String rootName, Object map) {
        if (map instanceof Map) {
            StringBuilder buffer = new StringBuilder();
            buffer.append("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
            write(buffer, rootName, (Map<String, Object>) map, 0);
            return buffer.toString();

        } else {
            throw new IllegalArgumentException("Input should be a Map object");
        }
    }

    private void write(StringBuilder buffer, String nodeName, Map<String, Object> map, int indent) {
        int currentIndent = indent;
        if (nodeName != null) {
            // Add one indent unit for the startTag
            indentBlock(buffer, currentIndent);
            buffer.append('<');
            buffer.append(escapeXml(nodeName, TagType.START));
            buffer.append('>');
            indent++;
        }
        // Next line after a startTag of a map or after a block of elements
        buffer.append('\n');

        List<String> keys = new ArrayList<String>();
        for (String k: map.keySet()) {
            keys.add(k);
        }
        // Arrange the map element in ascending order
        if (keys.size() > 1) {
            Collections.sort(keys);
        }
        for (String k: keys) {
            Object o = map.get(k);
            appendNode(buffer, k, o, indent);
        }
        // Go back one indent unit for the endTag
        // No need to add new line as the element block has already added one.
        indentBlock(buffer, currentIndent-1);
        if (nodeName != null) {
            buffer.append("</");
            buffer.append(escapeXml(nodeName, TagType.END));
            buffer.append('>');
        }
    }

    @SuppressWarnings("unchecked")
    private void appendNode(StringBuilder buffer, String nodeName, Object value, int indent) {
        // Skip null value
        if (value == null) {
            return;
        }
        if (value instanceof List) {
            List<Object> list = (List<Object>) value;
            // To preserve original sequence, DO NOT sort list elements
            for (Object object: list) {
                appendNode(buffer, nodeName, object, indent);
            }
        } else {
            // Add one indent unit for the startTag
            indentBlock(buffer, indent);
            buffer.append('<');
            buffer.append(escapeXml(nodeName, TagType.START));
            buffer.append('>');
            // util.Date is a parent of sql.Date and sql.Timestamp
            // so they will be shown correctly
            if (value instanceof Date) {
                ZonedDateTime zdt = ZonedDateTime.ofInstant(Instant.ofEpochMilli(((Date) value).getTime()), ZoneId.of("UTC"));
                buffer.append(zdt.format(isoDate));
            } else if (value instanceof Map) {
                write(buffer, null, (Map<String, Object>) value, indent+1);
            } else if (value instanceof String) {
                buffer.append(escapeXml((String) value, TagType.BODY));
            } else {
                buffer.append(escapeXml(value.toString(), TagType.BODY));
            }
            buffer.append("</");
            buffer.append(escapeXml(nodeName, TagType.END));
            buffer.append('>');
            // Next line after the endTag
            buffer.append('\n');
        }
    }

    private void indentBlock(StringBuilder buffer, int indent) {

        for (int i=0; i < indent; i++) {
            buffer.append(SPACES);
        }
    }

    private String escapeXml(String value, TagType type) {

        switch (type) {

            case START:
                if (value.startsWith("/") || value.startsWith("{")) {
                    return "node value=\""+value+"\"";
                }
                return safeXmlKey(value);
            case END:
                if (value.startsWith("/") || value.startsWith("{")) {
                    return "node";
                }
                return safeXmlKey(value);
            default:
                // escape special characters
                // http://en.wikipedia.org/wiki/XML#Escaping
                String result = value;
                if (result.contains("&")) {
                    result = result.replace("&", "&amp;");
                }
                if (result.contains("'")) {
                    result = result.replace("'", "&apos;");
                }
                if (result.contains("\"")) {
                    result = result.replace("\"", "&quot;");
                }
                if (result.contains(">")) {
                    result = result.replace(">", "&gt;");
                }
                if (result.contains("<")) {
                    result = result.replace("<", "&lt;");
                }
                return result;
        }
    }

    private String safeXmlKey(String str) {
        boolean valid = true;
        for (int i=0; i < str.length(); i++) {
            char c = str.charAt(i);
            if (c >= '0' && c <= '9') continue;
            if (c >= 'a' && c <= 'z') continue;
            if (c >= 'A' && c <= 'Z') continue;
            if (c == '-') continue;
            valid = false;
            break;
        }
        if (valid) {
            return str;
        }
        StringBuilder sb = new StringBuilder();
        for (int i=0; i < str.length(); i++) {
            char c = str.charAt(i);
            if (c >= '0' && c <= '9') {
                sb.append(c);
                continue;
            }
            if (c >= 'a' && c <= 'z') {
                sb.append(c);
                continue;
            }
            if (c >= 'A' && c <= 'Z') {
                sb.append(c);
                continue;
            }
            if (c == '-') {
                sb.append(c);
            }
            sb.append('_');
        }
        return sb.toString();
    }

}
