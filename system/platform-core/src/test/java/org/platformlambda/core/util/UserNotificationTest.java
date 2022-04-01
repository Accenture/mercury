package org.platformlambda.core.util;

import org.junit.Assert;
import org.junit.Test;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;

import java.io.IOException;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.TimeoutException;

public class UserNotificationTest {

    private static final String NOTIFICATION_MANAGER = "notification.manager";

    @Test
    public void webUserNotificationTest() throws IOException, AppException, TimeoutException {
        Platform platform = Platform.getInstance();
        String ME = platform.getOrigin();
        String TYPE = "type";
        String LIST = "list";
        String TOPIC = "topic";
        String PUBLISH = "publish";
        String TEST_TOPIC = "test.topic";
        LambdaFunction mock = (headers, body, instance) -> {
            String type = headers.get(TYPE);
            if (LIST.equals(type)) {
                if (headers.containsKey(TOPIC)) {
                    Map<String, List<String>> result = new HashMap<>();
                    result.put(ME, Collections.singletonList("ws.12345.1"));
                    return result;
                }
                return Collections.singletonList(TEST_TOPIC);
            }
            if (PUBLISH.equals(type)) {
                return true;
            }
            return false;
        };
        platform.registerPrivate(NOTIFICATION_MANAGER, mock, 1);
        UserNotification notifier = UserNotification.getInstance();
        List<String> topics = notifier.listTopics();
        Assert.assertTrue(topics.contains(TEST_TOPIC));
        Map<String, List<String>> connections = notifier.getTopic(TEST_TOPIC);
        Assert.assertTrue(connections.containsKey(ME));
        notifier.publish(TEST_TOPIC, "hello world");
    }

}
