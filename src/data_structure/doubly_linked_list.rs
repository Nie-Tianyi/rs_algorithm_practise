use std::{
    cell::RefCell,
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

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList {
            first: None,
            last: None,
        }
    }

    pub fn push(&mut self,data: T) {
        
    }
}
