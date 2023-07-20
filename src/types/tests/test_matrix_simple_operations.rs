use std::vec;

use crate::types::matrix::Matrix;
pub fn test_matrix_simple_operations_add() {
    println!("------------------------------------------------------");
    println!(" MATRIX");
    println!("------------------------------------------------------");
    println!("\x1B[32m                 ADD \x1B[0m");
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![0., 0.], vec![0., 0.]]);
    let v = Matrix::from(vec![vec![1., 0.], vec![-1., 0.]]);
    let mut u1 = Matrix::from(vec![vec![21.1, 21.2], vec![21.3, 21.4]]);
    let v1 = Matrix::from(vec![vec![21.1, 21.2], vec![21.3, 21.4]]);
    let mut z = Matrix::from(vec![vec![10.2, 19.3], vec![230.1, 344.0, 2010.0]]);
    let x = Matrix::from(vec![vec![100.2, 190.3], vec![2300.1, 3440.0, 20100.0]]);
    println!("vector 1 : \n{}vector 2 : \n{}", u, v);
    u.add(&v);
    println!(" resuslt v1 + v2 {}", u);
    assert_eq!(
        u.elements,
        vec![vec![0. + 1., 0.0], vec![0. + -1., 0. + 0.]]
    );
    println!("------------------------------------------------------");
    println!("vector 1 : \n{}vector 2 : \n{}", u1, v1);
    u1.add(&v1);
    println!(" resuslt v1 + v2 {}", u1);
    println!("------------------------------------------------------");
    println!("vector 1 : \n{}vector 2 : \n{}", z, x);
    z.add(&x);
    println!(" resuslt v1 + v2 {}", z);
    assert_eq!(
        z.elements,
        vec![
            vec![10.2 + 100.2, 19.3 + 190.3],
            vec![230.1 + 2300.1, 344.0 + 3440.0, 2010.0 + 20100.0]
        ]
    )
}
pub fn test_matrix_simple_operations_sub() {
    println!("------------------------------------------------------");
    println!(" MATRIX");
    println!("------------------------------------------------------");
    println!("\x1B[32m                 SUB \x1B[0m");
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![0., 0.], vec![0., 0.]]);
    let v = Matrix::from(vec![vec![1., 0.], vec![-1., 0.]]);
    let mut u1 = Matrix::from(vec![vec![21.1, 21.2], vec![21.3, 21.4]]);
    let v1 = Matrix::from(vec![vec![21.1, 21.2], vec![21.3, 21.4]]);
    let mut z = Matrix::from(vec![vec![10.2, 19.3], vec![230.1, 344.0, 2010.0]]);
    let x = Matrix::from(vec![vec![100.2, 190.3], vec![2300.1, 3440.0, 20100.0]]);
    println!("vector 1 : \n{}vector 2 : \n{}", u, v);
    u.sub(&v);
    println!(" resuslt v1 - v2 {}", u);
    assert_eq!(
        u.elements,
        vec![vec![0. - 1., 0.0 - 0.0], vec![0. - -1., 0. - 0.]]
    );
    println!("------------------------------------------------------");
    println!("vector 1 : \n{}vector 2 : \n{}", u1, v1);
    u1.sub(&v1);
    assert_eq!(
        u1.elements,
        vec![
            vec![(21.1 - 21.1), 21.2 - 21.2],
            vec![21.3 - 21.3, 21.4 - 21.4]
        ]
    );
    println!(" resuslt v1 - v2 {}", u1);
    println!("------------------------------------------------------");
    println!("vector 1 : \n{}vector 2 : \n{}", z, x);
    z.sub(&x);
    println!(" resuslt v1 - v2 {}", z);
    assert_eq!(
        z.elements,
        vec![
            vec![10.2 - 100.2, 19.3 - 190.3],
            vec![230.1 - 2300.1, 344.0 - 3440.0, 2010.0 - 20100.0]
        ]
    )
}
pub fn test_matrix_simple_operations_scl() {
    println!("------------------------------------------------------");
    println!("\x1B[32m                 SCL \x1B[0m");
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![1., 0.], vec![3., 4.]]);
    let mut v = Matrix::from(vec![vec![7., 4.], vec![-2., 2.]]);
    let mut u1 = Matrix::from(vec![vec![21., 21.], vec![21., 21.]]);
    let mut z = Matrix::from(vec![vec![21., 21.], vec![21., 21.]]);

    println!("\x1B[32m\nimpression de u {}\x1B[0m", u);
    u.scl(0.0);
    println!("resultat de U * 0.0 {}", u);
    assert_eq!(
        u.elements,
        vec![vec![1. * 0., 0. * 0.0], vec![3. * 0.0, 4. * 0.0]]
    );

    println!("\x1B[32mimpression de u {} \x1B[0m", v);
    v.scl(-2.);
    assert_eq!(
        v.elements,
        vec![vec![7. * -2., 4. * -2.0], vec![-2. * -2.0, 2. * -2.0]]
    );
    println!("resultat de u * 9. {}", v);
    println!("\x1B[32m\nimpression de v {}  \x1B[0m", u1);
    u1.scl(0.);
    assert_eq!(
        u1.elements,
        vec![vec![21. * 0., 21. * 0.0], vec![21. * 0.0, 21. * 0.0]]
    );
    println!("resultat de v * 0. {}", u1);
    println!("\x1B[32m\nimpression de u1 {} \x1B[0m", z);
    z.scl(0.5);
    println!("resultat de U1 * 0. {}", z);

    println!("------------------------------------------------------");
}
