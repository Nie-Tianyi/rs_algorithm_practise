#[derive(Debug, Default, Clone)]
pub struct Max;
#[derive(Debug, Default, Clone)]
pub struct Min;

pub type MaxHeap<T> = Heap<T, Max>;
pub type PriorityQueue<T> = Heap<T, Min>;

#[derive(Debug, Default, Clone)]
pub struct Heap<T: PartialOrd, Marker = Min> {
    _marker: std::marker::PhantomData<Marker>,
    data: Vec<T>,
}

impl<T: PartialOrd, Marker> Heap<T, Marker> {
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
    #[inline]
    pub fn parent_index(index: usize) -> usize {
        if index == 0 {
            return 0;
        }
        (index - 1) / 2
    }
    #[inline]
    pub fn leftchild_index(index: usize) -> usize {
        2 * index + 1
    }
    #[inline]
    pub fn rightchild_index(index: usize) -> usize {
        2 * index + 2
    }
    #[inline]
    pub fn get_parent(&self, index: usize) -> Option<&T> {
        if index == 0 {
            return None;
        }
        self.data.get((index - 1) / 2)
    }
    #[inline]
    pub fn get_leftchild(&self, index: usize) -> Option<&T> {
        self.data.get(2 * index + 1)
    }
    #[inline]
    pub fn get_rightchild(&self, index: usize) -> Option<&T> {
        self.data.get(2 * index + 2)
    }
    #[inline]
    pub fn get_node(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }
    #[inline]
    pub fn has_parent(&self, index: usize) -> bool {
        index != 0
    }
    #[inline]
    pub fn has_leftchild(&self, index: usize) -> bool {
        2 * index < self.len() // same as `2 * index + 1 <= self.len()`
    }
    pub fn has_rightchild(&self, index: usize) -> bool {
        2 * index + 2 <= self.len()
    }
    #[inline]
    pub fn shift_upwards(&mut self, index: usize) {
        self.data.swap(index, (index - 1) / 2);
    }
    #[inline]
    pub fn shift_leftchild(&mut self, index: usize) {
        self.data.swap(index, 2 * index + 1);
    }
    #[inline]
    pub fn shift_rightchild(&mut self, index: usize) {
        self.data.swap(index, 2 * index + 2);
    }
}

impl<T: PartialOrd> MaxHeap<T> {
    #[inline]
    pub fn new() -> MaxHeap<T> {
        MaxHeap {
            _marker: Default::default(),
            data: Vec::new(),
        }
    }
}
// A minimum heap that every node on the top is smaller than its children
impl<T: PartialOrd> PriorityQueue<T> {
    #[inline]
    pub fn new() -> PriorityQueue<T> {
        PriorityQueue {
            _marker: Default::default(),
            data: Vec::new(),
        }
    }
    // place the data at back of the tree, then shift it upwards till its right position
    pub fn push(&mut self, data: T) {
        self.data.push(data);
        let mut index = self.len() - 1;
        while self.has_parent(index)
            && self.get_parent(index).unwrap() > self.get_node(index).unwrap()
        {
            self.shift_upwards(index);
            index = Self::parent_index(index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new() {
        let pq = PriorityQueue::<i32>::new();
        println!("{}", pq.len());
    }

    #[test]
    fn test_push() {
        let mut pq = PriorityQueue::<i32>::new();
        pq.push(3);
        pq.push(2);
        pq.push(1);
        assert_eq!(vec![1, 3, 2], pq.into_inner());
    }
}
