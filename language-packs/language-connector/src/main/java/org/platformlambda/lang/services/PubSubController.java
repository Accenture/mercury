package org.platformlambda.lang.services;

import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.PubSub;
import org.platformlambda.lang.websocket.server.LanguageConnector;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.Collections;
import java.util.List;
import java.util.Map;

public class PubSubController implements LambdaFunction {
    private static final Logger log = LoggerFactory.getLogger(PubSubController.class);

    private static final String TYPE = "type";
    private static final String TOPIC = "topic";
    private static final String CREATE = "create";
    private static final String DELETE = "delete";
    private static final String PUBLISH = "publish";
    private static final String SUBSCRIBE = "subscribe";
    private static final String UNSUBSCRIBE = "unsubscribe";
    private static final String FEATURE = "feature";
    private static final String LIST = "list";
    private static final String EXISTS = "exists";
    private static final String HEADERS = "headers";
    private static final String BODY = "body";
    private static final String ROUTE = "route";

    @Override
    @SuppressWarnings("unchecked")
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {

        PubSub engine = PubSub.getInstance();
        if (headers.containsKey(TYPE)) {
            String type = headers.get(TYPE);
            if (FEATURE.equals(type)) {
                return engine.featureEnabled();
            }
            if (LIST.equals(type)) {
                return engine.list();
            }
            if (headers.containsKey(TOPIC)) {
                String topic = headers.get(TOPIC);
                if (EXISTS.equals(type)) {
                    return engine.exists(topic);
                }
                if (CREATE.equals(type)) {
                    return engine.createTopic(topic);
                }
                if (DELETE.equals(type)) {
                    engine.deleteTopic(topic);
                    return true;
                }
                if (PUBLISH.equals(type) && body instanceof Map) {
                    Map<String, Object> map = (Map<String, Object>) body;
                    engine.publish(topic, (Map<String, String>) map.get(HEADERS), map.get(BODY));
                    return true;
                }
                if (SUBSCRIBE.equals(type) && headers.containsKey(ROUTE)) {
                    String route = headers.get(ROUTE);
                    if (LanguageConnector.hasRoute(route)) {
                        List<String> para = body instanceof List? (List<String>) body : Collections.EMPTY_LIST;
                        TopicListener listener = TopicListener.getListener(topic);
                        if (listener != null) {
                            listener.addRoute(route);
                            log.info("Relaying topic {} to {}", topic, listener.getRoutes());
                        } else {
                            engine.subscribe(topic, new TopicListener(topic, route), getParameters(para));
                            log.info("Relaying topic {} to {}", topic, route);
                        }
                        return true;
                    } else {
                        throw new AppException(404, "Route "+route+" not registered with "+topic);
                    }
                }
                if (UNSUBSCRIBE.equals(type) && headers.containsKey(ROUTE)) {
                    String route = headers.get(ROUTE);
                    if (LanguageConnector.hasRoute(route)) {
                        TopicListener listener = TopicListener.getListener(topic);
                        if (listener != null) {
                            listener.removeRoute(route);
                            log.info("Unsubscribed {} from topic {}", route, topic);
                            return true;
                        } else {
                            throw new AppException(404, "Route "+route+" not registered with "+topic);
                        }
                    } else {
                        throw new AppException(404, "Route "+route+" not registered with "+topic);
                    }
                }
            }
        }
        return false;
    }

    private String[] getParameters(List<String> parameters) {
        String[] result = new String[parameters.size()];
        for (int i=0; i < parameters.size(); i++) {
            result[i] = parameters.get(i);
        }
        return result;
    }
}
