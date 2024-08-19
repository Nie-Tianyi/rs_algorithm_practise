use core::panic;
use std::fmt::Display;

/// a single linked list, T must implement `Display` trait.
/// this linked list will be displayed as ` a -> b -> c -> ... -> None`
#[derive(Default, Debug, Clone, PartialEq)]
pub struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>);

impl<T> LinkedList<T> {
    #[inline]
    pub fn new() -> Self {
        LinkedList(None)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    /// add an element at the head of the list
    ///
    /// # Arguments:
    /// `data: T` the data to be added into list
    #[inline]
    pub fn push_front(&mut self, data: T) {
        let t = self.0.take();
        self.0 = Some((data, Box::new(LinkedList(t))));
    }

    /// peak the first node's value
    ///
    /// # Returns
    /// `Option<&T>` return an option, if the list is None, return None.
    ///
    /// If the list is not None, return the reference of first value.
    #[inline]
    pub fn peak(&self) -> Option<&T> {
        match self.0 {
            Some((ref value, _)) => Some(value),
            None => None,
        }
    }

    /// delete the first element in the list and return
    /// the element.
    ///
    /// # Returns:
    /// `Option<T>`: return the deleted element,
    /// if the list is empty, return a `None`
    #[inline]
    pub fn pop_front(&mut self) -> Option<T> {
        match self.0.take() {
            Some((data, child)) => {
                self.0 = child.0;
                Some(data)
            }
            None => None,
        }
    }

    /// add and element at the end of the list,
    /// this function need to iterate over the linked list.
    ///
    /// # Arguments:
    /// `data:T` data to be inserted
    #[inline]
    pub fn push_back(&mut self, data: T) {
        match self.0 {
            Some((_, ref mut child)) => child.push_back(data),
            None => self.push_front(data),
        }
    }

    /// insert an element at a given index
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
            self.push_front(data);
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

        node.insert_next(data);
    }

    // insert a node with value `data: T` after this node
    fn insert_next(&mut self, data: T) {
        // if the list is None, panic
        if self.0.is_none() {
            panic!("index out of bound");
        }
        let (self_value, child) = self.0.take().unwrap();

        self.0 = Some((self_value, Box::new(LinkedList(Some((data, child))))));
    }
}

impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LinkedList(")?;
        let mut node = self;
        while let Some((ref data, ref child)) = node.0 {
            write!(f, "{} → ", data)?;
            node = child;
        }
        write!(f, "None)")
    }
}

/// init a linked list, and push the items into the list
/// in a reverse order.
///
/// # Examples
/// ```rust
/// use rs_algorithm_practise::data_structure::linked_list::LinkedList;
/// use rs_algorithm_practise::linked_list;
///
/// let ll: LinkedList<i32> = linked_list![1,2,3];
/// println!("{}", ll); //LinkedList(1 → 2 → 3 → None)
/// ```
#[macro_export]
macro_rules! linked_list {
    ($($elem:expr),*) => {
        {
            let mut temp_list = LinkedList::new();
            let mut vec = vec![$($elem),*];
            vec.reverse();
            for elem in vec {
                temp_list.push_front(elem);
            }
            temp_list
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list() {
        let mut ll = LinkedList::new();
        ll.push_front(12_usize);
        ll.push_back(13_usize);
        ll.push_front(11_usize);
        println!("{}", ll);
        let v = ll.pop_front().unwrap();
        println!("popped value: {}, linked list: {}", v, ll);
        let v = ll.pop_front().unwrap();
        println!("popped value: {}, linked list: {}", v, ll);
        let v = ll.pop_front().unwrap();
        println!("popped value: {}, linked list: {}", v, ll);
        assert_eq!(None, ll.pop_front());
        assert_eq!(None, ll.pop_front());
    }
    #[test]
    fn test_insert() {
        let mut ll = LinkedList::new();
        ll.push_front(4);
        ll.push_front(3);
        ll.push_front(2);
        ll.push_front(1);

        ll.insert(0, 0);
        println!("{}", ll);
        ll.insert(5, 5);
        println!("{}", ll);
    }

    #[test]
    fn test_insert_internal() {
        let mut ll = LinkedList::new();
        ll.push_front(4);
        ll.push_front(3);
        ll.push_front(2);
        ll.push_front(1);

        ll.insert_next(0);
        println!("{}", ll);
    }

    #[test]
    fn test_macro() {
        let ll: LinkedList<i32> = linked_list![1, 2, 3];
        println!("{}", ll);
    }
}
