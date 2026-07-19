Tutorial 8
----------
In this tutorial, you will use the JSON-Path search feature to retrieve key-values from the input
payload, then reshape the result with the f:listOfMap() and f:removeKey() plugins. Reshaping a
third-party API response into your own internal data contract — "impedance matching" — is one of
the most common jobs for a data mapper, and these tools let you do it without writing code.

Exercise
--------
You will import tutorial-7 and replace some data mapping statements with JSON-Path search requests.

To clear the previous graph session, click the Tools button in the top-right corner and click the
"Stop" and "Start" toggle button. A new graph session will start.

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
The account holder "Peter" has 2 accounts. We will assume the following input payload data
structure. You will copy-n-paste this JSON dataset when the "upload mock data" dialog box opens
later in this exercise.

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

A mapping source that starts with "$." is a JSON-Path expression evaluated over the state machine.
The three JSON-Path statements above use the [*] wildcard to extract the type, id and amount from
every element of the account list in the input payload. For a simple key, prefer the plain
dot-bracket form (like the first statement) and save JSON-Path for queries that need it.

Test the data mapper
--------------------
Enter the following to instantiate the graph and open a dialog box to enter the mock input data.

```
> instantiate graph
Graph instance created. Loaded 0 mock entries, model.ttl = 30000 ms
> upload mock data
Mock data loaded into 'input.body' namespace
```

The first data mapping statement maps input.body.profile.name into the "name" field of the output
body. The JSON-Path statements extract the type, id and amount key-values from the account list
and map them into the model variables type, id and amount accordingly.

When you enter the "upload mock data" command, an input dialog box will open. Please paste the
sample input payload listed above.

To confirm that you have uploaded the mock input, enter "inspect input".

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

You can now test the data mapper by executing it. Enter "execute data-mapper".

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

This confirms that the JSON-Path statements have extracted the key-values from the account list
successfully. However, three parallel lists — a "map of lists" — is usually not a good schema
design: easy for an application to parse, but harder for a human to read. Let's turn it into a
proper list of maps.

Using the listOfMap plugin
--------------------------
For proper data structure representation, use the plugin f:listOfMap() to consolidate the maps of
lists into a list of maps. Update the data mapper like this:

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

Note the extra level of key called "account" that holds the 3 lists for type, id and amount. The
f:listOfMap() plugin then consolidates the maps of lists into a list of maps.

Instantiate the graph, upload the same mock data and execute the data-mapper again. When you enter
'inspect model' and 'inspect output', you will see:

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

This illustrates that the listOfMap plugin can perform simple data transformation. It is handy
when your graph model uses API fetchers to retrieve data from multiple sources: without writing
code, you can group data from different data structures into the shape your consumers expect.

Using the removeKey plugin
--------------------------
When the data comes from a single source, it is even easier to use the f:removeKey() plugin to
drop the unwanted keys directly. Its form is:

```
f:removeKey(source, text(key1), text(key2), ...)
```

It removes the named keys from a map — or from every map in a list — and returns a copy of the
data structure. Here it strips the "description" field from every account:

```
mapping[]=f:removeKey(input.body.profile.account, text(description)) -> output.body.account
```

Let's prove this by editing the data-mapper again. We add a new data mapping statement at the end
to map the alternative solution to the "account2" field in the output payload.

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

Do 'instantiate graph' and 'upload mock data' with the same input payload. Then
'execute data-mapper' and 'inspect output' to see the outcome.

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
the "description" key-value has been removed from each map in the list successfully.

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
To deploy the graph model, copy "/tmp/graph/tutorial-8.json" to your application's
`resources/graph` folder. You can then test the deployed model with a curl command.

```
curl -X POST http://127.0.0.1:8100/api/graph/tutorial-8 \
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
In this tutorial, you have used JSON-Path retrieval to extract key-values from a list, applied the
f:listOfMap() plugin to consolidate maps of lists into a list of maps, and used the f:removeKey()
plugin to remove unwanted key-values from a list of maps — the building blocks for reshaping a
third-party API response into your internal data contract.

Note that JSON-Path also supports value comparison for selective key-value retrieval. Please refer
to a JSON-Path syntax reference on the web for more details.
