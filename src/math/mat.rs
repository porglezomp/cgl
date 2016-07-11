// use std::ops::{Add, Sub, Mul, Div};
use std::ops::{Index, IndexMut};

// Type Definitions ////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Mat2<T: Copy> {
    data: [[T; 2]; 2]
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Mat3<T: Copy> {
    data: [[T; 3]; 3]
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Mat4<T: Copy> {
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

// Indexing ////////////////////////////////////////////////////////////////////

macro_rules! impl_indexing {
    ($M:ident $n:expr) => {
        impl<T> Index<(usize, usize)> for $M<T> where T: Copy {
            type Output = T;

            fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
                assert!(row < $n);
                assert!(col < $n);
                &self.data[row][col]
            }
        }

        impl<T> IndexMut<(usize, usize)> for $M<T> where T: Copy {
            fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
                assert!(row < $n);
                assert!(col < $n);
                &mut self.data[row][col]
            }
        }
    }
}

impl_indexing!(Mat2 2);
impl_indexing!(Mat3 3);
impl_indexing!(Mat4 4);
