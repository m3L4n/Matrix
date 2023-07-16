use std::vec;

use crate::types::vector::Vector;
pub fn test_vector_simple_operations_add() {
    println!("------------------------------------------------------");
    println!(" VECTOR");
    println!("------------------------------------------------------");
    println!("\x1B[32m                 ADD \x1B[0m");
    println!("------------------------------------------------------");
    let mut first_vector = Vector::from(vec![2., 3.]);
    let mut second_vector = Vector::from(vec![5., 7.]);
    let mut third_vector = Vector::from(vec![9., -2.]);
    let mut four_vector = Vector::from(vec![19., 1.]);
    println!(
        "\x1B[32mimpression de u {}  v {}\x1B[0m",
        first_vector, second_vector
    );
    first_vector.add(&second_vector);
    println!("resultat de u + u1 {}", first_vector);
    println!("------------------------------------------------------");
    assert_eq!(first_vector.elements, vec![(2. + 5.), (3. + 7.)]);
    println!(
        "\x1B[32m\nimpression de second_vector {}  third_vector {}\x1B[0m",
        second_vector, third_vector
    );
    second_vector.add(&third_vector);
    assert_eq!(second_vector.elements, vec![(5. + 9.), (7. + -2.)]);
    println!("resultat de second_vector + third_vector {}", second_vector);

    println!("------------------------------------------------------");
    println!(
        "\x1B[32m\nimpression de third_vector {}  uv1 {}\x1B[0m",
        third_vector, four_vector
    );
    third_vector.add(&four_vector);
    assert_eq!(third_vector.elements, vec![(9. + 19.), (-2. + 1.)]);
    println!("resultat de U + four_vector {}", third_vector);
    println!("------------------------------------------------------");

    let mut first_vector = Vector::from(vec![0., 0.]);
    let mut second_vector = Vector::from(vec![0., 0.]);
    let mut third_vector = Vector::from(vec![-21., 21.]);
    let mut four_vector = Vector::from(vec![21., -21.]);

    println!(
        "\x1B[32mimpression de first {}  second {}\x1B[0m",
        first_vector, second_vector
    );
    first_vector.add(&second_vector);
    println!("resultat de u + u1 {}", first_vector);
    assert_eq!(first_vector.elements, vec![(0. + 0.), (0. + 0.)]);
    println!("------------------------------------------------------");
    println!(
        "\x1B[32m\nimpression de second_vector {}  third_vector {}\x1B[0m",
        third_vector, four_vector
    );
    third_vector.add(&four_vector);
    println!("resultat de u + u1 {}", third_vector);
    assert_eq!(third_vector.elements, vec![(-21. + 21.), (21. + -21.)]);
    println!("------------------------------------------------------");
    let mut first_vector = Vector::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let mut second_vector = Vector::from(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    println!(
        "\x1B[32mimpression de first {}  second {}\x1B[0m",
        first_vector, second_vector
    );
    first_vector.add(&second_vector);
    println!("resultat de u + u1 {}", first_vector);
    assert_eq!(first_vector.elements, vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9]);
}

pub fn test_vector_simple_operations_sub() {
    println!("------------------------------------------------------");
    println!(" VECTOR");
    println!("------------------------------------------------------");
    println!("\x1B[32m                 SUB \x1B[0m");
    println!("------------------------------------------------------");
    let mut first_vector = Vector::from(vec![2., 3.]);
    let mut second_vector = Vector::from(vec![5., 7.]);
    let mut third_vector = Vector::from(vec![9., -2.]);
    let mut four_vector = Vector::from(vec![19., 1.]);
    println!(
        "\x1B[32mimpression de u {}  v {}\x1B[0m",
        first_vector, second_vector
    );
    first_vector.sub(&second_vector);
    println!("resultat de u - u1 {}", first_vector);
    println!("------------------------------------------------------");
    assert_eq!(first_vector.elements, vec![(2. - 5.), (3. - 7.)]);
    println!(
        "\x1B[32m\nimpression de second_vector {}  third_vector {}\x1B[0m",
        second_vector, third_vector
    );
    second_vector.sub(&third_vector);
    assert_eq!(second_vector.elements, vec![(5. - 9.), (7. - -2.)]);
    println!("resultat de second_vector + third_vector {}", second_vector);

    println!("------------------------------------------------------");
    println!(
        "\x1B[32m\nimpression de third_vector {}  uv1 {}\x1B[0m",
        third_vector, four_vector
    );
    third_vector.sub(&four_vector);
    assert_eq!(third_vector.elements, vec![(9. - 19.), (-2. - 1.)]);
    println!("resultat de U + four_vector {}", third_vector);
    println!("------------------------------------------------------");

    let mut first_vector = Vector::from(vec![0., 0.]);
    let mut second_vector = Vector::from(vec![0., 0.]);
    let mut third_vector = Vector::from(vec![-21., 21.]);
    let mut four_vector = Vector::from(vec![21., -21.]);

    println!(
        "\x1B[32mimpression de first {}  second {}\x1B[0m",
        first_vector, second_vector
    );
    first_vector.sub(&second_vector);
    println!("resultat de u - u1 {}", first_vector);
    assert_eq!(first_vector.elements, vec![(0. - 0.), (0. - 0.)]);
    println!("------------------------------------------------------");
    println!(
        "\x1B[32m\nimpression de second_vector {}  third_vector {}\x1B[0m",
        third_vector, four_vector
    );
    third_vector.sub(&four_vector);
    println!("resultat de u - u1 {}", third_vector);
    assert_eq!(third_vector.elements, vec![(-21. - 21.), (21. - -21.)]);
    println!("------------------------------------------------------");
    let mut first_vector = Vector::from(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let mut second_vector = Vector::from(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    println!(
        "\x1B[32mimpression de first {}  second {}\x1B[0m",
        first_vector, second_vector
    );
    first_vector.sub(&second_vector);
    println!("resultat de u + u1 {}", first_vector);
    assert_eq!(
        first_vector.elements,
        vec![-9, -7, -5, -3, -1, 1, 3, 5, 7, 9]
    );
}
pub fn test_vector_simple_operations_scl() {
    println!("------------------------------------------------------");
    println!("\x1B[32m                 SCL \x1B[0m");
    println!("------------------------------------------------------");
    let mut u = Vector::from(vec![0., 0.]);
    let mut v = Vector::from(vec![1., 0.]);
    let mut w = Vector::from(vec![21, 21]);
    let mut x = Vector::from(vec![42., 42.]);
    let mut z = Vector::from(vec![1., 2., 3., 4., 5., 6., 7., 8., 9., 10.]);
    println!("\x1B[32m\nimpression de u {}\x1B[0m", u);
    u.scl(0.0);
    assert_eq!(u.elements, vec![0. * 0.0, 0. * 0.0]);
    println!("resultat de U * 2.0 {}", u);
    println!("------------------------------------------------------");
    println!("\x1B[32m\nimpression de v {}\x1B[0m", v);
    v.scl(4.0);
    assert_eq!(v.elements, vec![1. * 4.0, 0. * 4.0]);
    println!("resultat de v * 2.0 {}", v);
    println!("------------------------------------------------------");
    println!("\x1B[32m\nimpression de v {}\x1B[0m", w);
    w.scl(2);
    assert_eq!(w.elements, vec![21 * 2, 21 * 2]);
    println!("resultat de v * 2 {}", z);
    println!("\x1B[32m\nimpression de v {}\x1B[0m", x);
    x.scl(0.5);
    assert_eq!(x.elements, vec![42. * 0.5, 42. * 0.5]);
    println!("resultat de v * 0.5 {}", x);

    println!("------------------------------------------------------");
    println!("\x1B[32m\nimpression de v {}\x1B[0m", z);
    z.scl(10.1);
    assert_eq!(
        z.elements,
        vec![
            1. * 10.1,
            2. * 10.1,
            3. * 10.1,
            4. * 10.1,
            5. * 10.1,
            6. * 10.1,
            7. * 10.1,
            8. * 10.1,
            9. * 10.1,
            10. * 10.1
        ]
    );
    println!("resultat de x * 10.1 {}", x);
}
