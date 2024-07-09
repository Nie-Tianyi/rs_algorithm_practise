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

    /// add and element at the head of the list
    pub fn push(&mut self, data: T) {
        let t = self.0.take();
        self.0 = Some((data, Box::new(LinkedList(t))));
    }

    /// delete the first element in the list and return
    /// the element.
    pub fn pop(&mut self) -> Option<T> {
        match self.0.take() {
            Some((data, child)) => {
                self.0 = child.0;
                return Some(data);
            }
            None => return None,
        }
    }

    /// add and element at the end of the list
    pub fn shift(&mut self, data: T) {
        match self.0 {
            Some((_, ref mut child)) => child.shift(data),
            None => self.push(data),
        }
    }

    /// delete the element at the end
    /// todo!()
    pub fn unshift(&mut self) -> Option<T> {
        match self.0 {
            Some((_, ref mut child)) => child.unshift(),
            None => self.pop(),
        }
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
    
    }
}
