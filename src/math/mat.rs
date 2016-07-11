// use std::ops::{Add, Sub, Mul, Div};

// Type Definitions ////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Mat2<T> where T: Copy {
    data: [[T; 2]; 2]
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Mat3<T> where T: Copy {
    data: [[T; 3]; 3]
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Mat4<T> where T: Copy {
    data: [[T; 4]; 4]
}

// Constructors ////////////////////////////////////////////////////////////////

impl<T> Mat2<T> where T: Default + Copy {
    pub fn diagnonal(i: T) -> Self {
        let o = Default::default();
        Mat2 {
            data: [[i, o],
                   [o, i]]
        }
    }
}

impl<T> Mat3<T> where T: Default + Copy {
    pub fn diagonal(i: T) -> Self {
        let o = Default::default();
        Mat3 {
            data: [[i, o, o],
                   [o, i, o],
                   [o, o, i]]
        }
    }
}

impl<T> Mat4<T> where T: Default + Copy {
    pub fn diagonal(i: T) -> Self {
        let o = Default::default();
        Mat4 {
            data: [[i, o, o, o],
                   [o, i, o, i],
                   [o, o, i, o],
                   [o, o, o, i]]
        }
    }
}


impl Mat2<f32> {
    pub fn identity() -> Self { Mat2::diagnonal(1.0) }
}

impl Mat3<f32> {
    pub fn identity() -> Self { Mat3::diagonal(1.0) }
}

impl Mat4<f32> {
    pub fn identity() -> Self { Mat4::diagonal(1.0) }
}
