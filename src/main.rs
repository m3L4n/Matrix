mod cosine;
mod cross_product;
mod dot;
mod lerp;
mod linear_combination;
mod matrix;
mod matrix_multiplication;
mod norm;
mod row_echelon_form;
mod trace;
mod transpose;
mod vector;
use crate::cosine::angle_cos;
use crate::cross_product::cross_product;
use crate::lerp::lerp;
use crate::linear_combination::linear_combination;
use crate::matrix::Matrix;
use crate::vector::Vector;
fn main() {
    println!("------------------------------------------------------");
    println!(" VECTOR");
    println!("------------------------------------------------------");
    println!("\x1B[32m                 ADD \x1B[0m");
    println!("------------------------------------------------------");
    let mut u = Vector::from(vec![2., 3.]);
    let mut v = Vector::from(vec![5., 7.]);
    let mut u1 = Vector::from(vec![9., -2.]);
    let mut v1 = Vector::from(vec![19., 1.]);
    println!("\x1B[32mimpression de u {}  v {}\x1B[0m", u, v);
    u.add(&v);
    println!("resultat de u + u1 {}", u);
    println!("\x1B[32m\nimpression de v {}  u1 {}\x1B[0m", v, u1);
    v.add(&u1);
    println!("resultat de U + V {}", v);
    println!("\x1B[32m\nimpression de u1 {}  uv1 {}\x1B[0m", u1, v1);
    u1.add(&v1);
    println!("resultat de U1 + V1 {}", u1);
    println!("------------------------------------------------------");
    println!("\x1B[32m                 SUB \x1B[0m");
    println!("------------------------------------------------------");
    // [7.0]
    // [10.0]
    u = Vector::from(vec![2., 3.]);
    v = Vector::from(vec![5., 7.]);
    u1 = Vector::from(vec![9., -2.]);
    v1 = Vector::from(vec![19., 1.]);
    println!("\x1B[32m\nimpression de u {}  v {}\x1B[0m", u, v);
    u.sub(&v);
    println!("resultat de U - V {}", u);
    println!("\x1B[32mimpression de u {}  v {}\x1B[0m", u, v);
    u.sub(&v);
    println!("resultat de u - u1 {}", u);
    println!("\x1B[32m\nimpression de v {}  u1 {}\x1B[0m", v, u1);
    v.sub(&u1);
    println!("resultat de U - V {}", v);
    println!("\x1B[32m\nimpression de u1 {}  uv1 {}\x1B[0m", u1, v1);
    u1.sub(&v1);
    println!("resultat de U1 - V1 {}", u1);
    println!("------------------------------------------------------");
    println!("\x1B[32m                 SCL \x1B[0m");
    println!("------------------------------------------------------");
    u = Vector::from(vec![2., 3.]);
    v = Vector::from(vec![5., 7.]);
    u1 = Vector::from(vec![9., -2.]);
    println!("\x1B[32m\nimpression de u {}\x1B[0m", u);
    u.scl(2.0);
    println!("resultat de U * 2.0 {}", u);
    println!("\x1B[32mimpression de u {} \x1B[0m", u);
    u.scl(9.);
    println!("resultat de u * 9. {}", u);
    println!("\x1B[32m\nimpression de v {}  \x1B[0m", v);
    v.scl(0.);
    println!("resultat de v * 0. {}", v);
    println!("\x1B[32m\nimpression de u1 {} \x1B[0m", u1);
    u1.scl(-2.);
    println!("resultat de U1 * -2. {}", u1);
    println!("------------------------------------------------------");
    println!(" MATRIX");
    println!("------------------------------------------------------");
    println!("\x1B[32m                 ADD \x1B[0m");
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![1., 2.], vec![3., 4.]]);
    let mut v = Matrix::from(vec![vec![7., 4.], vec![-2., 2.]]);
    let mut u1 = Matrix::from(vec![vec![3., 2.], vec![5., 2.]]);
    let v1 = Matrix::from(vec![vec![3., 99.], vec![-4., 0.]]);
    println!("\x1B[32mimpression de u {}  v {}\x1B[0m", u, v);
    u.add(&v);
    println!("resultat de u + u1 {}", u);
    println!("\x1B[32m\nimpression de v {}  u1 {}\x1B[0m", v, u1);
    v.add(&u1);
    println!("resultat de U + V {}", v);
    println!("\x1B[32m\nimpression de u1 {}  uv1 {}\x1B[0m", u1, v1);
    u1.add(&v1);
    println!("resultat de U1 + V1 {}", u1);
    println!("------------------------------------------------------");
    println!("\x1B[32m                 SUB \x1B[0m");
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![1., 2.], vec![3., 4.]]);
    let mut v = Matrix::from(vec![vec![7., 4.], vec![-2., 2.]]);
    let mut u1 = Matrix::from(vec![vec![3., 2.], vec![5., 2.]]);
    let v1 = Matrix::from(vec![vec![3., 99.], vec![-4., 0.]]);
    println!("\x1B[32m\nimpression de u {}  v {}\x1B[0m", u, v);
    u.sub(&v);
    println!("resultat de U - V {}", u);
    println!("\x1B[32mimpression de u {}  v {}\x1B[0m", u, v);
    u.sub(&v);
    println!("resultat de u - u1 {}", u);
    println!("\x1B[32m\nimpression de v {}  u1 {}\x1B[0m", v, u1);
    v.sub(&u1);
    println!("resultat de U - V {}", v);
    println!("\x1B[32m\nimpression de u1 {}  uv1 {}\x1B[0m", u1, v1);
    u1.sub(&v1);
    println!("resultat de U1 - V1 {}", u1);
    println!("------------------------------------------------------");
    println!("\x1B[32m                 SCL \x1B[0m");
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![1., 2.], vec![3., 4.]]);
    let mut v = Matrix::from(vec![vec![7., 4.], vec![-2., 2.]]);
    let mut u1 = Matrix::from(vec![vec![3., 2.], vec![5., 2.]]);

    println!("\x1B[32m\nimpression de u {}\x1B[0m", u);
    u.scl(2.0);
    println!("resultat de U * 2.0 {}", u);
    println!("\x1B[32mimpression de u {} \x1B[0m", u);
    u.scl(9.);
    println!("resultat de u * 9. {}", u);
    println!("\x1B[32m\nimpression de v {}  \x1B[0m", v);
    v.scl(0.);
    println!("resultat de v * 0. {}", v);
    println!("\x1B[32m\nimpression de u1 {} \x1B[0m", u1);
    u1.scl(-2.);
    println!("resultat de U1 * -2. {}", u1);
    println!("------------------------------------------------------");
    println!("LINEAR COMBINATION");
    println!(
        "{}",
        linear_combination(
            &[
                Vector::from(vec![1., 0., 0.]),
                Vector::from(vec![0., 1., 0.]),
                Vector::from(vec![0., 0., 1.])
            ],
            &[10., -2., 0.5]
        )
    );

    println!(
        "{}",
        linear_combination(
            &[
                Vector::from(vec![1., 2., 3.]),
                Vector::from(vec![0., 10., -100.])
            ],
            &[10., -2.]
        )
    );
    println!("------------------------------------------------------");
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
    println!("{}", lerp(u, v, t));
    println!("{}", lerp(0., 1., 0.));
    // 0.0
    println!("{}", lerp(0., 1., 1.));
    // 1.0
    println!("{}", lerp(0., 1., 0.5));
    // 0.5
    println!("{}", lerp(21., 42., 0.3));
    // //     // Tests avec des Matrix<f32>
    let matrix_a = Matrix::from(vec![vec![2., 1.], vec![3., 4.]]);
    let matrix_b = Matrix::from(vec![vec![20., 10.], vec![30., 40.]]);
    println!("{}", lerp(matrix_a.clone(), matrix_b.clone(), 0.5));
    println!("------------------------------------------------------");
    println!("DOT PRODUCT");
    println!("------------------------------------------------------");
    let mut u = Vector::from(vec![0., 0.]);
    let v = Vector::from(vec![1., 1.]);
    println!("{}", u.dot(v));
    // 0.0
    let mut u = Vector::from(vec![1., 1.]);
    let v = Vector::from(vec![1., 1.]);
    println!("{}", u.dot(v));
    // 2.0
    let mut u = Vector::from(vec![-1., 6.]);
    let v = Vector::from(vec![3., 2.]);
    println!("{}", u.dot(v));
    println!("------------------------------------------------------");
    println!("NORM");
    println!("------------------------------------------------------");
    let mut u = Vector::from(vec![0., 0., 0.]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 0.0, 0.0, 0.0
    let mut u = Vector::from(vec![1., 2., 3.]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 6.0, 3.74165738, 3.0
    let mut u = Vector::from(vec![-1., -2.]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    let mut u = Vector::from(vec![4., 2.]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    let mut u = Vector::from(vec![2., 1.]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    let mut u = Vector::from(vec![-4., -2.]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 3.0, 2.236067977, 2.0
    println!("------------------------------------------------------");
    println!("COSINE");
    println!("------------------------------------------------------");
    let mut u = Vector::from(vec![1., 2., 3.]);
    let mut v = Vector::from(vec![4., 5., 6.]);
    println!("{}", angle_cos(&u, &v));
    let u = Vector::from(vec![1., 0.]);
    let v = Vector::from(vec![1., 0.]);
    println!("{}", angle_cos(&u, &v));
    // 1.0
    let u = Vector::from(vec![1., 0.]);
    let v = Vector::from(vec![0., 1.]);
    println!("{}", angle_cos(&u, &v));
    // 0.0
    let u = Vector::from(vec![-1., 1.]);
    let v = Vector::from(vec![1., -1.]);
    println!("{}", angle_cos(&u, &v));
    // -1.0
    let u = Vector::from(vec![2., 1.]);
    let v = Vector::from(vec![4., 2.]);
    println!("{}", angle_cos(&u, &v));
    // 1.0
    // 0.974631846
    println!("------------------------------------------------------");
    println!("COSINE");
    println!("------------------------------------------------------");
    let u = Vector::from(vec![0., 0., 1.]);
    let v = Vector::from(vec![1., 0., 0.]);
    println!("{}", cross_product(&u, &v));
    // [0.]
    // [1.]
    // [0.]
    let u = Vector::from(vec![1., 2., 3.]);
    let v = Vector::from(vec![4., 5., 6.]);
    println!("{}", cross_product(&u, &v));
    // [-3.]
    // [6.]
    // [-3.]
    let u = Vector::from(vec![4., 2., -3.]);
    let v = Vector::from(vec![-2., -5., 16.]);
    println!("{}", cross_product(&u, &v));
    // [17.]
    // [-58.]
    // [-16.]
    println!("------------------------------------------------------");
    println!("MATRIX MULTIPLICATION");
    println!("------------------------------------------------------");
    println!("MATRIX MULTIPLICATION WITH VECTOR");
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![1., 0.], vec![0., 1.]]);
    let v = Vector::from(vec![4., 2.]);
    println!("{}", u.mul_vec(v));
    // [4.]
    // [2.]
    let mut u = Matrix::from(vec![vec![2., 0.], vec![0., 2.]]);
    let v = Vector::from(vec![4., 2.]);
    println!("{}", u.mul_vec(v));
    // [8.]
    // [4.]
    let mut u = Matrix::from(vec![vec![2., -2.], vec![-2., 2.]]);
    let v = Vector::from(vec![4., 2.]);
    println!("{}", u.mul_vec(v));
    // [4.]
    // [-4.]
    println!("------------------------------------------------------");
    println!("MATRIX MULTIPLICATION WITH MATRIX");
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![3., -5.], vec![6., 8.]]);
    let mut v = Matrix::from(vec![vec![2., 1.], vec![4., 2.]]);
    println!("{}", u.mul_mat(v));
    let mut u = Matrix::from(vec![vec![3.], vec![6.]]);
    let mut v = Matrix::from(vec![vec![2.]]);
    println!("{}", u.mul_mat(v));
    println!("------------------------------------------------------");
    println!("TRACE ");
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![1., 0.], vec![0., 1.]]);
    println!("{}", u.trace());
    // 2.0
    let mut u = Matrix::from(vec![vec![2., -5., 0.], vec![4., 3., 7.], vec![-2., 3., 4.]]);
    println!("{}", u.trace());
    // 9.0
    let mut u = Matrix::from(vec![
        vec![-2., -8., 4.],
        vec![1., -23., 4.],
        vec![0., 6., 4.],
    ]);
    println!("{}", u.trace());
    // -21.0
    println!("------------------------------------------------------");
    println!("TRANSPOSE ");
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![1., -2., 3.]]);
    println!("{}", u.transpose());
    let mut u = Matrix::from(vec![vec![3., -1.], vec![0., 2.], vec![1., -4.]]);
    println!("{}", u.transpose());
    let mut u = Matrix::from(vec![vec![-1., 3.], vec![0., 2.]]);
    println!("{}", u.transpose());
    let mut u = Matrix::from(vec![
        vec![1., 0., -2.],
        vec![-4., 1., 7.],
        vec![5., -3., 2.],
    ]);
    println!("{}", u.transpose());
    let mut u = Matrix::from(vec![vec![1., 0., 0.], vec![0., -2., 0.], vec![0., 0., 5.]]);
    println!("{}", u.transpose());
    println!("------------------------------------------------------");
    println!("ROW ECHELON FORM ");
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![
        vec![8., 5., -2., 4., 28.],
        vec![4., 2.5, 20., 4., -4.],
        vec![8., 5., 1., 4., 17.],
    ]);
    println!("{}", u.row_echelon());
    // [1.0, 0.625, 0.0, 0.0, -12.1666667]
    // [0.0, 0.0, 1.0, 0.0, -3.6666667]
    // [0.0, 0.0, 0.0, 1.0, 29.5 ]
}
