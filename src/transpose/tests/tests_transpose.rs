use crate::Matrix;
use crate::Vector;
pub fn tests_transpose() {
    println!("------------------------------------------------------");
    println!("TRANSPOSE ");
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![1., -2., 3.]]);
    println!("test with :{}", u);
    println!("result {}", u.transpose());
    assert_eq!(u.transpose().elements, vec![vec![1.], vec![-2.], vec![3.]]);
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![3., -1.], vec![0., 2.], vec![1., -4.]]);
    println!("test with :{}", u);
    println!("result {}", u.transpose());
    assert_eq!(
        u.transpose().elements,
        vec![vec![3., 0., 1.], vec![-1., 2., -4.]]
    );
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![-1., 3.], vec![0., 2.]]);
    println!("test with :{}", u);
    println!("result {}", u.transpose());
    assert_eq!(u.transpose().elements, vec![vec![-1., 0.], vec![3., 2.]]);
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![
        vec![1., 0., -2.],
        vec![-4., 1., 7.],
        vec![5., -3., 2.],
    ]);
    println!("test with :{}", u);
    println!("result {}", u.transpose());
    assert_eq!(
        u.transpose().elements,
        vec![vec![1., -4., 5.], vec![0., 1., -3.], vec![-2., 7., 2.]]
    );
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![1., 0., 0.], vec![0., -2., 0.], vec![0., 0., 5.]]);
    println!("test with :{}", u);
    println!("result {}", u.transpose());
    assert_eq!(
        u.transpose().elements,
        vec![vec![1., 0., 0.], vec![0., -2., 0.], vec![0., 0., 5.]]
    );
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![1., 2.], vec![3., 4.], vec![5., 6.]]);
    println!("test with :{}", u);
    println!("result {}", u.transpose());
    assert_eq!(
        u.transpose().elements,
        vec![vec![1., 3., 5.], vec![2., 4., 6.]]
    );
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![1., 0.], vec![0., 1.]]);
    println!("test with :{}", u);
    println!("result {}", u.transpose());
    assert_eq!(u.transpose().elements, vec![vec![1., 0.], vec![0., 1.]]);
    println!("------------------------------------------------------");
}
