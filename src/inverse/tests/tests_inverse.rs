use std::vec;

use crate::Matrix;
pub fn tests_inverse() {
    println!("------------------------------------------------------");
    println!("INVERSE");
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![4., 7.], vec![2., 6.]]);
    println!("test with {}", u);
    let result = u.inverse();
    if let Ok(description) = &result {
        assert_eq!(description.elements, vec![vec![0.6, -0.7], vec![-0.2, 0.4]]);
        println!("{}", description);
    }

    if let Err(err) = &result {
        println!("Error: {}", err);
    }
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![8., 5., -2.], vec![4., 7., 20.], vec![7., 6., 1.]]);
    println!("test with {}", u);
    let tet = u.inverse();
    if let Ok(description) = &tet {
        println!("{}", description);
        assert_eq!(
            description.elements,
            vec![
                vec![0.649425287, 0.097701149, -0.655172414],
                vec![-0.781609195, -0.126436782, 0.965517241],
                vec![0.143678161, 0.07471265, -0.206896552]
            ]
        );
    }

    if let Err(err) = &tet {
        println!("Error: {}", err);
    }
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![1., 0., 0.], vec![0., 1., 0.], vec![0., 0., 1.]]);
    println!("test with {}", u);
    let tet = u.inverse();
    if let Ok(description) = &tet {
        println!("{}", description);
        assert_eq!(
            description.elements,
            vec![vec![1., 0.0, 0.0], vec![0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0]]
        );
    }

    if let Err(err) = &tet {
        println!("Error: {}", err);
    }
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![2., 0., 0.], vec![0., 2., 0.], vec![0., 0., 2.]]);
    println!("test with {}", u);
    let tet = u.inverse();
    if let Ok(description) = &tet {
        println!("{}", description);
        assert_eq!(
            description.elements,
            vec![
                vec![0.5, 0.0, 0.0],
                vec![0.0, 0.5, 0.0],
                vec![0.0, 0.0, 0.5]
            ]
        );
    }

    if let Err(err) = &tet {
        println!("Error: {}", err);
    }
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![1., 0.], vec![0., 1.]]);
    println!("test with {}", u);
    let tet = u.inverse();
    if let Ok(description) = &tet {
        println!("{}", description);
        assert_eq!(description.elements, vec![vec![1.0, 0.0,], vec![0.0, 1.0,]]);
    }

    if let Err(err) = &tet {
        println!("Error: {}", err);
    }
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![2., 0.], vec![0., 2.]]);
    println!("test with {}", u);
    let tet = u.inverse();
    if let Ok(description) = &tet {
        println!("{}", description);
        assert_eq!(description.elements, vec![vec![0.5, 0.0,], vec![0.0, 0.5,]]);
    }

    if let Err(err) = &tet {
        println!("Error: {}", err);
    }
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![1., 2.], vec![3., 4.]]);
    println!("test with {}", u);
    let tet = u.inverse();
    if let Ok(description) = &tet {
        println!("{}", description);
        assert_eq!(
            description.elements,
            vec![vec![-2.0000002, 1.0000001], vec![1.5000001, -0.50000006]]
        );
    }

    if let Err(err) = &tet {
        println!("Error: {}", err);
    }
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![1., 1.], vec![1., 0.]]);
    println!("test with {}", u);
    let tet = u.inverse();
    if let Ok(description) = &tet {
        println!("{}", description);
        assert_eq!(description.elements, vec![vec![0., 1.], vec![1., -1.]]);
    }

    if let Err(err) = &tet {
        println!("Error: {}", err);
    }
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![0., 3.], vec![0., 44.]]);
    println!("test with {}", u);
    let tet = u.inverse();
    if let Ok(description) = &tet {
        println!("{}", description);
        // assert_eq!(description.elements, vec![vec![0., 1.], vec![1., -1.]]);
    }

    if let Err(err) = &tet {
        println!("Error: {}", err);
    }
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![1., 2.], vec![1., 2.]]);
    println!("test with {}", u);
    let tet = u.inverse();
    if let Ok(description) = &tet {
        println!("{}", description);
        // assert_eq!(description.elements, vec![vec![0., 1.], vec![1., -1.]]);
    }

    if let Err(err) = &tet {
        println!("Error: {}", err);
    }
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![0., 1.], vec![0., 1.]]);
    println!("test with {}", u);
    let tet = u.inverse();
    if let Ok(description) = &tet {
        println!("{}", description);
        // assert_eq!(description.elements, vec![vec![0., 1.], vec![1., -1.]]);
    }

    if let Err(err) = &tet {
        println!("Error: {}", err);
    }
    println!("------------------------------------------------------");
    let mut u = Matrix::from(vec![vec![1., 2., 3.], vec![4., 5., 6.], vec![7., 8., 9.]]);
    println!("test with {}", u);
    let tet = u.inverse();
    if let Ok(description) = &tet {
        println!("{}", description);
        // assert_eq!(description.elements, vec![vec![0., 1.], vec![1., -1.]]);
    }

    if let Err(err) = &tet {
        println!("Error: {}", err);
    }
    println!("------------------------------------------------------");
    // match result {
    //     Ok(r) => {
    //         println!("result {}", r);

    //         assert_eq!(r.elements[0], vec![0.6, -0.7]);
    //         assert_eq!(r.elements[1], vec![-0.2, 0.4]);
    //     }
    //     Err(_) => {
    //         assert_eq!(0, 1);
    //     }
    // }
}
