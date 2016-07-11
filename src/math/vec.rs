use std::ops::{Add, Sub, Mul, Div};

// Type Definitions ////////////////////////////////////////////////////////////

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec2<T: Clone+Copy>(pub T, pub T);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3<T: Clone+Copy>(pub T, pub T, pub T);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec4<T: Clone+Copy>(pub T, pub T, pub T, pub T);

// Dot-product Etc. ////////////////////////////////////////////////////////////

impl<T> Vec2<T>
    where T: Add<Output=T> + Mul<Output=T> + Copy
{
    pub fn dot(&self, other: Vec2<T>) -> T {
        self.0 * other.0 + self.1 * other.1
    }

    pub fn len2(&self) -> T { self.dot(*self) }
}

impl<T> Vec3<T>
    where T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Copy
{
    pub fn cross(&self, other: Vec3<T>) -> Vec3<T> {
        Vec3(self.1 * other.2 - self.2 * other.1,
             self.2 * other.0 - self.0 * other.2,
             self.0 * other.1 - self.1 * other.0)
    }

    pub fn dot(&self, other: Vec3<T>) -> T {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn len2(&self) -> T { self.dot(*self) }
}

impl<T> Vec4<T>
    where T: Add<Output=T> + Mul<Output=T> + Copy
{
    pub fn dot(&self, other: Vec4<T>) -> T {
        self.0 * other.0 + self.1 * other.1 +
            self.2 * other.2 + self.3 * other.3
    }

    pub fn len2(&self) -> T { self.dot(*self) }
}

// Magnitude ///////////////////////////////////////////////////////////////////

impl Vec3<f32> {
    pub fn len(&self) -> f64 {
        (self.len2() as f64).sqrt()
    }

    pub fn normalized(self) -> Vec3<f32> {
        self * (1.0/self.len()) as f32
    }
}

impl Vec3<f64> {
    pub fn len(&self) -> f64 {
        self.len2().sqrt()
    }

    pub fn normalized(self) -> Vec3<f64> {
        self * (1.0/self.len())
    }
}

// Vector Arithmetic ///////////////////////////////////////////////////////////

macro_rules! expr { ($a:expr) => ($a) }
macro_rules! vector_op {
    ($V:ident<$T:ident> : $Trait:ident ($name:ident) ($op:tt) {$($part:tt),*}) => {
        impl<$T> $Trait for $V<$T> where $T: $Trait<Output=T> + Copy {
            type Output = $V<$T>;

            fn $name(self, other: $V<$T>) -> Self::Output {
                $V($(expr!(self.$part $op other.$part)),*)
            }
        }
    }
}

vector_op!(Vec2<T>: Add (add) (+) { 0, 1 });
vector_op!(Vec2<T>: Sub (sub) (-) { 0, 1 });
vector_op!(Vec3<T>: Add (add) (+) { 0, 1, 2 });
vector_op!(Vec3<T>: Sub (sub) (-) { 0, 1, 2 });
vector_op!(Vec4<T>: Add (add) (+) { 0, 1, 2, 3 });
vector_op!(Vec4<T>: Sub (sub) (-) { 0, 1, 2, 3 });

macro_rules! scalar_op {
    ($V:ident<$T:ident> : $Trait:ident ($name:ident) ($op:tt) {$($part:tt),*}) => {
        impl<$T> $Trait<$T> for $V<$T> where $T: $Trait<Output=T> + Copy {
            type Output = $V<$T>;

            fn $name(self, other: $T) -> Self::Output {
                $V($(expr!(self.$part $op other)),*)
            }
        }
    }
}

scalar_op!(Vec2<T>: Mul (mul) (*) { 0, 1 });
scalar_op!(Vec2<T>: Div (div) (/) { 0, 1 });
scalar_op!(Vec3<T>: Mul (mul) (*) { 0, 1, 2 });
scalar_op!(Vec3<T>: Div (div) (/) { 0, 1, 2 });
scalar_op!(Vec4<T>: Mul (mul) (*) { 0, 1, 2, 3});
scalar_op!(Vec4<T>: Div (div) (/) { 0, 1, 2, 3});

// Conversions /////////////////////////////////////////////////////////////////

impl<F, I> From<Vec3<F>> for Vec2<I>
    where I: From<F> + Copy,
          F: Copy
{
    fn from(from: Vec3<F>) -> Vec2<I> {
        Vec2(from.0.into(), from.1.into())
    }
}

impl<F, I> From<Vec4<F>> for Vec3<I>
    where I: From<F> + Copy,
          F: Copy
{
    fn from(from: Vec4<F>) -> Vec3<I> {
        Vec3(from.0.into(), from.1.into(), from.2.into())
    }
}

macro_rules! impl_extend {
    ($($T: ty)*) => {
        $(impl Vec3<$T> {
            /// Extend a `Vec3` into a `Vec4` with the last component a 1. This
            /// is frequently useful for matrix multiplication by a 4x4 matrix,
            /// e.g. while doing perspective transformations.
            pub fn extend(&self) -> Vec4<$T> { Vec4(self.0, self.1, self.2, 1) }
        })*
    }
}

impl_extend!(u8 u16 u32 u64 i8 i16 i32 i64 usize isize);

impl Vec3<f32> {
    /// Extend a `Vec3` into a `Vec4` with the last component a 1. This is
    /// frequently useful for matrix multiplication by a 4x4 matrix, e.g. while
    /// doing perspective transformations.
    pub fn extend(&self) -> Vec4<f32> { Vec4(self.0, self.1, self.2, 1.0) }
}

impl Vec3<f64> {
    /// Extend a `Vec3` into a `Vec4` with the last component a 1. This is
    /// frequently useful for matrix multiplication by a 4x4 matrix, e.g. while
    /// doing perspective transformations.
    pub fn extend(&self) -> Vec4<f64> { Vec4(self.0, self.1, self.2, 1.0) }
}
