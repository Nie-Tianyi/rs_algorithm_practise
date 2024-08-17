pub struct Heap<T>{
    data: Vec<T>,
}

impl<T> Heap<T>{
    #[inline]
    pub fn new() -> Heap<T> {
        Self {
            data: Vec::new(),
        }
    }
}