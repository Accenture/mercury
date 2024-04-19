package org.platformlambda.core.models;

import java.util.ArrayList;
import java.util.List;

public class PoJoList<T> {

    private final List<T> list;

    public PoJoList() {
        this.list = new ArrayList<>();
    }

    public PoJoList(List<T> list) {
        this.list = list;
    }

    public PoJoList<T> add(T item) {
        list.add(item);
        return this;
    }

    public List<T> getList() {
        return list;
    }

}
