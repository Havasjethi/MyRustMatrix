use super::types::*;

impl Matrix {

    pub fn new (x: U, y: U) -> Matrix {
        Matrix::create(x, y, 0 as F)
    }

    pub fn from_shape (p: Point) -> Matrix {
        Matrix::new(p.0, p.1)
    }

    pub fn new_with_default (x: U, y: U, default: F) -> Matrix {
        Matrix::create(x, y, default)
    }

    pub fn square (x: U) -> Matrix {
        Matrix::create(x, x, 0 as F)
    }

    pub fn square_with_default (x: U, default: F) -> Matrix {
        Matrix::create(x, x, default)
    }

    // TODO cast the given number to 'F'; This is terribe
    pub fn new_matrix (arrays: &[&[F]]) -> Matrix {
        let x: U = arrays[0].len();
        let mut y: U = 0;
        let mut matrix: Vec<Vec<F>> = vec![];

        for row in arrays.iter() {
            y += 1;
            
            if row.len() != x {
                panic!("Bad bad")
            }

            matrix.push(row.to_vec());
        }

        Matrix {
            x,
            y,
            matrix
        }
    }

    pub fn from_vectors (matrix: Vec<Vec<F>>) -> Matrix {

        let y: U = matrix.len();
        let mut x: U = matrix[0].len();
        
        for row in matrix.iter() {
            if x != row.len() {
                panic!("The rows has different size")
            }
        }

        Matrix {
            x,
            y,
            matrix
        }
    }

    pub fn create (x: U, y: U, value: F) -> Matrix {
        Matrix {
            x,
            y,
            matrix: vec![vec![value; x]; y]
        }
    }
}