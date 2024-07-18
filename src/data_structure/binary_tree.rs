pub struct BinaryTree<T>(Option<Box<TreeNode<T>>>);

pub struct TreeNode<T> {
    data: T,
    left_node: BinaryTree<T>,
    right_node: BinaryTree<T>,
}

impl<T> BinaryTree<T> {
    pub fn new() -> Self {
        BinaryTree(None)
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
                }))
            }
        }
    }
}
