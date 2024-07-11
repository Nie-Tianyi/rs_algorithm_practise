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

pub struct ListNode<T> {
    data: T,
    next: Option<Rc<RefCell<ListNode<T>>>>,
    prev: Option<Weak<RefCell<ListNode<T>>>>,
}

impl<T: Display> Display for DoublyLinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DoublyLinkedList(None ⇋")?;
        let mut node = self.first.clone();
        while let Some(r) = node {
            write!(f, " {} ⇋", r.borrow().data)?;
            node = r.borrow().next.clone();
        }
        write!(f, " None)")
    }
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            first: None,
            last: None,
        }
    }

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
}

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
mod tests{
    use super::*;

    #[test]
    fn test_doubly_linked_list(){
        let mut dll = DoublyLinkedList::new();
        dll.push(1);
        dll.push(2);
        dll.push(3);

        println!("{}", dll);
    }

    #[test]
    fn test_macro() {
        let dll = doubly_linked_list![1,2,3];
        println!("{}",dll)
    }
}
