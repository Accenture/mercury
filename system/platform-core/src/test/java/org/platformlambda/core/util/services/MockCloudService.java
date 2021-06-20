package org.platformlambda.core.util.services;

import org.platformlambda.core.annotations.CloudService;
import org.platformlambda.core.models.CloudSetup;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

@CloudService(name="mock.cloud.service")
public class MockCloudService implements CloudSetup {
    private static final Logger log = LoggerFactory.getLogger(MockCloudService.class);

    @Override
    public void initialize() {
        log.info("Mock cloud service started");
    }

}
