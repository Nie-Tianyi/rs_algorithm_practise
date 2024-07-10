use core::panic;
use std::fmt::Display;

#[derive(Debug, Clone)]
/// a single direction linked list,
/// it will be displayed as ` a -> b -> c -> ... -> None`
pub struct LinkedList<T: Display>(Option<(T, Box<LinkedList<T>>)>);

impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LinkedList(")?;
        let mut node = self;
        while let Some((ref data, ref child)) = node.0 {
            write!(f, "{} -> ", data)?;
            node = child;
        }
        write!(f, "None)")
    }
}

impl<T: Display> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList(None)
    }

    /// add an element at the head of the list
    ///
    /// # Arguments:
    /// `data: T` the data to be added into list
    pub fn push(&mut self, data: T) {
        let t = self.0.take();
        self.0 = Some((data, Box::new(LinkedList(t))));
    }

    /// delete the first element in the list and return
    /// the element.
    ///
    /// # Returns:
    /// `Option<T>`: return the deleted element,
    /// if the list is empty, return a `None``
    pub fn pop(&mut self) -> Option<T> {
        match self.0.take() {
            Some((data, child)) => {
                self.0 = child.0;
                return Some(data);
            }
            None => {
                return None;
            }
        }
    }

    /// add and element at the end of the list,
    /// this function need to iterate over the linked list.
    ///
    /// # Arguments:
    /// `data:T` data to be inserted
    pub fn shift(&mut self, data: T) {
        match self.0 {
            Some((_, ref mut child)) => child.shift(data),
            None => self.push(data),
        }
    }

    /// insert a element at a given index
    ///
    /// # Arguments
    /// `element: T`: the element to be inserted.
    ///
    /// `index: usize`: insert the element at this index.
    ///
    /// # Panics
    /// panic if the index is out of bound
    pub fn insert(&mut self, data: T, index: usize) {
        if index == 0 {
            self.push(data);
            return;
        }

        let mut node = self;

        for _ in 0..index - 1_usize {
            match node.0.as_mut() {
                Some((_, child)) => {
                    node = child;
                }
                None => {
                    panic!("index out of bound");
                }
            }
        }
        
        node.insert_internal(data);
    }

    // insert a node with value `data: T` after this node
    fn insert_internal(&mut self, data: T) {
        // if the list is None, panic
        if self.0.is_none() {
            panic!("index out of bound");
        }
        let (self_value,child) = self.0.take().unwrap();
        
        self.0 = Some((self_value, Box::new(LinkedList(Some((data,child))))));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list() {
        let mut ll = LinkedList::new();
        ll.push(12_usize);
        ll.shift(13_usize);
        ll.push(11_usize);
        println!("{}", ll);
        let v = ll.pop().unwrap();
        println!("popped value: {}, linked list: {}", v, ll);
        let v = ll.pop().unwrap();
        println!("popped value: {}, linked list: {}", v, ll);
        let v = ll.pop().unwrap();
        println!("popped value: {}, linked list: {}", v, ll);
        assert_eq!(None, ll.pop());
        assert_eq!(None, ll.pop());
    }
    #[test]
    fn test_insert(){
        let mut ll = LinkedList::new();
        ll.push(4);
        ll.push(3);
        ll.push(2);
        ll.push(1);
        
        ll.insert(0, 0);
        println!("{}",ll);
        ll.insert(5, 5);
        ll.insert(6, 7);
        println!("{}",ll);
    }

    #[test]
    fn test_insert_internal(){
        let mut ll = LinkedList::new();
        ll.push(4);
        ll.push(3);
        ll.push(2);
        ll.push(1);
        
        ll.insert_internal(0);
        println!("{}",ll);
    }
}
