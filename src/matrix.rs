use std::fmt;

#[derive(Debug, Clone)]

pub struct Matrix {
    pub data: Vec<f32>,
    pub rows: usize,
    pub cols: usize,
}


impl fmt::Display for Matrix{
    fn fmt(&self, f: &mut fmt::Formatter<'_>)-> fmt::Result{
        for i in 0..self.rows{
            for j in 0..self.cols{
               write!(f, "{:8.4}", self.get(i, j))?; 
            }
            writeln!(f, "")?
        }
        Ok(())
    }
}

impl Matrix{
    pub fn new(data: Vec<f32>, rows: usize, cols:usize)-> Self{
        assert!(data.len()== rows * cols);
        Matrix{data, rows, cols}
    }

    pub fn get(&self, row:usize, col:usize) -> f32{
        assert!(row < self.rows);
        assert!(col < self.cols);
        let  index = self.cols * row + col;
        self.data[index]
    }

    pub fn zeros(rows: usize, cols:usize) -> Self{
        let data = vec![0.0; rows * cols];
        Matrix {data, rows, cols}
    }

    pub fn set(&mut self, row: usize, col:usize, value:f32) {
        assert!(row < self.rows);
        assert!(col < self.cols);
        let  index = self.cols * row + col;
        self.data[index] = value
    }

    pub fn random(rows: usize, cols:usize, low:f32, high:f32) -> Self{
        let mut data = vec![0.0; rows * cols];
        for i in 0..rows*cols{
            let rnd = rand::random::<u32>();
            data[i] = low + (rnd as f32 / u32::MAX as f32) * (high-low);
            
        }
        Matrix {data, rows, cols}
    }

}


pub fn matmul(a: &Matrix, b:&Matrix)-> Matrix{
    assert!(a.cols== b.rows);
    let mut matrix = Matrix::zeros(a.rows, b.cols);
    for i in 0..a.rows{
        for j in 0..b.cols{
            let mut sum = 0.0;
            for k in 0..b.rows{
                sum += a.get(i, k) * b.get(k, j)
            }
           matrix.set(i, j, sum);
        }
    }
    matrix
}

#[cfg(test)]

mod test{
    use super::{matmul, Matrix};

    #[test]
    fn test_matmul(){
        let a = Matrix::new(vec![1.0,1.0, 0.0, 2.0], 2, 2);
        let b = Matrix::new(vec![0.0,1.0, 1.0, 0.0, 0.0, 1.0], 2, 3);
        let expected = vec![0.0, 1.0, 2.0, 0.0, 0.0, 2.0];

        let c = matmul(&a, &b);
       
        print!("{c}");
        for i in 0..2*3{
            assert!(c.data[i]== expected[i]); 
        }

    }
}