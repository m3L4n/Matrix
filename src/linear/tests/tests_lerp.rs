use std::vec;

use crate::linear::lerp::lerp;
use crate::Matrix;
use crate::Vector;
pub fn tests_lerp() {
    println!("------------------------------------------------------");
    println!("Linear interpolation");
    println!("------------------------------------------------------");
    let u = Vector {
        elements: vec![2.1, 1.0],
    };
    let v = Vector {
        elements: vec![4.0, 2.0],
    };
    let t = 0.3;
    println!("test with \n{}\n{}\nt {}", u, v, t);
    println!("{}", lerp(u, v, t));
    println!("------------------------------------------------------");
    println!("test with \n{}\n{}\nt {}", 0., 1., 0.);
    println!("{}", lerp(0., 1., 0.));
    println!("------------------------------------------------------");
    // 0.0
    println!("test with \n{}\n{}\nt {}", 0., 1., 1.);
    println!("{}", lerp(0., 1., 1.));
    println!("------------------------------------------------------");
    // 1.0
    println!("test with \n{}\n{}\nt {}", 0., 1., 0.5);
    println!("{}", lerp(0., 1., 0.5));
    println!("------------------------------------------------------");
    // 0.5
    println!("test with \n{}\n{}\nt {}", 21., 42., 0.3);
    println!("{}", lerp(21., 42., 0.3));
    println!("------------------------------------------------------");
    println!(
        "test with \n{:?}\n{:?}\nt {:?}",
        vec![-42., 42.],
        vec![42., -42.],
        0.5
    );
    println!(
        "{}",
        lerp(
            Vector::from(vec![-42., 42.]),
            Vector::from(vec![42., -42.]),
            0.5
        )
    );
    println!("------------------------------------------------------");
    println!(
        "test with\n{}\n{}\nt{}",
        Matrix::from(vec![vec![2., 1.], vec![3., 4.]]),
        Matrix::from(vec![vec![20., 10.], vec![30., 40.]]),
        0.5
    );
    println!(
        "{}",
        lerp(
            Matrix::from(vec![vec![2., 1.], vec![3., 4.]]),
            Matrix::from(vec![vec![20., 10.], vec![30., 40.]]),
            0.5
        )
    );
}
