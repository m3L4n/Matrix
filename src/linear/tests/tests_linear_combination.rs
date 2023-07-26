use crate::linear::linear_combination::linear_combination;
use crate::types::vector::Vector;
use std::vec;

pub fn tests_linear_combination() {
    println!("LINEAR COMBINATION");
    println!("---------------------------------");
    println!(
        " test with \n{:?}\n{:?}\n{:?}",
        vec![1., 0., 0.],
        vec![0., 1., 0.],
        vec![0., 0., 1.]
    );
    println!("coef :\n{:?}", [10., -2., 0.5]);
    println!(
        " result :\n{}",
        linear_combination(
            &[
                Vector::from(vec![1., 0., 0.]),
                Vector::from(vec![0., 1., 0.]),
                Vector::from(vec![0., 0., 1.])
            ],
            &[10., -2., 0.5]
        )
    );
    // assert_eq!(
    //     linear_combination(
    //         &[
    //             Vector::from(vec![1., 0., 0.]),
    //             Vector::from(vec![0., 1., 0.]),
    //             Vector::from(vec![0., 0., 1.])
    //         ],
    //         &[10., -2., 0.5]
    //     ),
    //     Vector::from(vec![10., -2., 0.5])
    // );
    println!("---------------------------------");
    println!(
        " test with \n{:?}\n{:?}\n . coef :{:?}",
        vec![1., 2., 3.],
        vec![0., 10., -100.],
        vec![10., -2.]
    );
    println!(
        "result :\n{}",
        linear_combination(
            &[
                Vector::from(vec![1., 2., 3.]),
                Vector::from(vec![0., 10., -100.])
            ],
            &[10., -2.]
        )
    );
    println!("---------------------------------");
    println!(" test with \n{:?}\ncoef :{:?}", vec![-42., 42.,], vec![-1.]);
    println!(
        "{}",
        linear_combination(&[Vector::from(vec![-42., 42.])], &[-1.])
    );
    println!("---------------------------------");
    println!(
        " test with \n{:?}\n{:?}\n{:?}",
        vec![-42.],
        vec![-42.],
        vec![-42.]
    );
    println!("coef :\n{:?}", [-1., 1., 0.]);
    println!(
        "{}",
        linear_combination(
            &[
                Vector::from(vec![-42.]),
                Vector::from(vec![-42.]),
                Vector::from(vec![-42.])
            ],
            &[-1., 1., 0.]
        )
    );
    println!("---------------------------------");
    println!(
        " test with \n{:?}\n{:?}",
        vec![-42., 100., -69.5],
        vec![1., 3., 5.],
    );
    println!("coef :\n{:?}", [1., -10.]);
    println!(
        "{}",
        linear_combination(
            &[
                Vector::from(vec![-42., 100., -69.5]),
                Vector::from(vec![1., 3., 5.])
            ],
            &[1., -10.,]
        )
    );
    let v1 = Vector::from(vec![2., 3.]);
    let v2 = Vector::from(vec![4., 8.]);
    let linear_combined = linear_combination::<f32>(&[v1, v2], &[4., 2.]);
    assert_eq!(linear_combined.elements, vec![16., 28.]);
    let v1 = Vector::from(vec![0., 2.]);
    let v2 = Vector::from(vec![-4., 2.]);
    let linear_combined = linear_combination::<f32>(&[v1, v2], &[-1., 2.]);
    assert_eq!(linear_combined.elements, vec![-8., 2.]);
}
