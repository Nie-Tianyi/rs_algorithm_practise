pub struct MaxHeap<T> {
    data: Vec<T>,
}

impl<T: Ord> MaxHeap<T> {
    #[inline]
    pub fn new() -> MaxHeap<T> {
        Self { data: Vec::new() }
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }
    #[inline]
    pub fn peek(&self) -> Option<&T> {
        self.data.get(0)
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.is_empty()
    }
    #[inline]
    pub fn into_inner(self) -> Vec<T> {
        self.data
    }
}

impl<T> Default for MaxHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}
// alias for MinHeap
pub type PriorityQueue<T> = MinHeap<T>;

pub struct MinHeap<T> {
    data: Vec<T>,
}

impl<T: Ord> MinHeap<T> {
    #[inline]
    pub fn new() -> MinHeap<T> {
        Self { data: Vec::new() }
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }
    #[inline]
    pub fn peek(&self) -> Option<&T> {
        self.data.get(0)
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.is_empty()
    }
    #[inline]
    pub fn into_inner(self) -> Vec<T> {
        self.data
    }
}

impl<T> Default for MinHeap<T> {
    fn default() -> Self {
        Self { data: Vec::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let pq = PriorityQueue::new();
    }
}
