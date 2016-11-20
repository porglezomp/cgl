use cgl_math::{Vec2, Vec3, Mat3};


// Vertex //////////////////////////////////////////////////////////////////////

pub trait Vertex {
    fn interpolate(x: Vec3<f32>, t0: &Self, t1: &Self, t2: &Self) -> Self;
}

impl Vertex for Vec3<f32> {
    fn interpolate(x: Vec3<f32>, t0: &Self, t1: &Self, t2: &Self) -> Self {
        *t0 * x.0 + *t1 * x.1 + *t2 * x.2
    }
}

impl Vertex for Vec2<f32> {
    fn interpolate(x: Vec3<f32>, t0: &Self, t1: &Self, t2: &Self) -> Self {
        *t0 * x.0 + *t1 * x.1 + *t2 * x.2
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vert {
    pub pos: Vec3<f32>,
    pub tex: Vec2<f32>,
    pub norm: Vec3<f32>,
}

impl Vertex for Vert {
    fn interpolate(x: Vec3<f32>, t0: &Self, t1: &Self, t2: &Self) -> Self {
        Vert {
            pos: Vertex::interpolate(x, &t0.pos, &t1.pos, &t2.pos),
            tex: Vertex::interpolate(x, &t0.tex, &t1.tex, &t2.tex),
            norm: Vertex::interpolate(x, &t0.norm, &t1.norm, &t2.norm),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TanVert {
    pub pos: Vec3<f32>,
    pub tex: Vec2<f32>,
    pub norm: Vec3<f32>,
    pub tan: Vec3<f32>,
    pub bitan: Vec3<f32>,
}

impl Vertex for TanVert {
    fn interpolate(x: Vec3<f32>, t0: &Self, t1: &Self, t2: &Self) -> Self {
        TanVert {
            pos: Vertex::interpolate(x, &t0.pos, &t1.pos, &t2.pos),
            tex: Vertex::interpolate(x, &t0.tex, &t1.tex, &t2.tex),
            norm: Vertex::interpolate(x, &t0.norm, &t1.norm, &t2.norm),
            tan: Vertex::interpolate(x, &t0.tan, &t1.tan, &t2.tan),
            bitan: Vertex::interpolate(x, &t0.bitan, &t1.bitan, &t2.bitan),
        }
    }
}


// Model ///////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Model<V: Vertex> {
    pub vertices: Vec<V>,
    pub triangles: Vec<[usize; 3]>,
}

pub fn compute_tangent_space(model: Model<Vert>) -> Model<TanVert> {
    let mut triangle_tangent_bundles = vec![vec![]; model.vertices.len()];
    let mut triangle_bitangent_bundles = vec![vec![]; model.vertices.len()];

    for tri in &model.triangles {
        let v0 = model.vertices[tri[0]];
        let v1 = model.vertices[tri[1]];
        let v2 = model.vertices[tri[2]];
        let edge1 =(v1.pos-v0.pos).into();
        let edge2 = (v2.pos-v0.pos).into();
        let a0 = Mat3::new([edge1, edge2, v0.norm.into()]).inverted().unwrap();
        let a1 = Mat3::new([edge1, edge2, v1.norm.into()]).inverted().unwrap();
        let a2 = Mat3::new([edge1, edge2, v2.norm.into()]).inverted().unwrap();
        let uvec = Vec3(v1.tex.0-v0.tex.0, v2.tex.0-v0.tex.0, 0.0);
        let vvec = Vec3(v1.tex.1-v0.tex.1, v2.tex.1-v0.tex.1, 0.0);
        triangle_tangent_bundles[tri[0]].push(a0 * uvec);
        triangle_tangent_bundles[tri[1]].push(a1 * uvec);
        triangle_tangent_bundles[tri[2]].push(a2 * uvec);
        triangle_bitangent_bundles[tri[0]].push(a0 * vvec);
        triangle_bitangent_bundles[tri[1]].push(a1 * vvec);
        triangle_bitangent_bundles[tri[2]].push(a2 * vvec);
    }

    let triangle_tangents = triangle_tangent_bundles.into_iter()
        .map(|v| v.into_iter()
             .fold(Vec3(0.0, 0.0, 0.0), |a, b| a + b)
             .normalized());
    let triangle_bitangents = triangle_bitangent_bundles.into_iter()
        .map(|v| v.into_iter()
             .fold(Vec3(0.0, 0.0, 0.0), |a, b| a + b)
             .normalized());

    Model {
        vertices: model.vertices.into_iter()
            .zip(triangle_tangents)
            .zip(triangle_bitangents)
            .map(|((v, tan), bitan)| {
                TanVert {
                    pos: v.pos,
                    tex: v.tex,
                    norm: v.norm,
                    tan: tan,
                    bitan: bitan,
                }
            }).collect::<Vec<_>>(),
        triangles: model.triangles,
    }
}
