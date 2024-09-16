pub struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    #[inline]
    pub fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }
    #[inline]
    pub fn push(&mut self, item: T) {
        self.elements.push(item);
    }
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }
    #[inline]
    pub fn peek(&self) -> Option<&T> {
        self.elements.last()
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
    #[inline]
    pub fn size(&self) -> usize {
        self.elements.len()
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn test_stack() {
        let mut stack = Stack::new();
        assert!(stack.is_empty());

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.size(), 3);
        assert_eq!(stack.peek(), Some(&3));

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert!(stack.is_empty());
    }
}