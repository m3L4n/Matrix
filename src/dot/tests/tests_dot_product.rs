use crate::types::vector::Vector;
use std::vec;

pub fn tests_dot_product() {
    println!("------------------------------------------------------");
    println!("DOT PRODUCT");
    println!("------------------------------------------------------");
    let u = Vector::from(vec![0., 0.]);
    let v = Vector::from(vec![1., 1.]);
    println!("test with\n{}\n{}", u, v);
    println!("{}", u.dot(v.clone()));
    assert_eq!(u.dot(v), 0.0);
    // 0.0
    println!("------------------------------------------------------");
    let u = Vector::from(vec![1., 1.]);
    let v = Vector::from(vec![1., 1.]);
    println!("test with\n{}\n{}", u, v);
    println!("{}", u.dot(v.clone()));
    assert_eq!(u.dot(v), 2.0);
    // 2.0
    println!("------------------------------------------------------");
    let u = Vector::from(vec![-1., 6.]);
    let v = Vector::from(vec![3., 2.]);
    println!("test with\n{}\n{}", u, v);
    println!("{}", u.dot(v.clone()));
    assert_eq!(u.dot(v), 9.0);
    println!("------------------------------------------------------");
    let u = Vector::from(vec![0., 0.]);
    let v = Vector::from(vec![1., 0.]);
    println!("test with\n{}\n{}", u, v);
    println!("{}", u.dot(v.clone()));
    assert_eq!(u.dot(v), 0.0);
    println!("------------------------------------------------------");
    let u = Vector::from(vec![4., 2.]);
    let v = Vector::from(vec![2., 1.]);
    println!("test with\n{}\n{}", u, v);
    println!("{}", u.dot(v.clone()));
    assert_eq!(u.dot(v), 10.0);
}
