use std::fmt;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;
pub struct Matrix<K> {
    pub elements: Vec<Vec<K>>,
}

impl<K: fmt::Display> fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Matrix : \n[");
        for (index, row) in self.elements.iter().enumerate() {
            let row_str = row
                .iter()
                .map(|elem| elem.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            if (self.elements.len() - 1) == index {
                write!(f, "[{}]", row_str)?;
            } else {
                write!(f, "[{}]\n", row_str)?;
            }
        }
        write!(f, "]");
        Ok(())
    }
}
impl<
        K: Copy + std::ops::Add<Output = K> + std::ops::Sub<Output = K> + std::ops::Mul<Output = K>,
    > Matrix<K>
{
    pub fn from(data: Vec<Vec<K>>) -> Matrix<K> {
        let vec = Matrix {
            // permet de demander au compilateur de savoir le type concret
            elements: Vec::from(data),
        };
        vec
    }
    pub fn add(&mut self, v: &Matrix<K>) {
        if !self.is_same_size(&v) {
            panic!("cant add to matrix bc its not the right size")
        }
        for (colum1, column2) in self.elements.iter_mut().zip(v.elements.iter()) {
            for (row1, row2) in colum1.iter_mut().zip(column2.iter()) {
                *row1 = row1.clone() + row2.clone();
            }
        }
    }
    pub fn adde(&mut self, v: &Matrix<K>) -> Matrix<K> {
        if !self.is_same_size(&v) {
            panic!("cant add to matrix bc its not the right size")
        }
        let mut newMatrix = self.clone();
        for (colum1, column2) in newMatrix.elements.iter_mut().zip(v.elements.iter()) {
            for (row1, row2) in colum1.iter_mut().zip(column2.iter()) {
                *row1 = row1.clone() + row2.clone();
            }
        }
        newMatrix
    }
    pub fn sub(&mut self, v: &Matrix<K>) {
        if !self.is_same_size(&v) {
            panic!("cant add to matrix bc its not the right size")
        }
        for (colum1, column2) in self.elements.iter_mut().zip(v.elements.iter()) {
            for (row1, row2) in colum1.iter_mut().zip(column2.iter()) {
                *row1 = row1.clone() - row2.clone();
            }
        }
    }
    pub fn sube(&mut self, v: &Matrix<K>) -> Matrix<K> {
        if !self.is_same_size(&v) {
            panic!("cant add to matrix bc its not the right size")
        }
        let mut newMatrix = self.clone();
        for (colum1, column2) in newMatrix.elements.iter_mut().zip(v.elements.iter()) {
            for (row1, row2) in colum1.iter_mut().zip(column2.iter()) {
                *row1 = row1.clone() - row2.clone();
            }
        }
        newMatrix
    }
    pub fn scl(&mut self, a: K) {
        for row in self.elements.iter_mut() {
            for column in row.iter_mut() {
                *column = column.clone() * a;
            }
        }
    }
    pub fn size(&self) -> (usize, usize) {
        let row_size: usize = self.elements.len();
        let mut column_size: usize = 0;
        for row in self.elements.iter() {
            for _column in row.iter() {
                column_size += 1;
            }
            break;
        }
        (row_size, column_size)
    }

    pub fn is_same_size(&self, matrix2: &Matrix<K>) -> bool {
        let size_matrix_1 = self.size();
        let size_matrix_2 = matrix2.size();
        if size_matrix_1.0 == size_matrix_2.0 {
            if size_matrix_1.1 == size_matrix_2.1 {
                return true;
            }
        }
        return false;
    }
}

impl<T: Clone> Clone for Matrix<T> {
    fn clone(&self) -> Self {
        let elements = self.elements.iter().map(|row| row.clone()).collect();
        Matrix { elements }
    }
}
impl<
        K: Copy
            + Clone
            + std::ops::Add<Output = K>
            + std::ops::Sub<Output = K>
            + std::ops::Mul<Output = K>,
    > Add<Matrix<K>> for Matrix<K>
{
    type Output = Matrix<K>;
    fn add(self, other: Matrix<K>) -> Matrix<K> {
        let mut new_Matrix = self.clone();
        let tmp = new_Matrix.adde(&other);
        tmp
    }
}
impl<K> Mul<K> for Matrix<K>
where
    K: Mul<Output = K> + Copy,
{
    type Output = Matrix<K>;

    fn mul(self, scalar: K) -> Matrix<K> {
        let mut result = self.clone();
        for row in result.elements.iter_mut() {
            for column in row.iter_mut() {
                *column = column.clone() * scalar;
            }
        }
        result
    }
}
impl<K> Sub<Matrix<K>> for Matrix<K>
where
    K: Copy
        + Clone
        + std::ops::Add<Output = K>
        + std::ops::Sub<Output = K>
        + std::ops::Mul<Output = K>,
{
    type Output = Matrix<K>;
    fn sub(self, other: Matrix<K>) -> Matrix<K> {
        let mut new_Matrix = self.clone();
        let tmp = new_Matrix.sube(&other);
        tmp
    }
}
