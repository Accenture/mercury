Tutorial 5
----------
In this tutorial, you will explore parallel processing and graph navigation using a
node with the skill 'graph.join'.

Exercise
--------
You will import the graph model from tutorial-3 and update it to fetch two user
profiles at the same time.

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

Review the fetcher node
-----------------------
Enter 'edit node fetcher' to review the configuration of the node. The system
displays the following:

```
update node fetcher
with type Fetcher
with properties
dictionary[]=person-name
dictionary[]=person-address
input[]=input.body.person_id -> person_id
output[]=result.name -> output.body.name
output[]=result.address -> output.body.address
skill=graph.api.fetcher
```

Create two new fetchers
-----------------------
Assume the use case is to fetch two user profiles at the same time. Create two
fetchers like this:

```
create node fetcher-1
with type Fetcher
with properties
dictionary[]=person-name
dictionary[]=person-address
input[]=input.body.person1 -> person_id
output[]=result.name -> model.fetcher-1.name
output[]=result.address -> model.fetcher-1.address
output[]=model.fetcher-1 -> output.body.profile[]
skill=graph.api.fetcher
```

```
create node fetcher-2
with type Fetcher
with properties
dictionary[]=person-name
dictionary[]=person-address
input[]=input.body.person2 -> person_id
output[]=result.name -> model.fetcher-2.name
output[]=result.address -> model.fetcher-2.address
output[]=model.fetcher-2 -> output.body.profile[]
skill=graph.api.fetcher
```

When two skilled nodes execute in parallel, pay attention to how they share the state
machine. Data mapping itself is thread-safe — state-machine operations are
serialized — but parallel branches must not write to the same scalar key: the last
writer wins, nondeterministically. Write to disjoint keys instead. Here, each fetcher
assembles its profile under its own temporary variable in the "model" namespace:
`model.fetcher-1` and `model.fetcher-2`.

The final output mapping uses the array append syntax `[]`, which appends the map
containing name and address to the 'profile' array. Appending with `[]` from parallel
branches is race-free, but the element order follows completion order — undetermined
across parallel branches. If you must guarantee that person1's result goes to array
element 0 and person2's to element 1, set the array element index directly:

```
output[]=model.fetcher-1 -> output.body.profile[0]
```

```
output[]=model.fetcher-2 -> output.body.profile[1]
```

Since profile order does not matter in this tutorial, we will use the append form `[]`.

Create a join node
------------------
Create a "join" node to synchronize the two parallel branches:

```
create node join
with type Join
with properties
skill=graph.join
```

Remove the original fetcher node
--------------------------------
Enter 'delete node fetcher' to remove the original fetcher node.

```
> delete node fetcher
node fetcher deleted
```

When the original fetcher is deleted, its connections to the root node and end node
are removed too.

Connect the new fetchers
------------------------
Enter the following to define the graph navigation.

```
connect root to fetcher-1 with one
connect root to fetcher-2 with two
connect fetcher-1 to join with join
connect fetcher-2 to join with join
connect join to end with done
```

Do a 'list connections' to confirm the setup.

```
> list connections
root -[one]-> fetcher-1
root -[two]-> fetcher-2
fetcher-1 -[join]-> join
fetcher-2 -[join]-> join
join -[done]-> end
```

Perform a dry-run
-----------------
Start the graph model with this mock input:

```
start graph
int(100) -> input.body.person1
int(200) -> input.body.person2
```

Then enter 'run' to execute the graph instance.

```
> run
Walk to root
Walk to fetcher-2
Walk to fetcher-1
Executed fetcher-1 with skill graph.api.fetcher in 1.048 ms
Walk to join
Executed fetcher-2 with skill graph.api.fetcher in 0.931 ms
Walk to join
Executed join with skill graph.join in 0.04 ms
Walk to end
Executed join with skill graph.join in 0.017 ms
{
  "output": {
    "body": {
      "profile": [
        {
          "address": "100 World Blvd",
          "name": "Mary"
        },
        {
          "address": "100 World Blvd",
          "name": "Peter"
        }
      ]
    }
  }
}
Graph traversal completed in 6 ms
```

If you check the application log, you will see the two fetchers executed in parallel.

```
2026-04-02T23:47:32.633Z INFO  [knowledge_graph::fetcher] GET http://127.0.0.1:8100/api/mdm/profile/100, with ["person_id"], ttl=30000
2026-04-02T23:47:32.633Z INFO  [knowledge_graph::fetcher] GET http://127.0.0.1:8100/api/mdm/profile/200, with ["person_id"], ttl=30000
```

Create an island to hold the data dictionary
--------------------------------------------
Just like tutorial 3, wire the data dictionary and provider nodes into the graph's
knowledge layer with an island node. This is the required convention — leave no node
unconnected: the island subgraph is the graph's entity-relationship diagram, turning
the graph into living documentation of enterprise knowledge.

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
root -[one]-> fetcher-1
root -[two]-> fetcher-2
dictionary -[data]-> person-address
dictionary -[data]-> person-name
fetcher-1 -[join]-> join
fetcher-2 -[join]-> join
person-address -[provider]-> mdm-profile
person-name -[provider]-> mdm-profile
join -[done]-> end
```

Export the graph model
----------------------
Save the graph model by exporting it.

```
> export graph as tutorial-5
Graph exported to /tmp/graph/tutorial-5.json
Described in /api/graph/model/tutorial-5/920-28
```

Deploy the graph model
----------------------
To deploy the graph model, copy "/tmp/graph/tutorial-5.json" to your application's
resources/graph folder. You can then test the deployed model with a curl command.

```
curl -X POST http://127.0.0.1:8100/api/graph/tutorial-5 \
  -H "Content-Type: application/json" \
  -d '{
    "person1": 100,
    "person2": 200
  }'
```

Summary
-------
In this tutorial, you created a graph model capable of parallel processing. It makes
two API requests at the same time; the two branches then converge into a "join" node
before reaching the "end" node.

The execution of a graph instance is guided by graph traversal: it follows the
connections you define between nodes. If a node has a skill, the graph executor runs
the composable function that provides the skill; if not, the graph executor continues
to the next downstream node.
