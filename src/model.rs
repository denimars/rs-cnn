use crate::nn::Linear;
use crate::matrix::Matrix;

pub struct Model {
    pub layers: Vec<Linear>,
}

impl Model {
    pub fn new(layers: Vec<Linear>)->Self{
        Model {layers}
    }

    pub fn forward (&self, x: &Matrix) -> Matrix{
        let mut x = x.clone();
        for i in 0..self.layers.len(){
            x = self.layers[i].forward(&x);
        }
        x
    }
}