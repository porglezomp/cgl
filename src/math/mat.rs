use std::ops::{Add, AddAssign, Mul, MulAssign};
use std::ops::{Index, IndexMut};

use super::{Vec2, Vec3, Vec4};


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
    pub fn new(rows: [[T; 2]; 2]) -> Self {
        Mat2 {
            cols: rows
        }.transpose()
    }

    pub fn diagnonal(i: T) -> Self {
        let o = Default::default();
        Mat2 {
            cols: [[i, o],
                   [o, i]]
        }
    }
}

impl<T> Mat3<T> where T: Default + Copy {
    pub fn new(rows: [[T; 3]; 3]) -> Self {
        Mat3 {
            cols: rows
        }.transpose()
    }

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
    pub fn new(rows: [[T; 4]; 4]) -> Self {
        Mat4 {
            cols: rows
        }.transpose()
    }

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

    pub fn perspective(c: f32) -> Self {
        let mut p = Mat4::identity();
        p[(3, 2)] = -1.0 / c;
        p
    }

    pub fn lookat(eye: Vec3<f32>, center: Vec3<f32>, up: Vec3<f32>) -> Self {
        let z = (center-eye).normalized();
        let x = up.cross(z).normalized();
        let y = z.cross(x).normalized();
        let mut minv = Mat4::identity();
        let mut tr = Mat4::identity();
        for i in 0..3 {
            minv[(0, i)] = x[i];
            minv[(1, i)] = y[i];
            minv[(2, i)] = z[i];
            tr[(i, 3)] = -center[i];
        }
        minv * tr
    }

    pub fn viewport(w: i32, h: i32) -> Self {
        let mut m = Mat4::identity();
        let depth = 256.0;

        m[(0, 3)] = w as f32 / 2.0;
        m[(1, 3)] = h as f32 / 2.0;
        m[(2, 3)] = depth / 2.0;

        m[(0, 0)] = w as f32 / 2.0;
        m[(1, 1)] = -h as f32 / 2.0;
        m[(2, 2)] = depth / 2.0;

        m
    }

    pub fn translate(offset: Vec3<f32>) -> Self {
        let mut m = Mat4::identity();
        for i in 0..3 { m[(0, i)] = offset[i]; }
        m
    }

    pub fn scale(factor: Vec3<f32>) -> Self {
        let mut m = Mat4::identity();
        for i in 0..3 { m[(i, i)] = factor[i]; }
        m
    }
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


// Row Operations //////////////////////////////////////////////////////////////

macro_rules! impl_row_ops {
    ($M:ident , $V:ident , $n:expr ; $($x:expr),*) => {
        impl $M<f32> {
            pub fn swap_rows(&mut self, a: usize, b: usize) {
                if a == b { return; }
                for i in 0..$n {
                    let c = self[(a, i)];
                    self[(a, i)] = self[(b, i)];
                    self[(b, i)] = c;
                }
            }

            pub fn add_row(&mut self, row: usize, other_row: $V<f32>) {
                for i in 0..$n {
                    self[(row, i)] += other_row[i];
                }
            }

            pub fn mul_row(&mut self, row: usize, factor: f32) {
                for i in 0..$n {
                    self[(row, i)] *= factor;
                }
            }
        }

        impl<T> $M<T> where T: Copy {
            pub fn row(&self, row: usize) -> $V<T> {
                assert!(row < 4);
                $V($(self[(row, $x)]),*)
            }

            pub fn col(&self, col: usize) -> $V<T> {
                assert!(col < 4);
                let col = self.cols[col];
                $V($(col[$x]),*)
            }
        }
    }
}

impl_row_ops!(Mat2, Vec2, 2; 0, 1);
impl_row_ops!(Mat3, Vec3, 3; 0, 1, 2);
impl_row_ops!(Mat4, Vec4, 4; 0, 1, 2, 3);


// Matrix Arithmetic ///////////////////////////////////////////////////////////

macro_rules! impl_ops {
    ($M:ident , $V:ident , $n:expr ; $($x:tt),* ) => {
        impl Mul for $M<f32> {
            type Output = $M<f32>;

            fn mul(self, other: $M<f32>) -> Self::Output {
                let mut out = $M::identity();
                for row in 0..$n {
                    for col in 0..$n {
                        out[(row, col)] = self.row(row).dot(other.col(col));
                    }
                }
                out
            }
        }

        impl<T> Mul<T> for $M<T> where T: MulAssign + Copy {
            type Output = $M<T>;

            fn mul(mut self, other: T) -> Self::Output {
                for col in self.cols.iter_mut() {
                    for entry in col.iter_mut() {
                        *entry *= other;
                    }
                }
                self
            }
        }

        impl Mul<$V<f32>> for $M<f32> {
            type Output = $V<f32>;

            fn mul(self, other: $V<f32>) -> Self::Output {
                $V($(self.row($x).dot(other)),*)
                    // self.row(0).dot(other),
                    //  self.row(1).dot(other),
                    //  self.row(2).dot(other),
                    //  self.row(3).dot(other))
            }
        }

        impl<T> Add for $M<T> where T: AddAssign + Copy {
            type Output = $M<T>;

            fn add(mut self, other: $M<T>) -> Self::Output {
                for row in 0..$n {
                    for col in 0..$n {
                        self[(row, col)] += other[(row, col)]
                    }
                }
                self
            }
        }
    }
}

impl_ops!(Mat2, Vec2, 2; 0, 1);
impl_ops!(Mat3, Vec3, 3; 0, 1, 2);
impl_ops!(Mat4, Vec4, 4; 0, 1, 2, 3);


// Inversion ///////////////////////////////////////////////////////////////////


impl Mat2<f32> {
    pub fn inverted(&self) -> Result<Self, ()> {
        let mut other = *self;
        other.invert()?;
        Ok(other)
    }

    pub fn invert(&mut self) -> Result<(), ()> {
        let (a, b, c, d) = (self[(0, 0)], self[(0, 1)], self[(1, 0)], self[(1, 1)]);
        let det = a*d - b*c;
        if det == 0.0 {
            return Err(());
        }
        let fac = 1.0 / det;
        *self = Mat2::new([[d * fac, -b * fac],
                           [-c * fac, a * fac]]);
        Ok(())
    }
}

macro_rules! impl_invert {
    ($M:ident , $n:expr) => {
        impl $M<f32> {
            pub fn inverted(&self) -> Result<Self, ()> {
                let mut other = *self;
                other.invert()?;
                Ok(other)
            }

            pub fn invert(&mut self) -> Result<(), ()> {
                let mut id: Self = Self::identity();
                // Perform partial pivoting: for each row, make all of the entries above
                // and below the largest number in that row a zero by performing
                // elementary row operations.
                for row_idx in 0..$n {
                    let row = self.row(row_idx);
                    let row_array: [f32; $n] = row.into();
                    let (row_max_idx, pivot) =
                        row_array.iter().enumerate()
                        .fold((0, 0.0_f32), |(i, a), (j, &b)| {
                            if a.abs() > b.abs() { (i, a) } else { (j, b) }
                        });

                    if pivot == 0.0 {
                        return Err(());
                    }

                    let id_row = id.row(row_idx);
                    for other_row_idx in 0..$n {
                        if row_idx == other_row_idx {
                            continue;
                        }
                        let entry = self[(other_row_idx, row_max_idx)];
                        let factor = -entry / pivot;
                        self.add_row(other_row_idx, row * factor);
                        id.add_row(other_row_idx, id_row * factor);
                    }
                }

                // Swap rows to place the pivots along the diagonal
                for row_idx in 0..$n {
                    let row = self.row(row_idx);
                    let row_array: [f32; $n] = row.into();
                    let pos = row_array.iter().position(|&x| x != 0.0)
                        .expect("Every row should have a nonzero value due to the early return above");
                    self.swap_rows(row_idx, pos);
                    id.swap_rows(row_idx, pos);
                }

                // Scale each diagonal entry to be 1
                for idx in 0..$n {
                    let factor = 1.0/self[(idx, idx)];
                    // Debug
                    self.mul_row(idx, factor);
                    id.mul_row(idx, factor);
                }

                *self = id;
                Ok(())
            }
        }
    }
}

impl_invert!(Mat3, 3);
impl_invert!(Mat4, 4);


// Transpose ///////////////////////////////////////////////////////////////////

macro_rules! impl_transpose {
    ($M:ident , $n:expr) => {
        impl<T> $M<T> where T: Copy + Default {
            pub fn transpose(&self) -> Self {
                let mut copy = Self::default();
                for i in 0..$n {
                    for j in 0..$n {
                        copy[(i, j)] = self[(j, i)];
                        copy[(j, i)] = self[(i, j)];
                    }
                }
                copy
            }
        }
    }
}

impl_transpose!(Mat2, 2);
impl_transpose!(Mat3, 3);
impl_transpose!(Mat4, 4);
