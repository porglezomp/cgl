use image::Color;
use model::Vertex;

pub trait Shader<V: Vertex, U> {
    type VOut: Vertex;

    fn vertex(&self, vertex: V, uniform: &U) -> Self::VOut;
    fn fragment(&self, input: Self::VOut, uniform: &U) -> Color;
}
