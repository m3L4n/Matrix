use crate::Matrix;
use std::vec;

// pub fn division_vec<K>(vec: Vec<K>, scl: K) -> Vec<K>
// where
//     K: Add<Output = K>
//         + Sub<Output = K>
//         + Mul<Output = K>
//         + Copy
//         + std::fmt::Debug
//         + std::fmt::Display
//         + std::cmp::PartialOrd
//         + std::ops::Div<Output = K>,
// {
//     let mut vec_to_send: Vec<K> = vec.clone();
//     for row in vec_to_send.iter_mut() {
//         *row = row.clone() / scl;
//     }
//     vec_to_send
// }
// pub fn multiply_vec<K>(vec: Vec<K>, scl: K) -> Vec<K>
// where
//     K: Add<Output = K>
//         + Sub<Output = K>
//         + Mul<Output = K>
//         + Copy
//         + std::fmt::Debug
//         + std::fmt::Display
//         + std::cmp::PartialOrd
//         + std::ops::Div<Output = K>,
// {
//     let mut vec_to_send: Vec<K> = vec.clone();
//     for row in vec_to_send.iter_mut() {
//         *row = row.clone() * scl;
//     }
//     vec_to_send
// }
// pub fn sub_vec<K>(vec: Vec<K>, vec1: Vec<K>) -> Vec<K>
// where
//     K: Add<Output = K>
//         + Sub<Output = K>
//         + Mul<Output = K>
//         + Copy
//         + std::fmt::Debug
//         + std::fmt::Display
//         + std::cmp::PartialOrd
//         + std::ops::Div<Output = K>,
// {
//     let mut new_vector = vec.clone();
//     for (elem1, elem2) in new_vector.iter_mut().zip(vec1.iter()) {
//         *elem1 = elem1.clone() - elem2.clone();
//     }
//     new_vector
// }

// fn get_identity_matrix<K>(n: usize, matrix_to_apprend: Matrix<K>) -> Matrix<K>
// where
//     K: Add<Output = K>
//         + Sub<Output = K>
//         + Mul<Output = K>
//         + Copy
//         + std::fmt::Debug
//         + std::fmt::Display
//         + std::ops::Div<Output = K>,
// {
//     let mut matrix_indentity: Matrix<f32> = Matrix { elements: vec![] };
//     for i in 0..n {
//         let mut tmp_vec: Vec<f32> = matrix_to_apprend.elements[i].clone();
//         for k in 0..n {
//             if k != i {
//                 tmp_vec.push(0.0);
//             } else {
//                 tmp_vec.push(1.0);
//             }
//         }
//         matrix_indentity.elements.push(tmp_vec);
//     }
//     matrix_indentity
// }
// impl Matrix<f32> {
//     fn found_pivot_and_transform_column_max_n(
//         &mut self,
//         pivot: &mut usize,
//         index_column: &mut usize,
//         index_initial: usize,
//         max_n: usize,
//     ) {
//         let mut max = (
//             self.elements[*pivot][*index_column],
//             *pivot,
//             self.elements[*pivot].clone(),
//         );
//         if *index_column >= max_n {
//             return;
//         }
//         for i in *pivot..self.elements.len() {
//             if max.0 < self.elements[i][*index_column] {
//                 max.0 = self.elements[i][*index_column]; // le nbr max
//                 max.1 = i; // la ou on a trouve le max
//                 max.2 = self.elements[i].clone(); // var  a changer de place
//             }
//         }

//         let zero = 0.0;
//         // println!("max{:?}", max);
//         // println!("max0 {}", max.0);
//         if max.0 == 0.0 {
//             let un: usize = 1;
//             *index_column = index_column.clone() + un;
//             if *index_column < self.elements[*pivot].len() {
//                 self.found_pivot_and_transform_column_max_n(
//                     pivot,
//                     index_column,
//                     index_initial,
//                     max_n,
//                 );
//             }
//             return;
//         }
//         if max.0 != 0.0 {
//             max.2 = division_vec(max.2, max.0);
//             if max.1 != *pivot {
//                 self.elements[max.1] = self.elements[*index_column].clone();
//             }
//             let pivot_tmp = *pivot;
//             self.elements[index_initial] = max.2.clone();
//             let mut tmp = self.clone();
//             *pivot = pivot_tmp + 1;
//             for i in 0..tmp.elements.len() {
//                 if i != (*pivot - 1) {
//                     if self.elements[i][*index_column] != 0.0 {
//                         let res_mult = multiply_vec(max.2.clone(), self.elements[i][*index_column]);
//                         let result_sub = sub_vec(tmp.elements[i].clone(), res_mult);
//                         self.elements[i] = result_sub;
//                     }
//                 }
//             }
//         }
//     }

//     fn get_inverse_matrix(&mut self) -> Matrix<K> {
//         let mut pivot: usize = 0;
//         let mut index_column: usize = 0;
//         let mut newMatrix = self.clone();
//         for (index, colum) in self.elements.iter_mut().enumerate() {
//             newMatrix.found_pivot_and_transform_column_max_n(
//                 &mut pivot,
//                 &mut index_column,
//                 index,
//                 newMatrix.size().0,
//             );
//             index_column = index_column + 1;
//         }

//         newMatrix
//     }

//     pub fn inverse(&mut self) -> Result<Matrix<K>, String> {
//         let size_self = self.size();
//         if size_self.0 != size_self.1 {
//             return Err("the matrix is not square".to_string());
//         }

//         let mut indentity = get_identity_matrix(size_self.0, self.clone());

//         // println!("indentity : {}", indentity);
//         let mut test = indentity.get_inverse_matrix();
//         for sub_vec in test.elements.iter_mut() {
//             if sub_vec.len() > size_self.0 {
//                 *sub_vec = sub_vec[size_self.0..].to_vec();
//             }
//         }
//         Ok(test)
//     }
// }
pub fn division_vec(vec: Vec<f32>, scl: f32) -> Vec<f32> {
    let mut vec_to_send: Vec<f32> = vec.clone();
    for row in vec_to_send.iter_mut() {
        *row = row.clone() / scl;
    }
    vec_to_send
}
pub fn multiply_vec(vec: Vec<f32>, scl: f32) -> Vec<f32> {
    let mut vec_to_send: Vec<f32> = vec;
    for row in vec_to_send.iter_mut() {
        *row = row.clone() * scl;
    }
    vec_to_send
}
pub fn sub_vec(vec: Vec<f32>, vec1: Vec<f32>) -> Vec<f32> {
    let mut new_vector = vec.clone();
    for (elem1, elem2) in new_vector.iter_mut().zip(vec1.iter()) {
        *elem1 = elem1.clone() - elem2.clone();
    }
    new_vector
}

fn get_identity_matrix(n: usize, matrix_to_apprend: Matrix<f32>) -> Matrix<f32> {
    let mut matrix_indentity: Matrix<f32> = Matrix { elements: vec![] };
    for i in 0..n {
        let mut tmp_vec: Vec<f32> = matrix_to_apprend.elements[i].clone();
        for k in 0..n {
            if k != i {
                tmp_vec.push(0.0);
            } else {
                tmp_vec.push(1.0);
            }
        }
        matrix_indentity.elements.push(tmp_vec);
    }
    matrix_indentity
}
impl Matrix<f32> {
    fn found_pivot_and_transform_column_max_n(
        &mut self,
        pivot: &mut usize,
        index_column: &mut usize,
        index_initial: usize,
        max_n: usize,
    ) {
        let mut max = (
            self.elements[*pivot][*index_column],
            *pivot,
            self.elements[*pivot].clone(),
        );
        if *index_column >= max_n {
            return;
        }
        for i in *pivot..self.elements.len() {
            if max.0 < self.elements[i][*index_column] {
                max.0 = self.elements[i][*index_column]; // le nbr max
                max.1 = i; // la ou on a trouve le max
                max.2 = self.elements[i].clone(); // var  a changer de place
            }
        }

        // let 0.0 = 0.0;
        // println!("max{:?}", max);
        // println!("max0 {}", max.0);
        let truncated_number = format!("{:.6}", max.0);
        let test: f32 = truncated_number.parse().unwrap();
        if test == 0.0 {
            let un: usize = 1;
            *index_column = index_column.clone() + un;
            if *index_column < self.elements[*pivot].len() {
                self.found_pivot_and_transform_column_max_n(
                    pivot,
                    index_column,
                    index_initial,
                    max_n,
                );
            }
            return;
        }
        if max.0 != 0.0 {
            max.2 = division_vec(max.2, max.0);
            if max.1 != *pivot {
                self.elements[max.1] = self.elements[*index_column].clone();
            }
            let pivot_tmp = *pivot;
            self.elements[index_initial] = max.2.clone();
            let tmp = self.clone();
            *pivot = pivot_tmp + 1;
            for i in 0..tmp.elements.len() {
                if i != (*pivot - 1) {
                    if self.elements[i][*index_column] != 0.0 {
                        let res_mult = multiply_vec(max.2.clone(), self.elements[i][*index_column]);
                        let result_sub = sub_vec(tmp.elements[i].clone(), res_mult);
                        self.elements[i] = result_sub;
                    }
                }
            }
        }
    }

    fn get_inverse_matrix(&mut self) -> Matrix<f32> {
        let mut pivot: usize = 0;
        let mut index_column: usize = 0;
        let mut new_matrix = self.clone();
        for (index, _colum) in self.elements.iter_mut().enumerate() {
            new_matrix.found_pivot_and_transform_column_max_n(
                &mut pivot,
                &mut index_column,
                index,
                new_matrix.size().0,
            );
            index_column = index_column + 1;
        }

        new_matrix
    }

    pub fn inverse(&mut self) -> Result<Matrix<f32>, String> {
        let size_self = self.size();
        if size_self.0 != size_self.1 {
            panic!("the matrix is not square");
        }
        if self.rank() != size_self.0 {
            return Err("the matrice is singular".to_string());
        }

        let mut indentity = get_identity_matrix(size_self.0, self.clone());

        // println!("indentity : {}", indentity);
        let mut test = indentity.get_inverse_matrix();
        for sub_vec in test.elements.iter_mut() {
            if sub_vec.len() > size_self.0 {
                *sub_vec = sub_vec[size_self.0..].to_vec();
            }
        }
        Ok(test)
    }
}
