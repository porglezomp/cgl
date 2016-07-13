use image::Color;
use model::Vertex;
use math::Vec4;

pub trait Shader<V: Vertex, U> {
    type VOut: Vertex;

    fn vertex(&self, vertex: V, uniform: &U, pos: &mut Vec4<f32>) -> Self::VOut;
    fn fragment(&self, input: Self::VOut, uniform: &U) -> Color;
}
