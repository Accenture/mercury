Tutorial 6
----------
In this tutorial, you will create a graph model that fetches an array list from one
service and iterates over the elements of the array to fetch more details from
another service, using the "for_each" keyword.

Exercise
--------
You will import the graph model from tutorial-3 as a template and expand it to handle
a multi-step data fetch use case.

Import a graph model
--------------------
Enter 'import graph from tutorial-3'.

```
> import graph from tutorial-3
Graph model not found in /tmp/graph/tutorial-3.json
Found deployed graph model in classpath:/graph
Please export an updated version and re-import to instantiate an instance model
```

If you have not exported tutorial-3 earlier, the system imports it from a demo graph.

Examine the graph model
-----------------------
Examine the graph model with the 'list nodes' and 'list connections' commands.

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

Create a new data dictionary node
---------------------------------
Enter the following to create a new data dictionary node "person-accounts". It uses
the same data provider "mdm-profile" to retrieve the list of accounts for a person —
an array of account numbers.

```
create node person-accounts
with type Dictionary
with properties
input[]=person_id
output[]=response.accounts -> result.account_numbers
provider=mdm-profile
purpose=accounts of a person
```

Update the fetcher
------------------
Add the dictionary item "person-accounts" to the original fetcher.

```
update node fetcher
with type Fetcher
with properties
dictionary[]=person-name
dictionary[]=person-address
dictionary[]=person-accounts
input[]=input.body.person_id -> person_id
output[]=result.name -> output.body.name
output[]=result.address -> output.body.address
skill=graph.api.fetcher
```

Create one more data dictionary node
------------------------------------
Create a data dictionary node "account-details", associated with the data provider
"account-details-provider", to retrieve account details based on person_id and
account_id.

```
create node account-details
with type Dictionary
with properties
input[]=person_id
input[]=account_id
output[]=response.account.details -> result.accounts
provider=account-details-provider
purpose=Account details
```

Create a new data provider
--------------------------
Enter the following to create the data provider that retrieves account details.

Its feature section declares oauth2-bearer, log-request-headers and
log-response-headers. The "oauth2-bearer" entry is a placeholder — implement it
according to your organization's security guidelines. Functionally, it would acquire
an OAuth2 bearer token from a security authority using a client id and secret
configured in the deployed environment, cache and refresh the access token as
required, and insert the "authorization" header in a pre-processing step of the Graph
API Fetcher. The log-request-headers and log-response-headers features can serve as
templates for implementing your own pre-processing and post-processing features.

```
create node account-details-provider
with type Provider
with properties
feature[]=oauth2-bearer
feature[]=log-request-headers
feature[]=log-response-headers
input[]=text(application/json) -> header.accept
input[]=text(application/json) -> header.content-type
input[]=person_id -> body.person_id
input[]=account_id -> body.account_id
method=POST
purpose=Account Management Endpoint
url=http://127.0.0.1:${rest.server.port}/api/account/details
```

Note that this is a POST provider: the `body.{key}` input targets build the JSON
request body, and the parameters travel in the body rather than the URL.

Create a second fetcher
-----------------------
Create a second fetcher as follows. The `for_each` statement iterates over the array
in the first fetcher's result set (`fetcher.result.account_numbers`), mapping each
element into "model.account_number".

For each element, the input statement block runs to populate the input parameters:
"person_id" is passed unchanged to every call, while "account_id" takes the current
element.

```
create node fetcher-2
with type Fetcher
with properties
dictionary[]=account-details
for_each[]=fetcher.result.account_numbers -> model.account_number
input[]=input.body.person_id -> person_id
input[]=model.account_number -> account_id
output[]=result.accounts -> output.body.accounts
skill=graph.api.fetcher
```

Each iteration's `result.accounts` value is appended into a single array on this
node's result set — with five account numbers, "output.body.accounts" becomes an
array of five account detail records.

Rearrange the connections
-------------------------
Connect the first fetcher to the second fetcher, delete the original connection
between the fetcher and the end node, then connect the second fetcher to the end node.

Enter 'list connections' to show the updated connections.

```
> connect fetcher to fetcher-2 with details
node fetcher connected to fetcher-2
> delete connection fetcher and end
fetcher -> end removed
> connect fetcher-2 to end with complete
node fetcher-2 connected to end
> list connections
root -[fetch]-> fetcher
fetcher -[details]-> fetcher-2
fetcher-2 -[complete]-> end
```

Update the root node
--------------------
Since you are using the tutorial-3 graph model as a template, it is good practice to
update the root node to describe the new purpose of tutorial-6. Enter the following.

```
update node root
with type Root
with properties
name=tutorial-6
purpose=Demonstrate multi-step API fetching and the "for_each" method
```

Perform a dry-run
-----------------
Enter the following to mock the input parameter "person_id = 100".

```
start graph
int(100) -> input.body.person_id
```

Then enter `run` to do a dry-run. You will see the following:

```
> start graph...
Graph instance created. Loaded 1 mock entry, model.ttl = 30000 ms
> run
Walk to root
Walk to fetcher
Executed fetcher with skill graph.api.fetcher in 12.085 ms
Walk to fetcher-2
Executed fetcher-2 with skill graph.api.fetcher in 14.326 ms
Walk to end
{
  "output": {
    "body": {
      "address": "100 World Blvd",
      "name": "Peter",
      "accounts": [
        {
          "balance": 25032.13,
          "id": "a101",
          "type": "Saving"
        },
        {
          "balance": 6020.68,
          "id": "b202",
          "type": "Current"
        },
        {
          "balance": 120000.0,
          "id": "c303",
          "type": "C/D"
        },
        {
          "balance": 6000.0,
          "id": "d400",
          "type": "apple"
        },
        {
          "balance": 8200.0,
          "id": "e500",
          "type": "google"
        }
      ]
    }
  }
}
Graph traversal completed in 28 ms
```

Parallelism
-----------
With the "for_each" method, the system performs the API fetches in parallel. The
default concurrency is 3; set "concurrency" in "fetcher-2" (1-30) to try other
values.

With a concurrency of 3 and five accounts, the system makes a batch of 3 followed by
a batch of 2 API requests. When you change the concurrency setting, the batch size
adjusts accordingly.

Aggregation order is guaranteed: batches execute in source-list order and responses
join in request order, so the aggregated result array preserves the order of the
source account numbers — regardless of the concurrency setting. You can see this in
the dry-run above: the account details appear in the same order as the account
numbers (a101 to e500).

Create an island to hold the data dictionary
--------------------------------------------
Wire the data dictionary and provider nodes into the graph's knowledge layer with an
island node. This is the required convention — leave no node unconnected: the island
subgraph is the graph's entity-relationship diagram, turning the graph into living
documentation of enterprise knowledge.

```
create node dictionary
with type Island
with properties
skill=graph.island
```

Then connect the data dictionary nodes and provider nodes to it.

```
> connect root to dictionary with contains
node root connected to dictionary
> connect dictionary to person-name with data
node dictionary connected to person-name
> connect dictionary to person-address with data
node dictionary connected to person-address
> connect dictionary to person-accounts with data
node dictionary connected to person-accounts
> connect person-name to mdm-profile with provider
node person-name connected to mdm-profile
> connect person-address to mdm-profile with provider
node person-address connected to mdm-profile
> connect person-accounts to mdm-profile with provider
node person-accounts connected to mdm-profile
> connect dictionary to account-details with data
node dictionary connected to account-details
> connect account-details to account-details-provider with provider
node account-details connected to account-details-provider
> list connections
root -[contains]-> dictionary
root -[fetch]-> fetcher
account-details -[provider]-> account-details-provider
dictionary -[data]-> account-details
dictionary -[data]-> person-accounts
dictionary -[data]-> person-address
dictionary -[data]-> person-name
fetcher -[details]-> fetcher-2
person-accounts -[provider]-> mdm-profile
person-address -[provider]-> mdm-profile
person-name -[provider]-> mdm-profile
fetcher-2 -[complete]-> end
```

Export the graph model
----------------------
Save the graph model by exporting it.

```
> export graph as tutorial-6
Graph exported to /tmp/graph/tutorial-6.json
Described in /api/graph/model/tutorial-6/775-18
```

Deploy the graph model
----------------------
To deploy the graph model, copy "/tmp/graph/tutorial-6.json" to your application's
resources/graph folder. You can then test the deployed model with a curl command.

```
curl -X POST http://127.0.0.1:8100/api/graph/tutorial-6 \
  -H "Content-Type: application/json" \
  -d '{
    "person_id": 100
  }'
```

Summary
-------
In this tutorial, you created a graph model that performs two steps of API fetching.
The first step gets the name, address and list of account numbers. The second step
uses the "for_each" method to fetch the account details for each account number, and
aggregates the results into a single array in source-list order.
