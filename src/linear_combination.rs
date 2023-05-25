use crate::Vector;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;
pub fn linear_combination<K>(u: &[Vector<K>], coefs: &[K]) -> Vector<K>
where
    K: Add<Output = K>
        + Sub<Output = K>
        + Mul<Output = K>
        + Copy
        + std::fmt::Debug
        + std::fmt::Display,
{
    if u.len() <= 0 || coefs.len() <= 0 {
        panic!("probleme dans les parametre");
    }
    let mut vec_to_send = u[0].clone();
    vec_to_send.scl(coefs[0]);
    for i in 1..u.len() {
        let mut _tmp_vec = u[i].clone();
        _tmp_vec.scl(coefs[i]);
        vec_to_send = vec_to_send + _tmp_vec;
    }
    return vec_to_send;
}
