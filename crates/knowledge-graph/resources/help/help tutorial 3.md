Tutorial 3
----------
In this session, you will learn about the data dictionary method to source data from an external service.

Exercise
--------
You will create a root node, an end node, a data dictionary node, a data provider node and an API fetcher node
as an exercise.

To clear the previous graph session, click the Tools button in the top-right corner and click the "Stop" and "Start"
toggle button. A new graph session will start.

Create root and end nodes
-------------------------
Enter the "create node" command for "root" and "end" nodes first.

```
create node root
with type Root
with properties
name=tutorial-3
purpose=Demonstrate data sourcing using the Data Dictionary method
```

```
create node end
with type End
```

Create data dictionary items
----------------------------
A data dictionary describes a "data attribute" and its "data provider". Please enter the following:

```
create node person-name
with type Dictionary
with properties
purpose=name of a person
provider=mdm-profile
input[]=person_id
output[]=response.profile.name -> result.name

create node person-address
with type Dictionary
with properties
purpose=address of a person
provider=mdm-profile
input[]=person_id
output[]=response.profile.address -> result.address
```

This command create two nodes called "person-name" and "person-address" with a data provider called "mdm-profile".
The input parameter to retrieve these data attribute from the data provider is "person_id".
The output section contains a data mapping statement that maps the response's key-value(s)
as the data dictionary's result set. The "response." and "result." are namespaces that
represent the response key-values from the data provider and the result key-values obtained
with this data dictionary.

In the "person-name" data dictionary, it tells the system to extract the "profile.name" data attribute from
the response's data structure and map it as the key "name".

Create a data provider
----------------------
The data dictionary assigns a data provider "mdm-profile". We will create a node for the
data provider.

```
create node mdm-profile
with type Provider
with properties
purpose=Master Data Management's profile management endpoint
url=http://127.0.0.1:${rest.server.port:8080}/api/mdm/profile/{id}
method=GET
feature[]=log-request-headers
feature[]=log-response-headers
input[]=text(application/json) -> header.accept
input[]=person_id -> path_parameter.id
```

The "url" is the REST endpoint of the target service for "mdm-profile".
The `${rest.server.port:8080}` is used to obtain a key-value from the application.properties or environment variable.
The colon syntax is optional. If yes, you can set a default value.

In this example, the url has a path parameter "id".

The "feature" section tells the system to apply pre-processing and/or post-processing of HTTP request/response.
The "log-request-headers" feature will log request headers, if any and the "log-response-headers" feature will
print the HTTP response headers from the target service. These 2 features are for demonstration purpose.
In real-world use case, you may implement an "oauth2-bearer" feature. We will discuss custom feature in a
subsequent tutorial.

The input section tells the system to map HTTP request headers, path parameter, query and/or body key-values.
The namespaces are:

```
header.
query.
path_parameter.
body.
```

The left hand side of the input mapping is the input parameter(s) from the associated data dictionary.

Create an API fetcher
---------------------
You will create a fetcher node like this:

```
create node fetcher
with type Fetcher
with properties
skill=graph.api.fetcher
dictionary[]=person-name
dictionary[]=person-address
input[]=input.body.person_id -> person_id
output[]=result.name -> output.body.name
output[]=result.address -> output.body.address
```

After this step, you will see 6 nodes in the graph diagram on the right panel.

Connect the fetcher
-------------------
You will connect the root node to the fetcher node and then connect it to the end node.

```
> connect root to fetcher with fetch
node root connected to fetcher
> connect fetcher to end with complete
node fetcher connected to end
```

Export the graph model
----------------------
The graph model is complete. Let's export it as 'tutorial-3'.

```
> export graph as tutorial-3
Graph exported to /tmp/graph/tutorial-3.json
Described in /api/graph/model/tutorial-3/849-13
```

Test the fetcher node
---------------------
Before you do a dry-run, you can test the fetcher alone because it is self-contained. It maps the input parameter
to 'person_id', makes an outgoing HTTP request using the data dictionary and returns the result as "output.body".

First, you can instantiate the graph model and mock the input parameter like this:

```
instantiate graph
int(100) -> input.body.person_id
```

The system will acknowledge your command as follows:

```
> instantiate graph...
Graph instance created. Loaded 1 mock entry, model.ttl = 30000 ms
```

Before you test the fetcher, you can check the input and output key-values with the `inspect` command:

```
> inspect input
{
  "inspect": "input",
  "outcome": {
    "body": {
      "person_id": 100
    }
  }
}
> inspect output
{
  "inspect": "output",
  "outcome": {}
}
```

When a graph model is instantiated, the system creates a temporary "state machine" for each graph instance.
The inspect command allows you to check the current key-values in the "state machine".

The above output shows that "person_id" of integer value 100 is stored in the input.body and there is nothing
in the "output.body".

You can now test the fetcher with the "execute" command:

```
> execute fetcher
node fetcher run for 0.266 ms with exit path 'next'
```

The system shows that fetcher has been executed and it is ready to continue to the next node.

Now you can inspect the "output" in the state machine again.

```
> inspect output
{
  "inspect": "output",
  "outcome": {
    "body": {
      "address": "100 World Blvd",
      "name": "Peter"
    }
  }
}
```

It shows that the result set contains name and address obtained from the target service correctly.

Dry-Run
-------

We know that the fetcher is configured correctly. You can do a dry-run from the beginning to the end.

You can clear the state machine by instantiating the graph model using the command earlier.

```
instantiate graph
int(100) -> input.body.person_id
```

```
> instantiate graph...
Graph instance created. Loaded 1 mock entry, model.ttl = 30000 ms
```

Verify that the output's key-values are cleared when you do `inspect output`. Then enter `run`.

```
> run
Walk to root
Walk to fetcher
Executed fetcher with skill graph.api.fetcher in 14.456 ms
Walk to end
{
  "output": {
    "body": {
      "address": "100 World Blvd",
      "name": "Peter"
    }
  }
}
Graph traversal completed in 15 ms
```

List nodes and connections
--------------------------
Before we close this session, let's check the nodes and connections for the graph model 'tutorial-3'.

Enter the `list nodes` and `list connections` commands:

```
> list nodes
root [Root]
fetcher [Fetcher]
mdm-profile [Provider]
person-address [Dictionary]
person-name [Dictionary]
end [End]
> list connections
root -[fetch]-> fetcher
fetcher -[complete]-> end
```

Note that data dictionary and data provider nodes do not need to be connected. It is because they are
"configuration" nodes. They are not active nodes that can be executed by themselves. The API fetcher node
uses the configuration given in the data dictionary and data provider to make an external API call.

For more details of the data dictionary method, you may enter "help data-dictionary".

Create an island to hold data dictionary
----------------------------------------
The data dictionary and data provider nodes are not connected. To organize, you can create an "island" node
to hold them.

```
create node dictionary
with type Island
with properties
skill=graph.island
```

Then you can connect the data dictionary nodes and provider node to it.

```
> connect root to dictionary with contains
node root connected to dictionary
> connect dictionary to person-name with data
node dictionary connected to person-name
> connect dictionary to person-address with data
node dictionary connected to person-address
> connect person-name to mdm-profile with provider
node person-name connected to mdm-profile
> connect person-address to mdm-profile with provider
node person-address connected to mdm-profile
> list connections
root -[contains]-> dictionary
root -[fetch]-> fetcher
dictionary -[data]-> person-address
dictionary -[data]-> person-name
person-address -[provider]-> mdm-profile
person-name -[provider]-> mdm-profile
fetcher -[complete]-> end
```

The purpose of an "island" node is to isolate sub-graph that does not require execution.
The data dictionary and provider nodes hold configuration for the API fetcher.
They are not executable by themselves.

Connecting data dictionary and provider nodes helps to describe the relationships, but this is not mandatory.

However, for data entities such as person, account and order, defining the directional connections with relationships
is a best practice that we recommend. It is because data entities and relationships represent enterprise knowledge.

To save the updated graph model, you should export it again.

```
> export graph as tutorial-3
Graph exported to /tmp/graph/tutorial-3.json
Described in /api/graph/model/tutorial-3/287-4
```

Deploy the graph model
----------------------
To deploy, you may copy "/tmp/graph/tutorial-3.json" into your application's main/resources/graph folder and
restart the application. You can use the following curl command to invoke the knowledge graph endpoint.

```
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-3 \
  -H "Content-Type: application/json" \
  -d '{
    "person_id": 100
  }'
```

Note that input parameters, if any, must be submitted as a POST request body with content type "application/json".

You will receive the following response:

```json
{
  "address": "100 World Blvd",
  "name": "Peter"
}
```

If you change the person_id to 10, you will receive an error because the test profile is set to 100.

```json
{
  "message": "Profile 10 not found",
  "type": "error",
  "target": "person-name",
  "status": 400
}
```

Well done! You have successfully created a graph model that can fetch external data.

API call optimization
---------------------
If you check the application log, you notice that each graph instance makes one HTTP call to
`http://127.0.0.1:8085/api/mdm/profile/10` only.

When the target URL and method for multiple data dictionary items and their input parameter(s)
are the same, the system will avoid making redundant API calls.

Therefore, it is important to configure the data dictionary and provider correctly so that
the system will efficiently fetch data.

Summary
-------
In this session, you have configured data dictionary and data provider. You have defined an API fetcher
node to use the data dictionary and data provider to fetch some data. You have deployed the graph model
and made an API request using CURL command.

You have also learnt how to organize data dictionary and provider nodes in an "island" (aka 'subgraph').
