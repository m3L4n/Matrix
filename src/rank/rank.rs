use crate::Matrix;

pub fn division_vec(vec: Vec<f32>, scl: f32) -> Vec<f32> {
    let mut vec_to_send: Vec<f32> = vec;
    for row in vec_to_send.iter_mut() {
        *row /= scl;
    }
    vec_to_send
}
pub fn multiply_vec(vec: Vec<f32>, scl: f32) -> Vec<f32> {
    let mut vec_to_send: Vec<f32> = vec;
    for row in vec_to_send.iter_mut() {
        *row *= scl;
    }
    vec_to_send
}
pub fn sub_vec(vec: Vec<f32>, vec1: Vec<f32>) -> Vec<f32> {
    let mut new_vector = vec;
    for (elem1, elem2) in new_vector.iter_mut().zip(vec1.iter()) {
        *elem1 -= *elem2;
    }
    new_vector
}

impl Matrix<f32> {
    fn found_pivot_and_transform_column_not_reduced(
        &mut self,
        pivot: &mut usize,
        index_column: &mut usize,
        index_initial: usize,
        max_n: usize,
    ) {
        if *index_column >= self.elements.len() {
            return;
        }
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

        let zero_k: f32 = 0.0;
        let truncated_number = format!("{:.6}", max.0);
        let test: f32 = truncated_number.parse().unwrap();

        if test == zero_k {
            let un: usize = 1;
            *index_column += un;
            if *index_column < self.elements[*pivot].len() {
                self.found_pivot_and_transform_column_not_reduced(
                    pivot,
                    index_column,
                    index_initial,
                    max_n,
                );
            }
            return;
        }
        if max.0 != zero_k {
            max.2 = division_vec(max.2, max.0);
            if max.1 != *pivot {
                self.elements[max.1] = self.elements[*index_column].clone();
            }
            let pivot_tmp = *pivot;
            self.elements[index_initial] = max.2.clone();
            let self_tmp = self.clone();
            *pivot = pivot_tmp + 1;
            for i in 0..self_tmp.elements.len() {
                if i != (*pivot - 1) {
                    if self.elements[i][*index_column] != zero_k {
                        let res_mult = multiply_vec(max.2.clone(), self.elements[i][*index_column]);
                        let result_sub = sub_vec(self_tmp.elements[i].clone(), res_mult);
                        self.elements[i] = result_sub;
                    }
                }
            }
        }
    }
    fn get_rank_matrix(&mut self) -> usize {
        let mut pivot: usize = 0;
        let mut index_column: usize = 0;
        let mut new_matrix = self.clone();
        for (index, _colum) in self.elements.iter_mut().enumerate() {
            new_matrix.found_pivot_and_transform_column_not_reduced(
                &mut pivot,
                &mut index_column,
                index,
                new_matrix.size().0,
            );
            index_column += 1;
        }

        pivot
    }
    pub fn rank(&mut self) -> usize {
        self.get_rank_matrix()
    }
}
