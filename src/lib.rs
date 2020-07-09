#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

pub mod matrix;

pub fn mast () {

}
// fn main() {
//     println!("Hello, world!");
    
//     // let mut m1 = Matrix::new(5, 5);
//     let mut m1 = Matrix::square_with_default(5, 3 as F);
//     m1.set(0,1, 5 as F);
    
//     println!("Hello, world! ~ {}", m1.get(0,1));
//     let mut k1 = Matrix::square_with_default(3, 1 as F);

//     // m1.set(2, 1, 5.2f32);
    
//     m1.convolute(&k1, SumMode::Cropped).print();
//     // m1.print();
// }

#[cfg(test)]
mod matrix_tests {
    use super::matrix::types::*;

    #[test]
    fn basic () {
        let m = Matrix::new(3, 4);
        assert_eq!(m.x, 3);
        assert_eq!(m.y, 4);
    }

    #[test]
    fn matrix_creation() {
        let arr: [[F; 3]; 2] = [ [1f32, 2f32, 3f32], [1f32, 2f32, 3f32]];
        // let m = Matrix::new_matrix(&arr);
        let m = Matrix::new_matrix(&[ &[1f32, 2f32, 3f32], &[1f32, 2f32, 3f32]]);
        assert_eq!(m.x, 3);
        assert_eq!(m.y, 2);

    }
    #[test]
    #[should_panic]
    fn matrix_creation_bad() {
        let m = Matrix::new_matrix(&[ &[1f32, 2f32, 3f32], &[1f32, 2f32, 3f32], &[1f32]]);
    }
    
    // fn square() {
    //     assert_eq!(Matrix::new(5, 5), Matrix::square(5))
    // }   
    #[test]
    fn eq () {
        assert!(Matrix::new(5, 5).equal(&Matrix::square(5)))
    }

    #[test]
    fn matrix_add () {
        assert_eq!(2+2, 4)
    }
    
    #[test]
    fn convolute() {
        let mut m1 = Matrix::square_with_default(5, 3 as F);
        m1.set(0,1, 5 as F);
    
        let mut k1 = Matrix::square_with_default(3, 1 as F);
        m1.convolute(&k1, SumMode::Cropped).print();
        m1.print();
        assert_eq!(2+2, 4)
    }
}
