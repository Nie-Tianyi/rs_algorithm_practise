pub struct Max;
pub struct Min;

pub type MaxHeap<T> = Heap<T, Max>;
pub type PriorityQueue<T> = Heap<T, Min>;

#[derive(Debug, Default, Clone)]
pub struct Heap<T: PartialOrd, Marker = Min> {
    marker: std::marker::PhantomData<Marker>,
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
}

impl<T: PartialOrd> MaxHeap<T> {
    #[inline]
    pub fn new() -> MaxHeap<T> {
        MaxHeap {
            marker: Default::default(),
            data: Vec::new(),
        }
    }
}

impl<T: PartialOrd> PriorityQueue<T> {
    #[inline]
    pub fn new() -> PriorityQueue<T> {
        PriorityQueue {
            marker: Default::default(),
            data: Vec::new(),
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
}
