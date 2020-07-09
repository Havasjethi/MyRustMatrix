use super::*;

pub struct PointIterator<'a> {
    pub start: Point,
    pub end: Point,
    pub current: Point,


    refered: &'a Matrix,
}

impl<'a> PointIterator<'a> {
    pub fn whole (matrix: &'a Matrix) -> PointIterator<'a>{
        PointIterator {
            start: Point(0, 0) ,
            current: Point(0, 0),
            end: Point(matrix.x - 1, matrix.y - 1),
            refered: matrix
        }
    } 

    pub fn row (matrix: &'a Matrix, row: U) -> PointIterator<'a> {
        PointIterator {
            start: Point(0, row),
            current: Point(0, 0),
            end: Point(0, row),
            refered: matrix
        }
    }

    pub fn new_interval (matrix: &'a Matrix, start: &Point, end: &Point) -> PointIterator<'a> {
        PointIterator {
            start: *start,
            current: *start,
            end: *end,
            refered: matrix
        }
    }
}

impl<'a> Iterator for PointIterator<'a> {
    type Item = Point;

    // TODO :: Being able to switch between point & value iteration
    fn next(&mut self) -> Option< <Self as std::iter::Iterator>::Item> { 
        if self.current.1 > self.end.1 {
            None
        } else {
            let value = Point(self.current.0, self.current.1);
            
            self.current.0 += 1;

            if self.current.0 > self.end.0 {
                self.current.0 = self.start.0;
                self.current.1 += 1;
            }

            Some(value)
        }
    }
}

impl Matrix{
    pub fn point_iter (&self) -> PointIterator {
        PointIterator::whole(self)
    }
    
    pub fn point_iter_row (&self, row: U) -> PointIterator {
        PointIterator::row(self, row)
    }
    
    pub fn point_iter_interval (&self, start: &Point, end: &Point) -> PointIterator {
        PointIterator::new_interval(self, start, end)
    }
}




