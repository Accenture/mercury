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

package org.platformlambda.quartz.services;

import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.system.Platform;
import org.platformlambda.core.system.PostOffice;
import org.platformlambda.quartz.MainScheduler;
import org.platformlambda.quartz.models.ScheduledJob;
import org.quartz.Job;
import org.quartz.JobDataMap;
import org.quartz.JobExecutionContext;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.io.IOException;
import java.util.Collections;
import java.util.Date;
import java.util.List;

public class JobExecutor implements Job {
    private static final Logger log = LoggerFactory.getLogger(JobExecutor.class);

    private static final String SCHEDULER_SERVICE = MainScheduler.SCHEDULER_SERVICE;

    /**
     * Quartz uses a thread pool to create a new instance of each job.
     * To alleviate this, we decouple it with the Mercury event driven service framework.
     * so it can release threads instantaneously for high performance scheduling.
     *
     * @param context for the job
     */
    @Override
    public void execute(JobExecutionContext context) {
        JobDataMap parameters = context.getJobDetail().getJobDataMap();
        ScheduledJob job = MainScheduler.getJob(parameters.getString(MainScheduler.JOB_ID));
        if (job != null) {
            EventEnvelope event = new EventEnvelope().setTo(job.service);
            event.setTrace(event.getId(), "JOB /schedule/"+job.name);
            event.setHeaders(job.parameters);
            /*
             * Leader election protocol
             * ------------------------
             *
             * When there are more than one instance of the scheduler running,
             * select the first one from a sorted list of app instance ID.
             *
             * This would avoid duplicated execution of a scheduled job.
             */
            String me = Platform.getInstance().getOrigin();
            PostOffice po = PostOffice.getInstance();
            List<String> peers = po.search(SCHEDULER_SERVICE, true);
            final String selected;
            if (peers.size() > 1) {
                Collections.sort(peers);
                selected = peers.get(0);
            } else  {
                selected = me;
            }
            if (me.equals(selected)) {
                if (po.exists(job.service)) {
                    try {
                        po.send(event);
                        job.lastExecution = new Date();
                        job.count++;
                        log.info("Execute service {} with parameters {}", job.service, job.parameters);
                    } catch (IOException e) {
                        log.error("Unable to execute service {} with parameters {} - {}",
                                job.service, job.parameters, e.getMessage());
                    }
                } else {
                    log.error("Unable to execute service {} with parameters {} - route {} not found",
                                job.service, job.parameters, job.service);
                }
            } else {
                log.info("Skip service {} with parameters {} because peer {} is handling it",
                        job.service, job.parameters, selected);
            }
        }
    }
}