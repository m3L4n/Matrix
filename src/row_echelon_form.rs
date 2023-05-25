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
    // fn found_pivot_and_transform_column( )
    pub fn row_echelon(&mut self) -> Matrix<K> {
        let mut pivot: usize = 0;
        let mut newMatrix = self.clone();
        for (index, colum) in self.elements.iter_mut().enumerate() {}

        // il faut qu il y ai un 1 en first position de la premiere colone
        // -> sois c un non null o debut et c ok
        //sois tu vas echanger avec une ligne si ya un en first ou

        self.clone()
    }
}
