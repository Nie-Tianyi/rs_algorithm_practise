#[derive(Default, Clone)]
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
        self.data.first()
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    #[inline]
    pub fn into_inner(self) -> Vec<T> {
        self.data
    }
}
// alias for MinHeap
pub type PriorityQueue<T> = MinHeap<T>;

#[derive(Clone, Default)]
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
        self.data.first()
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    #[inline]
    pub fn into_inner(self) -> Vec<T> {
        self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let _pq = PriorityQueue::<i32>::new();
    }
}
