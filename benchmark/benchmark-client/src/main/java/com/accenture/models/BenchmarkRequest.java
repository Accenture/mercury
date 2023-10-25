package com.accenture.models;

import org.platformlambda.core.util.Utility;

import java.util.Date;

public class BenchmarkRequest {

    public boolean isEcho;
    public int count;
    public int size;
    public Date start;
    public String cid;
    public long timeSpendPublishing = 0;
    public int received = 0;

    public BenchmarkRequest(boolean isEcho, int count, int size) {
        this.isEcho = isEcho;
        this.count = Math.max(1, count);
        this.size = Math.max(10, (size / 10) * 10); // to be nearest 10
        this.start = new Date();
        this.cid = Utility.getInstance().getUuid();
    }
}
