pub type F = f32;
pub type U = usize;

pub enum SumMode{
    Cropped,
    Calculted,
    DefaultValue(F)
}

pub struct Point(U, U);

#[derive(std::fmt::Debug)]
pub struct Matrix {
    pub x: U,
    pub y: U,
    pub matrix: Vec<Vec<F>>,
}

impl Matrix {
    pub fn new (x: U, y: U) -> Matrix {
        Matrix::create(x, y, 0 as F)
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

    pub fn get (&self, x: U, y: U) -> &F {
        &self.matrix[y][x]
    }

    pub fn set (&mut self, x: U, y: U, value: F) -> () {
        self.matrix[y][x] = value;
    }

    pub fn shape (&self) -> (U,U) {
        (self.x, self.y)
    }

    pub fn print(&self) {
        println!("{:?}", self);
        for y in 0..self.y {
            for x in 0..self.x {
                print!["{:^5}", self.get(x, y)];
            }  
            print!["\n"];
        }
    }

    pub fn convolute (&mut self, kernel: &Matrix, mode: SumMode) -> Matrix {
        // TODO :: Check  for redva szar
        match mode {
            SumMode::Cropped => Matrix::convolute_cropped(self, kernel),
            _ => panic!("Calling unimplemented method")
        }
    }

    pub fn convolute_cropped (matrix: &mut Matrix, kernel: &Matrix) -> Matrix {
        let start_x: U = (kernel.x - 1) / 2;
        let start_y: U = (kernel.y - 1) / 2;

        let end_x: U = matrix.x - start_x; 
        let end_y: U = matrix.y - start_y;

        let mut created_matrix: Vec<Vec<F>> = Vec::new();

        for y in start_y..end_y {
            let mut row :Vec<F> = vec![];

            for x in start_x..end_x {
                let cropped = matrix.crop_from_middle((x,y), (kernel.x, kernel.y));
                row.push(cropped.sum_multiply_matrix(&kernel));
            }

            created_matrix.push(row);
        }

        Matrix {
            x: created_matrix[0].len(),
            y: created_matrix.len(),
            matrix: created_matrix
        }
    }

    pub fn crop_from_middle (&self, point: (U, U), size: (U, U)) -> Matrix {
        let diff_x: U = (size.0 - 1) / 2;
        let diff_y: U = (size.1 - 1) / 2;
        
        let start_x: U = point.0 - diff_x;
        let start_y: U = point.1 - diff_y;

        let end_x: U = point.0 + diff_x; 
        let end_y: U = point.1 + diff_y;

        let mut matrix: Vec<Vec<F>> = Vec::new();

        for y in start_y..=end_y {
            let mut row :Vec<F> = vec![];

            for x in start_x..=end_x {
                row.push(*self.get(x,y));
            }

            matrix.push(row);
        }
        
        Matrix {
            x: size.0,
            y: size.1,
            matrix
        }
    }

    pub fn crop_form_corner (&self, point: (U,U), size: (U, U)) -> Matrix {
        let mut matrix: Vec<Vec<F>> = vec![];
        for y in point.1..point.1+size.1 {
            let mut row: Vec<F> = vec![];
            for x in point.0..point.0+size.0 {
                row.push(*self.get(x, y));
            }
            matrix.push(row);
        }
        Matrix {
            x: size.0,
            y: size.1,
            matrix
        }
    }

    pub fn sum_multiply_matrix (&self, other: &Matrix) -> F {
        let mut value: F = 0 as F;
        for y in 0..self.y {
            for x in 0..self.x {
                value += (*self.get(x,y)) * (*other.get(x,y));  
            }    
        }   
        value
    }

    pub fn equal( &self, other: &Matrix ) -> bool {
        if self.x != other.x || self.y != other.y {
            return false;
        }

        for y in 0..self.y {
            for x in 0..self.x {
                if self.get(x, y) != other.get(x, y) {
                    return false;
                }
            }
        }
        
        true
    }
}
