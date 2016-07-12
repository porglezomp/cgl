use math::{Vec2, Vec3};

pub trait Vertex {
    fn pos(&self) -> &Vec3<f32>;
    fn interpolate(x: Vec3<f32>, t0: &Self, t1: &Self, t2: &Self) -> Self;
}

#[derive(Debug, Clone, Copy)]
pub struct Vert {
    pub pos: Vec3<f32>,
    pub tex: Vec2<f32>,
    pub norm: Vec3<f32>,
}

impl Vertex for Vert {
    fn pos(&self) -> &Vec3<f32> { &self.pos }

    fn interpolate(x: Vec3<f32>, t0: &Self, t1: &Self, t2: &Self) -> Self {
        Vert {
            pos: t0.pos * x.0 + t1.pos * x.1 + t2.pos * x.2,
            tex: t0.tex * x.0 + t1.tex * x.1 + t2.tex * x.2,
            norm: t0.norm * x.0 + t1.norm * x.1 + t2.norm * x.2,
        }
    }
}

#[derive(Debug)]
pub struct Model<V: Vertex> {
    pub vertices: Vec<V>,
    pub triangles: Vec<[usize; 3]>,
}

impl Vertex for Vec3<f32> {
    fn pos(&self) -> &Vec3<f32> { &self }

    fn interpolate(x: Vec3<f32>, t0: &Self, t1: &Self, t2: &Self) -> Self {
        *t0 * x.0 + *t1 * x.1 + *t2 * x.2
    }
}
