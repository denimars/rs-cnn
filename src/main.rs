mod matrix;
use crate::matrix::Matrix;

fn main() {
    let m = Matrix::random(3, 3, -1.0, 1.0);
    println!("{m}")
}
