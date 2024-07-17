pub struct Stepper{
    current: i32,
    step: i32,
    max: i32,
}

impl Iterator for Stepper {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.max{
            return None;
        }
        let res = self.current;
        self.current += self.step;
        Some(res) 
    }
}

#[cfg(test)]
mod tests{
    use super::Stepper;

    #[test]
    fn test_loop(){
        let stepper = Stepper{
            current: 0,
            step: 2,
            max: 100,
        };

        // we can use for loop iterate over our struct if we implement Iterator
        for i in stepper{
            println!("{}",i);
        }
    }
}