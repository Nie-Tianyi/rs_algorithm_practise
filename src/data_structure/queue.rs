use std::fmt::{Display, Formatter};
use super::doubly_linked_list::DoublyLinkedList;

/// `Queue` is a generic queue structure that utilizes a doubly linked list as its underlying data structure.
/// It provides basic queue operations such as enqueue (to add an element to the end of the queue),
/// dequeue (to remove and return an element from the beginning of the queue), and checking if the queue is empty.
///
/// # Examples
///
/// ```
/// use rs_algorithm_practise::data_structure::queue::Queue;
///
/// let mut queue = Queue::new();
/// assert!(queue.is_empty());
///
/// queue.enqueue(1);
/// queue.enqueue(2);
/// assert!(!queue.is_empty());
///
/// assert_eq!(queue.dequeue(), Some(1));
/// assert_eq!(queue.dequeue(), Some(2));
/// assert!(queue.is_empty());
/// ```
#[derive(Debug)]
pub struct Queue<T> {
    data: DoublyLinkedList<T>,
}

impl<T: Display> Display for Queue<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl<T> Queue<T> {
    /// Creates a new, empty `Queue`.
    #[inline]
    pub fn new() -> Self {
        Queue {
            data: DoublyLinkedList::new(),
        }
    }

    /// Checks if the queue is empty.
    ///
    /// # Returns
    ///
    /// `true` if the queue is empty, `false` otherwise.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Adds an element to the end of the queue.
    ///
    /// # Parameters
    ///
    /// * `data` - The data to add to the queue.
    #[inline]
    pub fn enqueue(&mut self, data: T) {
        self.data.push_back(data)
    }

    /// Removes and returns an element from the beginning of the queue.
    ///
    /// # Returns
    ///
    /// `Some(T)` if the queue is not empty, where `T` is the type of the queue's elements;
    /// `None` if the queue is empty.
    #[inline]
    pub fn dequeue(&mut self) -> Option<T> {
        self.data.pop_front()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut q = Queue::new();
        q.enqueue(12);
        q.enqueue(13);

        assert_eq!(q.dequeue(),Some(12));
        assert_eq!(q.dequeue(),Some(13));
        assert_eq!(q.dequeue(),None);
        assert!(q.is_empty());
    }
}
