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
import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class SimpleXmlParser {
    private static final Logger log = LoggerFactory.getLogger(SimpleXmlParser.class);

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
        return parse(new ByteArrayInputStream(xml.getBytes("UTF-8")));
    }

    @SuppressWarnings("unchecked")
    public Map<String, Object> parse(InputStream res) throws IOException {
        Map<String, Object> map = new HashMap<>();

        Document doc;
        try {
            DocumentBuilderFactory dbFactory = DocumentBuilderFactory.newInstance();
            DocumentBuilder dBuilder = dbFactory.newDocumentBuilder();
            doc = dBuilder.parse(res);
            doc.getDocumentElement().normalize();
        } catch (ParserConfigurationException | SAXException e) {
            // Simplify by converting to IOException
            throw new IOException(e);
        }

        Element root = doc.getDocumentElement();

        Map<String, Object> rootMap = new HashMap<>();
        map.put(root.getNodeName(), rootMap);

        NamedNodeMap attributes = root.getAttributes();
        int count = attributes.getLength();
        for (int i=0; i < count; i++) {
            rootMap.put(attributes.item(i).getNodeName(), attributes.item(i).getNodeValue());
        }

        NodeList nodes = root.getChildNodes();
        if (nodes != null && nodes.getLength() > 0) {
            int len = nodes.getLength();
            for (int i=0; i < len; i++) {
                parseXML(nodes.item(i), rootMap, root.getNodeName(), rootMap);
            }
        }
        /*
         * Extract root node if any
         */
        if (map.size() == 1) {
            for (String key : map.keySet()) {
                Object rootNode = map.get(key);
                if (rootNode instanceof Map) {
                    return (Map<String, Object>) rootNode;
                }
            }
        }
        return map;
    }

    private void parseXML(Node node, Map<String, Object> parent, String grandParentName, Map<String, Object> grandParent) {

        if (node.getNodeType() == Node.TEXT_NODE) {
            String value = node.getTextContent().trim();
            if (value.length() > 0) {
                boolean hasAttributes = false;
                if (grandParent.containsKey(grandParentName)) {
                    hasAttributes = true;
                    if (parent.isEmpty()) {
                        grandParent.put(grandParentName, value);
                    } else {
                        parent.put(VALUE, value);
                    }

                }
                if (!hasAttributes) {
                    additivePut(grandParent, grandParentName, value);
                }
            }
        }
        if (node.getNodeType() == Node.ELEMENT_NODE) {
            Map<String, Object> childMap = new HashMap<>();
            additivePut(parent, node.getNodeName(), childMap);

            NamedNodeMap attributes = node.getAttributes();
            if (attributes != null) {
                int count = attributes.getLength();
                for (int i=0; i < count; i++) {
                    String attributeName = attributes.item(i).getNodeName();
                    if (!canIgnore(attributeName)) {
                        additivePut(childMap, attributeName, attributes.item(i).getNodeValue());
                    }
                }
            }
            NodeList nodes = node.getChildNodes();
            if (nodes != null && nodes.getLength() > 0) {
                int len = nodes.getLength();
                for (int i=0; i < len; i++) {
                    parseXML(nodes.item(i), childMap, node.getNodeName(), parent);
                }
            }

        }
    }

    @SuppressWarnings({ "unchecked", "rawtypes" })
    private void additivePut(Map<String, Object> map, String key, Object value) {
        if (map.containsKey(key)) {
            // turn into array
            Object o = map.get(key);
            if (o instanceof List) {
                List list = (List) o;
                list.add(value);
            } else {
                // skip empty map
                if (o instanceof Map) {
                    Map om = (Map) o;
                    if (om.isEmpty()) {
                        map.put(key, value);
                        return;
                    }
                }
                List<Object> nList = new ArrayList<>();
                nList.add(o);
                nList.add(value);
                map.put(key, nList);
            }

        } else {
            map.put(key, value);
        }
    }

    private boolean canIgnore(String attributeName) {
        return !drop.isEmpty() ? drop.contains(attributeName) : SimpleXmlParser.defaultDrop.contains(attributeName);
    }

}
