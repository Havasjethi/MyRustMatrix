// use super::*;

pub type F = f32;
pub type U = usize;

pub enum SumMode{
    Cropped,
    Calculted,
    DefaultValue(F)
}

#[derive(Copy, Clone)]
pub struct Point(
    pub U,
    pub U
);

impl Point { pub fn new (x: U,y: U) -> Point {Point(x,y)}}


#[derive(std::fmt::Debug)]
pub struct Matrix {
    pub x: U,
    pub y: U,
    pub matrix: Vec<Vec<F>>,
}
