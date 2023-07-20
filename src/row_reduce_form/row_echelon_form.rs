use crate::Matrix;
use std::ops::{Add, Mul, Sub};

pub fn division_vec(vec: Vec<f32>, scl: f32) -> Vec<f32> {
    let mut vec_to_send: Vec<f32> = vec.clone();
    for row in vec_to_send.iter_mut() {
        *row = row.clone() / scl;
    }
    vec_to_send
}
pub fn multiply_vec(vec: Vec<f32>, scl: f32) -> Vec<f32> {
    let mut vec_to_send: Vec<f32> = vec.clone();
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
impl Matrix<f32> {
    fn found_pivot_and_transform_column(
        &mut self,
        pivot: &mut usize,
        index_column: &mut usize,
        index_initial: usize,
    ) {
        if *pivot > self.elements.len() - 1 {
            return;
        }

        let mut max = (
            self.elements[*pivot][*index_column],
            *pivot,
            self.elements[*pivot].clone(),
        );
        for i in *pivot..self.elements.len() {
            if max.0 < self.elements[i][*index_column] {
                max.0 = self.elements[i][*index_column]; // le nbr max
                max.1 = i; // la ou on a trouve le max
                max.2 = self.elements[i].clone(); // var  a changer de place
            }
        }
        let zero: f32 = 0.0;
        let truncated_number = format!("{:.6}", max.0);
        let test: f32 = truncated_number.parse().unwrap();

        if test == zero {
            let un: usize = 1;
            *index_column = index_column.clone() + un;

            if *index_column < self.elements[*pivot].len() {
                self.found_pivot_and_transform_column(pivot, index_column, index_initial);
            }
            return;
        }
        if max.0 != zero {
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
                    // println!("self {} ", self);
                    if self.elements[i][*index_column] != zero {
                        let res_mult = multiply_vec(max.2.clone(), self.elements[i][*index_column]);
                        let result_sub = sub_vec(tmp.elements[i].clone(), res_mult);
                        self.elements[i] = result_sub;
                    }
                }
            }
        }
    }
    pub fn row_echelon(&mut self) -> Matrix<f32> {
        let mut pivot: usize = 0;
        let mut index_column: usize = 0;
        let mut new_matrix = self.clone();
        for (index, _colum) in self.elements.iter_mut().enumerate() {
            new_matrix.found_pivot_and_transform_column(&mut pivot, &mut index_column, index);
            index_column = index_column + 1;
        }

        // il faut qu il y ai un 1 en first position de la premiere colone
        // -> sois c un non null o debut et c ok
        //sois tu vas echanger avec une ligne si ya un en first ou

        new_matrix
    }
}
