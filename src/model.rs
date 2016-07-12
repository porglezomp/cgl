use math::Vec3;

pub trait Vertex {}

pub struct Model<V: Vertex> {
    pub vertices: Vec<V>,
    pub triangles: Vec<[usize; 3]>,
}

impl<T> Vertex for Vec3<T> where T: Copy {}
