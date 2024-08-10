use std::cmp::{max, Ordering};
use std::collections::VecDeque;
use std::fmt::Display;

/// a Self-Balancing Binary Search Tree
pub struct BinaryTree<T>(Option<Box<TreeNode<T>>>);

/// Tree node of the binary tree
pub struct TreeNode<T> {
    data: T,
    height: usize, // for balancing the tree
    left_node: BinaryTree<T>,
    right_node: BinaryTree<T>,
}

impl<T> TreeNode<T> {
    /// Rotates the current node to the left.
    ///
    /// This operation is used to maintain the balance of the binary tree.
    /// It takes the right child of the current node and makes it the new root of the subtree,
    /// with the current node becoming the left child of the new root.
    ///
    /// # Returns
    ///
    /// A `Box` containing the new root of the subtree after the rotation.
    pub fn rotate_left(mut self) -> Box<Self> {
        // take the right node of current node, if it is None, then it cannot rotate
        let mut res = match self.right_node.0.take() {
            None => return Box::new(self),
            Some(res) => res,
        };
        // change the left node of current node to right
        self.right_node = BinaryTree(res.left_node.0.take());
        self.right_node.set_height();
        // append current node to the right branch of the original right node
        res.left_node = BinaryTree(Some(Box::new(self)));
        res.left_node.set_height();
        // set height, return the original right node to be the root node
        res.height = 1 + max(res.right_node.height(), res.right_node.height());
        res
    }

    /// Rotates the current node to the right.
    ///
    /// This operation is used to maintain the balance of the binary tree.
    /// It takes the left child of the current node and makes it the new root of the subtree,
    /// with the current node becoming the right child of the new root.
    ///
    /// # Returns
    ///
    /// A `Box` containing the new root of the subtree after the rotation.
    pub fn rotate_right(mut self) -> Box<Self> {
        // take the left node of current node, if it is None, then it cannot rotate
        let mut res = match self.left_node.0.take() {
            None => return Box::new(self),
            Some(res) => res,
        };
        // change the right node of current node to left
        self.left_node = BinaryTree(res.right_node.0.take());
        self.left_node.set_height();
        // append current node to the left branch of the original left  node
        res.right_node = BinaryTree(Some(Box::new(self)));
        res.right_node.set_height();
        // set height, return the original right node to be the root node
        res.height = 1 + max(res.right_node.height(), res.right_node.height());
        res
    }
    /// return the balancing factor of a node,
    /// that is the height of the left node minus the height of right node
    pub fn balancing_factor(&self) -> isize {
        let left_height = self.left_node.height();
        let right_height = self.right_node.height();

        match left_height.cmp(&right_height) {
            Ordering::Less => {
                -((right_height - left_height) as isize)
            }
            Ordering::Equal => {
                0_isize
            }
            Ordering::Greater => {
                (left_height - right_height) as isize
            }
        }
    }
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
                        None => write!(f, "N{}", space_between)?,
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
    /// initiate an empty binary tree
    #[inline]
    pub fn new() -> Self {
        BinaryTree(None)
    }
    /// # Returns
    /// a boolean value that whether a Tree is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }
    /// rotate the root node of the binary tree one node left
    #[inline]
    pub fn rotate_left(&mut self) {
        self.0 = self.0.take().map(
            |node| node.rotate_left()
        )
    }
    /// rotate the root node of the binary tree one node right
    #[inline]
    pub fn rotate_right(&mut self) {
        self.0 = self.0.take().map(
            |node| node.rotate_right()
        )
    }

    /// calculate the depth of the tree **recursively**.
    ///
    /// this function should have the same result as `BinaryTree::height(&self)`
    ///
    /// # Returns
    /// the height of the root node
    pub fn depth(&self) -> usize {
        match self.0 {
            None => 0,
            Some(ref node) => 1 + max(node.left_node.depth(), node.right_node.depth()),
        }
    }
    /// get the height of the node
    ///
    /// if the node is None, then return 0 directly;
    /// if it is not None, return its height.
    ///
    /// # Returns
    /// the height of the root node, it is also the height of the whole tree
    #[inline]
    pub fn height(&self) -> usize {
        match self.0 {
            None => 0,
            Some(ref node) => node.height,
        }
    }
    /// set the node height when adding a new node
    #[inline]
    fn set_height(&mut self) {
        if let Some(ref mut node) = self.0 {
            node.height = 1 + max(node.left_node.height(), node.right_node.height());
        }
    }
}

impl<T: Clone> BinaryTree<T> {
    /// Traverse the binary tree in preorder **recursively**.
    ///
    /// This method performs a preorder traversal of the binary tree, visiting the root node first,
    /// then the left subtree, and finally the right subtree. The data of each visited node is
    /// pushed into the provided vector.
    ///
    /// # Arguments
    ///
    /// * `v` - A mutable reference to a vector where the data of the visited nodes will be stored.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rs_algorithm_practise::data_structure::binary_tree::BinaryTree;
    ///
    /// let mut tree = BinaryTree::new();
    /// tree.add_sort(10);
    /// tree.add_sort(5);
    /// tree.add_sort(15);
    ///
    /// let mut result = Vec::new();
    /// tree.preorder_traversal(&mut result);
    /// assert_eq!(result, vec![10, 5, 15]);
    /// ```
    pub fn preorder_traversal(&self, v: &mut Vec<T>) {
        if let Some(ref node) = self.0 {
            v.push(node.data.clone());
            node.left_node.preorder_traversal(v);
            node.right_node.preorder_traversal(v);
        }
    }
    /// Traverse the binary tree in inorder **recursively**.
    ///
    /// This method performs an inorder traversal of the binary tree, visiting the left subtree first,
    /// then the root node, and finally the right subtree. The data of each visited node is
    /// pushed into the provided vector.
    ///
    /// # Arguments
    ///
    /// * `v` - A mutable reference to a vector where the data of the visited nodes will be stored.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rs_algorithm_practise::data_structure::binary_tree::BinaryTree;
    ///
    /// let mut tree = BinaryTree::new();
    /// tree.add_sort(10);
    /// tree.add_sort(5);
    /// tree.add_sort(15);
    ///
    /// let mut result = Vec::new();
    /// tree.inorder_traversal(&mut result);
    /// assert_eq!(result, vec![5, 10, 15]);
    /// ```
    pub fn inorder_traversal(&self, v: &mut Vec<T>) {
        if let Some(ref node) = self.0 {
            node.left_node.inorder_traversal(v);
            v.push(node.data.clone());
            node.right_node.inorder_traversal(v);
        }
    }
    /// Traverse the binary tree in postorder **recursively**.
    ///
    /// This method performs a postorder traversal of the binary tree, visiting the left subtree first,
    /// then the right subtree, and finally the root node. The data of each visited node is
    /// pushed into the provided vector.
    ///
    /// # Arguments
    ///
    /// * `v` - A mutable reference to a vector where the data of the visited nodes will be stored.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rs_algorithm_practise::data_structure::binary_tree::BinaryTree;
    ///
    /// let mut tree = BinaryTree::new();
    /// tree.add_sort(10);
    /// tree.add_sort(5);
    /// tree.add_sort(15);
    ///
    /// let mut result = Vec::new();
    /// tree.postorder_traversal(&mut result);
    /// assert_eq!(result, vec![5, 15, 10]);
    /// ```
    pub fn postorder_traversal(&self, v: &mut Vec<T>) {
        if let Some(ref node) = self.0 {
            node.left_node.postorder_traversal(v);
            node.right_node.postorder_traversal(v);
            v.push(node.data.clone());
        }
    }
    /// Traverse the binary tree in breadth-first order.
    ///
    /// This method performs a breadth-first traversal of the binary tree, visiting each level
    /// of the tree from top to bottom and left to right. The data of each visited node is
    /// collected into a vector and returned.
    ///
    /// # Returns
    ///
    /// A `Vec<T>` containing the data of the visited nodes in breadth-first order.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rs_algorithm_practise::data_structure::binary_tree::BinaryTree;
    ///
    /// let mut tree = BinaryTree::new();
    /// tree.add_sort(10);
    /// tree.add_sort(5);
    /// tree.add_sort(15);
    ///
    /// let result = tree.breadth_first_traversal();
    /// assert_eq!(result, vec![10, 5, 15]);
    /// ```
    pub fn breadth_first_traversal(&self) -> Vec<T> {
        let mut res = Vec::new();
        let mut queue = VecDeque::new();

        if let Some(ref root_node) = self.0 {
            queue.push_front(root_node);
        }

        loop {
            if queue.is_empty() { break; }

            let node = queue.pop_back().unwrap();
            res.push(node.data.clone());

            if let Some(ref left_node) = node.left_node.0 {
                queue.push_front(left_node);
            }

            if let Some(ref right_node) = node.right_node.0 {
                queue.push_front(right_node);
            }
        }

        res
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
    /// binary_tree.add_sort(11); // 11 [10,12]
    /// assert_eq!(
    ///     vec![Some(11), Some(10), Some(12)],
    ///     binary_tree.to_vec()
    /// );
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
    /// Adds a new element to the binary tree while maintaining its balance.
    ///
    /// This method inserts a new element into the binary tree in a sorted manner.
    /// It ensures that the tree remains balanced by performing rotations if necessary.
    /// The balancing factor of each node is checked after insertion, and rotations
    /// are performed to maintain the balance of the tree.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to be added to the binary tree.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rs_algorithm_practise::data_structure::binary_tree::BinaryTree;
    ///
    /// let mut tree = BinaryTree::new();
    /// tree.add_sort(10);
    /// tree.add_sort(5);
    /// tree.add_sort(15);
    ///
    /// assert_eq!(tree.to_vec(), vec![Some(10), Some(5), Some(15)]);
    /// ```
    pub fn add_sort(&mut self, data: T) {
        let balancing_factor = match self.0 {
            Some(ref mut bd) => {
                if data < bd.data {
                    bd.left_node.add_sort(data);
                    bd.balancing_factor()
                } else {
                    bd.right_node.add_sort(data);
                    bd.balancing_factor()
                }
            }
            None => {
                self.0 = Some(Box::new(TreeNode {
                    data,
                    height: 0,
                    left_node: BinaryTree::new(),
                    right_node: BinaryTree::new(),
                }));
                0
            }
        };

        match balancing_factor {
            1.. => self.rotate_right(), // in the rotation, we have set height
            ..-1 => self.rotate_left(),
            -1..1 => self.set_height(),
        }
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

        binary_tree.add_sort(11); // tree: 11 [10,12]
        assert_eq!(binary_tree.depth(), 2);

        binary_tree.add_sort(13); // tree: 11 [10,12] [[None,None][None,13]]
        assert_eq!(binary_tree.depth(), 3);

        binary_tree.add_sort(9); // tree: 11 [10,12] [[9,None][None,13]]
        assert_eq!(binary_tree.depth(), 3);
    }

    #[test]
    fn test_to_vec() {
        let mut binary_tree = BinaryTree::new();

        binary_tree.add_sort(10); // tree: 10
        assert_eq!(vec![Some(10)], binary_tree.to_vec());

        binary_tree.add_sort(12); // tree: 10 [None,12]
        assert_eq!(vec![Some(10), None, Some(12)], binary_tree.to_vec());

        binary_tree.add_sort(11); // 11 [10,12]
        assert_eq!(
            vec![Some(11), Some(10), Some(12)],
            binary_tree.to_vec()
        );

        binary_tree.add_sort(13);
        assert_eq!(
            vec![Some(11), Some(10), Some(12), None, None, None, Some(13)],
            binary_tree.to_vec()
        );

        binary_tree.add_sort(9);
        assert_eq!(
            vec![Some(11), Some(9), Some(12), None, Some(10), None, Some(13)],
            binary_tree.to_vec()
        );
    }

    #[test]
    fn test_rotate() {
        let mut binary_tree = BinaryTree::new();
        binary_tree.add_sort(1);
        binary_tree.add_sort(2);
        binary_tree.add_sort(3);
        println!("{}", binary_tree);

        binary_tree.rotate_left();
        println!("{}", binary_tree);
    }

    #[test]
    fn test_add_sort() {
        let mut bt = BinaryTree::new();
        for i in 0..20 {
            bt.add_sort(i);
            println!("{}", bt);
        }
    }
}
