use rand::random;
use std::cell::RefCell;
use std::rc::Rc;

type Rcc<T> = Rc<RefCell<T>>;

#[inline]
pub fn rcc<T>(t: T) -> Rcc<T> {
    Rc::new(RefCell::new(t))
}
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SkipNode<T: PartialOrd> {
    right: Option<Rcc<SkipNode<T>>>,
    down: Option<Rcc<SkipNode<T>>>,
    data: Rcc<T>,
}

impl<T: PartialOrd> SkipNode<T> {
    pub fn new(t: T) -> Self {
        SkipNode {
            right: None,
            down: None,
            data: rcc(t),
        }
    }

    pub fn insert(&mut self, data: T) -> Option<Rcc<SkipNode<T>>> {
        if let Some(ref mut right) = self.right {
            if data > *right.borrow().data.borrow() {
                return right.borrow_mut().insert(data);
            }
        }

        if let Some(ref down) = self.down {
            return match down.borrow_mut().insert(data) {
                None => None,
                Some(child) => match random::<bool>() {
                    true => {
                        let data = child.borrow().data.clone();
                        let nn = SkipNode {
                            right: self.right.take(),
                            down: Some(child),
                            data,
                        };
                        let res = rcc(nn);
                        self.right = Some(res.clone());
                        Some(res)
                    }
                    false => None,
                },
            };
        }

        let mut nn = SkipNode::new(data);
        nn.right = self.right.take();
        let res = rcc(nn);
        self.right = Some(res.clone());
        Some(res)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        assert_eq!(1 + 1, 2);
    }
}
