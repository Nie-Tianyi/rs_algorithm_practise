use std::hash::{Hash, Hasher};

pub struct MHash {
    prev: u8,
    n: u128,
}

impl Hasher for MHash {
    fn finish(&self) -> u64 {
        self.n as u64
    }

    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.n = (self.n + 11) * (*byte as u128 + 13)
                + ((byte ^ self.prev) as u128) % (u64::MAX as u128);
            self.prev = *byte;
        }
    }
}

pub fn hash<T: Hash>(seed: u64, value: T) -> u64 {
    let mut h = MHash { n: 0, prev: 0 };
    h.write_u64(seed);
    value.hash(&mut h);
    h.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let m = hash(55, "cat");
        let n = hash(55, "cat");
        println!("m hash is 0x{:0X}", m);
        assert_eq!(m, n);
        assert_ne!(hash(23, "cat"), m);
        assert_ne!(hash(55, "cate"), m);
    }
}
