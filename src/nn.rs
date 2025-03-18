use crate::matrix::{Matrix, matmul};

pub struct Linear {
    pub weight: Matrix,
    pub bias: Matrix,
}

impl Linear {
    pub fn new(input: usize, output: usize)-> Self{
        let weight = Matrix::random(input, output, -1.0, 1.0);
        let bias = Matrix::zeros(1, output);
        Linear{weight, bias}
    }

    pub fn forward(&self, x: &Matrix)-> Matrix{
        //y = xW + b
        let mut output = matmul(&x, &self.weight);
        for i in 0..output.rows{
            for j in 0..self.bias.cols{
                let value = output.get(i, j);
                let bias = self.bias.get(0, j);
                output.set(i, j, value+bias);
            }
        }
        output   

    }

}

#[cfg(test)]
mod tests{
    use crate::matrix::Matrix;
    use super::Linear;
    
    #[test]
    pub fn test_forward(){
        let  layer = Linear::new(3, 2);
        let x = Matrix::random(2, 3, -1.0, 1.0);
        let output = layer.forward(&x);

        assert!(output.rows==2);
        assert!(output.cols==2);
    }
}