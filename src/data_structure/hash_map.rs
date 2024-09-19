use crate::supplementary::hash::hash;
use std::borrow::Borrow;
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

    // return an usize indicating whether a bucket is too full
    pub fn push(&mut self, key: K, value: V) -> usize {
        let index = hash(self.seed, &key) as usize % self.buckets.len();
        self.buckets[index].push((key, value));
        self.len += 1;
        self.buckets[index].len()
    }

    pub fn get<KB>(&self, key: &KB) -> Option<&V>
    where
        K: Borrow<KB>,
        KB: Hash + Eq + ?Sized,
    {
        let index = hash(self.seed, &key) as usize % self.buckets.len();
        for (ik, iv) in &self.buckets[index] {
            if ik.borrow() == key {
                return Some(iv);
            }
        }
        None
    }

    pub fn get_mut<KB>(&mut self, key: &KB) -> Option<&mut V>
    where
        K: Borrow<KB>,
        KB: Hash + Eq + ?Sized,
    {
        let index = hash(self.seed, &key) as usize % self.buckets.len();
        for (ik, iv) in &mut self.buckets[index] {
            if (ik as &K).borrow() == key {
                return Some(iv);
            }
        }
        None
    }

    pub fn bucket(&mut self, index: usize) -> Option<Vec<(K, V)>> {
        if index >= self.buckets.len() {
            return None;
        }
        let mut res = Vec::new();
        std::mem::swap(&mut res, &mut self.buckets[index]);
        self.len -= res.len();
        Some(res)
    }
}

impl<K: Hash + Eq, V> Default for BucketList<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut bl = BucketList::new();

        bl.push("port", 8080);
        assert_eq!(bl.get("port"), Some(&8080));

        *bl.get_mut("port").unwrap() += 1;
        assert_eq!(bl.get("port"), Some(&8081));
    }
}
