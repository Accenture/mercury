package org.platformlambda.core.models;

import akka.actor.ActorRef;

import java.util.List;

public class TargetRoute {

    public List<String> paths = null;
    public ActorRef actor = null;
    private boolean remote = false;

    public TargetRoute(List<String> paths) {
        this.paths = paths;
    }

    public TargetRoute(ActorRef actor, boolean remote) {
        this.actor = actor;
        this.remote = remote;
    }

    public boolean isWebsocket() {
        return paths != null;
    }

    public boolean isRemote() {
        return remote;
    }

    public String toString() {
        return "TARGET ROUTE: paths="+paths+", actor="+actor+", remote?"+remote;
    }

}
