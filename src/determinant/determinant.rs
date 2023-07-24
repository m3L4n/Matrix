use crate::Matrix;
use std::{
    ops::{Add, Mul, Sub},
    vec,
};

impl<K> Matrix<K>
where
    K: Add<Output = K>
        + Sub<Output = K>
        + Mul<Output = K>
        + Copy
        + std::fmt::Debug
        + std::fmt::Display
        + std::cmp::PartialOrd
        + std::ops::Div<Output = K>
        + Default,
{
    fn determinant_2_2(&mut self) -> K {
        let mut res_to_send = K::default();
        let size = self.size();
        if size.0 == 2 && size.1 == 2 {
            let res1 = self.elements[0][0] * self.elements[1][1];
            let res2 = self.elements[0][1] * self.elements[1][0];
            res_to_send = res1 - res2;
        }
        res_to_send
    }

    fn get_matrix_3_to_2(&self, index: usize) -> Matrix<K> {
        let mut matrix: Matrix<K> = Matrix {
            elements: vec![vec![], vec![]],
        };
        if index == 0 {
            matrix.elements[0] = vec![self.elements[1][1], self.elements[1][2]];
            matrix.elements[1] = vec![self.elements[2][1], self.elements[2][2]];
        } else if index == 1 {
            matrix.elements[0] = vec![self.elements[1][0], self.elements[1][2]];
            matrix.elements[1] = vec![self.elements[2][0], self.elements[2][2]];
        } else if index == 2 {
            matrix.elements[0] = vec![self.elements[1][0], self.elements[1][1]];
            matrix.elements[1] = vec![self.elements[2][0], self.elements[2][1]];
        }
        matrix
    }
    fn get_matrix_4_to_3(&self, index: usize) -> Matrix<K> {
        let mut matrix: Matrix<K> = Matrix {
            elements: vec![vec![], vec![], vec![]],
        };
        if index == 0 {
            matrix.elements[0] = vec![
                self.elements[1][1],
                self.elements[1][2],
                self.elements[1][3],
            ];
            matrix.elements[1] = vec![
                self.elements[2][1],
                self.elements[2][2],
                self.elements[2][3],
            ];
            matrix.elements[2] = vec![
                self.elements[3][1],
                self.elements[3][2],
                self.elements[3][3],
            ];
        } else if index == 1 {
            matrix.elements[0] = vec![
                self.elements[1][0],
                self.elements[1][2],
                self.elements[1][3],
            ];
            matrix.elements[1] = vec![
                self.elements[2][0],
                self.elements[2][2],
                self.elements[2][3],
            ];
            matrix.elements[2] = vec![
                self.elements[3][0],
                self.elements[3][2],
                self.elements[3][3],
            ];
        } else if index == 2 {
            matrix.elements[0] = vec![
                self.elements[1][0],
                self.elements[1][1],
                self.elements[1][3],
            ];
            matrix.elements[1] = vec![
                self.elements[2][0],
                self.elements[2][1],
                self.elements[2][3],
            ];
            matrix.elements[2] = vec![
                self.elements[3][0],
                self.elements[3][1],
                self.elements[3][3],
            ];
        } else if index == 3 {
            matrix.elements[0] = vec![
                self.elements[1][0],
                self.elements[1][1],
                self.elements[1][2],
            ];
            matrix.elements[1] = vec![
                self.elements[2][0],
                self.elements[2][1],
                self.elements[2][2],
            ];
            matrix.elements[2] = vec![
                self.elements[3][0],
                self.elements[3][1],
                self.elements[3][2],
            ];
        }
        matrix
    }
    fn determinant_3_3(&mut self) -> K {
        let clone = self.clone();
        let mut res = K::default();
        for (index, _row) in self.elements[0].iter().enumerate() {
            if index % 2 == 0 {
                let mut tmp = self.get_matrix_3_to_2(index);
                let mut resultat_2_2 = tmp.determinant_2_2();
                resultat_2_2 = resultat_2_2 * clone.elements[0][index];
                res = res + resultat_2_2;
            } else {
                let mut tmp = self.get_matrix_3_to_2(index);
                let mut resultat_2_2 = tmp.determinant_2_2();
                resultat_2_2 = resultat_2_2 * clone.elements[0][index];
                res = res - resultat_2_2;
            }
        }
        res
    }

    fn determinant_4_4(&mut self) -> K {
        let clone = self.clone();
        let mut res = K::default();
        for (index, _row) in self.elements[0].iter().enumerate() {
            if index % 2 == 0 {
                let mut tmp = clone.get_matrix_4_to_3(index);
                let mut result_3_3 = tmp.determinant_3_3();
                result_3_3 = result_3_3 * self.elements[0][index];
                res = res + result_3_3;
            } else {
                let mut tmp = clone.get_matrix_4_to_3(index);
                let mut result_3_3 = tmp.determinant_3_3();
                result_3_3 = result_3_3 * clone.elements[0][index];
                res = res - result_3_3;
            }
        }
        res
    }

    pub fn determinant(&mut self) -> K {
        let size_self = self.size();
        if size_self.0 != size_self.1 {
            panic!("the matrix is not square");
        }
        if size_self.0 < 1 && size_self.1 < 1 {
            panic!("problem with  the size of the matrix");
        }
        if size_self.0 == 1 && size_self.1 == 1 {
            return self.elements[0][0];
        }
        if size_self.0 == 2 && size_self.1 == 2 {
            return self.determinant_2_2();
        } else if size_self.0 == 3 && size_self.1 == 3 {
            return self.determinant_3_3();
        }
        self.determinant_4_4()
    }
}
