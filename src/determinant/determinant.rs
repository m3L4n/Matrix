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
    fn initialize_matrix(&mut self) -> Matrix<K> {
        if self.size().0 == 4 && self.size().1 == 4 {
            return Matrix {
                elements: vec![vec![], vec![], vec![]],
            };
        } else if self.size().0 == 3 && self.size().1 == 3 {
            return Matrix {
                elements: vec![vec![], vec![]],
            };
        }
        self.clone()
    }

    fn get_matrix_by_index(&mut self, index: usize) -> Matrix<K> {
        let mut new_matrix = self.initialize_matrix();

        let mut k = 0;
        for i in 1..self.elements.len() {
            for u in 0..self.elements[i].len() {
                if u != index {
                    new_matrix.elements[k].push(self.elements[i][u]);
                }
            }
            k += 1;
        }
        new_matrix
    }

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

    fn determinant_3_3(&mut self) -> K {
        let mut clone = self.clone();
        let mut res = K::default();
        for (index, _row) in self.elements[0].iter().enumerate() {
            if index % 2 == 0 {
                let mut tmp = clone.get_matrix_by_index(index);
                let mut resultat_2_2 = tmp.determinant_2_2();
                resultat_2_2 = resultat_2_2 * clone.elements[0][index];
                res = res + resultat_2_2;
            } else {
                let mut tmp = clone.get_matrix_by_index(index);
                let mut resultat_2_2 = tmp.determinant_2_2();
                resultat_2_2 = resultat_2_2 * clone.elements[0][index];
                res = res - resultat_2_2;
            }
        }
        res
    }

    fn determinant_4_4(&mut self) -> K {
        let mut clone = self.clone();
        let mut res = K::default();
        for (index, _row) in self.elements[0].iter().enumerate() {
            if index % 2 == 0 {
                let mut tmp = clone.get_matrix_by_index(index);
                let mut result_3_3 = tmp.determinant_3_3();
                result_3_3 = result_3_3 * clone.elements[0][index];
                res = res + result_3_3;
            } else {
                let mut tmp = clone.get_matrix_by_index(index);
                let mut result_3_3 = tmp.determinant_3_3();
                result_3_3 = result_3_3 * clone.elements[0][index];
                res = res - result_3_3;
            }
        }
        res
    }

    // fn while_to_2_dimension(&mut self, index: usize, res: K) -> K {
    //     // println!("mat{}", matrix_clone);
    //     let mut matrix_clone = self.clone();
    //     let mut size = matrix_clone.size();
    //     let mut res_to_send = res - res;

    //     let mut tmp = matrix_clone.get_matrix_by_index(index);
    //     let mut tmp_do_thing = tmp.clone();
    //     if tmp.size().0 == 2 && tmp.size().1 == 2 {
    //         if index % 2 != 0 {
    //             let neg = (matrix_clone.elements[0][index] - matrix_clone.elements[0][index])
    //                 - matrix_clone.elements[0][index];
    //             return neg * tmp.determinant_2_2();
    //         }
    //         let result = tmp.determinant_2_2();
    //         return result * matrix_clone.elements[0][index];
    //     }
    //     for (indexo, row) in tmp.elements[0].iter().enumerate() {
    //         if tmp.size().0 > 2 && tmp.size().1 > 2 {
    //             let test = tmp_do_thing.while_to_2_dimension(indexo, res_to_send);
    //             res_to_send = res_to_send + test;
    //         }
    //     }
    //     res_to_send
    // }
    // fn other_determinant(&mut self) -> K {
    //     let mut clone = self.clone();
    //     let mut res = self.elements[0][0] - self.elements[0][0];
    //     for (index, row) in self.elements[0].iter().enumerate() {
    //         if index > 3 {
    //             break;
    //         }
    //         let tmp = clone.while_to_2_dimension(index, res);
    //         if index % 2 != 0 {
    //             let neg =
    //                 (self.elements[0][index] - self.elements[0][index]) - self.elements[0][index];
    //             res = res + (neg * tmp);
    //         } else {
    //             res = res + (tmp * self.elements[0][index]);
    //         }
    //     }
    //     res
    // }

    pub fn determinant(&mut self) -> K {
        let size_self = self.size();
        if size_self.0 != size_self.1 {
            panic!("the matrix is not square");
        }
        if size_self.0 <= 1 && size_self.1 <= 1 {
            panic!("problem with  the size of the matrix");
        }
        if size_self.0 == 2 && size_self.1 == 2 {
            return self.determinant_2_2();
        } else if size_self.0 == 3 && size_self.1 == 3 {
            return self.determinant_3_3();
        }
        self.determinant_4_4()
    }
}
