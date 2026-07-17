Tutorial 7
----------
In this session, we will discuss data mapping in more details.

Exercise
--------
You will create a new graph model with to test various data mapping methods.

To clear the previous graph session, click the Tools button in the top-right corner and click the "Stop" and "Start"
toggle button. A new graph session will start.

Create a root node and an end node
----------------------------------
Enter the following to create a root node and an end node

```
create node root
with type Root
with properties
name=tutorial-7
purpose=Demonstrate various data mapping methods
```

```
create node end
with type End
with properties
```

Create a data mapper node
-------------------------
Let's try some data mapping methods. Please enter the following:

```
create node data-mapper
with type Mapper
with properties
mapping[]=text(world) -> output.body.hello
mapping[]=input.body.profile.name -> output.body.name
mapping[]=model.none -> model.address
mapping[]=input.body.profile.address1 -> model.address[]
mapping[]=input.body.profile.address2 -> model.address[]
mapping[]=model.address -> output.body.address
mapping[]=f:now(text(local)) -> output.body.time
```

`mapping[]` tells the system to create a data mapping statement in "append mode"
so that the statements will be evaluated in the order that they are provided.

Each data mapping statement has a left-hand-side and right-hand-side separated by the "map to" (`->`) indicator.

The value of the left-hand-side will be mapped to the key of the right-hand-side.

The MiniGraph system uses the same Event Script's data mapping syntax. For more details, please refer to
[Data Mapping Syntax](https://accenture.github.io/mercury-composable/guides/CHAPTER-4/#tasks-and-data-mapping)
(*right-click to open new tab*).

*Constant* - 'text(world)' means a constant of "world". `output.body.` is the namespace for the output payload
when a graph finishes execution. In this example, the output.body will be populated with "hello=world".

*Input* - `input.body` is the namespace for input payload that is provided to a graph instance when it is started.

Assuming the input payload looks like this:

```json
{ 
  "profile": {
    "name": "Peter",
    "address1": "100 World Blvd",
    "address2": "New York"
  }
}
```

The value "Peter" will be mapped to the "name" field and the address1 and address2 as the first and second element
of an array in "model.address". The `model.` namespace refers to a temporal state machine during the execution of 
the graph instance. You can use the model key-values as temporary data buffer for data transformation.

*Output* - the mapping statement `model.address -> output.body.address` maps the address array with 2 elements
into the output payload of the graph instance when it finishes execution.

*Idempotent design* - the array append syntax (`[]`) would create side effect when the same array key has been used
more than once. For example, during testing, you may execute the same node multiple times. This would create
duplicated entries in the array. To ensure idempotence, you can clear the model array key before you append values.
This is done by mapping an non-existent model key (e.g. `model.none`) to the model.address array field.

For this exercise, a better solution would be direct addressing instead of "append" mode:

```
mapping[]=input.body.profile.address1 -> model.address[0]
mapping[]=input.body.profile.address2 -> model.address[1]
mapping[]=model.address -> output.body.address
```

It achieves the same outcome without using the clear variable method (`model.none -> model.address`).

*plugin functions* - the left-hand-side of `f:now(text(local)) -> output.body.time` uses the "f:" syntax
to execute a "plugin" function called "now". It takes the constant value of "local" to return a local time stamp.

A number of built-in data mapping plugins are available. Please refer to the Event Script syntax page above for
more details.

Test the data mapper
--------------------
You can test the data mapper before you complete the whole graph model.

Enter the following to instantiate the graph and open a dialog box to enter the mock input data.

```
> instantiate graph
Graph instance created. Loaded 0 mock entries, model.ttl = 30000 ms
> upload mock data
Mock data loaded into 'input.body' namespace
```

When you enter the "upload mock data" command, an input dialog box will be opened. Please paste the sample
input payload for the "profile" of "Peter" listed above.

To confirm that you have uploaded the mock input. Enter "inspect input".

```
> inspect input
{
  "inspect": "input",
  "outcome": {
    "body": {
      "profile": {
        "address2": "New York",
        "address1": "100 World Blvd",
        "name": "Peter"
      }
    }
  }
}
```

You can now test the data mapper by "executing" it. Enter "execute data-mapper".

```
> execute data-mapper
ERROR: node data-mapper does not have a skill property
```

The system rejects the request with an error message telling that the data mapper is missing a skill.

You can update the data-mapper node with the 'edit node data-mapper' command and copy-n-paste the content
to the inbox box for editing. Add "skill=graph.data.mapper" and submit.

```
> edit node data-mapper
update node data-mapper
with type Mapper
with properties
mapping[]=text(world) -> output.body.hello
mapping[]=input.body.profile.name -> output.body.name
mapping[]=model.none -> model.address
mapping[]=input.body.profile.address1 -> model.address[]
mapping[]=input.body.profile.address2 -> model.address[]
mapping[]=model.address -> output.body.address
mapping[]=f:now(text(local)) -> output.body.time
skill=graph.data.mapper
```

The system will display "node data-mapper updated".

To activate the updated node, you can re-start the graph instance by entering 'instantiate graph' and
'update mock data'. Submit the mock input payload.

Then execute the data-mapper again.

```
> execute data-mapper
node data-mapper run for 0.488 ms with exit path 'next'
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
    "address": [
      "100 World Blvd",
      "New York"
    ]
  }
}
> inspect output
{
  "inspect": "output",
  "outcome": {
    "body": {
      "address": [
        "100 World Blvd",
        "New York"
      ],
      "name": "Peter",
      "hello": "world",
      "time": "2026-04-11 19:52:22.527"
    }
  }
}
```

Connect the nodes to complete the graph model
---------------------------------------------
Enter the two connect commands below.

```
> connect root to data-mapper with mapping
node root connected to data-mapper
> connect data-mapper to end with complete
node data-mapper connected to end
```

The graph model will be shown in the right panel.

Export the graph model
----------------------
You may save the graph model by exporting it.

```
> export graph as tutorial-7
Graph exported to /tmp/graph/tutorial-7.json
Described in /api/graph/model/tutorial-7/152-13
```

Deploy the graph model
----------------------
To deploy the graph model, copy "/tmp/graph/tutorial-7.json" to your application's `main/resources/graph` folder.
You can then test the deployed model with a curl command.

```
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-7 \
  -H "Content-Type: application/json" \
  -d '{ 
  "profile": {
    "name": "Peter",
    "address1": "100 World Blvd",
    "address2": "New York"
  }
}'
```

Summary
-------
In this session, you have created a graph model that data mapping. You used the array append method to transform
the input address1 and address2 into an array. You learnt how to clear model variable using an non-existing variable
`model.none`. You also applied the "f:now()" plugin function to return the current time.
