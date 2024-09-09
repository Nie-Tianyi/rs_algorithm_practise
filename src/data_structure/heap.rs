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

impl<T: PartialOrd> PriorityQueue<T> {
    #[inline]
    pub fn new() -> PriorityQueue<T> {
        PriorityQueue {
            _marker: Default::default(),
            data: Vec::new(),
        }
    }

    pub fn push(&mut self, data: T) {
        self.data.push(data);
        let mut i = self.data.len() - 1;
        if i == 0 {
            return;
        }
        let mut p = parent_index(i);
        while self.data.get(p).unwrap() > self.data.get(i).unwrap() {
            self.data.swap(p, i);
            i = p;
            if i == 0 {
                return;
            }
            p = parent_index(i);
        }
    }
}

// get the parent node index, panics if n = 0
#[inline]
fn parent_index(n: usize) -> usize {
    (n - 1) / 2
}
// get the left child node index
#[inline]
fn lc_index(n: usize) -> usize {
    2 * n + 1
}
// get the right child node index
#[inline]
fn rc_index(n: usize) -> usize {
    2 * n + 2
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
    fn test_parent_index() {
        for i in 1..=100 {
            println!("{}", parent_index(i));
        }
    }

    #[test]
    fn test_push() {
        let mut pq = MaxHeap::<i32>::new();
        pq.push(1);
        pq.push(2);
        pq.push(3);
        println!("{:?}", pq);
    }
}
