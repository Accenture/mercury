/*

    Copyright 2018-2020 Accenture Technology

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

package org.platformlambda.core.util;

import org.junit.Test;
import org.quartz.*;
import org.quartz.impl.StdSchedulerFactory;

import java.util.Date;

public class SchedulerTest {

    private static final String JOB_ID = "job_id";
    private static final String GLOBAL_GROUP = "global";

    /**
     * enable "@Test" to do simple confirmation test
     *
     * @throws SchedulerException in case of error
     * @throws InterruptedException in case of job creation error
     */
//    @Test
    public void cronTest() throws SchedulerException, InterruptedException {

        String MY_JOB = "my_job";

        SchedulerFactory schedulerFactory = new StdSchedulerFactory();
        Scheduler scheduler = schedulerFactory.getScheduler();

        JobDetail job  =  JobBuilder.newJob(SimpleJob.class).storeDurably(true)
                                    .usingJobData(JOB_ID, MY_JOB)
                                    .withIdentity(MY_JOB, GLOBAL_GROUP).build();
        scheduler.addJob(job, true);

        CronTrigger trigger = TriggerBuilder.newTrigger()
                .withSchedule(CronScheduleBuilder.cronSchedule("0 0/1 * 1/1 * ? *"))
                .forJob(MY_JOB, GLOBAL_GROUP)
                .build();

        scheduler.scheduleJob(trigger);
        scheduler.start();

        for (int i=1; i < 240; i++) {
            System.out.println("waiting..."+i);
            Thread.sleep(10000);
        }

        scheduler.shutdown();
    }

    public static class SimpleJob implements Job {
        /**
         * Quartz uses a thread pool to create a new instance of each job
         * It is heavy weight. However, coupling it with an event driven framework
         * means it can release the thread instantaneously so that the scheduler
         * becomes non-blocking.
         *
         * @param context for the job
         */
        @Override
        public void execute(JobExecutionContext context) {
            JobDataMap parameters = context.getJobDetail().getJobDataMap();
            String job = parameters.getString(JOB_ID);
            System.out.println("running----"+job+"-----"+new Date());
        }
    }

}
