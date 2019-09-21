package org.platformlambda.automation.init;

import org.platformlambda.automation.MainApp;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;

import java.util.Date;
import java.util.Map;

public class InitialLoad implements LambdaFunction {

    private static final String TYPE = "type";
    private static final String DOWNLOAD = "download";
    private static final String INITIAL_LOAD = "initial.load";
    private static final String ORIGIN = "origin";
    private static final String DONE = "done";
    private static final long MAX_PING = 60000;
    private long start = System.currentTimeMillis();

    @Override
    public Object handleEvent(Map<String, String> headers, Object body, int instance) throws Exception {
        Platform platform = Platform.getInstance();
        long now = System.currentTimeMillis();
        if (DONE.equals(body) || now - start > MAX_PING) {
            platform.release(INITIAL_LOAD);
            return DONE;
        }
        // keep ping until we get an answer from a peer or the maximum ping interval is reached
        PostOffice po = PostOffice.getInstance();
        EventEnvelope download = new EventEnvelope().setTo(MainApp.NOTIFICATION_SERVICE);
        download.setHeader(TYPE, DOWNLOAD);
        download.setHeader(ORIGIN, platform.getOrigin());
        po.broadcast(download);
        // do it again
        po.sendLater(new EventEnvelope().setTo(INITIAL_LOAD).setBody("go"), new Date(now+5000));
        return null;
    }

}
