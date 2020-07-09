use super::*;

pub struct MatrixIterator<'a> {
    pub start: Point,
    pub end: Point,
    pub current: Point,


    refered: &'a Matrix,
}

impl<'a> MatrixIterator<'a> {
    pub fn whole (matrix: &'a Matrix) -> MatrixIterator<'a>{
        MatrixIterator {
            start: Point(0, 0) ,
            current: Point(0, 0),
            end: Point(matrix.x - 1, matrix.y - 1),
            refered: matrix
        }
    } 

    pub fn row (matrix: &'a Matrix, row: U) -> MatrixIterator<'a> {
        MatrixIterator {
            start: Point(0, row),
            current: Point(0, 0),
            end: Point(0, row),
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

    // TODO :: Being able to switch between point & value iteration
    fn next(&mut self) -> Option< <Self as std::iter::Iterator>::Item> { 
        if self.current.1 > self.end.1 {
            None
        } else {
            let value = self.refered.get_by_point(&self.current);
            
            self.current.0 += 1;

            if self.current.0 > self.end.0 {
                self.current.0 = self.start.0;
                self.current.1 += 1;
            }

            Some(*value)
        }
    }
}

impl Matrix{
    pub fn iter (&self) -> MatrixIterator {
        MatrixIterator::whole(self)
    }
    
    pub fn iter_row (&self, row: U) -> MatrixIterator {
        MatrixIterator::row(self, row)
    }
    
    pub fn iter_interval (&self, start: &Point, end: &Point) -> MatrixIterator {
        MatrixIterator::new_interval(self, start, end)
    }
}
