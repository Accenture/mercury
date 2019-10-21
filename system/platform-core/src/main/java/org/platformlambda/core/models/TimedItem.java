package org.platformlambda.core.models;

public class TimedItem {

    public Long time;
    public Object payload;

    public TimedItem(Long time, Object payload) {
        this.time = time;
        this.payload = payload;
    }
}
