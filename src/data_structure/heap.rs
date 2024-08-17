pub struct Heap<T>{
    data: Vec<T>,
}

impl<T> Heap<T>{
    #[inline]
    pub fn new() -> Self<T> {
        Self {
            data: Vec::new(),
        }
    }
}