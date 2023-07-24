use crate::Vector;
use std::ops::{Add, Mul};
fn sqrt(result: f32) -> f32 {
    if result < 0. {
        return f32::NAN;
    }

    let mut guess = result;
    let mut prev_guess = 0.;
    let mut guess_result = prev_guess - guess;

    if guess_result < 0. {
        guess_result *= -1.;
    }
    while guess_result > 0.00000001 {
        prev_guess = guess;
        guess = 0.5 * (guess + result / guess);
        guess_result = prev_guess - guess;
        if guess_result < 0. {
            guess_result *= -1.;
        }
    }

    guess
}
impl<K> Vector<K>
where
    K: Clone + Mul<K, Output = K> + Add<f32> + Into<f32> + std::fmt::Display,
{
    pub fn norm_k(&self) -> f32 {
        let mut result_euclidean: f32 = 0.0;
        for element in &self.elements {
            let squared = (element.clone()).into() * (element.clone()).into();
            result_euclidean = result_euclidean + squared;
        }

        let sqrt_rounded = sqrt(result_euclidean);
        sqrt_rounded
    }
}
