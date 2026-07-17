Tutorial 8
----------
In this session, we will use JSON-Path search feature to retrieve key-values from input payload.

Exercise
--------
You will import tutorial-7 and replace some data mapping statements with JSON-Path search requests.

To clear the previous graph session, click the Tools button in the top-right corner and click the "Stop" and "Start"
toggle button. A new graph session will start.

Import tutorial-7
-----------------
Enter 'import graph from tutorial-7' first.

```
> import graph from tutorial-7
Found deployed graph model in classpath:/graph
Please export an updated version and re-import to instantiate an instance model
Graph model imported as draft
```

Input payload
-------------
The account holder "Peter" has 2 accounts.
We will assume the following input payload data structure. You would copy-n-paste this JSON dataset
when using the "upload mock data" dialog box in this tutorial exercise.

```json
{ 
  "profile": {
    "name": "Peter",
    "account": [
      {
        "id": "100",
        "amount": 18000.30,
        "description": "Time deposit",
        "type": "C/D"
      },
      {
        "id": "200",
        "amount": 62050.80,
        "description": "Saving account",
        "type": "Saving"
      }
    ]
  }
}
```

Edit the data mapper node
-------------------------
Let's try some data mapping methods. Please enter the following:

```
update node data-mapper
with type Mapper
with properties
mapping[]=input.body.profile.name -> output.body.name
mapping[]=$.input.body.profile.account[*].type -> model.type
mapping[]=$.input.body.profile.account[*].id -> model.id
mapping[]=$.input.body.profile.account[*].amount -> model.amount
skill=graph.data.mapper
```

The above data mapping statements extract the type, id and amount from the account list in the
input payload using JSON-Path search syntax.

Test the data mapper
--------------------
Let's test the data mapper first.

Enter the following to instantiate the graph and open a dialog box to enter the mock input data.

```
> instantiate graph
Graph instance created. Loaded 0 mock entries, model.ttl = 30000 ms
> upload mock data
Mock data loaded into 'input.body' namespace
```

The first data mapping statement maps the input.body.profile.name into the "name" field of the output body.
The subsequent data mapping statements extract the type, id and amount key-values form the account list and
map them into the model variables type, id and amount accordingly.

When you enter the "upload mock data" command, an input dialog box will be opened. Please paste the sample
input payload listed above.

To confirm that you have uploaded the mock input. Enter "inspect input".

```
> inspect input
{
  "inspect": "input",
  "outcome": {
    "body": {
      "profile": {
        "name": "Peter",
        "account": [
          {
            "amount": 18000.3,
            "description": "Time deposit",
            "id": "100",
            "type": "C/D"
          },
          {
            "amount": 62050.8,
            "description": "Saving account",
            "id": "200",
            "type": "Saving"
          }
        ]
      }
    }
  }
}
```

You can now test the data mapper by "executing" it. Enter "execute data-mapper".

```
> execute data-mapper
node data-mapper run for 0.589 ms with exit path 'next'
```

The data-mapper runs successfully.

Inspect the model and output
----------------------------
You can inspect the model and the output key-values to see what values are mapped.

```
> inspect model
{
  "inspect": "model",
  "outcome": {
    "amount": [
      18000.3,
      62050.8
    ],
    "id": [
      "100",
      "200"
    ],
    "type": [
      "C/D",
      "Saving"
    ]
  }
}
> inspect output
{
  "inspect": "output",
  "outcome": {
    "body": {
      "name": "Peter"
    }
  }
}
```

This confirms that the JSON-Path commands have extracted the key-values from the account list successfully.
However, presenting data in list of key-values in maps is usually not a good schema design. It may be easier
for an application to parse the key-values but it reduces readability for a human operator.

This is just a demo to illustrate that we can use JSON-Path retrieval syntax.

Using the listOfMap plugin
--------------------------
For proper data structure representation, we can use the plugin "f:listOfMap()" to consolidate the map of lists.
You can add a data mapping statement to use the listOfMap plugin like this:

```
update node data-mapper
with type Mapper
with properties
mapping[]=input.body.profile.name -> output.body.name
mapping[]=$.input.body.profile.account[*].type -> model.account.type
mapping[]=$.input.body.profile.account[*].id -> model.account.id
mapping[]=$.input.body.profile.account[*].amount -> model.account.amount
mapping[]=f:listOfMap(model.account) -> output.body.account
skill=graph.data.mapper
```

Note that you add one level of key called "account" to hold the 3 maps of lists for type, id and amount.
Then you apply the plugin "f:listOfMap()" to consolidate the maps of lists into a list of maps.

When you enter 'inspect model' and 'inspect output', you will see:

```
> inspect model
{
  "inspect": "model",
  "outcome": {
    "account": {
      "amount": [
        18000.3,
        62050.8
      ],
      "id": [
        "100",
        "200"
      ],
      "type": [
        "C/D",
        "Saving"
      ]
    }
  }
}
> inspect output
{
  "inspect": "output",
  "outcome": {
    "body": {
      "name": "Peter",
      "account": [
        {
          "amount": 18000.3,
          "id": "100",
          "type": "C/D"
        },
        {
          "amount": 62050.8,
          "id": "200",
          "type": "Saving"
        }
      ]
    }
  }
}
```

This illustrates that the `listOfMap` plugin can perform simple data transformation.
This is handy when your graph model uses API fetchers to retrieve data from multiple sources.
Without writing code, you can group data from different data structures.

Using the removeKey plugin
--------------------------
For a single data source, it is indeed easier to use the plugin `f:removeKey()` to remove one or more keys
from the data structure.

```
mapping[]=f:removeKey(input.body.profile.account, text(description)) -> output.body.account
```

Let's prove this by editing the data-mapper again. We add a new data mapping statement at the end to map
the alternative solution to the "account2" field in the output payload.

```
update node data-mapper
with type Mapper
with properties
mapping[]=input.body.profile.name -> output.body.name
mapping[]=$.input.body.profile.account[*].type -> model.account.type
mapping[]=$.input.body.profile.account[*].id -> model.account.id
mapping[]=$.input.body.profile.account[*].amount -> model.account.amount
mapping[]=f:listOfMap(model.account) -> output.body.account
mapping[]=f:removeKey(input.body.profile.account, text(description)) -> output.body.account2
skill=graph.data.mapper
```

You will do 'instantiate graph' and 'upload mock data' with the same input payload.
Then 'execute data-mapper' and 'inspect output' to see the outcome.

```
> execute data-mapper
node data-mapper run for 2.826 ms with exit path 'next'
> inspect output
{
  "inspect": "output",
  "outcome": {
    "body": {
      "name": "Peter",
      "account2": [
        {
          "amount": 18000.3,
          "id": "100",
          "type": "C/D"
        },
        {
          "amount": 62050.8,
          "id": "200",
          "type": "Saving"
        }
      ],
      "account": [
        {
          "amount": 18000.3,
          "id": "100",
          "type": "C/D"
        },
        {
          "amount": 62050.8,
          "id": "200",
          "type": "Saving"
        }
      ]
    }
  }
}
```

Note that "account" and "account2" have the same key-values and data structure. This confirms that
the "description" key-value has been removed from each map in a list successfully.

Export the graph model
----------------------
As a good practice, you may save the graph model by exporting it.

```
> export graph as tutorial-8
Graph exported to /tmp/graph/tutorial-8.json
Described in /api/graph/model/tutorial-8/315-6
```

Deploy the graph model
----------------------
To deploy the graph model, copy "/tmp/graph/tutorial-8.json" to your application's `main/resources/graph` folder.
You can then test the deployed model with a curl command.

```
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-8 \
  -H "Content-Type: application/json" \
  -d '{ 
  "profile": {
    "name": "Peter",
    "account": [
      {
        "id": "100",
        "amount": 18000.30,
        "description": "Time deposit",
        "type": "C/D"
      },
      {
        "id": "200",
        "amount": 62050.80,
        "description": "Saving account",
        "type": "Saving"
      }
    ]
  }
}'
```

Summary
-------
In this session, you have created a graph model that uses JSON-Path retrieval and search features. 
You have applied the plugin "f:listOfMap()" to consolidate maps of lists into a list of maps.
You have also tested the plugin "f:removeKey()" to remove unwanted key-values from a list of maps.

Note that JSON-Path retrieval and search syntax supports value comparison for selective key-value retrieval.
Please refer to JSON-Path syntax on the web for more details.
