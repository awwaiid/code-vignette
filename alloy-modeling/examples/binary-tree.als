// Binary Tree Model
// Models a binary tree data structure with proper constraints

sig Node {
    left: lone Node,
    right: lone Node,
    value: one Int
}

// Facts that make this a proper binary tree
fact ProperTree {
    // No cycles
    no n: Node | n in n.^(left + right)

    // Each node has at most one parent
    all n: Node | lone (left.n + right.n)

    // Exactly one root (node with no parent)
    one n: Node | no (left.n + right.n)

    // Left and right children are disjoint
    no n: Node | some n.left & n.right
}

// Binary search tree property
pred isBST {
    all n: Node {
        // All values in left subtree are less
        all l: n.left.*(left + right) | l.value < n.value

        // All values in right subtree are greater
        all r: n.right.*(left + right) | r.value > n.value
    }
}

// Balanced tree (heights differ by at most 1)
fun height[n: Node]: Int {
    n in Node implies
        plus[1, max[height[n.left] + height[n.right]]]
    else
        0
}

run isBST for 5
run {} for 7 Node
