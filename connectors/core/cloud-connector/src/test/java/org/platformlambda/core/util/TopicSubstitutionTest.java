package org.platformlambda.core.util;

import org.junit.Assert;
import org.junit.Test;
import org.platformlambda.cloud.ConnectorConfig;

import java.io.IOException;
import java.util.Map;

public class TopicSubstitutionTest {

    @Test
    public void substitution() throws IOException {
        Map<String, String> topicMap = ConnectorConfig.getTopicSubstitution();
        Assert.assertTrue(topicMap.containsKey("multiplex.0001.0"));
        Assert.assertEquals("user.topic.one", topicMap.get("multiplex.0001.0"));
    }
}
