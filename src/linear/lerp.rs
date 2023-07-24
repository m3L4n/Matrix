//https://www.alanzucconi.com/2021/01/24/linear-interpolation/

pub fn lerp<
    V: std::ops::Add<Output = V> + std::ops::Mul<f32, Output = V> + std::ops::Sub<Output = V> + Clone,
>(
    u: V,
    v: V,
    t: f32,
) -> V {
    u * (1. - t) + v * t
}
