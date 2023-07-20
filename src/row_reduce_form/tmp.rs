use crate::Matrix;
use std::ops::{Add, Mul, Sub};

pub fn division_vec<K>(vec: Vec<K>, scl: K) -> Vec<K>
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
    let mut vec_to_send: Vec<K> = vec.clone();
    for row in vec_to_send.iter_mut() {
        *row = row.clone() / scl;
    }
    vec_to_send
}
pub fn multiply_vec<K>(vec: Vec<K>, scl: K) -> Vec<K>
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
    let mut vec_to_send: Vec<K> = vec.clone();
    for row in vec_to_send.iter_mut() {
        *row = f32::trunc(row.clone() * scl);
    }
    vec_to_send
}
pub fn sub_vec<K>(vec: Vec<K>, vec1: Vec<K>) -> Vec<K>
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
    let mut new_vector = vec.clone();
    for (elem1, elem2) in new_vector.iter_mut().zip(vec1.iter()) {
        *elem1 = elem1.clone() - elem2.clone();
    }
    new_vector
}
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
    fn found_pivot_and_transform_column(
        &mut self,
        pivot: &mut usize,
        index_column: &mut usize,
        index_initial: usize,
    ) {
        if *index_column > self.elements[0].len() - 1 {
            return;
        }
        println!(
            "index {} {} {} {}",
            *index_column,
            *pivot,
            self.elements.len(),
            self
        );
        let mut max = (
            self.elements[*pivot][*index_column],
            *pivot,
            self.elements[*pivot].clone(),
        );
        for i in *pivot..self.elements.len() {
            if max.0 < self.elements[i][*index_column] {
                // println!(
                //     " dans pivot{}{} {} ",
                //     *pivot, self.elements[i][*index_column], max.0
                // );
                max.0 = f32::trunc(self.elements[i][*index_column]); // le nbr max
                max.1 = i; // la ou on a trouve le max
                max.2 = self.elements[i].clone(); // var  a changer de place
            }
        }
        let zero: K = K::default();
        // if (max.1 >= self.elements.len()) {
        //     println!("ici cc");
        //     if (max.0 == zero) {
        //         return;
        //     }
        // }
        println!("max{:?}", max);
        // println!("max0 {}", max.0);
        if max.0 == zero {
            let un: usize = 1;
            *index_column = index_column.clone() + un;
            println!("index af{}", *index_column);
            if *index_column >= self.elements[0].len() - 1 {
                return;
            }
            if *index_column < self.elements[*pivot].len() {
                self.found_pivot_and_transform_column(pivot, index_column, index_initial);
            }
            return;
        }
        if max.0 != zero {
            max.2 = division_vec(max.2, max.0);
            if max.1 != *pivot {
                self.elements[max.1] = self.elements[*index_column].clone();
                // println!("cc {:?}", self.elements[*index_column]);
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
    pub fn row_echelon(&mut self) -> Matrix<K> {
        let mut pivot: usize = 0;
        let mut index_column: usize = 0;
        let mut new_matrix = self.clone();
        for (index, _colum) in self.elements.iter_mut().enumerate() {
            new_matrix.found_pivot_and_transform_column(&mut pivot, &mut index_column, index);
            // println!("newmatric {}", new_matrix);
            index_column = index_column + 1;
        }

        // il faut qu il y ai un 1 en first position de la premiere colone
        // -> sois c un non null o debut et c ok
        //sois tu vas echanger avec une ligne si ya un en first ou

        new_matrix
    }
}
