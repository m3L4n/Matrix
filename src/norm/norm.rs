pub struct Vector {
    pub values: Vec<f32>,
}

pub struct NumericValue(f32);

impl From<i8> for NumericValue {
    fn from(item: i8) -> Self {
        NumericValue(item as f32)
    }
}

impl From<i16> for NumericValue {
    fn from(item: i16) -> Self {
        NumericValue(item as f32)
    }
}

impl From<i32> for NumericValue {
    fn from(item: i32) -> Self {
        NumericValue(item as f32)
    }
}

impl From<i64> for NumericValue {
    fn from(item: i64) -> Self {
        NumericValue(item as f32)
    }
}

impl From<u8> for NumericValue {
    fn from(item: u8) -> Self {
        NumericValue(item as f32)
    }
}

impl From<u16> for NumericValue {
    fn from(item: u16) -> Self {
        NumericValue(item as f32)
    }
}

impl From<u32> for NumericValue {
    fn from(item: u32) -> Self {
        NumericValue(item as f32)
    }
}

impl From<u64> for NumericValue {
    fn from(item: u64) -> Self {
        NumericValue(item as f32)
    }
}

impl From<f32> for NumericValue {
    fn from(item: f32) -> Self {
        NumericValue(item)
    }
}

impl From<f64> for NumericValue {
    fn from(item: f64) -> Self {
        NumericValue(item as f32)
    }
}

impl Vector {
    pub fn new<T: Into<NumericValue>>(values: Vec<T>) -> Self {
        Vector {
            values: values.into_iter().map(|v| v.into().0).collect(),
        }
    }

    pub fn sum(&self) -> f32 {
        self.values.iter().sum()
    }
    pub fn norm(&mut self) -> f32 {
        let mut result_euclidean: f32 = 0.0;
        if self.values.len() < 0 {
            panic!(" the len is equal to 0");
        }
        for element in &self.values {
            let squared: f32 = element.clone() * element.clone();
            result_euclidean = result_euclidean + squared;
        }

        let sqrt_rounded = result_euclidean.sqrt();
        sqrt_rounded
    }
    pub fn norm_1(&mut self) -> f32 {
        let mut norm: f32 = 0.0;
        if self.values.len() < 0 {
            panic!(" the len is equal to 0");
        }
        for element in &self.values {
            norm = (element.clone()) + norm;
        }
        if norm < 0. {
            norm = -norm;
        }
        norm
    }
    pub fn norm_inf(&mut self) -> f32 {
        if self.values.len() < 0 {
            panic!(" the len is equal to 0");
        }
        let mut norm: f32 = self.values[0].clone().into();

        for element in &self.values {
            let mut elem_f32: f32 = element.clone().into();
            if elem_f32 < 0. {
                elem_f32 = -elem_f32;
            }
            if norm >= elem_f32 {
                norm = norm
            } else {
                norm = elem_f32;
            }
        }
        norm
    }
}
