use crate::Vector;
use std::ops::MulAssign;

impl<K> Vector<K>
where
    K: MulAssign<K> + Clone + std::ops::Mul<Output = K> + std::ops::Add<Output = K> + Default,
{
    pub fn dot(&self, v: Vector<K>) -> K {
        if v.elements.len() != self.elements.len()
            || self.elements.len() <= 0
            || v.elements.len() <= 0
        {
            panic!("can't do operation");
        }
        let mut result_dot_operations: K = K::default();
        for i in 0..self.elements.len() {
            result_dot_operations =
                result_dot_operations + self.elements[i].clone() * v.elements[i].clone()
        }

        result_dot_operations
    }
}
