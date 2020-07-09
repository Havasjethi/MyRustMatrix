pub mod matrix_iteration;
pub mod point_iteration;
pub mod types; // Import library; Export module

pub use types::*; // Export every element of it

mod creation;
pub use creation::*;

impl Matrix {
    
    pub fn get (&self, x: U, y: U) -> &F {
        &self.matrix[y][x]
    }

    pub fn get_by_point (&self, point: &Point) -> &F {
        self.get(point.0, point.1)
    }

    pub fn set (&mut self, x: U, y: U, value: F) -> () {
        self.matrix[y][x] = value;
    }

    pub fn shape (&self) -> Point {
        Point(self.x, self.y)
    }

    pub fn print(&self) {
        self.print_interval(&Point(0, 0), &self.shape())
    }

    pub fn print_interval(&self, start: &Point, end: &Point) {
        println!("{:?}", self);
        let iterator = self.iter_interval(start, end);
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

    pub fn convolute_cropped (&mut self, kernel: &Matrix) -> Matrix {
        let start_x: U = (kernel.x - 1) / 2;
        let start_y: U = (kernel.y - 1) / 2;

        let end_x: U = self.x - start_x; 
        let end_y: U = self.y - start_y;


        // for point in self.point_iter_interval(&Point(start_x, start_y),&Point(end_x, end_y)) {
            
        // }
        // template_method_unary_point(|point :&Point, m: &Matrix| {
        //     let cropped = m.crop_from_middle(point, &Point(kernel.x, kernel.y));
        //     cropped.sum_multiply_matrix(&kernel)
        // })


        let mut created_matrix: Vec<Vec<F>> = Vec::new();

        for y in start_y..end_y {
            let mut row :Vec<F> = vec![];

            for x in start_x..end_x {
                let cropped = self.crop_from_middle(&Point(x,y), &Point(kernel.x, kernel.y));
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

    pub fn crop_from_middle (&self, point: &Point, size: &Point) -> Matrix {
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

    pub fn crop_form_corner (&self, point: &Point, size: &Point) -> Matrix {
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

    pub fn add (&self, other: &Matrix) -> Matrix {
        self.template_method_binary(other, |x, y| x+y)
    }
    pub fn substract (&self, other: &Matrix) -> Matrix {
        self.template_method_binary(other, |x, y| x-y)
    }
    pub fn divide (&self, other: &Matrix) -> Matrix {
        self.template_method_binary(other, |x, y| x*y)
    }
    pub fn multiply (&self, other: &Matrix) -> Matrix {
        self.template_method_binary(other, |x, y| x/y)
    }


    // Creates a binary operation from unary
    // Insufficient; need to be changed
    pub fn template_method_unary<Lambda: Fn(&F) -> F> (&self, template_m: Lambda) -> Matrix {
        let other = Matrix::from_shape(self.shape());
        self.template_method_binary(&other, |x, _| template_m(x) )
    }

    pub fn template_method_unary_point<Lambda: Fn(&Point, &Matrix) -> F> (&self, template_m: Lambda) -> Matrix {
        let mut columms = vec![];

        let mut row: Vec<F>;

        for y in 0..self.y {
            row = vec![];
            for x in 0..self.x {
                let point = Point(x,y);
                row.push(template_m(&point, self));
            }
            columms.push(row);
        }

        Matrix::from_vectors(columms)
    }

    // Dot operation
    pub fn template_method_binary<Lambda: Fn(&F, &F) -> F> (&self, other: &Matrix, template_m: Lambda) -> Matrix {
        let mut columns: Vec<Vec<F>> = vec![];

        for y in 0..self.y {
            let mut row: Vec<F> = vec![]; 

            for x in 0..self.x {
                let p = Point(x,y);

                row.push(
                    template_m(
                        self.get_by_point(&p), other.get_by_point(&p)
                    )
                );
            
            }
            columns.push(row);
        }

        Matrix {
            y: columns.len(),
            x: columns[0].len(),
            matrix: columns
        }
    }
}
