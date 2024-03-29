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
        + std::fmt::Display
        + Default,
{
    pub fn mul_vec(&mut self, vec: Vector<K>) -> Vector<K> {
        if self.elements[0].len() != vec.elements.len() {
            panic!("NOT THE same size");
        }
        let size_matrix_1 = self.size();
        if size_matrix_1.0 == 0 || size_matrix_1.1 == 0 || vec.elements.len() == 0 {
            panic!("the len is size 0 ")
        }
        let mut vector_to_send = Vector {
            elements: Vec::with_capacity(self.elements.len()),
        };
        for (_index, row) in self.elements.iter().enumerate() {
            let mut res = K::default();
            for i in 0..row.len() {
                res = res + (row[i] * vec.elements[i]);
            }
            vector_to_send.elements.push(res);
        }
        vector_to_send
    }

    pub fn dot_product_with_vec(vec_a: Vec<K>, vec_b: Vec<K>) -> K {
        let mut res_to_send: K = K::default();
        if vec_a.len() != vec_b.len() {
            panic!("error on the len");
        }
        for i in 0..vec_a.len() {
            res_to_send = res_to_send + vec_a[i] * vec_b[i];
        }
        res_to_send
    }
    // pub fn get_matrix_colum(&self) -> Matrix<K> {
    //     let mut matrix_to_send = Matrix { elements: vec![] };
    //     for _i in 0..self.elements[0].len() {
    //         matrix_to_send.elements.push(vec![]);
    //     }
    //     for row in self.elements.iter() {
    //         for (index, column) in row.iter().enumerate() {
    //             matrix_to_send.elements[index].push(column.clone());
    //         }
    //     }
    //     matrix_to_send
    // }
    pub fn get_matrix_colum(&self) -> Matrix<K> {
        let mut matrix_to_send = Matrix {
            elements: Vec::with_capacity(self.elements[0].len()),
        };

        for i in 0..self.elements[0].len() {
            matrix_to_send
                .elements
                .push(self.elements.iter().map(|row| row[i].clone()).collect());
        }

        matrix_to_send
    }

    pub fn mul_mat(&mut self, mat: Matrix<K>) -> Matrix<K> {
        let size_matrix_1 = self.size();
        let size_matrix_2 = mat.size();
        if size_matrix_1.1 != size_matrix_2.0 {
            panic!("cant do that not good size, the row doesnt match with column");
        }
        if size_matrix_1.0 == 0
            || size_matrix_1.1 == 0
            || size_matrix_2.0 == 0
            || size_matrix_2.1 == 0
        {
            panic!("cant do that");
        }
        let mat_column_good = mat.get_matrix_colum();
        let mut matrix_to_send = Matrix {
            elements: Vec::with_capacity(self.elements.len()),
        };
        for (_index, row) in self.elements.iter().enumerate() {
            let mut new_row: Vec<K> = Vec::with_capacity(mat_column_good.elements.len());
            for mat_row in mat_column_good.elements.iter() {
                new_row.push(Self::dot_product_with_vec(row.clone(), mat_row.clone()));
            }
            matrix_to_send.elements.push(new_row);
        }
        matrix_to_send
    }
}
