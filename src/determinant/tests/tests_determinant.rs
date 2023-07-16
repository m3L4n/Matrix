use crate::Matrix;
use std::ops::{Add, Mul, Sub};
pub fn tests_determinant() {
    println!("------------------------------------------------------");
    println!("DETERMINANT");
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![2., 0., 0.], vec![0., 2., 0.], vec![0., 0., 2.]]);
    println!(" U : {}", u);
    println!("result :{:?}", u.determinant());
    assert_eq!(u.determinant(), 8.0);
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![
        vec![2., 4., 5., 6.],
        vec![-1., 5., 6., 9.],
        vec![3., 7., 1., -6.],
        vec![4., 2., 3., 5.],
    ]);
    println!(" U : {}", u);
    println!("result :{:?}", u.determinant());
    // assert_eq!(u.determinant(), 1032.0);
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![8., 5., -2.], vec![4., 7., 20.], vec![7., 6., 1.]]);
    println!(" U : {}", u);
    println!("{}", u.determinant());
    assert_eq!(u.determinant(), -174.0);
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![
        vec![8., 5., -2., 4.],
        vec![4., 2.5, 20., 4.],
        vec![8., 5., 1., 4.],
        vec![28., -4., 17., 1.],
    ]);
    println!(" U : {}", u);
    println!("result :{:?}", u.determinant());
    assert_eq!(u.determinant(), 1032.0);
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![1., -1.], vec![-1., 1.]]);
    println!(" U : {}", u);
    println!("result :{:?}", u.determinant());
    assert_eq!(u.determinant(), 0.0);
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![-7., 5.], vec![4., 6.]]);
    println!(" U : {}", u);
    println!("result :{:?}", u.determinant());
    assert_eq!(u.determinant(), -62.0);
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![1., 0., 0.], vec![0., 1., 0.], vec![0., 0., 1.0]]);
    println!(" U : {}", u);
    println!("result :{:?}", u.determinant());
    assert_eq!(u.determinant(), 1.0);
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![1., 2.], vec![3., 4.]]);
    println!(" U : {}", u);
    println!("result :{:?}", u.determinant());
    assert_eq!(u.determinant(), -2.0);
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![0., 1.], vec![1., 0.]]);
    println!(" U : {}", u);
    println!("result :{:?}", u.determinant());
    assert_eq!(u.determinant(), -1.0);
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![1., 1.], vec![1., 1.]]);
    println!(" U : {}", u);
    println!("result :{:?}", u.determinant());
    assert_eq!(u.determinant(), 0.0);
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![2., 0.], vec![0., 2.]]);
    println!(" U : {}", u);
    println!("result :{:?}", u.determinant());
    assert_eq!(u.determinant(), 4.0);
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![1., 0.], vec![0., 1.]]);
    println!(" U : {}", u);
    println!("result :{:?}", u.determinant());
    assert_eq!(u.determinant(), 1.0);
    println!("------------------------------------------------------");
}
