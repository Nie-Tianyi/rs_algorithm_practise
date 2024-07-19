use std::fmt::Display;

pub struct BinaryTree<T>(Option<Box<TreeNode<T>>>);

pub struct TreeNode<T> {
    data: T,
    left_node: BinaryTree<T>,
    right_node: BinaryTree<T>,
}

impl<T: Display> Display for BinaryTree<T> {
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

    pub fn traverse_preorder(&self) -> Vec<T> {
        vec![]
    }

    pub fn traverse_inorder(&self) -> Vec<T> {
        vec![]
    }

    pub fn traverse_postorder(&self) -> Vec<T> {
        vec![]
    }

    pub fn breadth_first_traverse(&self) -> Vec<T> {
        vec![]
    }
}

impl<T:Clone> BinaryTree<T>{
    pub fn to_vec(&self) -> Vec<T> {
        let mut vec = Vec::new();
        self.to_vec_recursive(&mut vec);
        vec
    }

    fn to_vec_recursive(&self, vec: &mut Vec<T>) {
        if let Some(ref node) = self.0 {
            vec.push(node.data.clone());
            node.left_node.to_vec_recursive(vec);
            node.right_node.to_vec_recursive(vec);
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
        let mut bt: BinaryTree<i32> = BinaryTree::new();
        bt.add_sort(10);
        bt.add_sort(8);
        bt.add_sort(12);
        bt.add_sort(7);
        bt.add_sort(9);
        bt.add_sort(11);
        bt.add_sort(13);

        println!("{:?}", bt.to_vec())
    }
}
