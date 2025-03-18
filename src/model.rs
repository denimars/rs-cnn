use crate::nn::Linear;

pub struct Model {
    pub layers: Vec<Linear>,
}

impl Model {
    pub fn new(layers: Vec<Linear>)->Self{
        Model {layers}
    }
}