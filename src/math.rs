use std::ops::{Add, Sub, Mul};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec2<T: Clone+Copy>(pub T, pub T);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3<T: Clone+Copy>(pub T, pub T, pub T);

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

// impl<F, I> Into<Vec3<I>> for Vec3<F>
//     where F: Into<I>
// {
//     fn into(self) -> Vec3<F> {
//         Vec3(self.0.into(), self.1.into(), self.2.into())
//     }
// }

impl<F, I> From<Vec3<F>> for Vec2<I>
    where I: From<F> + Copy,
          F: Copy
{
    fn from(from: Vec3<F>) -> Vec2<I> {
        Vec2(from.0.into(), from.1.into())
    }
}

pub fn barycentric((t0, t1, t2): (Vec2<isize>, Vec2<isize>, Vec2<isize>),
               point: Vec2<isize>)
               -> Vec3<f32>
{
    let u = Vec3((t2.0-t0.0) as f32, (t1.0-t0.0) as f32, (t0.0-point.0) as f32)
        .cross(Vec3((t2.1-t0.1) as f32, (t1.1-t0.1) as f32, (t0.1-point.1) as f32));
    if u.2.abs() < 1.0 {
        Vec3(-1.0, 1.0, 1.0)
    } else {
        Vec3(1.0 - (u.0 + u.1)/u.2, u.1/u.2, u.0/u.2)
    }
}
