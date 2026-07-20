Tutorial 7
----------
In this tutorial, you will explore data mapping in more detail.

Exercise
--------
You will create a new graph model to test various data mapping methods.

To clear the previous graph session, click the Tools button in the top-right corner
and click the "Stop" and "Start" toggle button. A new graph session will start.

Create a root node and an end node
----------------------------------
Enter the following to create a root node and an end node.

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

`mapping[]` builds the node's data mapping statement list in "append mode": the
statements are evaluated in the order provided.

Each data mapping statement has a left-hand side (the source) and a right-hand side
(the target), separated by the "map to" indicator (`->`). The value of the source is
mapped to the target key.

MiniGraph uses the same data mapping syntax as Event Script. For a quick reference,
enter "help graph-data-mapper"; the full syntax lives in
docs/guides/event-script/syntax.md in this repository.

*Constant* — `text(world)` means a constant of "world". `output.body.` is the
namespace for the output payload when a graph finishes execution. In this example,
output.body is populated with "hello=world".

*Input* — `input.body` is the namespace for the input payload provided to a graph
instance when it starts.

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

The value "Peter" is mapped to the "name" field, and address1 and address2 become the
first and second elements of an array in "model.address". The `model.` namespace is a
temporary state machine that lives for the duration of the graph instance — use it as
a scratch buffer for data transformation.

*Output* — the mapping statement `model.address -> output.body.address` maps the
two-element address array into the output payload of the graph instance.

Building an array — two techniques
----------------------------------
*Direct addressing (preferred)* — set the array element index explicitly:

```
mapping[]=input.body.profile.address1 -> model.address[0]
mapping[]=input.body.profile.address2 -> model.address[1]
mapping[]=model.address -> output.body.address
```

Numeric indices write each value into a known slot, so the result is deterministic
and the mapping is idempotent — executing the node again simply overwrites the same
slots. Use direct addressing whenever you know where each value belongs.

*Append + clear (for append-mode workflows)* — the array append syntax (`[]`) adds
one element to the end of the array on every execution. That is what you want when a
workflow accumulates an unknown number of elements — but it is not idempotent: during
testing, you may execute the same node several times, and each pass would append
duplicate entries. To make an append sequence repeatable, clear the array first by
mapping a non-existent key (conventionally `model.none`) to it:

```
mapping[]=model.none -> model.address
mapping[]=input.body.profile.address1 -> model.address[]
mapping[]=input.body.profile.address2 -> model.address[]
```

Mapping a source that does not exist removes the target key — the `model.none` clear
idiom. This exercise deliberately uses the append + clear form so you can observe the
idiom at work.

*Plugin functions* — the left-hand side of `f:now(text(local)) -> output.body.time`
uses the `f:` syntax to execute a "plugin" function called "now". It takes the
constant value "local" and returns a local timestamp. A number of built-in data
mapping plugins are available — see the simple-plugin catalog in
docs/guides/event-script/syntax.md in this repository.

Test the data mapper
--------------------
You can test the data mapper before you complete the whole graph model.

Enter the following to instantiate the graph and open a dialog box for the mock
input data.

```
> instantiate graph
Graph instance created. Loaded 0 mock entries, model.ttl = 30000 ms
> upload mock data
Mock data loaded into 'input.body' namespace
```

When you enter the "upload mock data" command, an input dialog box opens. Paste the
sample input payload for the "profile" of "Peter" listed above.

To confirm that you have uploaded the mock input, enter "inspect input".

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

The system rejects the request with an error message: the data-mapper node is missing
a skill.

Enter 'edit node data-mapper', copy the printed "update node" block into the input
box, add "skill=graph.data.mapper" and submit.

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

To activate the updated node, restart the graph instance by entering
'instantiate graph' and 'upload mock data'. Submit the mock input payload again.

Then execute the data-mapper again.

```
> execute data-mapper
node data-mapper run for 0.488 ms with exit path 'next'
```

The data-mapper runs successfully.

Inspect the model and output
----------------------------
Inspect the model and the output key-values to see what values were mapped.

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

The graph model is shown in the right panel.

Export the graph model
----------------------
Save the graph model by exporting it.

```
> export graph as tutorial-7
Graph exported to /tmp/graph/tutorial-7.json
Described in /api/graph/model/tutorial-7/152-13
```

Deploy the graph model
----------------------
To deploy the graph model, copy "/tmp/graph/tutorial-7.json" to your application's
resources/graph folder. You can then test the deployed model with a curl command.

```
curl -X POST http://127.0.0.1:8100/api/graph/tutorial-7 \
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
In this tutorial, you created a graph model that performs data mapping. You compared
the two array-building techniques — direct addressing (preferred) and append + clear
with the `model.none` idiom — transformed address1 and address2 into an array, and
applied the "f:now()" plugin function to return the current time.
