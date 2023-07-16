use crate::cross_product::cross_product::cross_product;
use crate::Matrix;
use crate::Vector;
use std::vec;
pub fn tests_cross_product() {
    println!("------------------------------------------------------");
    println!("CROSS PRODUCT");
    println!("------------------------------------------------------");
    let u = Vector::from(vec![0., 0., 1.]);
    let v = Vector::from(vec![1., 0., 0.]);
    println!("test with {}\n {}", u, v);
    println!("{}", cross_product(&u, &v));
    assert_eq!(cross_product(&u, &v).elements, vec![0., 1., 0.]);
    println!("------------------------------------------------------");
    // // [0.]
    // // [1.]
    // // [0.]
    let u = Vector::from(vec![1., 2., 3.]);
    let v = Vector::from(vec![4., 5., 6.]);
    println!("test with {}\n {}", u, v);
    println!("{}", cross_product(&u, &v));
    assert_eq!(cross_product(&u, &v).elements, vec![-3., 6., -3.]);
    println!("------------------------------------------------------");
    // // [-3.]
    // // [6.]
    // // [-3.]
    let u = Vector::from(vec![4., 2., -3.]);
    let v = Vector::from(vec![-2., -5., 16.]);
    println!("test with {}\n {}", u, v);
    println!("resultat: {}", cross_product(&u, &v));
    assert_eq!(cross_product(&u, &v).elements, vec![17., -58., -16.]);
    println!("------------------------------------------------------");
    let u = Vector::from(vec![0., 0., 0.]);
    let v = Vector::from(vec![0., 0., 0.]);
    println!("test with {}\n {}", u, v);
    println!("resultat: {}", cross_product(&u, &v));
    assert_eq!(cross_product(&u, &v).elements, vec![0., 0., 0.]);
    println!("------------------------------------------------------");
    let u = Vector::from(vec![1., 0., 0.]);
    let v = Vector::from(vec![0., 0., 0.]);
    println!("test with {}\n {}", u, v);
    println!("resultat: {}", cross_product(&u, &v));
    assert_eq!(cross_product(&u, &v).elements, vec![0., 0., 0.]);
    println!("------------------------------------------------------");
    let u = Vector::from(vec![1., 0., 0.]);
    let v = Vector::from(vec![0., 1., 0.]);
    println!("test with {}\n {}", u, v);
    println!("resultat: {}", cross_product(&u, &v));
    assert_eq!(cross_product(&u, &v).elements, vec![0., 0., 1.]);
    println!("------------------------------------------------------");
    let u = Vector::from(vec![8., 7., -4.]);
    let v = Vector::from(vec![3., 2., 1.]);
    println!("test with {}\n {}", u, v);
    println!("resultat: {}", cross_product(&u, &v));
    assert_eq!(cross_product(&u, &v).elements, vec![15., -20., -5.]);
    println!("------------------------------------------------------");
    let u = Vector::from(vec![1., 1., 1.]);
    let v = Vector::from(vec![1., 1., 1.]);
    println!("test with {}\n {}", u, v);
    println!("resultat: {}", cross_product(&u, &v));
    assert_eq!(cross_product(&u, &v).elements, vec![0., 0., 0.]);
    println!("------------------------------------------------------");
    let u = Vector::from(vec![1., 1., 1.]);
    let v = Vector::from(vec![0., 0., 0.]);
    println!("test with {}\n {}", u, v);
    println!("resultat: {}", cross_product(&u, &v));
    assert_eq!(cross_product(&u, &v).elements, vec![0., 0., 0.]);
    println!("------------------------------------------------------");
    // // [17.]
    // // [-58.]
    // // [-16.]
}
