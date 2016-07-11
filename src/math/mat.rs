use std::ops::{Add, AddAssign, Mul, MulAssign};
use std::ops::{Index, IndexMut};

use super::Vec4;

// Type Definitions ////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Mat2<T: Copy> {
    cols: [[T; 2]; 2]
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Mat3<T: Copy> {
    cols: [[T; 3]; 3]
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub struct Mat4<T: Copy> {
    cols: [[T; 4]; 4]
}

// Constructors ////////////////////////////////////////////////////////////////

impl<T> Mat2<T> where T: Default + Copy {
    pub fn diagnonal(i: T) -> Self {
        let o = Default::default();
        Mat2 {
            cols: [[i, o],
                   [o, i]]
        }
    }
}

impl<T> Mat3<T> where T: Default + Copy {
    pub fn diagonal(i: T) -> Self {
        let o = Default::default();
        Mat3 {
            cols: [[i, o, o],
                   [o, i, o],
                   [o, o, i]]
        }
    }
}

impl<T> Mat4<T> where T: Default + Copy {
    pub fn diagonal(i: T) -> Self {
        let o = Default::default();
        Mat4 {
            cols: [[i, o, o, o],
                   [o, i, o, o],
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
                &self.cols[col][row]
            }
        }

        impl<T> IndexMut<(usize, usize)> for $M<T> where T: Copy {
            fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
                assert!(row < $n);
                assert!(col < $n);
                &mut self.cols[col][row]
            }
        }
    }
}

impl_indexing!(Mat2 2);
impl_indexing!(Mat3 3);
impl_indexing!(Mat4 4);

// Matrix Arithmetic ///////////////////////////////////////////////////////////

impl<T> Mat4<T> where T: Copy {
    pub fn row(&self, row: usize) -> Vec4<T> {
        assert!(row < 4);
        Vec4(self[(row, 0)], self[(row, 1)], self[(row, 2)], self[(row, 3)])
    }

    pub fn col(&self, col: usize) -> Vec4<T> {
        assert!(col < 4);
        let col = self.cols[col];
        Vec4(col[0], col[1], col[2], col[3])
    }
}

impl Mul for Mat4<f32> {
    type Output = Mat4<f32>;

    fn mul(self, other: Mat4<f32>) -> Self::Output {
        let mut out = Mat4::identity();
        for row in 0..4 {
            for col in 0..4 {
                out[(row, col)] = self.row(row).dot(other.col(col));
            }
        }
        out
    }
}

impl<T> Mul<T> for Mat4<T> where T: MulAssign + Copy {
    type Output = Mat4<T>;

    fn mul(mut self, other: T) -> Self::Output {
        for col in self.cols.iter_mut() {
            for entry in col.iter_mut() {
                *entry *= other;
            }
        }
        self
    }
}

impl Mul<Vec4<f32>> for Mat4<f32> {
    type Output = Vec4<f32>;

    fn mul(self, other: Vec4<f32>) -> Self::Output {
        Vec4(self.row(0).dot(other),
             self.row(1).dot(other),
             self.row(2).dot(other),
             self.row(3).dot(other))
    }
}

impl<T> Add for Mat4<T> where T: AddAssign + Copy {
    type Output = Mat4<T>;

    fn add(mut self, other: Mat4<T>) -> Self::Output {
        for row in 0..4 {
            for col in 0..4 {
                self[(row, col)] += other[(row, col)]
            }
        }
        self
    }
}
