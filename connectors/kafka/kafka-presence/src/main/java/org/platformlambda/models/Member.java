package org.platformlambda.models;

import org.platformlambda.core.util.Utility;

public class Member {

    public int token;
    public long updated;

    public Member(String token, long updated) {
        this.token = Utility.getInstance().str2int(token);
        this.updated = updated;
    }
}
