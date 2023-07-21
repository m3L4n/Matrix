use crate::Vector;
use std::ops::Add;
use std::ops::Mul;

use std::ops::MulAssign;
pub fn get_dot_f32(n_u: f32, n_v: f32) -> f32 {
    n_u * n_v
}
pub fn angle_cos<K>(u: &Vector<K>, v: &Vector<K>) -> f32
where
    K: Clone
        + Mul<K, Output = K>
        + Add<f32>
        + Into<f32>
        + std::fmt::Display
        + MulAssign<K>
        + Clone
        + std::ops::Mul<Output = K>
        + std::ops::Add<Output = K>
        + std::ops::Div<f32, Output = f32>,
    K: Add,
    K: MulAssign,
{
    if u.elements.len() != v.elements.len() {
        panic!(" not the same size");
    }
    let mut vec_u = Vector {
        elements: u.elements.clone(),
    };
    let mut vec_v = Vector {
        elements: v.elements.clone(),
    };
    let norm_u: f32 = vec_u.norm_k();
    let norm_v: f32 = vec_v.norm_k();
    let dot_u_v = vec_u.dot(vec_v);
    let dot_norm_u_v = get_dot_f32(norm_u, norm_v); //32.832910318764
    let angle_cos = dot_u_v / dot_norm_u_v;
    angle_cos
    // norm (u ) norm (v)
    // dot (u, v)
    // dot(u, v) / dot(norm(u), norm(v))
}
