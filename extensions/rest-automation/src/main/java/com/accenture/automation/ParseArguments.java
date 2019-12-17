package com.accenture.automation;

import org.platformlambda.core.annotations.BeforeApplication;
import org.platformlambda.core.models.EntryPoint;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

@BeforeApplication(sequence = 1)
public class ParseArguments implements EntryPoint {
    private static final Logger log = LoggerFactory.getLogger(ParseArguments.class);

    private static final String HTML_FOLDER = "spring.resources.static-locations";
    private static final String HTML_ARG = "-html";
    private static final String FILE_PATH = "file:/";

    @Override
    public void start(String[] args) {
        if (args.length > 0) {
            if (args.length != 2 || !HTML_ARG.equals(args[0])) {
                log.error("Usage: java -jar rest-automation.jar -html file_path");
                System.exit(-1);
            }
            if (!args[1].startsWith(FILE_PATH)) {
                log.error("Usage: java -jar rest-automation.jar -html file_path");
                log.error("       file_path must start with {}", FILE_PATH);
                System.exit(-1);
            }
            System.setProperty(HTML_FOLDER, args[1]);
            log.info("Using HTML folder at {}", args[1]);
        }
    }
}
