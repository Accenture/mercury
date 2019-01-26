/*

    Copyright 2018-2019 Accenture Technology

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

import org.platformlambda.core.util.Utility;

import java.text.DateFormat;
import java.text.FieldPosition;
import java.text.NumberFormat;
import java.text.ParsePosition;
import java.util.Calendar;
import java.util.Date;

/**
 * This is a thread-safe replacement for SimpleDateFormatter
 */
public class FastDateFormatter extends DateFormat {

	private static final long serialVersionUID = 5502835009949348866L;

	@Override
    public StringBuffer format(Date date, StringBuffer toAppendTo, FieldPosition fieldPosition) {
        // this is used by ObjectMapper so we can ignore fieldPosition
        String result = Utility.getInstance().date2str(date);
        return toAppendTo == null? new StringBuffer(result) : toAppendTo.append(result);
    }

    @Override
    public Date parse(String source, ParsePosition pos) {
        // this is used by ObjectMapper so we can ignore pos
        return Utility.getInstance().str2date(source);
    }

    @Override
    public Object clone() {
        FastDateFormatter other = new FastDateFormatter();
        other.calendar = Calendar.getInstance();
        other.numberFormat = NumberFormat.getInstance();
        return other;
    }

}
