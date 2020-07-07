pub type F = f32;
pub type U = usize;

pub enum SumMode{
    Cropped,
    Calculted,
    DefaultValue(F)
}

#[derive(Copy, Clone)]
pub struct Point(U, U);
impl Point { pub fn new (x: U,y: U) -> Point {Point(x,y)}}

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


    pub fn xd (arr: [[F; 100]; 100]) -> () {

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

    pub fn get (&self, x: U, y: U) -> &F {
        &self.matrix[y][x]
    }

    pub fn get_by_point (&self, point: &Point) -> &F {
        &self.matrix[point.1][point.0]
    }

    pub fn set (&mut self, x: U, y: U, value: F) -> () {
        self.matrix[y][x] = value;
    }

    pub fn shape (&self) -> Point {
        Point(self.x, self.y)
    }

    pub fn print(&self) {
        println!("{:?}", self);
        // for y in 0..self.y {
        //     for x in 0..self.x {
        //         print!["{:^5}", self.get(x, y)];
        //     }  
        //     print!["\n"];
        // }
        let iterator = self.iter();
        let mut printed = 0;
        let frequvency = iterator.end.0 - iterator.current.0 + 1;
        
        for x in iterator {
            print!["{:^5}", x];
            printed += 1;
            
            if printed == frequvency {
                printed = 0;
                print!["\n"];
            }
        }
    }

    // , 
    pub fn print_interval(&self, start: &Point, end: &Point) {
        println!("{:?}", self);
        let iterator = self.iter_interval(start, end);
        // let iterator = self.iter();
        let mut printed = 0;
        let frequvency = iterator.end.0 - iterator.current.0 + 1;
        
        for x in iterator {
            print!["{:^5}", x];
            printed += 1;
            
            if printed == frequvency {
                printed = 0;
                print!["\n"];
            }
        }
    }

    pub fn convolute (&mut self, kernel: &Matrix, mode: SumMode) -> Matrix {
        // TODO :: Check  for redva szar
        match mode {
            SumMode::Cropped => Matrix::convolute_cropped(self, kernel),
            // SumMode::DefaultValue => Matrix::
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

    pub fn iter (&self) -> MatrixIterator {
        MatrixIterator::new(self)
    }

    pub fn iter_interval (&self, start: &Point, end: &Point) -> MatrixIterator {
        MatrixIterator::new_interval(self, start, end)
    }
}


pub struct MatrixIterator<'a> {
    pub start: Point,
    pub end: Point,
    refered: &'a Matrix,

    current: Point 
}

impl<'a> MatrixIterator<'a> {
    pub fn new (matrix: &'a Matrix) -> MatrixIterator<'a>{
        MatrixIterator {
            start: Point(0, 0) ,
            current: Point(0, 0),
            end: Point(matrix.x - 1, matrix.y - 1),
            refered: matrix
        }
    } 

    pub fn new_interval (matrix: &'a Matrix, start: &Point, end: &Point) -> MatrixIterator<'a> {
        MatrixIterator {
            start: *start,
            current: *start,
            end: *end,
            refered: matrix
        }
    }
}

impl<'a> Iterator for MatrixIterator<'a> {
    type Item = F;

    fn next(&mut self) -> Option< <Self as std::iter::Iterator>::Item> { 
        // if self.current.0 >= self.end.0 && self.current.1 >= self.end.1 {
        if self.current.1 > self.end.1 {
            None
        } else {
            let value = self.refered.get_by_point(&self.current);
            
            self.current.0 += 1;

            // if {self.current.0 %= self.end.0; self.current.0} == 0 {
            if self.current.0 > self.end.0 {
                self.current.0 = self.start.0;
                self.current.1 += 1;
            }

            Some(*value)
        }
    }
}