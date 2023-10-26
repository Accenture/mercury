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

package com.accenture.services;

import com.accenture.models.BenchmarkRequest;
import com.accenture.models.BenchmarkResponse;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.EventEmitter;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.util.Utility;

import java.io.IOException;
import java.net.URI;
import java.net.URISyntaxException;
import java.text.NumberFormat;
import java.util.Date;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

public class BenchmarkService implements LambdaFunction {

    private static final String BENCHMARK_USERS = "benchmark.users";
    private static final String BENCHMARK_CALLBACK = "benchmark.callback";
    private static final String NETWORK_ONE_WAY = "network.one.way";
    private static final String NETWORK_ECHO = "network.echo";
    private static final String SENDER = "sender";
    private static final String COMMAND = "command";
    private static final String PAYLOAD = "payload";
    private static final String START = "start";
    private static final String ASYNC = "async";
    private static final String ECHO = "echo";
    private static final String HTTP = "http";
    private static final String HTTPS = "https";
    private static final String TIME = "time";
    private static final String EVENTS_PER_SECOND = " events per second";
    private static final String MS = " ms";
    private static final String BPS = " bps";
    private static final String WARNING = " WARN: ";
    private static boolean testRunning = false;
    private static BenchmarkRequest benchmarkRequest;
    private static String httpTarget;
    private static final ConcurrentMap<Integer, BenchmarkResponse> responses = new ConcurrentHashMap<>();

    @SuppressWarnings("unchecked")
    public BenchmarkService() throws IOException {
        LambdaFunction callback = (headers, input, instance) -> {
            Utility util = Utility.getInstance();
            EventEmitter po = EventEmitter.getInstance();
            benchmarkRequest.received++;
            if (benchmarkRequest.received % 100 == 0) {
                po.send(BENCHMARK_USERS, util.getLocalTimestamp()+
                        " INFO: received "+benchmarkRequest.received+" event responses");
            }
            if (input instanceof Map) {
                Map<String, Object> data = (Map<String, Object>) input;
                if (data.containsKey(TIME)) {
                    Date oneTrip = util.str2date(data.get(TIME).toString());
                    responses.put(benchmarkRequest.received, new BenchmarkResponse(oneTrip));
                }
            }
            if (benchmarkRequest.received >= benchmarkRequest.count) {
                testRunning = false;
                po.send(BENCHMARK_USERS, util.getLocalTimestamp()+" INFO: Benchmark completed.");
                calculateBenchmark();
            }
            return true;
        };
        Platform.getInstance().registerPrivate(BENCHMARK_CALLBACK, callback,1);
    }

    private void calculateBenchmark() throws IOException {
        NumberFormat number = NumberFormat.getInstance();
        int count = benchmarkRequest.count;
        int size = benchmarkRequest.size;
        Date start = benchmarkRequest.start;
        Utility util = Utility.getInstance();
        EventEmitter po = EventEmitter.getInstance();
        String command = benchmarkRequest.type+" "+number.format(count)+" "+ PAYLOAD+" "+number.format(size);
        po.send(BENCHMARK_USERS, util.getLocalTimestamp()+" INFO: Command = "+command);
        po.send(BENCHMARK_USERS, util.getLocalTimestamp()+" INFO: Sent = "+number.format(count));
        po.send(BENCHMARK_USERS, util.getLocalTimestamp()+" INFO: Received = "+number.format(responses.size()));
        po.send(BENCHMARK_USERS, util.getLocalTimestamp()+
                " INFO: Total time spent in publishing = "+number.format(benchmarkRequest.timeSpendPublishing)+MS);

        if (benchmarkRequest.timeSpendPublishing > 0) {
            float publishRate = ((float) count * 1000) / benchmarkRequest.timeSpendPublishing;
            float publishBps = publishRate * size * 8;
            po.send(BENCHMARK_USERS, util.getLocalTimestamp()+
                    " INFO: Publish rate = "+number.format(publishRate)+EVENTS_PER_SECOND+", "+
                    number.format(publishBps)+BPS);
        }

        long minOneTrip = Long.MAX_VALUE;
        long maxOneTrip = 0;
        long minRoundTrip = Long.MAX_VALUE;
        long maxRoundTrip = 0;
        Date now = new Date();
        long totalTime = now.getTime() - start.getTime();
        po.send(BENCHMARK_USERS, util.getLocalTimestamp()+
                " INFO: Total time spent end-to-end = "+number.format(totalTime)+" ms");

        for (Map.Entry<Integer, BenchmarkResponse> entry: responses.entrySet()) {
            BenchmarkResponse res = entry.getValue();
            long oneTripLatency = Math.abs(res.oneTrip.getTime() - start.getTime());
            long roundTripLatency = Math.abs(res.roundTrip.getTime() - start.getTime());
            if (oneTripLatency < minOneTrip) {
                minOneTrip = oneTripLatency;
            }
            if (oneTripLatency > maxOneTrip) {
                maxOneTrip = oneTripLatency;
            }
            if (roundTripLatency < minRoundTrip) {
                minRoundTrip = roundTripLatency;
            }
            if (roundTripLatency > maxRoundTrip) {
                maxRoundTrip = roundTripLatency;
            }
        }
        po.send(BENCHMARK_USERS, util.getLocalTimestamp()+
                " INFO: First oneTrip event arrives in "+number.format(minOneTrip)+MS);
        po.send(BENCHMARK_USERS, util.getLocalTimestamp()+
                " INFO: Last oneTrip event arrives in "+number.format(maxOneTrip)+MS);

        po.send(BENCHMARK_USERS, util.getLocalTimestamp()+
                " INFO: First roundTrip event returns in "+number.format(minRoundTrip)+MS);
        po.send(BENCHMARK_USERS, util.getLocalTimestamp()+
                " INFO: Last roundTrip event returns in "+number.format(maxRoundTrip)+MS);

        if (maxOneTrip > 0) {
            float avgOneTrip = ((float) count * 1000) / maxOneTrip;
            float oneTripBps = avgOneTrip * size * 8;
            po.send(BENCHMARK_USERS, util.getLocalTimestamp()+
                    " INFO: oneTrip = "+number.format(avgOneTrip)+EVENTS_PER_SECOND+", "+
                    number.format(oneTripBps)+BPS);
        }
        if (maxRoundTrip > 0) {
            float avgRoundTrip = ((float) count * 1000) / maxRoundTrip;
            float roundTripBps = avgRoundTrip * size * 8;
            // if payload is echoed, traffic volume should be doubled.
            if (!ASYNC.equals(benchmarkRequest.type)) {
                roundTripBps *= 2;
            }
            po.send(BENCHMARK_USERS, util.getLocalTimestamp() +
                    " INFO: roundTrip = " + number.format(avgRoundTrip) + EVENTS_PER_SECOND+", "+
                    number.format(roundTripBps)+BPS);
        }
    }
    @Override
    public Object handleEvent(Map<String, String> headers, Object input, int instance) throws IOException {
        String me = Platform.getInstance().getOrigin();
        Utility util = Utility.getInstance();
        EventEmitter po = EventEmitter.getInstance();

        if (headers.containsKey(SENDER) && headers.containsKey(COMMAND)) {
            String sender = headers.get(SENDER);
            String command = headers.get(COMMAND);
            if ("help".equals(command)) {
                po.send(sender, util.getLocalTimestamp()+" INFO: Available commands");
                po.send(sender, "Note: 'async' is one-way event and 'echo' is two-way. 'http' is event over HTTP.");
                po.send(sender, "help <-- this command");
                po.send(sender, "reset <-- restart benchmark in case of failure");
                po.send(sender, "clear <-- clear browser display area");
                po.send(sender, "set target {EventOverHttpTarget} <-- e.g. set target http://127.0.0.1:8083");
                po.send(sender, "async | echo | http {count} payload {size in bytes}");
                return true;
            }
            if ("reset".equals(command)) {
                testRunning = false;
                po.send(sender, util.getLocalTimestamp()+" INFO: Benchmark reset.");
                return true;
            }
            List<String> parts = util.split(command.replace(",", ""), " ");
            if ("get".equals(parts.get(0)) && "target".equals(parts.get(1))) {
                po.send(sender, util.getLocalTimestamp()+" INFO: EventOverHTTP = "+
                        (httpTarget == null? " not set" : httpTarget));
                return true;
            }
            if (parts.size() == 3 && "set".equals(parts.get(0)) && "target".equals(parts.get(1))) {
                final URI url;
                try {
                    url = new URI(parts.get(2));
                } catch (URISyntaxException e) {
                    po.send(sender, util.getLocalTimestamp()+ WARNING+e.getMessage());
                    return false;
                }
                if (!(url.getPath().isEmpty() || url.getPath().equals("/api/event"))) {
                    po.send(sender, util.getLocalTimestamp()+
                            " WARN: Invalid target. It should be http or https with host and port");
                    return false;
                }
                String scheme = url.getScheme();
                if (!(HTTP.equals(scheme) || HTTPS.equals(scheme))) {
                    po.send(sender, util.getLocalTimestamp()+
                            " WARN: Invalid target. Protocol should be http or https");
                    return false;
                }
                int port = url.getPort();
                if (port == -1) {
                    port = HTTP.equals(scheme)? 80 : 443;
                }
                httpTarget = url.getScheme()+"://"+url.getHost()+":"+port+"/api/event";
                po.send(sender, util.getLocalTimestamp()+" INFO: EventOverHttp endpoint set to "+httpTarget);
                return true;
            }
            if (testRunning) {
                po.send(sender, util.getLocalTimestamp()+
                        " WARN: Benchmark in progress. Please try later or enter 'reset' to continue.");
                return false;
            }
            try {
                benchmarkRequest = parseCommand(parts);
                if (benchmarkRequest.count > 5000) {
                    throw new IllegalArgumentException("Max event count is 5,000");
                }
                if (benchmarkRequest.size > 2000 * 1000) {
                    throw new IllegalArgumentException("Max payload size is 2,000,000");
                }
                String target = ASYNC.equals(benchmarkRequest.type)? NETWORK_ONE_WAY : NETWORK_ECHO;
                StringBuilder sb = new StringBuilder();
                int cycles = benchmarkRequest.size / 10;
                for (int i=0; i < cycles; i++) {
                    sb.append("123456789.");
                }
                Map<String, Object> data = new HashMap<>();
                data.put(SENDER, sender);
                data.put(PAYLOAD, sb.toString());
                data.put(START, new Date());

                responses.clear();
                testRunning = true;

                long start = System.currentTimeMillis();

                if (HTTP.equals(benchmarkRequest.type)) {
                    if (httpTarget == null) {
                        throw new IllegalArgumentException("Please set target for EventOverHttp first");
                    }
                    for (int i=0; i < benchmarkRequest.count; i++) {
                        EventEnvelope request = new EventEnvelope().setTo(target).setBody(data)
                                .setCorrelationId(benchmarkRequest.cid)
                                .setReplyTo(BENCHMARK_CALLBACK+"@"+me);
                        po.asyncRequest(request, 30000, new HashMap<>(), httpTarget, true)
                            .onSuccess(res -> {
                                try {
                                    po.send(new EventEnvelope().setTo(BENCHMARK_CALLBACK)
                                            .setBody(res.getBody()).setCorrelationId(benchmarkRequest.cid));
                                } catch (IOException e) {
                                    try {
                                        po.send(sender, util.getLocalTimestamp()+WARNING+e.getMessage());
                                    } catch (IOException ex) {
                                        // ok to ignore
                                    }
                                }
                            });
                    }
                } else {
                    for (int i=0; i < benchmarkRequest.count; i++) {
                        EventEnvelope request = new EventEnvelope().setTo(target).setBody(data)
                                .setCorrelationId(benchmarkRequest.cid)
                                .setReplyTo(BENCHMARK_CALLBACK+"@"+me);
                        po.send(request);
                    }
                }
                benchmarkRequest.timeSpendPublishing = System.currentTimeMillis() - start;

            } catch(IllegalArgumentException e) {
                po.send(sender, util.getLocalTimestamp()+WARNING+e.getMessage());
            }
        }
        return true;
    }

    private BenchmarkRequest parseCommand(List<String> parts) {
        Utility util = Utility.getInstance();
        if (parts.size() == 4 &&
                ((parts.get(0).equals(ASYNC) || parts.get(0).equals(ECHO) || parts.get(0).equals(HTTP)) &&
                    parts.get(2).equals(PAYLOAD) &&
                    util.isDigits(parts.get(1)) && util.isDigits(parts.get(3)))) {
                int count = util.str2int(parts.get(1));
                int size = util.str2int(parts.get(3));
                return new BenchmarkRequest(parts.get(0), count, size);

        }
        throw new IllegalArgumentException("Syntax: async | echo {count} payload {size in bytes}");
    }

}
