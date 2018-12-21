package org.platformlambda.core.models;

public class TypedPayload {

    private String type, parametricType;
    private Object payload;

    public TypedPayload(String type, Object payload) {
        this.type = type;
        this.payload = payload;
    }

    public String getType() {
        return type;
    }

    public Object getPayload() {
        return payload;
    }

    public void setParametricType(String parametricType) {
        this.parametricType = parametricType;
    }

    public String getParametricType() {
        return parametricType;
    }

}
