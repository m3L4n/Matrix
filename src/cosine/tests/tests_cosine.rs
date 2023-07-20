use crate::cosine::cosine::angle_cos;
use crate::Vector;
use std::vec;
pub fn tests_cosine() {
    println!("------------------------------------------------------");
    println!("COSINE");
    println!("------------------------------------------------------");
    let u = Vector::from(vec![1., 2., 3.]);
    let v = Vector::from(vec![4., 5., 6.]);
    println!("test with\n{}\n{}", u, v);
    println!("{}", angle_cos(&u, &v));
    assert_eq!(angle_cos(&u, &v), 0.9746318);
    println!("------------------------------------------------------");
    let u = Vector::from(vec![1., 0.]);
    let v = Vector::from(vec![1., 0.]);
    println!("test with\n{}\n{}", u, v);
    println!("{}", angle_cos(&u, &v));
    assert_eq!(angle_cos(&u, &v), 1.0);
    println!("------------------------------------------------------");
    // 1.0
    let u = Vector::from(vec![1., 0.]);
    let v = Vector::from(vec![0., 1.]);
    println!("test with\n{}\n{}", u, v);
    println!("{}", angle_cos(&u, &v));
    assert_eq!(angle_cos(&u, &v), 0.0);
    println!("------------------------------------------------------");
    // 0.0
    let u = Vector::from(vec![-1., 1.]);
    let v = Vector::from(vec![1., -1.]);
    println!("test with\n{}\n{}", u, v);
    println!("{}", angle_cos(&u, &v));
    assert_eq!(angle_cos(&u, &v), -1.0000001);
    println!("------------------------------------------------------");
    // -1.0
    let u = Vector::from(vec![2., 1.]);
    let v = Vector::from(vec![4., 2.]);
    println!("test with\n{}\n{}", u, v);
    println!("{}", angle_cos(&u, &v));
    assert_eq!(angle_cos(&u, &v), 1.0);
    println!("------------------------------------------------------");
    let u = Vector::from(vec![1., 0.]);
    let v = Vector::from(vec![0., 1.]);
    println!("test with\n{}\n{}", u, v);
    println!("{}", angle_cos(&u, &v));
    assert_eq!(angle_cos(&u, &v), 0.0);
    println!("------------------------------------------------------");
    let u = Vector::from(vec![3., 4.]);
    let v = Vector::from(vec![4., 3.]);
    println!("test with\n{}\n{}", u, v);
    println!("{}", angle_cos(&u, &v));
    assert_eq!(angle_cos(&u, &v), 0.96);
    println!("------------------------------------------------------");
    let u = Vector::from(vec![7., 1.]);
    let v = Vector::from(vec![5., 5.]);
    println!("test with\n{}\n{}", u, v);
    println!("{}", angle_cos(&u, &v));
    assert_eq!(angle_cos(&u, &v), 0.8);
    println!("------------------------------------------------------");
    let mut u = Vector::from(vec![8., 7.]);
    let mut v = Vector::from(vec![3., 2.]);
    println!("test with\n{}\n{}", u, v);
    println!("{}", angle_cos(&u, &v));
    assert_eq!(angle_cos(&u, &v), 0.99145424);
    let norm_u_f32 = u.norm();
    let norm_calculator_u_f32 = 10.630145812735 as f32;
    assert_eq!(norm_u_f32, norm_calculator_u_f32);
    let norm_v_f32 = v.norm();
    let norm_calculator_v_f32 = 3.605551275464 as f32;
    assert_eq!(norm_v_f32, norm_calculator_v_f32);
    let dot_f32 = u.dot(v);
    let dot_calculator = 38 as f32;
    assert_eq!(dot_f32, dot_calculator);
    println!(
        "norm u : {} | norm from calculator online {}\nnorm v {} | norm v from calculator online {}\ndot {} | dot on calculator {}",
        norm_u_f32, norm_calculator_u_f32, norm_v_f32, norm_calculator_v_f32,dot_f32, dot_calculator
    );
    let dot_norm = norm_u_f32 * norm_v_f32;
    let dot_norm_calculator = norm_calculator_u_f32 * norm_calculator_v_f32;
    assert_eq!(dot_norm, dot_norm_calculator);
    println!(
        "dot product on norm {:?} | dot product with value of generator {}",
        (dot_norm),
        (norm_calculator_u_f32 * norm_calculator_v_f32)
    );
    let angle_cos_result = dot_f32 / dot_norm;
    let angle_cos_calculator = dot_calculator / dot_norm_calculator;
    assert_eq!(angle_cos_result, angle_cos_calculator);
    println!(
        "angle cos result : {} | angle cos result calculator : {}",
        angle_cos_result, angle_cos_calculator
    );
    println!("------------------------------------------------------");
    let u = Vector::from(vec![1., 1.]);
    let v = Vector::from(vec![1., 1.]);
    println!("test with\n{}\n{}", u, v);
    println!("{}", angle_cos(&u, &v));
    assert_eq!(angle_cos(&u, &v), 1.0000001);
    println!("------------------------------------------------------");
    let u = Vector::from(vec![4., 2.]);
    let v = Vector::from(vec![1., 1.]);
    println!("test with\n{}\n{}", u, v);
    println!("{}", angle_cos(&u, &v));
    assert_eq!(angle_cos(&u, &v), 0.94868326);
    println!("------------------------------------------------------");
    let u = Vector::from(vec![2., 4.]);
    let v = Vector::from(vec![1., 1.]);
    println!("test with\n{}\n{}", u, v);
    println!("{}", angle_cos(&u, &v));
    assert_eq!(angle_cos(&u, &v), 0.94868326);
    println!("------------------------------------------------------");
    let mut u = Vector::from(vec![-7., 3.]);
    let mut v = Vector::from(vec![6., 4.]);
    println!("test with\n{}\n{}", u, v);
    println!("{}", angle_cos(&u, &v));
    assert_eq!(angle_cos(&u, &v), -0.54626775);
    let norm_u_f32 = u.norm();
    let norm_calculator_u_f32 = 7.6157731058639 as f32;
    assert_eq!(norm_u_f32, norm_calculator_u_f32);
    let norm_v_f32 = v.norm();
    let norm_calculator_v_f32 = 7.211102550928 as f32;
    assert_eq!(norm_v_f32, norm_calculator_v_f32);
    let dot_f32 = u.dot(v);
    let dot_calculator = -30 as f32;
    assert_eq!(dot_f32, dot_calculator);
    println!(
        "norm u : {} | norm from calculator online {}\nnorm v {} | norm v from calculator online {}\ndot {} | dot on calculator {}",
        norm_u_f32, norm_calculator_u_f32, norm_v_f32, norm_calculator_v_f32,dot_f32, dot_calculator
    );
    let dot_norm = norm_u_f32 * norm_v_f32;
    let dot_norm_calculator = norm_calculator_u_f32 * norm_calculator_v_f32;
    assert_eq!(dot_norm, dot_norm_calculator);
    println!(
        "dot product on norm {:?} | dot product with value of generator {}",
        (dot_norm),
        (norm_calculator_u_f32 * norm_calculator_v_f32)
    );
    let angle_cos_result = dot_f32 / dot_norm;
    let angle_cos_calculator = dot_calculator / dot_norm_calculator;
    assert_eq!(angle_cos_result, angle_cos_calculator);
    println!(
        "angle cos result : {} | angle cos result calculator : {}",
        angle_cos_result, angle_cos_calculator
    );
    println!("------------------------------------------------------");
    let u = Vector::from(vec![4., 6.]);
    let v = Vector::from(vec![3., -7.]);
    println!("test with\n{}\n{}", u, v);
    println!("{}", angle_cos(&u, &v));
    assert_eq!(angle_cos(&u, &v), -0.54626775);

    // 1.0
    // 1.0
    // // 0.974631846
}
