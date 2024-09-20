use crate::supplementary::hash::hash;
use std::borrow::Borrow;
use std::hash::Hash;

/// This is an internal data structure of a HashMap
#[derive(Debug, Clone, Default)]
struct BucketList<K, V> {
    seed: u64,
    len: usize,
    buckets: Vec<Vec<(K, V)>>,
}

impl<K: Hash + Eq, V> BucketList<K, V> {
    #[inline]
    fn new() -> Self {
        BucketList {
            seed: rand::random(),
            len: 0,
            buckets: vec![Vec::new()],
        }
    }

    // return an usize indicating whether a bucket is too full
    fn push(&mut self, key: K, value: V) -> usize {
        let index = hash(self.seed, &key) as usize % self.buckets.len();
        self.buckets[index].push((key, value));
        self.len += 1;
        self.buckets[index].len()
    }

    fn get<KB>(&self, key: &KB) -> Option<&V>
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

    fn get_mut<KB>(&mut self, key: &KB) -> Option<&mut V>
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

    fn bucket(&mut self, index: usize) -> Option<Vec<(K, V)>> {
        if index >= self.buckets.len() {
            return None;
        }
        let mut res = Vec::new();
        std::mem::swap(&mut res, &mut self.buckets[index]);
        self.len -= res.len();
        Some(res)
    }

    /// grow the buckets to a given size.
    #[inline]
    fn set_buckets(&mut self, n: usize) {
        for _ in self.buckets.len()..n {
            self.buckets.push(Vec::new());
        }
    }
}

#[cfg(test)]
mod bucketlist_tests {
    use super::*;

    #[test]
    fn test_bucketlist() {
        let mut bl = BucketList::new();

        bl.push("port", 8080);
        assert_eq!(bl.get("port"), Some(&8080));

        *bl.get_mut("port").unwrap() += 1;
        assert_eq!(bl.get("port"), Some(&8081));
    }
}

const BUCKET_SIZE: usize = 8;

#[derive(Debug, Default, Clone)]
pub struct HashMap<K, V> {
    n_moved: usize,
    main: BucketList<K, V>,
    grow: BucketList<K, V>,
}

impl<K: Hash + Eq, V> HashMap<K, V> {
    #[inline]
    pub fn new() -> Self {
        HashMap {
            n_moved: 0,
            main: BucketList::new(),
            grow: BucketList::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        // check if the key already exists, mutate original value
        if let Some(iv) = self.main.get_mut(&key) {
            *iv = value;
            return;
        }
        if let Some(iv) = self.grow.get_mut(&key) {
            *iv = value;
            return;
        }

        if self.n_moved > 0 {
            self.grow.push(key, value);
            self.move_buckets();
            return;
        }
        if self.main.push(key, value) > BUCKET_SIZE / 2 {
            // grow buckets
            self.move_buckets();
        }
    }

    pub fn move_buckets(&mut self) {
        if self.n_moved == 0 {
            self.grow.set_buckets(self.main.buckets.len() * 2);
        }
        if let Some(b) = self.main.bucket(self.n_moved) {
            for (k, v) in b {
                self.grow.push(k, v);
            }
            self.n_moved += 1;
            return;
        }
        std::mem::swap(&mut self.main, &mut self.grow);
        self.n_moved = 0;
    }

    #[inline]
    pub fn get<KR>(&self, kr: &KR) -> Option<&V>
    where
        K: Borrow<KR>,
        KR: Hash + Eq + ?Sized,
    {
        self.main.get(kr).or_else(|| self.grow.get(kr))
    }

    #[inline]
    pub fn get_mut<KR>(&mut self, kr: &KR) -> Option<&mut V>
    where
        K: Borrow<KR>,
        KR: Hash + Eq + ?Sized,
    {
        self.main.get_mut(kr).or_else(|| self.grow.get_mut(kr))
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.main.len + self.grow.len
    }
}

#[cfg(test)]
mod hashmap_test {
    use super::*;

    #[test]
    fn test_hashmap() {
        let mut hm = HashMap::new();
        hm.insert("key", "value");
        assert_eq!(hm.get("key"), Some(&"value"));
    }
}
