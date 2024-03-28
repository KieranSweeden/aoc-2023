# Challenge Notes

## Part 1

Puzzle input = documents on how to navigate a desert.

One document contains list of **left/right** instructions.

Others documents describe a **network** of labeled notes.

You're meant to use the **left/right** instructions to _navigate_ the **network**.

We start at **AAA** and our final destination will always be **ZZZ**.

Each **node** in the **network** is formatted like such:

```
AAA = (BBB, CCC)
```

Which translates to:

```
<node-id> = (left-node-id, right-node-id)
```

The instructions are formatted like such:

```
RLRLLLRRL
```

L means navigate to the current node's left node & R means navigate to the current node's right node.

We'll need to repeat the whole sequence of instructions if we run out of instructions.

Our **answer** is the amount of steps required in order to reach the **ZZZ** when starting from **AAA**.

### Todos

- [] Parse Node from "GLJ = (QQV, JTL)"
- [] Parse Network using Nodes
