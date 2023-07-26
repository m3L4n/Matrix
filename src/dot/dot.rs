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
        for (elem1, elem2) in self.elements.iter().zip(v.elements.iter()) {
            result_dot_operations = result_dot_operations + (elem1.clone() * elem2.clone());
        }
        result_dot_operations
    }
}
