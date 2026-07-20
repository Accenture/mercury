Tutorial 3
----------
In this tutorial, you will learn the data dictionary method to source data from an
external service.

Exercise
--------
You will create a root node, an end node, two data dictionary nodes, a data provider
node and an API fetcher node.

To clear the previous graph session, click the Tools button in the top-right corner
and click the "Stop" and "Start" toggle button. A new graph session will start.

Create root and end nodes
-------------------------
Enter the "create node" command for the "root" and "end" nodes first.

```
create node root
with type Root
with properties
name=tutorial-3
purpose=Demonstrate data sourcing using the Data Dictionary method - fetch one person profile (name and address) by person_id
```

```
create node end
with type End
```

Create data dictionary items
----------------------------
A data dictionary describes a "data attribute" and its "data provider". Please enter
the following:

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

This creates two Dictionary nodes, "person-name" and "person-address", both served by
a data provider called "mdm-profile".

In a Dictionary node, `input[]` entries are **bare parameter names** — not
`source -> target` mappings. Here, the parameter required to retrieve these data
attributes is "person_id". If a parameter has a sensible default, supply it with an
optional `:{default}` suffix (e.g. `input[]=detail:true`) — a default value is the
only meaning of `:` in a Dictionary input entry.

The `output[]` section maps the provider's response into the dictionary's result set.
The `response.` and `result.` namespaces represent the response key-values from the
data provider and the result key-values produced by this data dictionary.

In the "person-name" data dictionary, the output mapping extracts the "profile.name"
attribute from the response's data structure and exposes it as the key "name".

Create a data provider
----------------------
The data dictionaries name a data provider "mdm-profile". Create a node for it:

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

The "url" is the REST endpoint of the target service. `${rest.server.port:8080}`
resolves a key-value from the application configuration or an environment variable;
the value after the optional `:` is a default.

In this example, the url has a path parameter "id" — filled by the `input[]` line
that targets `path_parameter.id`.

The "feature" section tells the system to apply pre-processing and/or post-processing
to the HTTP request/response. "log-request-headers" logs the request headers, if any,
and "log-response-headers" logs the HTTP response headers from the target service.
These two features are for demonstration; in a real-world use case, you might
implement an "oauth2-bearer" feature. Custom features are discussed in a subsequent
tutorial.

The input section maps values into the outgoing HTTP request — headers, path
parameters, query and/or body key-values. The target namespaces are:

```
header.
query.
path_parameter.
body.
```

The left-hand side of a provider input mapping is a constant (e.g.
`text(application/json)`) or an input parameter declared by the associated data
dictionary (e.g. `person_id`).

Create an API fetcher
---------------------
Create a fetcher node like this:

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
Connect the root node to the fetcher node, then the fetcher to the end node.

```
> connect root to fetcher with fetch
node root connected to fetcher
> connect fetcher to end with complete
node fetcher connected to end
```

Export the graph model
----------------------
The execution path is complete. Let's export it as 'tutorial-3'.

```
> export graph as tutorial-3
Graph exported to /tmp/graph/tutorial-3.json
Described in /api/graph/model/tutorial-3/849-13
```

Test the fetcher node
---------------------
Before you do a dry-run, you can test the fetcher alone because it is self-contained:
it maps the input parameter to 'person_id', makes an outgoing HTTP request using the
data dictionary and returns the result as "output.body".

First, instantiate the graph model and mock the input parameter like this:

```
instantiate graph
int(100) -> input.body.person_id
```

The system acknowledges your command as follows:

```
> instantiate graph...
Graph instance created. Loaded 1 mock entry, model.ttl = 30000 ms
```

Before you test the fetcher, check the input and output key-values with the
`inspect` command:

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

When a graph model is instantiated, the system creates a temporary "state machine"
for the graph instance. The inspect command lets you check the current key-values in
that state machine.

The above output shows that "person_id" with the integer value 100 is stored in
input.body, and there is nothing in the output yet.

You can now test the fetcher with the "execute" command:

```
> execute fetcher
node fetcher run for 0.266 ms with exit path 'next'
```

The fetcher has been executed and it is ready to continue to the next node.

Now inspect the "output" in the state machine again.

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

The result set contains the name and address obtained from the target service.

Dry-run
-------
The fetcher is configured correctly, so you can do a dry-run from beginning to end.

Clear the state machine by instantiating the graph model again:

```
instantiate graph
int(100) -> input.body.person_id
```

```
> instantiate graph...
Graph instance created. Loaded 1 mock entry, model.ttl = 30000 ms
```

Verify that the output key-values are cleared with `inspect output`. Then enter `run`.

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
Let's check the nodes and connections of the graph model 'tutorial-3'.

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

Note that the data dictionary and data provider nodes have no connections yet. They
are "configuration" nodes — not active nodes that execute on their own. The API
fetcher references them by name and uses their configuration to make an external
API call.

For more details of the data dictionary method, enter "help data-dictionary".

Configuration nodes must still not be left floating — the next step wires them into
the graph's knowledge layer.

Create an island to hold the data dictionary
--------------------------------------------
The required convention is: **leave no node unconnected**. Configuration nodes are
wired into the graph's knowledge layer with an "island" node.

```
create node dictionary
with type Island
with properties
skill=graph.island
```

Then connect the data dictionary nodes and the provider node to it.

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

A "graph.island" node is isolated from graph traversal: it never hands execution to
the next node, so the execution path is unaffected. Its purpose is knowledge
structure — the island subgraph is the graph's entity-relationship diagram.

Data entities such as person, account and order, and the directional relationships
between them, represent enterprise knowledge. With the dictionaries, providers and
entities wired under the island, the graph becomes living documentation: a new team
member (or an AI agent) can read the knowledge layer to discover the domain model,
not just the execution path.

To save the updated graph model, export it again.

```
> export graph as tutorial-3
Graph exported to /tmp/graph/tutorial-3.json
Described in /api/graph/model/tutorial-3/287-4
```

Deploy the graph model
----------------------
To deploy, copy "/tmp/graph/tutorial-3.json" into your application's resources/graph
folder and restart the application. You can then invoke the knowledge graph endpoint
with the following curl command.

```
curl -X POST http://127.0.0.1:8100/api/graph/tutorial-3 \
  -H "Content-Type: application/json" \
  -d '{
    "person_id": 100
  }'
```

Note that input parameters, if any, must be submitted as a POST request body with
content type "application/json".

You will receive the following response:

```json
{
  "address": "100 World Blvd",
  "name": "Peter"
}
```

If you change the person_id to 10, you will receive an error because the test profile
is set to 100.

```json
{
  "message": "Profile 10 not found",
  "type": "error",
  "target": "person-name",
  "status": 400
}
```

Well done! You have successfully created a graph model that fetches external data.

API call optimization
---------------------
If you check the application log, you will notice that each graph instance makes only
one HTTP call to `http://127.0.0.1:8100/api/mdm/profile/10`.

When multiple data dictionary items share the same target URL, method and input
parameter values, the system avoids making redundant API calls.

Therefore, it is important to configure the data dictionary and provider correctly so
that the system fetches data efficiently.

Summary
-------
In this tutorial, you configured a data dictionary and a data provider, and defined an
API fetcher node that uses them to fetch data. You deployed the graph model and made
an API request with a curl command.

You also organized the data dictionary and provider nodes under an "island" — the
required knowledge-layer convention that leaves no node unconnected.
