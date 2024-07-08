/// a pseudo random integer generator.
///
/// just for demo purpose.
pub struct RandomGen {
    cur: usize,
    mul: usize,
    inc: usize,
    modulo: usize,
}

impl RandomGen {
    pub fn new(seed: usize) -> Self {
        RandomGen {
            cur: seed,
            mul: 56394237_usize,
            inc: 346423496_usize,
            modulo: 25254463563_usize,
        }
    }

    pub fn rand_usize(&mut self, max: usize) -> usize {
        self.cur = (self.cur * self.mul + self.inc) % self.modulo;
        self.cur % max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rand() {
        let mut r = RandomGen::new(123);
        for _ in 0..100 {
            let a = r.rand_usize(100_usize);
            println!("{}", a)
        }
    }
}
