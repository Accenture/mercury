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

package org.platformlambda.quartz;

import org.platformlambda.core.annotations.MainApplication;
import org.platformlambda.core.models.EntryPoint;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.LambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.EventEmitter;
import org.platformlambda.core.util.AppConfigReader;
import org.platformlambda.core.util.ConfigReader;
import org.platformlambda.core.util.Utility;
import org.platformlambda.quartz.models.ScheduledJob;
import org.platformlambda.quartz.services.JobExecutor;
import org.platformlambda.rest.RestServer;
import org.quartz.*;
import org.quartz.impl.StdSchedulerFactory;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.ArrayList;
import java.util.Date;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

@MainApplication
public class MainScheduler implements EntryPoint {
    private static final Logger log = LoggerFactory.getLogger(MainScheduler.class);
    public static final String JOB_ID = "job_id";
    public static final String SCHEDULER_SERVICE = "cron.scheduler";
    public static final String GLOBAL_GROUP = "global";
    public static final String TYPE = "type";
    public static final String START_COMMAND = "start";
    public static final String STOP_COMMAND = "stop";
    public static final String ORIGIN = "origin";
    private static final String SCHEDULER = "scheduler";
    private static final String JOBS = "jobs[";
    private static final ConcurrentMap<String, ScheduledJob> scheduledJobs = new ConcurrentHashMap<>();

    private static Scheduler schedulerService;

    public static void main(String[] args) {
        RestServer.main(args);
    }

    public static ScheduledJob getJob(String id) {
        return id == null? null : scheduledJobs.get(id);
    }

    public static List<String> getJobs() {
        return new ArrayList<>(scheduledJobs.keySet());
    }

    public static void stopJob(String id) throws SchedulerException {
        ScheduledJob j = getJob(id);
        if (j != null && j.stopTime == null) {
            schedulerService.deleteJob(j.job.getKey());
            j.stopTime = new Date();
            j.startTime = null;
            log.info("Stopped job={}, service={}, cron={}", id, j.service, j.cronSchedule);
        }
    }

    public static void startJob(String id) throws SchedulerException {
        ScheduledJob j = getJob(id);
        if (j != null && j.startTime == null) {
            schedulerService.addJob(j.job, true);
            CronTrigger trigger = TriggerBuilder.newTrigger()
                    .withSchedule(CronScheduleBuilder.cronSchedule(j.cronSchedule))
                    .forJob(id, GLOBAL_GROUP).build();
            j.startTime = new Date();
            j.stopTime = null;
            schedulerService.scheduleJob(trigger);
            log.info("Scheduled job={}, service={}, cron={}", id, j.service, j.cronSchedule);
        }
    }

    public static void executeJobNow(String id) throws SchedulerException {
        ScheduledJob j = getJob(id);
        if (j != null && j.stopTime == null) {
            EventEnvelope event = new EventEnvelope().setTo(j.service);
            event.setHeaders(j.parameters);
            try {
                EventEmitter.getInstance().send(event);
                j.lastExecution = new Date();
                j.count++;
            } catch (IOException e) {
                log.error("Unable to execute service {} with parameters {}", j.service, j.parameters);
            }
        } else {
            throw new IllegalArgumentException("Job "+id+" has not started");
        }
    }

    @Override
    @SuppressWarnings("unchecked")
    public void start(String[] args) throws Exception {
        Platform platform = Platform.getInstance();
        String origin = platform.getOrigin();
        // create service for leader election and synchronization of job start/stop status
        LambdaFunction f = (headers, input, instance) -> {
            if (!origin.equals(headers.get(ORIGIN)) && headers.containsKey(JOB_ID)) {
                if (START_COMMAND.equals(headers.get(TYPE))) {
                    startJob(headers.get(JOB_ID));
                }
                if (STOP_COMMAND.equals(headers.get(TYPE))) {
                    stopJob(headers.get(JOB_ID));
                }
                return true;
            } else {
                return false;
            }
        };
        platform.register(SCHEDULER_SERVICE, f, 1);
        // setup cron jobs
        ConfigReader config = getConfig();
        Object o = config.get("jobs");
        if (o instanceof List) {
            List<Object> list = (List<Object>) o;
            for (int i=0; i < list.size(); i++) {
                // use composite key so values can be overridden by application.properties and system.properties
                String name = config.getProperty(JOBS+i+"].name");
                String service = config.getProperty(JOBS+i+"].service");
                String schedule = config.getProperty(JOBS+i+"].cron");
                String desc = config.getProperty(JOBS+i+"].description");
                Object parameters = config.get(JOBS+i+"].parameters");
                if (name != null && schedule != null && service != null) {
                    ScheduledJob j = new ScheduledJob(name, service, schedule);
                    j.description = desc == null? name : desc;
                    if (parameters instanceof Map) {
                        Map<String, Object> map = (Map<String, Object>) parameters;
                        for (String k: map.keySet()) {
                            j.addParameter(k, map.get(k));
                        }
                    }
                    j.addParameter(SCHEDULER, origin);
                    scheduledJobs.put(name, j);

                } else {
                    log.error("Invalid job entry - missing name, schedule and service - {}", list.get(i));
                }
            }
        }
        // Schedule jobs now
        SchedulerFactory schedulerFactory = new StdSchedulerFactory();
        schedulerService = schedulerFactory.getScheduler();
        // load jobs
        for (String id: scheduledJobs.keySet()) {
            ScheduledJob j = scheduledJobs.get(id);
            j.job = JobBuilder.newJob(JobExecutor.class).storeDurably(true)
                              .usingJobData(JOB_ID, id).withIdentity(id, GLOBAL_GROUP).build();
            schedulerService.addJob(j.job, true);
            CronTrigger trigger = TriggerBuilder.newTrigger()
                    .withSchedule(CronScheduleBuilder.cronSchedule(j.cronSchedule))
                    .forJob(id, GLOBAL_GROUP).build();
            j.startTime = new Date();
            schedulerService.scheduleJob(trigger);
            log.info("Scheduled job={}, service={}, cron={}", id, j.service, j.cronSchedule);
        }
        schedulerService.start();
        // shutdown schedule when app stops
        Runtime.getRuntime().addShutdownHook(new Thread(() -> {
            try {
                schedulerService.shutdown();
            } catch (SchedulerException e) {
                log.error("Error while stopping scheduler - {}", e.getMessage());
            }
        }));
        // connect to cloud
        platform.connectToCloud();
    }

    private ConfigReader getConfig() throws IOException {
        AppConfigReader reader = AppConfigReader.getInstance();
        List<String> paths = Utility.getInstance().split(reader.getProperty("cron.yaml",
                "file:/tmp/config/cron.yaml, classpath:/cron.yaml"), ", ");
        for (String p: paths) {
            ConfigReader config = new ConfigReader();
            try {
                config.load(p);
                log.info("Loading config from {}", p);
                return config;
            } catch (IOException e) {
                log.warn("Skipping {} - {}", p, e.getMessage());
            }
        }
        throw new IOException("Scheduler configuration not found in "+paths);
    }

}
