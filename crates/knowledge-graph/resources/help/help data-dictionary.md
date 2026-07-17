Data Dictionary
---------------
Based on the MiniGraph technology, the data dictionary method requires (1) Data Dictionary items,
(2) Data Providers and (3) API Fetchers.

1. You can create a node holding a data dictionary item
2. A data dictionary item presents a data attribute that can be retrieved from a data provider using an API fetcher
3. It has 'input' and 'output' statements to define input parameter(s) and output data mapping respectively
4. Default value is supported using the colon (':') character (see example below)

Syntax
------
```
create node {name}
with type Dictionary
with properties
purpose={something about this data dictionary item}
provider={data provider}
input[]={parameter}
output[]={data mapping from response object to result set}
```

Example
-------
```
create node person-name
with type Dictionary
with properties
purpose=name of a person
provider=mdm-profile
input[]=person_id
input[]=detail:true
output[]=response.profile.name -> result.person_name
```

Data dictionary node holds key-values and it does not execute by itself. It is used by an API fetcher node.
Instead, the result set will be saved in the API fetcher node.

One or more data dictionary items can share the same data provider. For example, a complex data structure
is returned by a data provider, a single data dictionary item will get one or more data attributes.
If the same input key-values are applied to the same data provider, the API fetcher will only issue a single
API request.

Data Provider
-------------
1. A data provider is also a node
2. It describes the communication protocol with a target system providing a set of data attributes
3. It has 'url', 'method', 'feature', 'and 'input' statements

Syntax
------
```
create node {name}
with type Provider
with properties
purpose={something about this provider if any}
url={url to target system}
method={GET | POST | PUT | PATCH | HEAD, etc.}
feature[]={authentication mechanism, encryption, etc.}
input[]={source -> target}
```

Feature
-------
The list contains one of more optional features that an API fetcher using this provider must support.

Two built-in features are `log-request-headers` and `log-response-headers`. When these features are included, 
the fetcher will log request/response headers into the "header" section of its properties.

Input data mapping
------------------
The input data mapping is designed to do simple mapping with the following restriction:
- The left hand side (source) is limited to parameter of the data dictionary item or constants
- The right hand side (target) is allowed to use the following namespaces:

*Left hand side*

1. Constant
2. Input parameter for a data dictionary
3. Other value that is available in the state machine. e.g. "model." namespace.

*Right hand side*

1. `body.` - request body
2. `header.` - request header
3. `query.` - request query parameter
4. `path_parameter.` - URI path parameter

The following two examples illustrate a data provider configuration for a hypothetical profile management system

Example one
-----------
In the first example, it maps the parameter 'person_id' of the data dictionary to the path parameter 'id'.
It also maps the parameter 'detail' of the data dictionary to the query parameter 'id'

```
create node mdm-profile
with type Provider
with properties
purpose=MDM profile management system
url=${HOST}/api/mdm/profile/{id}
method=GET
feature[]=oauth-bearer
input[]=text(application/json) -> header.accept
input[]=person_id -> path_parameter.id
input[]=detail -> query.detail
```

Example two
-----------
In the second example, it uses POST method and expects a request body containing the 'person_id' parameter.
Since it is a POST request, it requires the configuration of 'content-type' in the header section.
The 'body.' namespace is used to tell the system to map the input parameter in the API request body.
For some use cases, you may set the input parameter as the whole 'body'.
e.g. setting a string or an array as request body instead of key-values.

The 'feature' statement section contains 'oauth-bearer'. Therefore, you must configure an API fetcher that
supports this feature. Otherwise, the fetcher may throw exception. For demo purpose, we will configure
the 'graph.api.fetcher' that will just print a warning message if the feature is not supported.

Since the MiniGraph Playground system is extensible, you can always write a custom API fetcher to handle
new communication protocols and features.

```
create node mdm-profile
with type Provider
with properties
purpose=MDM profile management system
url=${HOST}/api/mdm/profile
method=POST
feature[]=oauth-bearer
input[]=text(application/json) -> header.accept
input[]=text(application/json) -> header.content-type
input[]=person_id -> body.id
input[]=detail -> query.detail
```

API Fetcher
-----------
Data dictionary items are consumed by API fetcher. A built-in API fetcher is called "graph.api.fetcher".

Skill: Graph API Fetcher
------------------------
When a node is configured with this skill of "graph API fetcher", it will make an API call to a backend service
and collect result set into the "result" property of the node. In case of exception, the "status" and "error"
fields will be set to the node's properties and the graph execution will stop.

Execution will start when the GraphExecutor reaches the node containing this skill.

Route name
----------
"graph.api.fetcher"

Setup
-----
To enable this skill for a node, set "skill=graph.api.fetcher" as a property in a node.
It will find out the data provider from a given data dictionary item to make an outgoing API call.

The following are required in the properties of the node:

1. dictionary - this is a list of valid data dictionary node names configured in the same graph model
2. input - one or more data mapping as input parameters to invoke the API call
3. output - one of more data mapping to map result set to another node or the 'output.' namespace

The parameter name in each mapping statement must match that in the data dictionary item.
Otherwise, execution will fail.

The system uses the same syntax of Event Script for data mapping.

Properties
----------
```
skill=graph.api.fetcher
dictionary[]={data dictionary item}
input[]={mapping of key-value from input or another node to input parameter(s) of the data dictionary item(s)}
output[]={optional mapping of result set to one or more variables in the 'model.' or 'output.' namespace}
```

Optional properties
-------------------
```
for_each[]={map a result parameter that is an array into a model variable for iterative API execution}
concurrency={controls parallel API calls for an "iterative API request". Default 3, max 30}
```

Dictionary
----------
This list contains one or more data dictionary item (aka 'data attribute')

Feature
-------
This API fetcher supports features configured in a data provider's node.

There are 2 built-in features that are convenience for development and tests:
- log-request-headers
- log-response-headers

When either or both of these features are added to a data provider's node,
the fetcher will log request/response headers into the "header" section
of its properties.

Input/Output Data mapping
-------------------------
source.composite.key -> target.composite.key

For input data mapping, the source can use a key-value from the `input.` namespace or another node.
The target can be a key-value in the state machine (`model.` namespace) or an input parameter name of the
data dictionary.

For output data mapping, the source can be a key-value from the result set and the target can use
the `output.` or `model.` namespace.

Output data mapping is optional because you can use another data mapper to map result set of the fetcher
to another node.

Result set
----------
Upon successful execution, the result set will be stored in the "result" parameter in the properties of
the node. A subsequent data mapper can then map the key-values in the result set to one or more nodes.

Example
-------
```
create node fetcher-1
with properties
skill=graph.api.fetcher
dictionary[]=person-name
dictionary[]=person-address
dictionary[]=person-accounts
input[]=input.body.person_id -> person_id
output[]=result.person_name -> output.body.name
output[]=result.person_address -> output.body.address
```

Iterative API call
------------------
Using the optional `for_each` statement, you can tell the API fetcher to do "fork-n-join" of API requests.

A "for_each" statement extracts the next array element from result set of a prior API call into a model variable.
You can then put the model variable in the "left-hand-side" of an input statement. The API fetcher will then
issue multiple API calls using an iterative stream of the model variable.

If your API call needs more than one parameter, you can configure more than one "for_each" statement.

Example
-------
In this example, the "for_each" statement extracts the "person_accounts" from the result of a prior API call
by "fetcher-1" and map the array into an iterative stream of elements using the model variable "account_id".

The concurrency property tells the API fetcher to limit parallelism to avoid overwhelming the target service.
```
create node fetcher-2
with properties
skill=graph.api.fetcher
dictionary[]=person-id
dictionary[]=account-id
for_each[]=fetcher-1.result.person_accounts -> model.account_id
concurrency=3
input[]=input.body.person_id -> person_id
input[]=model.account_id -> account_id
output[]=result.person_name -> output.body.name
output[]=result.person_address -> output.body.address
```

- The "[]" syntax is used to create and append a list of one or more data mapping entries
- The "->" signature indicates the direction of mapping where the left-hand-side is a source
  and right-hand-side is a target

Caution
-------
API fetchers can be chained together to make multiple API calls.
However, you should design the API chain to be minimalist.

An overly complex chain of API requests would mean slow performance. Just take the minimal set of data that are
required by your application. Don't abuse the flexibility of the API fetcher.
