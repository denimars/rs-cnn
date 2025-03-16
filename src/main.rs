mod matrix;
use crate::matrix::Matrix;

fn main() {
    let m = Matrix::new(vec![1.0,2.0,3.0,4.0,5.0,6.0], 2, 3);
    println!("{m}")
}
