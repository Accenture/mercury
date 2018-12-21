package org.platformlambda.core.annotations;

import java.lang.annotation.*;

/**
 * The selection of cloud service modules is defined at run-time using application.properties "cloud.services"
 */
@Target({ElementType.TYPE})
@Retention(RetentionPolicy.RUNTIME)
@Documented
public @interface CloudService {

    String value();
}
