use std::fmt;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;
pub struct Vector<K> {
    pub elements: Vec<K>,
}

impl<K: fmt::Display + std::fmt::Debug> fmt::Display for Vector<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let elements_str = self
            .elements
            .iter()
            .map(|elem| elem.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "Vector : [{}]", elements_str).expect("can't write in stdout");
        Ok(())
    }
}

impl<
        K: Copy
            + Clone
            + std::ops::Add<Output = K>
            + std::ops::Sub<Output = K>
            + std::ops::Mul<Output = K>,
    > Vector<K>
{
    pub fn from(data: Vec<K>) -> Vector<K> {
        let vec = Vector {
            // permet de demander au compilateur de savoir le type concret
            elements: Vec::from(data),
        };
        vec
    }
    pub fn add(&mut self, v: &Vector<K>) {
        for (elem1, elem2) in self.elements.iter_mut().zip(v.elements.iter()) {
            *elem1 = elem1.clone() + elem2.clone();
        }
    }
    pub fn sub(&mut self, v: &Vector<K>) {
        for (elem1, elem2) in self.elements.iter_mut().zip(v.elements.iter()) {
            *elem1 = elem1.clone() - elem2.clone();
        }
    }
    pub fn scl(&mut self, a: K) {
        for element in self.elements.iter_mut() {
            *element = element.clone() * a;
        }
    }
    // pub fn size(self) -> usize {
    //     let row_size: usize = self.elements.len();
    //     row_size
    // }
    fn adde(&mut self, v: &Vector<K>) -> Vector<K> {
        let mut new_vector = self.clone();
        for (elem1, elem2) in new_vector.elements.iter_mut().zip(v.elements.iter()) {
            *elem1 = elem1.clone().add(*elem2);
        }
        new_vector
    }
    fn sube(&mut self, v: &Vector<K>) -> Vector<K> {
        let mut new_vector = self.clone();
        for (elem1, elem2) in new_vector.elements.iter_mut().zip(v.elements.iter()) {
            *elem1 = elem1.clone().sub(*elem2);
        }
        new_vector
    }
}
impl<K: Copy> Clone for Vector<K> {
    fn clone(&self) -> Self {
        let elements = self.elements.iter().cloned().collect();
        Vector { elements }
    }
}

impl<
        K: Copy
            + Clone
            + std::ops::Add<Output = K>
            + std::ops::Sub<Output = K>
            + std::ops::Mul<Output = K>,
    > Add<Vector<K>> for Vector<K>
{
    type Output = Vector<K>;
    fn add(self, other: Vector<K>) -> Vector<K> {
        let mut new_vector = self.clone();
        let tmp = new_vector.adde(&other);
        tmp
    }
}
impl<K> Mul<K> for Vector<K>
where
    K: Mul<Output = K> + Copy,
{
    type Output = Vector<K>;

    fn mul(self, scalar: K) -> Vector<K> {
        let mut result = self.clone();
        for element in result.elements.iter_mut() {
            *element = *element * scalar;
        }
        result
    }
}
impl<K> Sub<Vector<K>> for Vector<K>
where
    K: Copy
        + Clone
        + std::ops::Add<Output = K>
        + std::ops::Sub<Output = K>
        + std::ops::Mul<Output = K>,
{
    type Output = Vector<K>;
    fn sub(self, other: Vector<K>) -> Vector<K> {
        let mut new_vector = self.clone();
        let tmp = new_vector.sube(&other);
        tmp
    }
}
