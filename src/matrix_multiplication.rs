use crate::Matrix;
use crate::Vector;
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
    pub fn mul_vec(&mut self, vec: Vector<K>) -> Vector<K> {
        if self.elements[0].len() != vec.elements.len() {
            panic!("NOT THE same size");
        }
        let mut vector_to_send = Vector {
            elements: vec.elements.clone(),
        };

        for (index, row) in self.elements.iter().enumerate() {
            let mut res = vec.elements[0].clone() - vec.elements[0].clone();
            for i in 0..row.len() {
                res = res + (row[i].clone() * vec.elements[i].clone());
            }
            vector_to_send.elements[index] = res;
        }
        vector_to_send
    }
    pub fn dot_product_with_vec(vec_a: Vec<K>, vec_b: Vec<K>) -> K {
        let mut res_to_send: K = vec_a[0] - vec_a[0];
        for i in 0..vec_a.len() {
            res_to_send = res_to_send + vec_a[i] * vec_b[i];
        }
        for (index, row) in vec_a.iter().enumerate() {}
        res_to_send
    }
    pub fn get_matrix_colum(&mut self) -> Matrix<K> {
        let mut matrix_to_send = Matrix { elements: vec![] };
        for i in 0..self.elements[0].len() {
            matrix_to_send.elements.push(vec![]);
        }
        for row in self.elements.iter() {
            for (index, column) in row.iter().enumerate() {
                matrix_to_send.elements[index].push(column.clone());
            }
        }
        matrix_to_send
    }

    pub fn mul_mat(&mut self, mat: Matrix<K>) -> Matrix<K> {
        let size_matrix_1 = self.size();
        let size_matrix_2 = mat.size();
        let mut matrix_to_send: Matrix<K> = Matrix { elements: vec![] };
        let mat_column_good = mat.clone().get_matrix_colum();
        if size_matrix_1.1 != size_matrix_2.0 {
            panic!("cant do that not good size, the row doesnt match with column");
        }
        for (index, row) in self.elements.iter().enumerate() {
            let mut new_new_row: Vec<K> = vec![];
            for mat_row in mat_column_good.elements.iter() {
                new_new_row.push(Self::dot_product_with_vec(row.clone(), mat_row.clone()));
            }
            matrix_to_send.elements.push(new_new_row);
        }
        matrix_to_send
    }
}
