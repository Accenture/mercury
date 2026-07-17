Tutorial 6
----------
In this session, we will create a graph model that would fetch an array list from one service and iterate
the elements in the array to fetch more details from another service. We will examine the use of the
"for_each" keyword.

Exercise
--------
You will import the graph model from tutorial-3 as a template and expand it to handle a multi-step
data fetch use case.

Import a graph model
--------------------
Enter 'import graph from tutorial-3'

```
> import graph from tutorial-3
Graph model not found in /tmp/graph/tutorial-3.json
Found deployed graph model in classpath:/graph
Please export an updated version and re-import to instantiate an instance model
```

If you have not exported tutorial-3 earlier, the system will import it from a demo graph.

Examine the graph model
-----------------------
You can examine the graph model with the 'list nodes' and 'list connections' commands.

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
Enter the following to create a new data dictionary node "person-accounts". This uses the same data provider
"mdm-profile" to retrieve a list of accounts for the user. The list of accounts is an array of account numbers.

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
Add the dictionary item "person-accounts" in the original fetcher.

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
Create a data dictionary node "account-details" that is associated with the data provider "account-details-provider"
to retrieve account details based on person_id and account_id.

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
Enter the following to create a data provider that retrieves account details.
In the feature section, there are oauth2-bearer, log-request-headers and log-response-headers.
The "oauth2-bearer" is a placeholder and you should implement according to your organization
security guideline. Functionally, it would acquire OAuth2 bearer token from a security authority 
using client-id and secret configured in the deployed environment. It should cache and refresh
the access token as required and insert the "authorization" header in a pre-processing step
for the Graph API Fetcher. The log-request-headers and log-response-headers can be used as
templates to implement your own pre-processing and post-processing features.

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

Create a second fetcher
-----------------------
You will create a second fetcher as follows. You will apply the `for_each` statement to iterate
the array in the fetcher's result set and map each element into "model.account_number".

For each element, the input statement block will be executed to populate the input parameter "account_id".

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

Rearrange the connections
-------------------------
You will connect the first fetcher to the second fetcher, delete the original connection between fetcher and
the end node. Then connect the second fetcher to the end node.

Then enter 'list connections' to show the updated connections.

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
Since you are using tutorial-3 graph model as a template, it is a good practice to update the root node
to describe the new purpose of tutorial-6. Enter the following.

```
update node root
with type Root
with properties
name=tutorial-6
purpose=Demonstrate multi-step API fetching and the "for_each" method
```

Perform a dry-run
-----------------
Enter the following to mock the input parameter of "person_id = 100".

```
start graph
int(100) -> input.body.person_id
```

Then enter `run` to do a dry-run.

You will see the following:

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
When using the "for_each" method, the system will perform parallel API fetching. The default concurrency is 3.
If you want to change this value, set "concurrency" in "fetcher-2" to try.

With concurrency of 3 and there are 5 accounts, the system will perform a batch of 3 and a batch of 2 API requests.
When you changed the concurrency setting, you will see the batch size will be adjusted accordingly.

Create an island to hold data dictionary
----------------------------------------
You will create an island node to organize the data dictionary and provider nodes.

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
> connect account-details to account-details-provider with data
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
You may save the graph model by exporting it.

```
> export graph as tutorial-6
Graph exported to /tmp/graph/tutorial-6.json
Described in /api/graph/model/tutorial-6/775-18
```

Deploy the graph model
----------------------
To deploy the graph model, copy "/tmp/graph/tutorial-6.json" to your application's `main/resources/graph` folder.
You can then test the deployed model with a curl command.

```
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-6 \
  -H "Content-Type: application/json" \
  -d '{
    "person_id": 100
  }'
```

Summary
-------
In this session, you have created a graph model that performs 2 steps of API fetching. The first one gets the
name, address and list of account numbers. The second one uses the account numbers to fetch the account details
for each account using the "for_each" method.
