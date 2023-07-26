use crate::inverse::inverse::multiply_vec;
use crate::inverse::inverse::sub_vec;
use crate::Matrix;

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
            Self::division_vec(&mut max.2, max.0);
            if max.1 != *pivot {
                self.elements[max.1] = self.elements[*index_column].clone();
            }
            self.elements[index_initial] = max.2.clone();
            *pivot += 1;
            for (i, colum) in self.elements.iter_mut().enumerate() {
                if i != (*pivot - 1) {
                    if colum[*index_column] != 0.0 {
                        let res_mult = multiply_vec(&max.2, colum[*index_column]);
                        let result_sub = sub_vec(colum.clone(), res_mult);
                        *colum = result_sub;
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
