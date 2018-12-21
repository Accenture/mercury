/*

    Copyright 2018 Accenture Technology

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

package org.platformlambda.core.serializers;

import com.fasterxml.jackson.core.JsonGenerator;
import com.fasterxml.jackson.databind.SerializerProvider;
import com.fasterxml.jackson.databind.ser.std.DateSerializer;
import org.platformlambda.core.util.Utility;

import java.io.IOException;
import java.util.Date;

public class IsoDateSerializer extends DateSerializer {

	private static final long serialVersionUID = 8461265902067852400L;

	@Override
    public void serialize(Date value, JsonGenerator jgen, SerializerProvider provider) throws IOException {

        if (_asTimestamp(provider)) {
            jgen.writeNumber(_timestamp(value));
        } else {
            jgen.writeString(Utility.getInstance().date2str(value));
        }
    }
}
