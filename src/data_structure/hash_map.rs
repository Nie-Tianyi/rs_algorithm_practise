use crate::supplementary::hash::hash;
use std::hash::Hash;

#[derive(Debug)]
pub struct BucketList<K, V> {
    seed: u64,
    len: usize,
    buckets: Vec<Vec<(K, V)>>,
}

impl<K: Hash + Eq, V> BucketList<K, V> {
    pub fn new() -> Self {
        BucketList {
            seed: rand::random(),
            len: 0,
            buckets: vec![Vec::new()],
        }
    }

    // return a usize indicating whether a buckets is too full
    pub fn push(&mut self, key: K, value: V) -> usize {
        let index = hash(self.seed, &key) as usize % self.buckets.len();
        self.buckets[index].push((key, value));
        self.len += 1;
        self.buckets[index].len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut bl = BucketList::new();
        bl.push("abc", 123);
    }
}
