# Shelob Design

## Shelob Core

Dependencies: None
Crate Type: Library

This is the heart of shelob. A dependency graph that evaluates really fast.
There might be multiple types of graphs (which might even deserve their own libraries):

1. push/pull evaluation: for graphs that don't have one clear entry point.

   Pick a node in the graph that we want to evaluate, recursively walk the graph and tag the nodes that need re-evaluating (Dirty propagation).
   Then evaluate the dirty nodes that don't have dirty nodes connected in their inputs. repeat until all the dirty nodes are evaluated up until the node that was picked in the first place.

2. Forward execution: Graphs that do have an entry point. similar to Unreal's blueprint. These let you control the flow of execution explicitly

3. Something else?

Shelob Core should expose the base API to create nodes, typeless attributes.

### Nodes

There are two types of nodes:

1. Built in nodes, which are compiled nodes written in rust, with Shelob's API
2. Compound Nodes: These are actually Shelob graphs that contain other nodes. Evaluating a compound node actually evaluates its inner graph. It would be cool to be able to add a forward graph compound inside a push/pull graph and vice versa but that might be complex? To be discussed.

### Attributes

They can be:

- Single values or arrays
- Input, output or both
- Private or public. Private nodes are only accessible from inside the compound
- get/set from within the compound with get/set nodes


## Shelob Data Types

Dependencies: Shelob Core
Crate Type: Library

Exposes important data types for other libraries to use. Here's a non exhaustive list of them:

- Integer
- Float
- String
- Boolean
- Enum
- Vector: an attribute with n values. not to be mistaken with arrays. We will have arrays of vectors
- Matrix (not just 4x4 matrices, any matrix type) (Too specific? could be it's own crate)
- Inferred (automatic type detection, can be limited to some types)

## Shelob Value Nodes

One node per data type listed above that just outputs a value

## Shelob Logic Nodes

Dependencies: Shelob Core, Shelob Data Types
Crate Type: Library

Depending on whether we split the different graphs into separate libraries, we might need multiple libraries that work with the different types of evaluation

This library adds a set of logic nodes:

* Branch: Evaluates one part of the graph or another based on a Boolean
* Switch/case: evaluates different branches of the graph based on an input
* Loops:
  Loop nodes are compound nodes, they evaluate their inner graph over and over until 
  * While: Loops until the break condition is met.
  * Do While: Same as above but evaluates at least once.
  * For each: Loop over an array.
  * Iterate: Loops n times.
* Break (probably only useful in forward evaluation graphs): Exit out of a loop
* Continue (probably only useful in forward evaluation graphs): Skips the rest of the evaluation and goes to the next iteration of the loop

## Shelob Math Nodes

Dependencies: Shelob Core, Shelob Data Types
Crate Type: Library

* Add/Substract/Multiply/divide: self explanatory. All attributes types are inferred (Limited to numeric types and vectors of numeric types)
* Trig nodes (to lazy to list right now)
* Matrix Math (could be in a dedicated matrix crate along with the matrix datatype)

## Shelob OS Nodes

Dependencies: Shelob Core, Shelob Data Types
Crate Type: Library

Nodes to interact with the OS. manipulate files, spawn/kill processes etc.

Note: This might end up being OS dependent if there are no good rust way of handling all of this

