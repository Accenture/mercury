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

package org.platformlambda.services;

import org.platformlambda.core.annotations.CoroutineRunner;
import org.platformlambda.core.annotations.PreLoad;
import org.platformlambda.core.exception.AppException;
import org.platformlambda.core.models.AsyncHttpRequest;
import org.platformlambda.core.models.EventEnvelope;
import org.platformlambda.core.models.TypedLambdaFunction;
import org.platformlambda.core.system.Platform;
import org.platformlambda.models.ObjectWithGenericType;
import org.platformlambda.models.SamplePoJo;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

import java.util.Date;
import java.util.Map;
import java.util.concurrent.atomic.AtomicInteger;
import org.platformlambda.example.MainApp;

import com.mongodb.ConnectionString;
import com.mongodb.MongoClientSettings;
import com.mongodb.MongoException;
import com.mongodb.ServerApi;
import com.mongodb.ServerApiVersion;
import com.mongodb.client.MongoClient;
import com.mongodb.client.MongoClients;
import com.mongodb.client.MongoDatabase;
import org.bson.Document;

@PreLoad(route = "write.mdb.record", instances = 10)
public class WriteMDBRecord implements TypedLambdaFunction<AsyncHttpRequest, Object> {
    private static final MongoDatabase db = MainApp.getDBConnection();
    @Override
    public Object handleEvent(Map<String, String> headers, AsyncHttpRequest input, int instance) {
        String collection = "users";
        Object data = input.body;

        MongoCollection<Document> collection = db.getCollection(collection);
        try{
            JSONParser parser = new JSONParser();
            try{
                JSONObject json = (JSONObject) parser.parse(data);
                Set<String> keyset = json.keySet();
                Iterator<String> keys = keyset.iterator();
    
                Document insertDoc = new Document();
                insertDoc.append( "_id", new ObjectId());
                while (keys.hasNext()){
                        String key = keys.next();
                        Object value = json.get(key);
                        insertDoc.append( key, value);
                }
                InsertOneResult result = collection.insertOne(insertDoc);

            } catch (ParseException e) { e.printStackTrace();}
                
            log.info("Inserted document:" + data, instance);
        } catch (MongoException me) {log.info("Unable to insert due to error: " + me, instance);}
        return result;
    }
}
