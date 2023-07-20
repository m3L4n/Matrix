use crate::Matrix;
pub fn tests_trace() {
    println!("------------------------------------------------------");
    println!("TRACE ");
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![1., 0.], vec![0., 1.]]);
    println!("test with {}", u);
    println!("result {}", u.trace());
    assert_eq!(u.trace(), 2.);
    println!("------------------------------------------------------");
    // 2.0
    let mut u = Matrix::from(vec![vec![2., -5., 0.], vec![4., 3., 7.], vec![-2., 3., 4.]]);
    println!("test with {}", u);
    println!("result {}", u.trace());
    assert_eq!(u.trace(), 9.);
    println!("------------------------------------------------------");
    // 9.0
    let mut u = Matrix::from(vec![
        vec![-2., -8., 4.],
        vec![1., -23., 4.],
        vec![0., 6., 4.],
    ]);
    println!("test with {}", u);
    println!("result {}", u.trace());
    assert_eq!(u.trace(), -21.);
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![0., 0.], vec![0., 0.]]);
    println!("test with {}", u);
    println!("result {}", u.trace());
    assert_eq!(u.trace(), 0.);
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![1., 0.], vec![0., 1.]]);
    println!("test with {}", u);
    println!("result {}", u.trace());
    assert_eq!(u.trace(), 2.);
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![1., 2.], vec![3., 4.]]);
    println!("test with {}", u);
    println!("result {}", u.trace());
    assert_eq!(u.trace(), 5.);
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![8., -7.], vec![4., 2.]]);
    println!("test with {}", u);
    println!("result {}", u.trace());
    assert_eq!(u.trace(), 10.);
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![1., 0., 0.], vec![0., 1., 0.], vec![0., 0., 1.]]);
    println!("test with {}", u);
    println!("result {}", u.trace());
    assert_eq!(u.trace(), 3.);
    println!("------------------------------------------------------");
}
