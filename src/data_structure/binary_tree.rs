use std::cmp::max;
use std::fmt::Display;

pub struct BinaryTree<T>(Option<Box<TreeNode<T>>>);

pub struct TreeNode<T> {
    data: T,
    height: usize, // for balancing the tree
    left_node: BinaryTree<T>,
    right_node: BinaryTree<T>,
}

impl<T: Display + Clone> Display for BinaryTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let arr = self.to_vec(); // nodes to be displayed
        let mut iter = arr.iter();
        let depth = self.depth(); // the depth of the tree
        let max_width = 2_usize.pow(depth as u32 - 1) * 2;

        writeln!(f, "BinaryTree: {{ ")?;
        for level in 0..depth {
            let num_of_items_to_display = 2_usize.pow(level as u32);
            let spaces_before = max_width / (2_usize.pow(level as u32 + 1));
            let space_between_items = spaces_before * 2 - 1;

            let space_before = " ".repeat(spaces_before);
            let space_between = " ".repeat(space_between_items);

            write!(f, "{}", space_before)?;
            for _ in 0..num_of_items_to_display {
                if let Some(next_item) = iter.next() {
                    match next_item {
                        None => write!(f, "None{}", space_between)?,
                        Some(data) => write!(f, "{}{}", data, space_between)?,
                    }
                }
            }
            writeln!(f)?;
        }
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

    /// calculate the depth of the tree **recursively**.
    ///
    /// this function should have the same result as `BinaryTree::height(&self)`
    pub fn depth(&self) -> usize {
        match self.0 {
            None => 0,
            Some(ref node) => 1 + max(node.left_node.depth(), node.right_node.depth()),
        }
    }
    /// get the height of the node
    #[inline]
    pub fn height(&self) -> usize {
        match self.0 {
            None => 0,
            Some(ref node) => node.height,
        }
    }
    /// set the node height when add a new node
    #[inline]
    fn set_height(&mut self) {
        if let Some(ref mut node) = self.0 {
            node.height = 1 + max(node.left_node.height(), node.right_node.height());
        }
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

    /// transit a binary tree to a vector of Option **recursively**:
    ///
    /// the index of the first node is 0 in the vector, the index of
    /// the other node is 2N + 1 (if it is a left node) or 2N + 2 (
    /// if it is a right node), where N is the index of its parent
    /// node.
    ///
    /// # Returns
    /// `Vec<Option<T>>`: a vector of option<T>
    ///
    /// # Examples
    /// ```rust
    /// use rs_algorithm_practise::data_structure::binary_tree::BinaryTree;
    ///
    /// let mut binary_tree = BinaryTree::new();
    ///
    /// binary_tree.add_sort(10); // tree: 10
    /// assert_eq!(vec![Some(10)], binary_tree.to_vec());
    ///
    /// binary_tree.add_sort(12); // tree: 10 [None,12]
    /// assert_eq!(vec![Some(10), None, Some(12)], binary_tree.to_vec());
    ///
    /// binary_tree.add_sort(11); // tree: 10 [None,12] [[None,None][11,None]]
    /// assert_eq!(vec![Some(10), None, Some(12), None, None, Some(11), None], binary_tree.to_vec());
    /// ```
    pub fn to_vec(&self) -> Vec<Option<T>> {
        let mut arr = vec![None; 2_usize.pow(self.depth() as u32) - 1];
        self.fill(&mut arr, 0, 0);
        arr
    }
    // n stand for parent node index, b stand for bias (left node's bias is 1, right node's bias is 2)
    fn fill(&self, arr: &mut Vec<Option<T>>, n: usize, b: usize) {
        match self.0 {
            None => (),
            Some(ref node) => {
                let index = 2 * n + b;
                arr[index] = Some(node.data.clone());
                node.left_node.fill(arr, index, 1);
                node.right_node.fill(arr, index, 2);
            }
        }
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
                    height: 0,
                    left_node: BinaryTree::new(),
                    right_node: BinaryTree::new(),
                }));
            }
        }
        self.set_height();
    }
}

impl<T> Default for BinaryTree<T> {
    fn default() -> Self {
        Self::new()
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

        println!("{}", bt)
    }

    #[test]
    fn test_max_depth() {
        let mut binary_tree = BinaryTree::new();

        binary_tree.add_sort(10); // tree: 10
        assert_eq!(binary_tree.depth(), 1);

        binary_tree.add_sort(12); // tree: 10 [None,12]
        assert_eq!(binary_tree.depth(), 2);

        binary_tree.add_sort(11); // tree: 10 [None,12] [[None,None][11,None]]
        assert_eq!(binary_tree.depth(), 3);

        binary_tree.add_sort(13); // tree: 10 [None,12] [[None,None][11,13]]
        assert_eq!(binary_tree.depth(), 3);

        binary_tree.add_sort(9); // tree: 10 [9,12] [[None,None][11,13]]
        assert_eq!(binary_tree.depth(), 3);
    }

    #[test]
    fn test_to_vec() {
        let mut binary_tree = BinaryTree::new();

        binary_tree.add_sort(10); // tree: 10
        assert_eq!(vec![Some(10)], binary_tree.to_vec());

        binary_tree.add_sort(12); // tree: 10 [None,12]
        assert_eq!(vec![Some(10), None, Some(12)], binary_tree.to_vec());

        binary_tree.add_sort(11); // tree: 10 [None,12] [[None,None][11,None]]
        assert_eq!(
            vec![Some(10), None, Some(12), None, None, Some(11), None],
            binary_tree.to_vec()
        );

        binary_tree.add_sort(13); // tree: 10 [None,12] [[None,None][11,13]]
        assert_eq!(
            vec![Some(10), None, Some(12), None, None, Some(11), Some(13)],
            binary_tree.to_vec()
        );

        binary_tree.add_sort(9); // tree: 10 [9,12] [[None,None][11,13]]
        assert_eq!(
            vec![Some(10), Some(9), Some(12), None, None, Some(11), Some(13)],
            binary_tree.to_vec()
        );
    }
}
