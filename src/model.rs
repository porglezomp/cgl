use math::{Vec2, Vec3};

pub trait Vertex {}

#[derive(Debug)]
pub struct Vert {
    pub pos: Vec3<f32>,
    pub tex: Vec2<f32>,
    pub norm: Vec3<f32>,
}

impl Vertex for Vert {}

#[derive(Debug)]
pub struct Model<V: Vertex> {
    pub vertices: Vec<V>,
    pub triangles: Vec<[usize; 3]>,
}

impl<T> Vertex for Vec3<T> where T: Copy {}
