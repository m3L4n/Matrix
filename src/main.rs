// mod cosine;
// mod cross_product;
// mod determinant;
// mod dot;
// mod inverse;

mod linear {
    pub mod lerp;
    pub mod linear_combination;
    pub mod tests {
        pub mod tests_lerp;
        pub mod tests_linear_combination;
    }
}
mod dot {
    pub mod dot;
    pub mod tests {
        pub mod tests_dot_product;
    }
}
mod types {
    pub mod matrix;
    pub mod tests {
        pub mod test_matrix_simple_operations;
        pub mod test_vector_simple_operations;
    }
    pub mod vector;
}
mod norm {
    pub mod norm;
    pub mod norm_k;
    pub mod tests {
        pub mod tests_norme;
    }
}
mod cosine {
    pub mod cosine;
    pub mod tests {
        pub mod tests_cosine;
    }
}
mod cross_product {
    pub mod cross_product;
    pub mod tests {
        pub mod tests_cross_product;
    }
}
mod matrix_mul {
    pub mod matrix_multiplication;
    pub mod tests {
        pub mod tests_matrix_mul;
    }
}
mod trace {
    pub mod trace;
    pub mod tests {
        pub mod tests_trace;
    }
}
mod transpose {
    pub mod transpose;
    pub mod tests {
        pub mod tests_transpose;
    }
}
mod row_reduce_form {
    pub mod row_echelon_form;
    pub mod tests {
        pub mod tests_reduce_row_echelon;
    }
}
mod determinant {
    pub mod determinant;
    pub mod tests {
        pub mod tests_determinant;
    }
}
mod inverse {
    pub mod inverse;
    pub mod tests {
        pub mod tests_inverse;
    }
}
mod rank {
    pub mod rank;
    pub mod tests {
        pub mod tests_rank;
    }
}

use crate::cosine::tests::tests_cosine::tests_cosine;
use crate::cross_product::tests::tests_cross_product::tests_cross_product;
use crate::determinant::tests::tests_determinant::tests_determinant;
use crate::dot::tests::tests_dot_product::tests_dot_product;
use crate::inverse::tests::tests_inverse::tests_inverse;
use crate::linear::tests::tests_lerp::tests_lerp;
use crate::linear::tests::tests_linear_combination::tests_linear_combination;
use crate::matrix_mul::tests::tests_matrix_mul::tests_linear_map;
use crate::matrix_mul::tests::tests_matrix_mul::tests_matrix_mul;
use crate::norm::tests::tests_norme::tests_norme_ec;
use crate::norm::tests::tests_norme::tests_norme_inf;
use crate::norm::tests::tests_norme::tests_norme_mh;
use crate::rank::tests::tests_rank::tests_rank;
use crate::row_reduce_form::tests::tests_reduce_row_echelon::tests_row_echelon_form;
use crate::trace::tests::tests_trace::tests_trace;
use crate::transpose::tests::tests_transpose::tests_transpose;
use crate::types::matrix::Matrix;
use crate::types::tests::test_matrix_simple_operations::test_matrix_simple_operations_add;
use crate::types::tests::test_matrix_simple_operations::test_matrix_simple_operations_scl;
use crate::types::tests::test_matrix_simple_operations::test_matrix_simple_operations_sub;
use crate::types::tests::test_vector_simple_operations::test_vector_simple_operations_add;
use crate::types::tests::test_vector_simple_operations::test_vector_simple_operations_scl;
use crate::types::tests::test_vector_simple_operations::test_vector_simple_operations_sub;
use crate::types::vector::Vector;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let props = args[1].clone();
        if props == "vec_add" {
            test_vector_simple_operations_add();
        } else if props == "matrix_add" {
            test_matrix_simple_operations_add();
        } else if props == "add" {
            test_matrix_simple_operations_add();
            test_vector_simple_operations_add();
        } else if props == "vec_sub" {
            test_vector_simple_operations_sub();
        } else if props == "matrix_sub" {
            test_matrix_simple_operations_sub();
        } else if props == "sub" {
            test_vector_simple_operations_sub();
            test_matrix_simple_operations_sub();
        } else if props == "vector_scl" {
            test_vector_simple_operations_scl();
        } else if props == "matrix_scl" {
            test_matrix_simple_operations_scl();
        } else if props == "scl" {
            test_vector_simple_operations_scl();
            test_matrix_simple_operations_scl();
        } else if props == "linear_comb" {
            tests_linear_combination();
        } else if props == "lerp" {
            tests_lerp();
        } else if props == "dot" {
            tests_dot_product();
        } else if props == "EC" {
            tests_norme_ec();
        } else if props == "MH" {
            tests_norme_mh();
        } else if props == "inf" {
            tests_norme_inf();
        } else if props == "cosine" {
            tests_cosine();
        } else if props == "cp" {
            tests_cross_product();
        } else if props == "lm" {
            tests_linear_map();
        } else if props == "mm" {
            tests_matrix_mul();
        } else if props == "trace" {
            tests_trace();
        } else if props == "transpose" {
            tests_transpose();
        } else if props == "echelon" {
            tests_row_echelon_form();
        } else if props == "determinant" {
            tests_determinant();
        } else if props == "inverse" {
            tests_inverse();
        } else if props == "rank" {
            tests_rank();
        }
    }

    // println!("------------------------------------------------------");
    // println!("RANK");
    // println!("------------------------------------------------------");
    // let mut u = Matrix::from(vec![vec![8., 5., -2.], vec![4., 7., 20.], vec![7., 6., 1.]]);
    // println!(" U : {}", u);
    // println!("{}", u.rank());

    // [1.0, 0.625, 0.0, 0.0, -12.1666667]
    // [0.0, 0.0, 1.0, 0.0, -3.6666667]
    // [0.0, 0.0, 0.0, 1.0, 29.5 ]
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tests_simple_operations_vec() {
        test_vector_simple_operations_add()
    }
    // #[test]
    // fn tests_simple_operation_vec() {
    //     test_simple_operation_vec();
    // }
}
