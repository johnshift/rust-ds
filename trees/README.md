## Binary Tree
  - each node can either have 0, 1, or 2 nodes
  - each child can only have 1 parent
  - ideal trees: number of nodes doubles on each level
  - ideal trees: number of nodes on last level is equal to the sum of the number of nodes on all the other levels plus 1
  - no. of nodes per level = 2^#_levels
    - level 0: 2^0 = 1
    - level 1: 2^1 = 2
    - level 2: 2^2 = 4
    - level 3: 2^3 = 8
  - no. of nodes = 2^#_levels - 1

  - complexity:
    - lookup: O(log N)
    - insert: O(log N)
    - delete: O(log N)
    - "log N" means maximum number of steps needed to find a node
    - "O(log N)" simply means you only need to explore a subset instead of
      iterating its elements 1 by 1
    - Note: log2 n = k  <===> n = 2^k
    - e.g. 1sec for 10 elements, 2sec for 20 elements, 3sec for 30 elements
    - "Time goes up linearly, while the number of elements is exponential."
    - "if time:element = 1:1, then O(log N) means more elements for lesser 1:1 time"

- - -

## Binary Search Tree
  - Rules:
    1. If you keep going right/left, the child nodes should always be greater/lesser than the current node
    2. A node can only have 2 children

- - - 

## Pro vs Cons
  - Pros:
    - Better than O(n)
    - Sorted
    - Flexible Size
  - Cons:
    - No O(1) operations

- - -

## Balanced vs Unbalanced Trees
  - Unbalanced Trees worst case: O(log n) becomes ---> O(n)
  - Algorithms to balance binary trees:
    - AVL Tree
    - Red Black Tree

