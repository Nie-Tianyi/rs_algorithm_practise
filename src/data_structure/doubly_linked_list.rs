use std::{
    cell::RefCell,
    fmt::Display,
    rc::{Rc, Weak},
};

/// a doubly linked list
#[derive(Default, Debug, Clone)]
pub struct DoublyLinkedList<T> {
    first: Option<Rc<RefCell<ListNode<T>>>>,
    last: Option<Weak<RefCell<ListNode<T>>>>,
}
/// list node of doubly linked list
#[derive(Default, Debug, Clone)]
pub struct ListNode<T> {
    data: T,
    next: Option<Rc<RefCell<ListNode<T>>>>,
    prev: Option<Weak<RefCell<ListNode<T>>>>,
}

impl<T> DoublyLinkedList<T> {
    /// Creates a new empty doubly linked list.
    ///
    /// # Returns
    ///
    /// - `DoublyLinkedList<T>` - An empty doubly linked list.
    #[inline]
    pub fn new() -> Self {
        DoublyLinkedList {
            first: None,
            last: None,
        }
    }
    /// Checks whether the doubly linked list is empty.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the list is empty, `false` otherwise.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.first.is_none() && self.last.is_none()
    }

    /// Adds a new element at the front of the list.
    ///
    /// # Arguments
    ///
    /// - `data: T` - The data to be pushed into the list.
    pub fn push_front(&mut self, data: T) {
        match self.first.take() {
            Some(node) => {
                let new_front = Rc::new(RefCell::new(ListNode {
                    data,
                    next: Some(node.clone()),
                    prev: None,
                }));
                let mut m = node.borrow_mut();
                m.prev = Some(Rc::downgrade(&new_front));
                self.first = Some(new_front);
            }
            None => {
                let new_node = Rc::new(RefCell::new(ListNode {
                    data,
                    next: None,
                    prev: None,
                }));
                self.last = Some(Rc::downgrade(&new_node));
                self.first = Some(new_node);
            }
        }
    }

    /// Removes the first element from the list and returns it.
    ///
    /// This operation takes constant time, `O(1)`, because it only involves updating
    /// a few pointers. If the list is empty, it returns `None`.
    ///
    /// # Returns
    ///
    /// - `Some(T)` - The value of the removed element if the list was not empty.
    /// - `None` - If the list was empty.
    ///
    /// # Panics
    ///
    /// This function will panic if the internal `Rc` has more than one strong reference.
    /// This scenario should not occur in typical usage because the list itself should
    /// be the only owner of its nodes.
    ///
    /// # Examples
    ///
    /// ```
    /// use rs_algorithm_practise::data_structure::doubly_linked_list::DoublyLinkedList;
    /// let mut list = DoublyLinkedList::new();
    /// list.push_back(1);
    /// list.push_back(2);
    /// assert_eq!(list.pop_front(), Some(1));
    /// assert_eq!(list.pop_front(), Some(2));
    /// assert_eq!(list.pop_front(), None);
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        self.first.take().map(|first_node_rc| {
            let first_node = Rc::try_unwrap(first_node_rc)
                .ok()
                .expect("DoublyLinkedList::pop_front(): Rc has more than one strong reference")
                .into_inner();
            let next_node = first_node.next;
            match next_node {
                Some(next_node) => {
                    next_node.borrow_mut().prev = None;
                    self.first = Some(next_node);
                }
                None => {
                    self.first = None;
                    self.last = None;
                }
            }
            first_node.data
        })
    }

    /// Adds a new element at the end of the list.
    ///
    /// # Arguments
    ///
    /// - `data: T` - The data to be pushed into the list.
    pub fn push_back(&mut self, data: T) {
        match self.last.take() {
            Some(node) => {
                let new_back = Rc::new(RefCell::new(ListNode {
                    data,
                    next: None,
                    prev: Some(node.clone()),
                }));
                let st = Weak::upgrade(&node).unwrap();
                let mut m = st.borrow_mut();
                self.last = Some(Rc::downgrade(&new_back));
                m.next = Some(new_back);
            }
            None => {
                let new_node = Rc::new(RefCell::new(ListNode {
                    data,
                    next: None,
                    prev: None,
                }));
                self.last = Some(Rc::downgrade(&new_node));
                self.first = Some(new_node);
            }
        }
    }

    /// Remove the last element of the list and returns it
    ///
    /// This operation takes constant time, `O(1)`, because it only involves updating
    /// a few pointers. If the list is empty, it returns `None`.
    ///
    /// # Returns
    ///
    /// - `Some(T)` - The value of the removed element if the list was not empty.
    /// - `None` - If the list was empty.
    ///
    /// # Panics
    ///
    /// This function will panic if the internal `Rc` has more than one strong reference.
    /// This scenario should not occur in typical usage because the list itself should
    /// be the only owner of its nodes.
    ///
    /// # Examples
    /// ```rust
    /// use rs_algorithm_practise::data_structure::doubly_linked_list::DoublyLinkedList;
    /// use rs_algorithm_practise::doubly_linked_list;
    ///
    /// let mut dll = doubly_linked_list![1,2];
    /// assert_eq!(dll.pop_back(), Some(2));
    /// assert_eq!(dll.pop_back(), Some(1));
    /// assert_eq!(dll.pop_back(), None);
    /// assert_eq!(dll, doubly_linked_list![]);
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        self.last.take().map(|last_node_weak| {
            let last_node_rc = last_node_weak.upgrade().unwrap();
            let last_node = last_node_rc.borrow();
            let prev_node = last_node.prev.clone();
            match prev_node {
                Some(prev_node_weak) => {
                    let prev_node_rc = prev_node_weak.upgrade().unwrap();
                    let mut prev_node = prev_node_rc.borrow_mut();
                    prev_node.next = None;
                    self.last = Some(prev_node_weak);
                }
                None => {
                    self.last = None;
                    self.first = None;
                }
            }
            drop(last_node);
            let last_node = Rc::try_unwrap(last_node_rc)
                .ok()
                .expect("DoublyLinkedList::pop_back(): Rc has more than one strong reference")
                .into_inner();
            last_node.data
        })
    }
}

impl<T: PartialEq> PartialEq for DoublyLinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        // init two pointers
        let mut a = self.first.clone();
        let mut b = other.first.clone();
        // compare the elements in two lists one by one
        loop {
            match (a, b) {
                (None, None) => return true,     // both None, it is equal
                (None, Some(_)) => return false, // one None one Some, not equal
                (Some(_), None) => return false,
                (Some(a_rc), Some(b_rc)) => {
                    // both Some, compare the value inside Option
                    let a_ref = a_rc.borrow();
                    let b_ref = b_rc.borrow();

                    if a_ref.data != b_ref.data {
                        return false;
                    }
                    // move both pointers to next nodes
                    a = a_ref.next.clone();
                    b = b_ref.next.clone();
                }
            }
        }
    }
}

/// Display the linked list
impl<T: Display> Display for DoublyLinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DoublyLinkedList(None ⇌")?;
        let mut node = self.first.clone();
        while let Some(r) = node {
            write!(f, " {} ⇌", r.borrow().data)?;
            node = r.borrow().next.clone();
        }
        write!(f, " None)")
    }
}

/// Initializes a linked list and pushes elements into the list
/// in **reverse order**.
///
/// # Examples
///
/// ```rust
/// use rs_algorithm_practise::data_structure::doubly_linked_list::DoublyLinkedList;
/// use rs_algorithm_practise::doubly_linked_list;
///
/// let dll: DoublyLinkedList<i32> = doubly_linked_list![1,2,3];
/// println!("{}", dll); // DoublyLinkedList(None ⇌ 1 ⇌ 2 ⇌ 3 ⇌ None)
/// ```
#[macro_export]
macro_rules! doubly_linked_list {
    ($($elem:expr),*) => {
        {
            let mut temp_list = DoublyLinkedList::new();
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
    fn test_eq() {
        let empty_list: DoublyLinkedList<i32> = doubly_linked_list![];
        let empty_list_2: DoublyLinkedList<i32> = doubly_linked_list![];
        assert_eq!(empty_list, empty_list_2);
        assert_eq!(doubly_linked_list![1, 2], doubly_linked_list![1, 2]);
        assert_ne!(doubly_linked_list![], doubly_linked_list![1]);
        assert_ne!(doubly_linked_list![1, 2], doubly_linked_list![1]);
    }

    #[test]
    fn test_doubly_linked_list() {
        let mut dll = DoublyLinkedList::new();
        dll.push_front(1);
        dll.push_front(2);
        dll.push_front(3);
        dll.push_back(4);
        dll.push_back(5);
        dll.push_back(6);

        assert_eq!(doubly_linked_list![3, 2, 1, 4, 5, 6], dll)
    }

    #[test]
    fn test_pop() {
        let mut dll: DoublyLinkedList<i32> = doubly_linked_list![1, 2, 3, 4];
        let a = dll.pop_front();
        assert_eq!(a, Some(1));
        let b = dll.pop_back();
        assert_eq!(b, Some(4));
    }

    #[test]
    fn test_pop_back() {
        let mut dll = doubly_linked_list![1, 2, 3, 4];
        assert_eq!(dll.pop_back(), Some(4));
        assert_eq!(dll.pop_back(), Some(3));
        assert_eq!(dll.pop_back(), Some(2));
        assert_eq!(dll.pop_back(), Some(1));
        assert_eq!(dll.pop_back(), None);
        assert_eq!(dll, doubly_linked_list![]);
    }

    #[test]
    fn test_pop_empty() {
        let mut dll: DoublyLinkedList<i32> = DoublyLinkedList::new();
        let a = dll.pop_front();
        assert_eq!(a, None);
        let b = dll.pop_back();
        assert_eq!(b, None);
    }
}
