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

/**
 * Behavior and limitations
 * ------------------------
 * Text content in a TEXT_NODE will be trimmed.
 * For array, empty text element will be set to null.
 * If the empty text element is in the end of the array list, it will be dropped, therefore return null
 * when the application retrieves it from the resultant map.
 *
 * For non-array, empty text element will be dropped, therefore returning null when the application
 * reads it from the resultant map.
 *
 * Element attributes are rendered as a regular key-value under the element.
 *
 * Purpose
 * -------
 * The FasterXml Jackson implementation "XmlMapper" is a general purpose Xml Parser.
 * For some edge cases, you may prefer to use XmlMapper in some apps.
 *
 * Since Mercury is using Google Guava as JSON serializer, we develop this simple Xml Parser
 * as an alternative.
 */
public class SimpleXmlParser {
    private static final Logger log = LoggerFactory.getLogger(SimpleXmlParser.class);
    /*
     * Acknowledgement:
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
    private static final String INDEX = "index";
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
        // childMap is used to keep track of array elements in the Xml
        Map<String, Map<String, Integer>> childMap = new HashMap<>();
        List<List<String>> kvList = new ArrayList<>();
        NodeList nodes = root.getChildNodes();
        if (nodes.getLength() > 0) {
            int len = nodes.getLength();
            for (int i=0; i < len; i++) {
                parseXML(childMap, kvList, nodes.item(i), root.getNodeName());
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

    private String normalizeParentPath(String parent) {
        if (parent.endsWith("]")) {
            int sep = parent.lastIndexOf('[');
            return sep == -1? parent : parent.substring(0, sep);
        } else {
            return parent;
        }
    }

    private void parseXML(Map<String, Map<String, Integer>> childMap,
                          List<List<String>> kvList, Node node, String parent) {

        String parentPath = normalizeParentPath(parent);
        if (node.getNodeType() == Node.TEXT_NODE) {
            String value = node.getTextContent().trim();
            if (value.length() > 0) {
                saveKv(kvList, parent, value);
            }
        }
        if (node.getNodeType() == Node.ELEMENT_NODE) {
            NamedNodeMap attributes = node.getAttributes();
            int count = attributes.getLength();
            for (int i=0; i < count; i++) {
                String attributeName = attributes.item(i).getNodeName();
                if (!canIgnore(attributeName)) {
                    saveKv(kvList, parent+"."+attributeName, attributes.item(i).getNodeValue());
                }
            }
            if (!childMap.containsKey(parentPath)) {
                Map<String, Integer> childCounts = new HashMap<>();
                NodeList peers = node.getParentNode().getChildNodes();
                if (peers.getLength() > 0) {
                    int len = peers.getLength();
                    for (int i = 0; i < len; i++) {
                        Node current = peers.item(i);
                        if (current.getNodeType() == Node.ELEMENT_NODE) {
                            String nodeName = current.getNodeName();
                            int counter = childCounts.getOrDefault(nodeName, 0);
                            current.setUserData(INDEX, counter, null);
                            childCounts.put(nodeName, ++counter);
                        }
                    }
                }
                childMap.put(parentPath, childCounts);
            }
            Map<String, Integer> childCounts = childMap.get(parentPath);
            NodeList nodes = node.getChildNodes();
            if (nodes.getLength() > 0) {
                int len = nodes.getLength();
                for (int i=0; i < len; i++) {
                    Node current = nodes.item(i);
                    String nodeName = node.getNodeName();
                    Object index = node.getUserData(INDEX);
                    String path = parent + "." + node.getNodeName();
                    int counter = childCounts.getOrDefault(nodeName, 0);
                    if (counter > 1 && index instanceof Integer) {
                        parseXML(childMap, kvList, current, path + "["+index+"]");
                    } else {
                        parseXML(childMap, kvList, current, path);
                    }
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
