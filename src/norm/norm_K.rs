use crate::Vector;
use std::ops::{Add, Mul};
impl<K> Vector<K>
where
    K: Clone + Mul<K, Output = K> + Add<f32> + Into<f32> + std::fmt::Display,
{
    pub fn norm(&mut self) -> f32 {
        let mut result_euclidean: f32 = 0.0;
        // if self.elements.len() < 0 {
        //     panic!(" the len is equal to 0");
        // }
        for element in &self.elements {
            let squared = (element.clone()).into() * (element.clone()).into();
            result_euclidean = result_euclidean + squared;
        }

        let sqrt_rounded = result_euclidean.sqrt();
        sqrt_rounded
    }
    // pub fn norm_1(&mut self) -> f32 {
    //     let mut norm: f32 = 0.0;
    //     if self.elements.len() < 0 {
    //         panic!(" the len is equal to 0");
    //     }
    //     for element in &self.elements {
    //         norm = (element.clone()).into() + norm;
    //     }
    //     if norm < 0. {
    //         norm = -norm;
    //     }
    //     norm
    // }
    // pub fn norm_inf(&mut self) -> f32 {
    //     if self.elements.len() < 0 {
    //         panic!(" the len is equal to 0");
    //     }
    //     let mut norm: f32 = self.elements[0].clone().into();

    //     for element in &self.elements {
    //         let mut elem_f32: f32 = element.clone().into();
    //         if elem_f32 < 0. {
    //             elem_f32 = -elem_f32;
    //         }
    //         if norm >= elem_f32 {
    //             norm = norm
    //         } else {
    //             norm = elem_f32;
    //         }
    //     }
    //     norm
    // }
}
