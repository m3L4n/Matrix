use crate::Vector;
use std::ops::MulAssign;

impl<K> Vector<K>
where
    K: MulAssign<K> + Clone + std::ops::Mul<Output = K> + std::ops::Add<Output = K>,
{
    pub fn dot(&self, v: Vector<K>) -> K {
        if self.elements.is_empty() || v.elements.is_empty() {
            panic!("can't do operation");
        }
        if v.elements.len() != self.elements.len()
            || self.elements.len() <= 0
            || v.elements.len() <= 0
        {
            panic!("can't do operation");
        }
        let mut result_dot_operations: K = self.elements[0].clone() * v.elements[0].clone();
        for i in 1..self.elements.len() {
            result_dot_operations =
                result_dot_operations + self.elements[i].clone() * v.elements[i].clone()
        }

        result_dot_operations
    }
}
