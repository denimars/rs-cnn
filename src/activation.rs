use crate::matrix::Matrix;

pub fn relu(x: &Matrix)-> Matrix{
    //max(0,x)
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

