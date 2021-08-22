package org.platformlambda.models;

import org.platformlambda.core.util.Utility;

import java.util.List;

public class EtagFile {

    public String eTag;
    public byte[] content;

    public EtagFile(String eTag, byte[] content) {
        this.eTag = "\""+ eTag +"\"";
        this.content = content;
    }

    public boolean equals(String eTag) {
        if (eTag == null) {
            return false;
        }
        if (eTag.contains(",")) {
            List<String> parts = Utility.getInstance().split(eTag, ", ");
            for (String p: parts) {
                if (this.eTag.equals(p)) {
                    return true;
                }
            }
            return false;
        } else {
            return this.eTag.equals(eTag);
        }
    }
}
