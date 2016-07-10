use std::ops::{Add, Sub, Mul};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec2<T: Clone+Copy>(pub T, pub T);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3<T: Clone+Copy>(pub T, pub T, pub T);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec4<T: Clone+Copy>(pub T, pub T, pub T, pub T);

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

    pub fn len2(&self) -> T {
        self.dot(*self)
    }
}

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

impl<T> Add for Vec3<T>
    where T: Add<Output=T> + Copy
{
    type Output = Vec3<T>;

    fn add(self, other: Vec3<T>) -> Vec3<T> {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl<T> Sub for Vec3<T>
    where T: Sub<Output=T> + Copy
{
    type Output = Vec3<T>;

    fn sub(self, other: Vec3<T>) -> Vec3<T> {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl<T> Mul<T> for Vec3<T>
    where T: Mul<Output=T> + Copy
{
    type Output = Vec3<T>;

    fn mul(self, other: T) -> Vec3<T> {
        Vec3(self.0 * other, self.1 * other, self.2 * other)
    }
}

// Conversions ////////////////////////////////////////////////////////////////

impl<F, I> From<Vec3<F>> for Vec2<I>
    where I: From<F> + Copy,
          F: Copy
{
    fn from(from: Vec3<F>) -> Vec2<I> {
        Vec2(from.0.into(), from.1.into())
    }
}
