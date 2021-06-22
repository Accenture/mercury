package com.accenture.mock;

import org.platformlambda.core.annotations.OptionalService;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import javax.servlet.ServletContextEvent;
import javax.servlet.ServletContextListener;
import javax.servlet.annotation.WebListener;

@WebListener
@OptionalService("no.such.parameter")
public class ConditionalWebListener implements ServletContextListener {
    private static final Logger log = LoggerFactory.getLogger(MockWebListener.class);

    public void contextInitialized(ServletContextEvent event) {
        log.info("Initialized {}", event);
    }

    public void contextDestroyed(ServletContextEvent event) {
        log.info("Destroyed {}", event);
    }

}
