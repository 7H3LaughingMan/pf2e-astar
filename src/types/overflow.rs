use std::ops::{Add, Sub};

#[derive(Clone, Copy)]
#[derive(Debug)]
pub struct Overflow {
    pub value: usize,
    pub limit: usize,
}

impl Add<usize> for Overflow {
    type Output = Overflow;

    fn add(self, rhs: usize) -> Self::Output {
        Overflow { value: ((self.value as isize + rhs as isize) % self.limit as isize).unsigned_abs(), limit: self.limit }
    }
}

impl Sub<usize> for Overflow {
    type Output = Overflow;

    fn sub(self, rhs: usize) -> Self::Output {
        Overflow { value: ((self.value as isize - rhs as isize) % self.limit as isize).unsigned_abs(), limit: self.limit }
    }
}
