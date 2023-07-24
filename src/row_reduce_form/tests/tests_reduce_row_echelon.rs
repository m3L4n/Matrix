use crate::Matrix;
use std::vec;

pub fn tests_row_echelon_form() {
    println!("------------------------------------------------------");
    println!("ROW ECHELON FORM ");
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![
        vec![8., 5., -2., 4., 28.],
        vec![4., 2.5, 20., 4., -4.],
        vec![8., 5., 1., 4., 17.],
    ]);
    println!(" U : {}", u);
    println!("result: {}", u.row_echelon());
    assert_eq!(
        u.row_echelon().elements,
        vec![
            vec![1.0, 0.625, 0.0, 0.0, -12.166668],
            vec![0.0, 0.0, 1.0, 0.0, -3.666666666666667],
            vec![0.0, 0.0, 0.0, 1.0, 29.500000000000004]
        ]
    );
    println!("------------------------------------------------------");
    let mut u: Matrix<f32> =
        Matrix::from(vec![vec![1., 0., 0.], vec![0., 1., 0.], vec![0., 0., 1.]]);
    println!(" U : {}", u);
    println!("result: {}", u.row_echelon());
    assert_eq!(
        u.row_echelon().elements,
        vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0],
            vec![0.0, 0.0, 1.0]
        ]
    );
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![1., 2.], vec![3., 4.]]);
    println!(" U : {}", u);
    // println!("result: {}", u.row_echelon());
    assert_eq!(
        u.row_echelon().elements,
        vec![vec![1.0, 0.0], vec![0.0, 1.0],]
    );
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![1., 2.], vec![2., 4.]]);
    println!(" U : {}", u);
    println!("result: {}", u.row_echelon());
    assert_eq!(
        u.row_echelon().elements,
        vec![vec![1.0, 2.0], vec![0.0, 0.0],]
    );
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![4., 2.], vec![2., 1.]]);
    println!(" U : {}", u);
    println!("result: {}", u.row_echelon());
    assert_eq!(
        u.row_echelon().elements,
        vec![vec![1.0, 0.5], vec![0.0, 0.0],]
    );
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![-7., 2.], vec![4., 8.]]);
    println!(" U : {}", u);
    println!("result: {}", u.row_echelon());
    assert_eq!(
        u.row_echelon().elements,
        vec![vec![1.0, 0.0], vec![0.0, 1.0],]
    );
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![1., 2.], vec![4., 8.]]);
    println!(" U : {}", u);
    println!("result: {}", u.row_echelon());
    assert_eq!(
        u.row_echelon().elements,
        vec![vec![1.0, 2.0], vec![0.0, 0.0],]
    );
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![
        vec![2., -1., 0.],
        vec![-1., 2., -1.],
        vec![0., -1., 2.],
    ]);
    println!(" U : {}", u);
    println!("result: {}", u.row_echelon());
    assert_eq!(
        u.row_echelon().elements,
        vec![vec![1.0, 0.0, 0.], vec![0.0, 1.0, 0.], vec![0.0, 0.0, 1.]]
    );
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![1., 2., 3.], vec![4., 5., 6.], vec![7., 8., 9.]]);
    println!(" U : {}", u);
    println!("result: {}", u.row_echelon());
    assert_eq!(
        u.row_echelon().elements,
        vec![
            vec![1.0, 0.0, -1.0000004],
            vec![0.0, 1.0, 2.0000002],
            vec![0.0, 0.0, 0.00000035762787]
        ]
    );
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![8., 5., -2.], vec![4., 7., 20.], vec![7., 6., 1.]]);
    let result = u.row_echelon();
    assert_eq!(result.elements[0], Vec::from(vec![1., 0., 0.]));
    assert_eq!(result.elements[1], Vec::from(vec![0., 1., 0.]));
    assert_eq!(result.elements[2], Vec::from(vec![0., 0., 1.]));
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![
        vec![1., -1., 2.],
        vec![3., 2., 1.],
        vec![2., -3., -2.],
    ]);
    println!(" U : {}", u);
    let result = u.row_echelon();
    println!("result: {}", result);
    assert_eq!(result.elements[0], Vec::from([1., 0., 0.]));
    assert_eq!(result.elements[1], Vec::from([0., 1., 0.]));
    assert_eq!(result.elements[2], Vec::from([0., 0., 1.]));
}
