use crate::matrix::Matrix;

pub enum Activation{
    RELU,
    SOFTMAX
}

pub fn apply_activation(x: &Matrix, activation: &Activation) -> Matrix{
    match activation {
        Activation::RELU => relu(&x),
        Activation::SOFTMAX => softmax(&x),
    }
}

pub fn relu(x: &Matrix)-> Matrix{
    //relum (x) = max(0,x)
    let mut output = Matrix::zeros(x.rows, x.cols);
    for i in 0..x.rows{
        for j in 0..x.cols{
            let value = x.get(i, j);
            if value > 0.0{
                output.set(i,j, value);
            }
        }
    }
    output
}


pub fn softmax(x: &Matrix) -> Matrix{
    //softmax = exp(x[i]) / sum(exp(x))
    let mut output = Matrix::zeros(x.rows, x.cols);
    for i in 0..x.rows{
        let mut sum = 0.0;
        for j in 0..x.cols{
            sum += x.get(i,j).exp()
        }
        for j in 0..x.cols{
            let exp_j = x.get(i, j).exp();
            let value = exp_j/sum;
            output.set(i, j, value)
        }
    }
    output
}

#[cfg(test)]

mod test{
    use crate::matrix::Matrix;

    use super::{relu, softmax};
    #[test]
    pub fn relu_test(){
        let x = Matrix::new(vec![0.0, -1.0, 1.0, 2.0], 2, 2);
        let output = relu(&x);
        let expected = vec![0.0, 0.0, 1.0, 2.0];
        for i in 0..2*2{
            assert!(output.data[i]==expected[i])
        }
    }
    #[test]
    pub fn text_softmax(){
        let x = Matrix::new(vec![1.0, 1.0], 1, 2);
        let output = softmax(&x);
        let expected = vec![0.5, 0.5];
        for i in 0..2{
            assert!(output.data[i]==expected[i])
        }
    }

}
