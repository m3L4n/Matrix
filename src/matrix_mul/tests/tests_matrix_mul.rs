use crate::Matrix;
use crate::Vector;

pub fn tests_linear_map() {
    println!("------------------------------------------------------");
    println!("MATRIX MULTIPLICATION");
    println!("------------------------------------------------------");
    println!("MATRIX MULTIPLICATION WITH VECTOR");
    println!("------------------------------------------------------");

    let mut u = Matrix::from(vec![vec![1., 0.], vec![0., 1.]]);
    let v = Vector::from(vec![4., 2.]);
    println!("test with\n{}\n{}", u, v);
    println!("resultat : {}", u.mul_vec(v.clone()));
    assert_eq!(u.mul_vec(v).elements, vec![4., 2.]);
    println!("------------------------------------------------------");
    // // [4.]
    // // [2.]
    let mut u = Matrix::from(vec![vec![2., 0.], vec![0., 2.]]);
    let v = Vector::from(vec![4., 2.]);
    println!("test with\n{}\n{}", u, v);
    println!("resultat : {}", u.mul_vec(v.clone()));
    assert_eq!(u.mul_vec(v).elements, vec![8., 4.]);
    println!("------------------------------------------------------");
    // // [8.]
    // // [4.]
    let mut u = Matrix::from(vec![vec![2., -2.], vec![-2., 2.]]);
    let v = Vector::from(vec![4., 2.]);
    println!("test with\n{}\n{}", u, v);
    println!("resultat : {}", u.mul_vec(v.clone()));
    assert_eq!(u.mul_vec(v).elements, vec![4., -4.]);
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![0., 0.], vec![0., 0.]]);
    let v = Vector::from(vec![10., 2.]);
    println!("test with\n{}\n{}", u, v);
    println!("resultat : {}", u.mul_vec(v.clone()));
    assert_eq!(u.mul_vec(v).elements, vec![0., -0.]);
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![1., 0.], vec![0., 1.]]);
    let v = Vector::from(vec![10., 2.]);
    println!("test with\n{}\n{}", u, v);
    println!("resultat : {}", u.mul_vec(v.clone()));
    assert_eq!(u.mul_vec(v).elements, vec![10., 2.]);
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![1., 1.], vec![1., 1.]]);
    let v = Vector::from(vec![4., 2.]);
    println!("test with\n{}\n{}", u, v);
    println!("resultat : {}", u.mul_vec(v.clone()));
    assert_eq!(u.mul_vec(v).elements, vec![6., 6.]);
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![2., 0.], vec![0., 2.]]);
    let v = Vector::from(vec![2., 1.]);
    println!("test with\n{}\n{}", u, v);
    println!("resultat : {}", u.mul_vec(v.clone()));
    assert_eq!(u.mul_vec(v).elements, vec![4., 2.]);
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![0.5, 0.], vec![0., 0.5]]);
    let v = Vector::from(vec![4., 2.]);
    println!("test with\n{}\n{}", u, v);
    println!("resultat : {}", u.mul_vec(v.clone()));
    assert_eq!(u.mul_vec(v).elements, vec![2., 1.]);
    println!("------------------------------------------------------");
    // // [-4.]
}
pub fn tests_matrix_mul() {
    println!("------------------------------------------------------");
    println!("MATRIX MULTIPLICATION WITH MATRIX");
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![3., -5.], vec![6., 8.]]);

    let v = Matrix::from(vec![vec![2., 1.], vec![4., 2.]]);
    println!("test with\n{}\n{}", u, v);
    println!("resultat : {}", u.mul_mat(v.clone()));
    assert_eq!(u.mul_mat(v).elements, vec![vec![-14., -7.], vec![44., 22.]]);
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![3.], vec![6.]]);
    let v = Matrix::from(vec![vec![2.]]);
    println!("test with\n{}\n{}", u, v);
    println!("resultat : {}", u.mul_mat(v.clone()));
    assert_eq!(u.mul_mat(v).elements, vec![vec![6.], vec![12.]]);
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![1., 0.], vec![0., 1.]]);
    let v = Matrix::from(vec![vec![2., 1.], vec![4., 2.]]);
    println!("test with\n{}\n{}", u, v);
    println!("resultat : {}", u.mul_mat(v.clone()));
    assert_eq!(u.mul_mat(v).elements, vec![vec![2., 1.], vec![4., 2.]]);
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![1., 10.], vec![-0.5, 1.]]);
    let v = Matrix::from(vec![vec![4., 4.], vec![4., 2.]]);
    println!("test with\n{}\n{}", u, v);
    println!("resultat : {}", u.mul_mat(v.clone()));
    assert_eq!(u.mul_mat(v).elements, vec![vec![44., 24.], vec![2., 0.]]);
    println!("------------------------------------------------------");

    // println!("{}", u.mul_mat(v));
}
