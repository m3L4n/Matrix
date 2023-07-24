use crate::Matrix;
use std::ops::{Add, Mul, Sub};
impl<K> Matrix<K>
where
    K: Add<Output = K>
        + Sub<Output = K>
        + Mul<Output = K>
        + Copy
        + std::fmt::Debug
        + std::fmt::Display
        + Default,
{
    pub fn transpose(&mut self) -> Matrix<K> {
        if self.elements.len() <= 0 {
            panic!("error param");
        }
        self.get_matrix_colum()
    }
}
