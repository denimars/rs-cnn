mod matrix;
mod nn;
// mod model;
use crate::nn::Linear;

use crate::matrix::Matrix;

fn main() {
    let x = Matrix::random(3, 5, -1.0, 1.0);
    let linear = Linear::new(5, 1);
    let output = linear.forward(&x);
    println!("x=\n{x}");
    println!("w=\n{}", linear.weight);
    println!("b=\n{}", linear.bias);
    println!("{output}");
}
