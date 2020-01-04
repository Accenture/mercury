/*

    Copyright 2018-2020 Accenture Technology

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

package org.platformlambda.core.models;

public class VersionInfo {

    private String groupId, artifactId, version;

    public String getGroupId() {
        return groupId;
    }

    public String getArtifactId() {
        return artifactId;
    }

    public String getVersion() {
        return version;
    }

    public VersionInfo setGroupId(String groupId) {
        this.groupId = groupId;
        return this;
    }

    public VersionInfo setArtifactId(String artifactId) {
        this.artifactId = artifactId;
        return this;
    }

    public VersionInfo setVersion(String version) {
        this.version = version;
        return this;
    }

    public boolean isEmpty() {
        return groupId == null;
    }

    public String toString() {
        return groupId+", "+artifactId+", version "+version;
    }

}
