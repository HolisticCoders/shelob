# Shelob Design

## Shelob Core

Dependencies: None
Crate Type: Library
Description: This is the heart of shelob. A dependency graph that evaluates really fast.
There might be multiple types of graphs (which might even deserve their own libraries):

1. push/pull evaluation: for graphs that don't have one clear entry point.

   Pick a node in the graph that we want to evaluate, recursively walk the graph and tag the nodes that need re-evaluating (Dirty propagation).
   Then evaluate the dirty nodes that don't have dirty nodes connected in their inputs. repeat until all the dirty nodes are evaluated up until the node that was picked in the first place.

2. Forward execution: Graphs that do have an entry point. similar to Unreal's blueprint. These let you control the flow of execution explicitly

3. Something else?

Shelob Core should expose the base API to create nodes, attributes types.
I think having infered types for attributes should be done here?

### Nodes

There are two types of nodes:

1. Built in nodes, which are compiled nodes written in rust, with Shelob's API
2. Compound Nodes: These are actually Shelob graphs that contain other nodes. Evaluating a compound node actually evaluates its inner graph. It would be cool to be able to add a forward graph compound inside a push/pull graph and vice versa but that might be complex? To be discussed.

### Attributes

They can be:

- Single values or arrays. An array attribute has is exactly like Maya's Multi attributes
- Input, output or both
- Private or public. Private nodes are only accessible from inside the compound
- get/set from within the compound with get/set nodes
- Inferred (automatic type detection, can be limited to some types)

## Shelob Array

Dependencies: Shelob Core
Crate Type: Library
Description: Array Attributes and array manipulation. It might need to be a dependency for every crate that introduce a new attribute type.

## Shelob Numeric

Dependencies: Shelob Core
Crate Type: Library
Description: Basic numerical operations

### Attributes

- Integer

- Float

### Nodes

* Add/Substract/Multiply/divide: self explanatory. All attributes types are inferred (Limited to numeric types and vectors of numeric types)
* Remap. Two options there:
  * Remap Range: just take an old_min/old_max and remap that to a new_min, new_max range
  * Remap Curve: Let the user control the remap precisely with a curve and do some fancy non linear stuff
* Clamp
* Power
* Absolute
* Round
* Truncate
* Lerp
* more stuff

## Shelob Trigonometry

Dependencies: Shelob Core, Shelob Numeric
Crate Type: Library
Description: Everything trigonometry, probably a good place to introduce radians, degrees, etc.

## Shelob String

Dependencies: Shelob Core
Crate Type: Library
Description: String manipulation

### Attributes
- String
### Nodes

- TBD

## Shelob Logic

Dependencies: Shelob Core
Crate Type: Library

### Attributes:

- Boolean

### Nodes

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

## Shelob Vector

Dependencies: Shelob Core, Shelob Numeric
Crate Type: Library

### Attributes:

* Vector: an attribute with n values. not to be mistaken with arrays. We will have arrays of vectors

### Nodes: 

* Vector math stuff, TBD

## Shelob Matrix

Dependencies: Shelob Core
Crate Type: Library

Looks like a good reference for matrix operations: https://en.wikipedia.org/wiki/Matrix_(mathematics)#Basic_operations

Attributes Types:

- Matrix (not just 4x4 matrices, any matrix size)

Nodes:

* Add Matrix
* Scalar Multiplication
* Transpose Matrix
* Multiply matrix
* Inverse matrix

## Shelob OS Nodes

Dependencies: Shelob Core, Shelob Strings
Crate Type: Library
Description: Nodes to interact with the OS. manipulate files, spawn/kill processes etc.

## Shelob 3D

Dependencies: Shelob Core, Shelob Array, Shelob Numeric, Shelob Matrix, Shelob Trigonometry
Crate Type: Library
Description: This is a big and definitely not exhaustive one.

### Attributes

* Mesh: Dictionary like data that describes the mesh vertices, their position and how they are connected to each other. This will probably be more complex as we go and might include color data, material data, etc. For now let's just focus on the shape of the node.

### Nodes

* Transform: Most basic 3D node. Basically just a results in 4x4 matrix describing its transformation.
  The way I see it, it'll include 4 different matrices evaluated in the described order:

  1. Rest Matrix. Similar to blender's edit mode for bones

  2. Pre transform matrices: This is an array of matrices that constraints will use to modify the node's transformation without locking its transforms from the user.

     A neat thing there would be to be able to change how those matrices work together per matrix (think blending modes in photoshop) here's a few ideas

     - Multiplication: The matrices would just add up to each other
     - Linear interpolation: The matrix that's below would override the one(s) above by 0-1 amount

  3. Transformation matrix: This is the user manipulated matrix, maybe only exposed through translation/rotation/scale/shear

  4. Post Transform Matrices: just like the pre transform matrices but evaluated after the transformation

  They'll output both a local matrix and a world matrix. These are the result of the evaluation of all the matrices described above.

* Constraints:

  * Parent Constraint
  * Point Constraint
  * Orient Constraint
  * Scale Constraint
  * Aim Constraint
  * IK Constraint(s)
  * Twist extractor (possibly with infinite twist capabilities?)

* Shape nodes:

  I don't have a ton of things to say right now but I'd like them to have a stack of modifiers, similar to blender with a twist: Modifiers can be either built-in rust modifiers or Shelob graphs that have access to the shape's data and can modify vertex position.