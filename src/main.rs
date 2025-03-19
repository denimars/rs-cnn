mod matrix;
mod nn;
mod model;
mod activation;


// mod model;
use nn::Linear;

use matrix::Matrix;
use model::Model;

fn main() {
    let x = Matrix::random(10, 5, -1.0, 1.0);
    let layers = vec![
        Linear::new(5, 10),
        Linear::new(10, 2),

    ];
    let model = Model::new(layers);

    let output = model.forward(&x);
    println!("x=\n{x}");

    println!("{output}");
}
