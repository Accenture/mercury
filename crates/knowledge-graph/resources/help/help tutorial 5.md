Tutorial 5
----------
In this session, we will explore parallel processing and sophisticated graph navigation using a node
with the skill 'graph.join'.

Exercise
--------
You will import the graph model from tutorial-3 and update it to fetch two user profiles at the same time.

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

Review the fetcher node
-----------------------
Enter 'edit node fetcher' to review the configuration of the node. The system displays the following:

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
Assume the use case that we want to fetch two user profiles at the same time. You will create two fetchers
like this:

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

When two skilled nodes are executed in parallel, we must pay attention to avoid one execution stepping
on the memory space of another one. In this case, we can use two temporary variables in the "state machine".

The state machine uses the namespace "model", we therefore use two variables `model.fetcher-1` and `model.fetcher-2`
to avoid concurrent updates to the same variable.

The final step of output data mapping is the use of array append syntax `[]`. This tells the system to append
the map containing name and address to the variable 'profile'.

Due to parallelism, the order of the array is undetermined. If you want to guarantee person1's result go to array
element-0 and person2 to element-1, set the array element index directly. e.g.

```
output[]=model.fetcher-1 -> output.body.profile[0]
```

```
output[]=model.fetcher-2 -> output.body.profile[1]
```

Since profile order does not matter in this tutorial, we will use the array append feature `[]`.

Create a join node
------------------
You can now create a "join" node like this:

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

After you have deleted the original fetcher, its connections to the root node and end node will be removed too.

Connect the new fetchers
------------------------
Please enter the following to define the graph navigation.

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
You may start the graph model with this mock input:

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

If you check the application log, you will see the two fetchers are executed in parallel.

```
2026-04-02 16:47:32.633 INFO  com.accenture.minigraph.skills.GraphApiFetcher:410 - 
           GET http://127.0.0.1:8085/api/mdm/profile/100, with [person_id], ttl=30000
2026-04-02 16:47:32.633 INFO  com.accenture.minigraph.skills.GraphApiFetcher:410 - 
           GET http://127.0.0.1:8085/api/mdm/profile/200, with [person_id], ttl=30000
```

Create an island to hold data dictionary
----------------------------------------
Just like tutorial 3, you will create an island node to hold the data dictionary and provider nodes.

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
You may save the graph model by exporting it.

```
> export graph as tutorial-5
Graph exported to /tmp/graph/tutorial-5.json
Described in /api/graph/model/tutorial-5/920-28
```

Deploy the graph model
----------------------
To deploy the graph model, copy "/tmp/graph/tutorial-5.json" to your application's `main/resources/graph` folder.
You can then test the deployed model with a curl command.

```
curl -X POST http://127.0.0.1:8085/api/graph/tutorial-5 \
  -H "Content-Type: application/json" \
  -d '{
    "person1": 100,
    "person2": 200
  }'
```

Summary
-------
In this session, you have created a graph model that is capable of doing parallel processing. It makes two
API requests to fetch data at the same time. The two nodes then converge into a "join" node before reaching
the "end" node.

The execution of a graph instance is guided by "graph traversal". It will follow the connections that you define
for the nodes. If a node has a skill assigned, the graph executor will run the composable function that provides
the skill. If the node does not have a skill, the graph executor will find the next 'downstream' node from there.
