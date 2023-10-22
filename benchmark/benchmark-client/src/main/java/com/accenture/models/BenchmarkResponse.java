package com.accenture.models;

import java.util.Date;

public class BenchmarkResponse {

    public Date oneTrip;
    public Date roundTrip;

    public BenchmarkResponse(Date oneTrip) {
        this.oneTrip = oneTrip;
        this.roundTrip = new Date();
    }

}
