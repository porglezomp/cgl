pub mod vec;
pub mod mat;

pub use self::vec::{Vec2, Vec3, Vec4};
pub use self::mat::{Mat2, Mat3, Mat4};

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
