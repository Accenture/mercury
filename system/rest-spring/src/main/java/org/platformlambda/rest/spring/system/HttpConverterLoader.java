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

package org.platformlambda.rest.spring.system;

import org.platformlambda.rest.spring.serializers.HttpConverterHtml;
import org.platformlambda.rest.spring.serializers.HttpConverterJson;
import org.platformlambda.rest.spring.serializers.HttpConverterText;
import org.platformlambda.rest.spring.serializers.HttpConverterXml;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.springframework.boot.autoconfigure.http.HttpMessageConverters;
import org.springframework.context.annotation.Bean;
import org.springframework.stereotype.Component;

@Component
public class HttpConverterLoader {
    private static final Logger log = LoggerFactory.getLogger(HttpConverterLoader.class);

    @Bean
    public HttpMessageConverters getMessageConverters() {
        log.info("Loading HTTP serializers");
        HttpConverterJson json = new HttpConverterJson();
        HttpConverterXml xml = new HttpConverterXml();
        HttpConverterHtml html = new HttpConverterHtml();
        HttpConverterText text = new HttpConverterText();
        return new HttpMessageConverters(json, xml, html, text);
    }



}
