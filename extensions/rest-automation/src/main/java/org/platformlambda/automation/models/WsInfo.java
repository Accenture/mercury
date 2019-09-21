package org.platformlambda.automation.models;

public class WsInfo {

    public String appName, authService, userService;

    public WsInfo(String appName, String authService, String userService) {
        this.appName = appName;
        this.authService = authService;
        this.userService = userService;
    }
}
