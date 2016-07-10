use std::ops::{Sub, Mul};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec2<T: Clone+Copy>(pub T, pub T);

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3<T: Clone+Copy>(pub T, pub T, pub T);

impl<T> Vec3<T>
    where T: Sub<Output=T> + Mul<Output=T> + Clone + Copy
{
    pub fn cross(self, other: Vec3<T>) -> Vec3<T> {
        Vec3(self.1 * other.2 - self.2 * other.1,
             self.2 * other.0 - self.0 * other.2,
             self.0 * other.1 - self.1 * other.0)
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
