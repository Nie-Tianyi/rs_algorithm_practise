use std::{
    cell::RefCell,
    fmt::Display,
    rc::{Rc, Weak},
};

/// a doubly linked list
pub struct DoublyLinkedList<T> {
    first: Option<Rc<RefCell<ListNode<T>>>>,
    last: Option<Weak<RefCell<ListNode<T>>>>,
}

#[derive(Default)]
pub struct ListNode<T> {
    data: T,
    next: Option<Rc<RefCell<ListNode<T>>>>,
    prev: Option<Weak<RefCell<ListNode<T>>>>,
}

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

impl<T:Default> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            first: None,
            last: None,
        }
    }

    /// add a new element at the front
    pub fn push(&mut self, data: T) {
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
    /// delete the first element, return it
    pub fn pop(&mut self) -> Option<T> {
        match self.first.take() {
            Some(first_node) => {
                let first_node = RefCell::take(&first_node);
                let next_node = first_node.next;
                match next_node {
                    Some(next_node) => {
                        next_node.borrow_mut().prev = None;
                        self.first = Some(next_node.clone());
                    },
                    None => {
                        self.first = None;
                        self.last = None;
                    },
                }
                let res = first_node.data;
                Some(res)
                
            }
            None => None,
        }
    }

    /// add a new element at the end of the list
    pub fn shift(&mut self, data: T) {
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
}

/// init a linked list, and push the items into the list
/// in a reverse order.
///
/// # Examples
/// ```rust
/// use rs_algorithm_practise::data_structure::doubly_linked_list::DoublyLinkedList;
/// use rs_algorithm_practise::doubly_linked_list;
///
/// let dll: DoublyLinkedList<i32> = doubly_linked_list![1,2,3];
/// println!("{}", dll); // DoublyLinkedList(None ⇋ 1 ⇋ 2 ⇋ 3 ⇋ None)
/// ```
#[macro_export]
macro_rules! doubly_linked_list {
    ($($elem:expr),*) => {
        {
            let mut temp_list = DoublyLinkedList::new();
            let mut vec = vec![$($elem),*];
            vec.reverse();
            for elem in vec {
                temp_list.push(elem);
            }
            temp_list
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doubly_linked_list() {
        let mut dll = DoublyLinkedList::new();
        dll.push(1);
        dll.push(2);
        dll.push(3);
        dll.shift(4);
        dll.shift(5);
        dll.shift(6);

        let a = dll.pop();

        println!("{}", dll);
        println!("{}", a.unwrap())
    }

    #[test]
    fn test_macro() {
        let dll: DoublyLinkedList<i32> = doubly_linked_list![1, 2, 3, 4, 5];
        println!("{}", dll)
    }
}
