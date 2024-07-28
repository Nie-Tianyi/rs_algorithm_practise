use std::cmp::max;
use std::fmt::Display;

pub struct BinaryTree<T>(Option<Box<TreeNode<T>>>);

pub struct TreeNode<T> {
    data: T,
    left_node: BinaryTree<T>,
    right_node: BinaryTree<T>,
}

impl<T: Display + Clone> Display for BinaryTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BinaryTree: {{ \n")?;
        write!(f, "}}")
    }
}

impl<T> BinaryTree<T> {
    #[inline]
    pub fn new() -> Self {
        BinaryTree(None)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    pub fn max_depth(&self) -> usize {
        return match self.0 {
            None => {
                0
            }
            Some(ref node) => {
                1 + max(node.left_node.max_depth(), node.right_node.max_depth())
            }
        };
    }
}

impl<T: Clone> BinaryTree<T> {
    /// traverse the binary tree in preorder **recursively**
    pub fn preorder_traversal(&self, v: &mut Vec<T>) {
        if let Some(ref node) = self.0 {
            v.push(node.data.clone());
            node.left_node.preorder_traversal(v);
            node.right_node.preorder_traversal(v);
        }
    }
    /// traverse the binary tree in inorder **recursively**
    pub fn inorder_traversal(&self, v: &mut Vec<T>) {
        if let Some(ref node) = self.0 {
            node.left_node.inorder_traversal(v);
            v.push(node.data.clone());
            node.right_node.inorder_traversal(v);
        }
    }
    /// traverse the binary tree in postorder **recursively**
    pub fn postorder_traversal(&self, v: &mut Vec<T>) {
        if let Some(ref node) = self.0 {
            node.left_node.postorder_traversal(v);
            node.right_node.postorder_traversal(v);
            v.push(node.data.clone());
        }
    }

    pub fn breadth_first_traversal(&self) -> Vec<T> {
        vec![]
    }
}

impl<T: Ord> BinaryTree<T> {
    pub fn add_sort(&mut self, data: T) {
        match self.0 {
            Some(ref mut bd) => {
                if data < bd.data {
                    bd.left_node.add_sort(data);
                } else {
                    bd.right_node.add_sort(data);
                }
            }
            None => {
                self.0 = Some(Box::new(TreeNode {
                    data,
                    left_node: BinaryTree::new(),
                    right_node: BinaryTree::new(),
                }));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt() {
        let mut bt = BinaryTree::new();
        bt.add_sort(10);
        bt.add_sort(8);
        bt.add_sort(12);
        bt.add_sort(7);
        bt.add_sort(9);
        bt.add_sort(11);
        bt.add_sort(13);

        let mut v = Vec::new();

        bt.postorder_traversal(&mut v);

        println!("{:?}", v)
    }

    #[test]
    fn test_max_depth() {
        let mut binary_tree = BinaryTree::new();

        binary_tree.add_sort(10); // tree: 10
        assert_eq!(binary_tree.max_depth(), 1);

        binary_tree.add_sort(12); // tree: 10 [None,12]
        assert_eq!(binary_tree.max_depth(), 2);

        binary_tree.add_sort(11); // tree: 10 [None,12] [[None,None][11,None]]
        assert_eq!(binary_tree.max_depth(), 3);

        binary_tree.add_sort(13); // tree: 10 [None,12] [[None,None][11,13]]
        assert_eq!(binary_tree.max_depth(), 3);

        binary_tree.add_sort(9); // // tree: 10 [9,12] [[None,None][11,13]]
        assert_eq!(binary_tree.max_depth(), 3);
    }
}
