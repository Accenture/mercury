/*

    Copyright 2018-2023 Accenture Technology

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.

 */

package org.platformlambda.automation.models;

import org.platformlambda.core.util.Utility;

import java.util.List;

public class EtagFile {

    public String eTag;
    public String name;
    public byte[] content;

    public EtagFile(String eTag, byte[] content) {
        this.eTag = "\""+ eTag +"\"";
        this.content = content;
    }

    public boolean sameTag(String eTag) {
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
