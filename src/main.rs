mod matrix;
mod nn;
mod model;
mod activation;


// mod model;
use nn::Linear;

use matrix::Matrix;
use model::Model;
use activation::Activation;
use std::fs::File;
use std::io::{self, BufReader, Read};

fn load_mnist_as_matrix(filepath: &str) -> io::Result<Matrix> {
    let file = File::open(filepath)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    let num_images = buffer.len()/(784 * 4);
    let mut data = Vec::with_capacity(num_images * 784);

    for chunk in buffer.chunks( 4) {
        if chunk.len() == 4{
            let value = f32::from_be_bytes(chunk.try_into().unwrap());
            data.push(value);
        }
    }
    Ok(Matrix::new(data, num_images, 784))

}

fn main() {
    let x_train = load_mnist_as_matrix("data/x_train.bin").unwrap();
    println!("num_images={}", x_train.rows);
    println!("dim={}", x_train.cols)
}
