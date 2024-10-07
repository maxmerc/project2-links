#[allow(unused_imports)]
use std::{cmp::Ord, mem};

#[derive(Clone, Debug)]
pub enum TreeNode<T: Ord> {
    Leaf,
    Node(T, Box<TreeNode<T>>, Box<TreeNode<T>>),
}

// Provided functions
impl<T: Ord> TreeNode<T> {
    pub fn height(&self) -> usize {
        match self {
            TreeNode::Leaf => 0,
            TreeNode::Node(_, left, right) => 1 + std::cmp::max(left.height(), right.height()),
        }
    }

    /// Verifies that the tree is a binary search tree
    fn is_bst(&self) -> bool {
        fn is_bst_helper<T: Ord>(tree: &TreeNode<T>, min: Option<&T>, max: Option<&T>) -> bool {
            match tree {
                TreeNode::Leaf => true,
                TreeNode::Node(value, left, right) => {
                    match min {
                        Some(min) => {
                            if value <= min {
                                return false;
                            }
                        }
                        _ => {}
                    }
                    match max {
                        Some(max) => {
                            if value >= max {
                                return false;
                            }
                        }
                        _ => {}
                    }
                    is_bst_helper(left, min, Some(value)) && is_bst_helper(right, Some(value), max)
                }
            }
        }
        is_bst_helper(self, None, None)
    }

    /// Verifies that the tree is balanced
    pub fn is_balanced(&self) -> bool {
        match self {
            TreeNode::Leaf => true,
            TreeNode::Node(_, left, right) => {
                let left_height = left.height();
                let right_height = right.height();
                let diff = (left_height as i32 - right_height as i32).abs();
                diff <= 1 && left.is_balanced() && right.is_balanced()
            }
        }
    }

    /// Verifies that the tree is a valid balanced binary search tree
    pub fn validate(&self) -> bool {
        self.is_bst() && self.is_balanced()
    }
}

// Required functions
impl<T: Ord> TreeNode<T> {
    /// Creates a new `TreeNode<T>` with value `value` and children `left` and `right`
    pub fn node(value: T, left: TreeNode<T>, right: TreeNode<T>) -> TreeNode<T> {
        TreeNode::Node(value, Box::new(left), Box::new(right))
    }

    /// Creates a new `TreeNode<T>` with no children
    pub fn new() -> TreeNode<T> {
        TreeNode::Leaf
    }

    /// Inserts a new node with value `value` into the tree. If the value already exists in the tree,
    /// the function does nothing.
    ///
    /// After insertion, the tree is rebalanced if necessary
    pub fn insert(&mut self, value: T) {
        match self {
            TreeNode::Leaf => {
                *self = TreeNode::Node(value, Box::new(TreeNode::Leaf), Box::new(TreeNode::Leaf));
            }
            TreeNode::Node(ref mut node_value, ref mut left, ref mut right) => {
                if value < *node_value {
                    left.insert(value);
                } else if value > *node_value {
                    right.insert(value);
                }
                self.rebalance();
            }
        }
    }

    /// Computes the balance factor of the tree (the difference between the height of the left and right subtrees)
    fn balance_factor(&self) -> i32 {
        match self {
            TreeNode::Leaf => 0,
            TreeNode::Node(_, left, right) => (left.height() as i32) - (right.height() as i32),
        }  
    }

    /// Performs a left rotation on the tree
    pub fn left_rotate(&mut self) {
        if let TreeNode::Node(_, _, right) = self {
            // Take the right child, which should be a Node
            if let TreeNode::Node(right_value, right_left, right_right) = *mem::take(right) {
                // Replace current node with the right child (new root)
                *right = right_left; // The new right of the current node (left of the old right)
                let old_root = mem::replace(self, TreeNode::Node(right_value, Box::new(TreeNode::Leaf), right_right));
                
                // Set the current node (old root) as the left child of the new root
                if let TreeNode::Node(_, ref mut new_left, _) = self {
                    *new_left = Box::new(old_root);
                }

            }
        }
    }
    /// Performs a right rotation on the tree
    pub fn right_rotate(&mut self) {
        if let TreeNode::Node(_, left, _) = self {
            if let TreeNode::Node(left_value, left_left, left_right) = *mem::take(left) {
                // Replace current node with the left child (new root)
                *left = left_right; // The new left of the current node (right of the old left)
                let old_root = mem::replace(self, TreeNode::Node(left_value, left_left, Box::new(TreeNode::Leaf)));
                // Set the current node (old root) as the right child of the new root
                if let TreeNode::Node(_, _, ref mut new_right) = self {
                    *new_right = Box::new(old_root);
                }
            }
        }
    }

    /// Rebalances the tree using either a single or double rotation, as specified in the AVL tree
    /// rebalancing algorithm.
    fn rebalance(&mut self) {
        let bf = self.balance_factor();
        if bf > 1 {
            // Left-heavy
            if let TreeNode::Node(_, ref mut left, _) = self {
                if left.balance_factor() < 0 {
                    left.left_rotate(); // Left-Right case
                }
            }
            self.right_rotate(); // Left-Left case
        } else if bf < -1 {
            // Right-heavy
            if let TreeNode::Node(_, _, ref mut right) = self {
                if right.balance_factor() > 0 {
                    right.right_rotate(); // Right-Left case
                }
            }
            self.left_rotate(); // Right-Right case
        }
    }
}

// Implement `Default` for `TreeNode<T>`
impl<T: Ord> Default for TreeNode<T> {
    fn default() -> Self {
        TreeNode::Leaf
    }
}


impl<T: Ord + PartialEq> PartialEq for TreeNode<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TreeNode::Leaf, TreeNode::Leaf) => true,
            (TreeNode::Node(val1, left1, right1), TreeNode::Node(val2, left2, right2)) => {
                val1 == val2 && left1 == left2 && right1 == right2
            }
            _ => false,
        }
    }
}

impl<T: Ord + Eq> Eq for TreeNode<T> {}


impl<T: Ord> From<Vec<T>> for TreeNode<T> {
    fn from(vec: Vec<T>) -> Self {
        let mut tree = TreeNode::new();
        for value in vec {
            tree.insert(value);
        }
        tree
    }
}

impl<T: Ord + Clone> From<TreeNode<T>> for Vec<T> {
    fn from(tree: TreeNode<T>) -> Vec<T> {
        let mut result = Vec::new();
        fn inorder_traversal<T: Ord + Clone>(node: &TreeNode<T>, vec: &mut Vec<T>) {
            if let TreeNode::Node(value, left, right) = node {
                inorder_traversal(left, vec);
                vec.push(value.clone());
                inorder_traversal(right, vec);
            }
        }
        inorder_traversal(&tree, &mut result);
        result
    }
}
