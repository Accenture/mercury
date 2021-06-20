/*

    Copyright 2018-2021 Accenture Technology

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

import org.platformlambda.core.util.MultiLevelMap;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.w3c.dom.*;
import org.xml.sax.SAXException;

import javax.xml.parsers.DocumentBuilder;
import javax.xml.parsers.DocumentBuilderFactory;
import javax.xml.parsers.ParserConfigurationException;
import java.io.ByteArrayInputStream;
import java.io.IOException;
import java.io.InputStream;
import java.nio.charset.StandardCharsets;
import java.util.*;

public class SimpleXmlParser {
    private static final Logger log = LoggerFactory.getLogger(SimpleXmlParser.class);
    /*
     * Acknowledgement:
     *
     * The solution to avoid XML external entity (XXE) injection attack was contributed by Sajeeb Lohani
     * (github handles @prodigysml and @n33dle) on 1/6/2020.
     *
     * The corresponding Unit test is org.platformlambda.core.util.XmlParserFeatureTest
     */
    private static final String[] FEATURES_TO_ENABLE = {
            "http://apache.org/xml/features/disallow-doctype-decl"
    };
    private static final String[] FEATURES_TO_DISABLE = {
            "http://xml.org/sax/features/external-general-entities",
            "http://xml.org/sax/features/external-parameter-entities",
            "http://apache.org/xml/features/nonvalidating/load-external-dtd"
    };

    private static final String VALUE = "value";
    private static List<String> defaultDrop = new ArrayList<>();
    private List<String> drop = new ArrayList<>();

    public static void setDefaultDropAttributes(List<String> attributesToIgnore) {
        if (attributesToIgnore != null) {
            SimpleXmlParser.defaultDrop = attributesToIgnore;
            log.info("Default XML attributes to ignore - {}", attributesToIgnore);
        }
    }

    public void setDropAttributes(List<String> attributesToIgnore) {
        if (attributesToIgnore != null) {
            this.drop = attributesToIgnore;
            log.info("XML attributes to ignore - {}", attributesToIgnore);
        }
    }

    public Map<String, Object> parse(String xml) throws IOException {
        return parse(new ByteArrayInputStream(xml.getBytes(StandardCharsets.UTF_8)));
    }

    @SuppressWarnings("unchecked")
    public Map<String, Object> parse(InputStream res) throws IOException {
        Map<String, Object> result = new HashMap<>();
        Document doc;
        try {
            DocumentBuilderFactory dbf = DocumentBuilderFactory.newInstance();
            for (String feature: FEATURES_TO_ENABLE) {
                setFeature(dbf, feature, true);
            }
            for (String feature: FEATURES_TO_DISABLE) {
                setFeature(dbf, feature, false);
            }
            dbf.setXIncludeAware(false);
            dbf.setExpandEntityReferences(false);
            DocumentBuilder dBuilder = dbf.newDocumentBuilder();
            dBuilder.setErrorHandler(null);
            doc = dBuilder.parse(res);
            doc.getDocumentElement().normalize();
        } catch (ParserConfigurationException | SAXException e) {
            // Simplify by converting to IOException
            throw new IOException(e);
        }
        List<List<String>> kvList = new ArrayList<>();
        Element root = doc.getDocumentElement();
        Map<String, Object> seed = new HashMap<>();
        result.put(root.getNodeName(), seed);
        NamedNodeMap attributes = root.getAttributes();
        int count = attributes.getLength();
        for (int i=0; i < count; i++) {
            String attributeName = attributes.item(i).getNodeName();
            if (!canIgnore(attributeName)) {
                seed.put(attributeName, attributes.item(i).getNodeValue());
            }
        }
        NodeList nodes = root.getChildNodes();
        if (nodes != null && nodes.getLength() > 0) {
            int len = nodes.getLength();
            for (int i=0; i < len; i++) {
                parseXML(kvList, nodes.item(i), root.getNodeName());
            }
        }
        while (true) {
            String duplicated = findFirstDuplicatedKey(kvList);
            if (duplicated != null) {
                kvList = updateKvListAsArray(kvList, duplicated);
            } else {
                break;
            }
        }
        MultiLevelMap multi = new MultiLevelMap(result);
        for (List<String> kv: kvList) {
            String composite = kv.get(0);
            String value = kv.get(1);
            multi.setElement(composite, value);
        }
        result = multi.getMap();
        /*
         * Extract root node if any
         */
        if (result.size() == 1) {
            for (String key : result.keySet()) {
                Object rootNode = result.get(key);
                if (rootNode instanceof Map) {
                    return (Map<String, Object>) rootNode;
                }
            }
        }
        return result;
    }

    private String findFirstDuplicatedKey(List<List<String>> kvList) {
        Map<String, Integer> keys = new HashMap<>();
        for (List<String> kv: kvList) {
            String k = kv.get(0);
            // scan for non-array keys only
            if (!k.endsWith("]")) {
                int n = keys.getOrDefault(k, 0);
                n++;
                keys.put(k, n);
            }
        }
        List<String> duplicated = new ArrayList<>();
        for (String k: keys.keySet()) {
            if (keys.get(k) > 1) {
                duplicated.add(k);
            }
        }
        if (duplicated.size() > 1) {
            Collections.sort(duplicated);
        }
        return duplicated.isEmpty()? null : duplicated.get(0);
    }

    private List<List<String>> updateKvListAsArray(List<List<String>> kvList, String duplicated) {
        String prefix = duplicated+".";
        List<List<String>> result = new ArrayList<>();
        int n = 0;
        boolean deferredIncrement = false;
        for (List<String> kv: kvList) {
            String k = kv.get(0);
            String v = kv.get(1);
            if (k.equals(duplicated)) {
                if (deferredIncrement) {
                    n++;
                }
                List<String> updated = new ArrayList<>();
                updated.add(k + "[" + n + "]");
                updated.add(v);
                result.add(updated);
                n++;
                deferredIncrement = false;
            } else if (k.startsWith(prefix)) {
                String part1 = k.substring(0, duplicated.length());
                String part2 = k.substring(prefix.length());
                List<String> updated = new ArrayList<>();
                updated.add(part1 + "[" + n + "]."+part2);
                updated.add(v);
                result.add(updated);
                deferredIncrement = true;
            } else {
                result.add(kv);
            }
        }
        return result;
    }

    private void parseXML(List<List<String>> kvList, Node node, String parent) {
        if (node.getNodeType() == Node.TEXT_NODE) {
            String value = node.getTextContent().trim();
            if (value.length() > 0) {
                saveKv(kvList, parent, value);
            }
        }
        if (node.getNodeType() == Node.ELEMENT_NODE) {
            NamedNodeMap attributes = node.getAttributes();
            if (attributes != null) {
                int count = attributes.getLength();
                for (int i=0; i < count; i++) {
                    String attributeName = attributes.item(i).getNodeName();
                    if (!canIgnore(attributeName)) {
                        saveKv(kvList, parent+"."+attributeName, attributes.item(i).getNodeValue());
                    }
                }
            }
            NodeList nodes = node.getChildNodes();
            if (nodes != null && nodes.getLength() > 0) {
                int len = nodes.getLength();
                for (int i=0; i < len; i++) {
                    parseXML(kvList, nodes.item(i), parent+"."+node.getNodeName());
                }
            }
        }
    }

    private void saveKv(List<List<String>> kvList, String key, String value) {
        List<String> kv = new ArrayList<>();
        kv.add(key);
        kv.add(value);
        kvList.add(kv);
    }

    private boolean canIgnore(String attributeName) {
        return !drop.isEmpty() ? drop.contains(attributeName) : SimpleXmlParser.defaultDrop.contains(attributeName);
    }

    private boolean setFeature(DocumentBuilderFactory dbf, String feature, boolean enable) {
        try {
            dbf.setFeature(feature, enable);
            return dbf.getFeature(feature) == enable;
        } catch (ParserConfigurationException e) {
            log.error("Unable to {} feature - {}", enable? "enable" : "disable", e.getMessage());
            return false;
        }
    }

}
