//https://www.alanzucconi.com/2021/01/24/linear-interpolation/

pub fn lerp<
    V: std::ops::Add<Output = V> + std::ops::Mul<f32, Output = V> + std::ops::Sub<Output = V> + Clone,
>(
    u: V,
    v: V,
    t: f32,
) -> V {
    let mut ue = v.clone() - u.clone();
    ue = ue * t;
    ue = ue + u;
    ue
}
