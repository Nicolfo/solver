use gpurs::linalg::MatrixUtilities;
use gpurs::linalg::Matrix;
#[no_mangle]
unsafe extern "C" fn solve(
    size: usize,
    a: *const f64,
    b: *const f64,
    x: *mut f64,
) {
    // Convert raw pointers to slices
    let a_slice = std::slice::from_raw_parts(a, size * size);
    let b_slice = std::slice::from_raw_parts(b, size);

    // Convert slices to Vec for ease of use
    let a: Vec<f64> = a_slice.to_vec();
    let b: Vec<f64> = b_slice.to_vec();

    let matrix_size = size;
    let temp_matrix_a: Matrix<f64> = Matrix::new(a, matrix_size, matrix_size).unwrap();
    let temp_matrix_b: Matrix<f64> = Matrix::new(b, matrix_size, 1).unwrap();

    let result = <Matrix<f64> as MatrixUtilities<f64>>::linear_solve_matrix(&temp_matrix_a,&temp_matrix_b).unwrap();

    for i in 0..size {
        *x.add(i) = result.lindex(i);
    }


    /*println!("======== RUST ========");
    println!("A: {:?}", mat);
    println!("B: {:?}", b);
    println!("X: {:?}", x);
    println!("======================")*/
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let x = [0.0; 2].as_mut_ptr();
        unsafe { solve(2,
                       [1.0, 2.0, 3.0, 4.0].as_ptr(),
                       [5.0, 6.0].as_ptr(),
                       x
        ) };
        unsafe {
            assert_eq!(*x, -4.0);
            assert_eq!(*x.add(1), 4.5);
        }
    }
}
