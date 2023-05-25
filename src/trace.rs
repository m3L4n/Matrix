use crate::Matrix;
use std::ops::{Add, Mul, Sub};
impl<K> Matrix<K>
where
    K: Add<Output = K>
        + Sub<Output = K>
        + Mul<Output = K>
        + Copy
        + std::fmt::Debug
        + std::fmt::Display,
{
    pub fn trace(&mut self) -> K {
        if self.size().0 != self.size().1 {
            panic!("need to be a square matrix");
        }
        let mut k_to_send: K = self.elements[0][0].clone() - self.elements[0][0].clone();
        for i in 0..self.elements.len() {
            k_to_send = k_to_send + self.elements[i][i].clone();
        }
        k_to_send
    }
}
