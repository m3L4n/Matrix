use crate::Matrix;
pub fn tests_rank() {
    println!("------------------------------------------------------");
    println!("RANK");
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![8., 5., -2.], vec![4., 7., 20.], vec![7., 6., 1.]]);
    println!("test with : {}", u);
    println!("{}", u.rank());
    assert_eq!(u.rank(), 3);
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![0., 1.], vec![0., 1.]]);
    println!("test with : {}", u);
    println!("{}", u.rank());
    assert_eq!(u.rank(), 1);

    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![
        vec![1., 0., 2., 3.],
        vec![2., 0., 4., 6.],
        vec![0., 2., 2., 0.],
        vec![1., 2., 4., 3.],
    ]);
    println!("test with : {}", u);
    println!("{}", u.rank());
    assert_eq!(u.rank(), 2);
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![
        vec![1., 2., 0., 0.],
        vec![2., 4., 0., 0.],
        vec![-1., 2., 1., 1.],
    ]);
    println!("test with : {}", u);
    println!("{}", u.rank());
    assert_eq!(u.rank(), 2);
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![1., 0., 0.], vec![0., 1., 0.], vec![0., 0., 1.]]);
    println!("test with : {}", u);
    println!("{}", u.rank());
    assert_eq!(u.rank(), 3);
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![-7., 5.], vec![4., 6.]]);
    println!("test with : {}", u);
    println!("{}", u.rank());
    assert_eq!(u.rank(), 2);
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![1., 2.], vec![3., 4.]]);
    println!("test with : {}", u);
    println!("{}", u.rank());
    assert_eq!(u.rank(), 2);
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![0., 1.], vec![1., 0.]]);
    println!("test with : {}", u);
    println!("{}", u.rank());
    assert_eq!(u.rank(), 2);
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![1., 1.], vec![1., 1.]]);
    println!("test with : {}", u);
    println!("{}", u.rank());
    assert_eq!(u.rank(), 1);
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![2., 0.], vec![0., 2.]]);
    println!("test with : {}", u);
    println!("{}", u.rank());
    assert_eq!(u.rank(), 2);
    let mut u = Matrix::from(vec![vec![1., 2., 3.], vec![4., 5., 6.], vec![7., 8., 9.]]);
    println!("test with : {}", u);
    println!("{}", u.rank());
    assert_eq!(u.rank(), 2);
    // [1.0, 0.625, 0.0, 0.0, -12.1666667]
    // [0.0, 0.0, 1.0, 0.0, -3.6666667]
    // [0.0, 0.0, 0.0, 1.0, 29.5 ]
}
