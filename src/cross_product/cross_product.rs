use crate::Vector;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;
pub fn cross_product<K: Clone>(u: &Vector<K>, v: &Vector<K>) -> Vector<K>
where
    K: Add<Output = K>
        + Sub<Output = K>
        + Mul<Output = K>
        + Copy
        + std::fmt::Debug
        + std::fmt::Display,
{
    if u.elements.len() != 3 || v.elements.len() != 3 {
        panic!("vector are not 3 dimensions");
    }
    let vector_u = Vector {
        elements: u.elements.clone(),
    };
    let vector_v = Vector {
        elements: v.elements.clone(),
    };
    let mut vector_to_send = Vector {
        elements: u.elements.clone(),
    };
    vector_to_send.elements[0] =
        vector_u.elements[1] * vector_v.elements[2] - vector_u.elements[2] * vector_v.elements[1];
    vector_to_send.elements[1] =
        vector_u.elements[2] * vector_v.elements[0] - vector_u.elements[0] * vector_v.elements[2];
    vector_to_send.elements[2] =
        vector_u.elements[0] * vector_v.elements[1] - vector_u.elements[1] * vector_v.elements[0];
    vector_to_send
}
