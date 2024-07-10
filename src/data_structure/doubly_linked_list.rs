use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

/// a doubly linked list
pub struct DoublyLinkedList<T> {
    first: Option<Rc<RefCell<DbNode<T>>>>,
    last: Option<Weak<RefCell<DbNode<T>>>>,
}  

pub struct DbNode<T> {
    data: T,
    next: Option<Rc<RefCell<DbNode<T>>>>,
    prev: Option<Weak<RefCell<DbNode<T>>>>,
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
