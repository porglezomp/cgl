use math::{Vec2, Vec3, barycentric};
use image::{Image, Color};
use shader::Shader;
use model::{Model, Vertex};

pub struct Renderer {
    color: Image<Color>,
    zbuf: Image<f32>,
}

impl Renderer {
    pub fn with_dimensions(w: usize, h: usize) -> Self {
        Renderer {
            color: Image::with_dimensions(w, h),
            zbuf: Image::filled(w, h, 0.0),
        }
    }

    pub fn width(&self) -> usize { self.color.width }
    pub fn height(&self) -> usize { self.color.height }
    pub fn image(&self) -> &Image<Color> { &self.color }

    pub fn line(&mut self, t0: Vec3<isize>, t1: Vec3<isize>, color: Color) {
        self.color.line(t0.0, t0.1, t1.0, t1.1, color);
    }

    pub fn triangle(&mut self, t0: Vec3<isize>, t1: Vec3<isize>,
                     t2: Vec3<isize>, color: Color)
    {
        let ((x0, y0), (x1, y1)) = self.clip(t0, t1, t2);

        for x in (x0..x1).chain(Some(x1)) {
            for y in (y0..y1).chain(Some(y1)) {
                let bc_screen = barycentric((t0.into(), t1.into(), t2.into()),
                                            Vec2(x as isize, y as isize));
                if bc_screen.0 < 0.0 || bc_screen.1 < 0.0 || bc_screen.2 < 0.0 {
                    continue;
                }
                let z = t0.2 as f32 * bc_screen.0 +
                    t1.2 as f32 * bc_screen.1 +
                    t2.2 as f32 * bc_screen.2;
                if self.zbuf[(x as usize, y as usize)] < z {
                    self.zbuf[(x as usize, y as usize)] = z;
                    self.color[(x as usize, y as usize)] = color;
                }
            }
        }
    }

    pub fn tri<S, V, U>(&mut self, shader: &S, uniform: &U, t0: V, t1: V, t2: V)
        where V: Vertex, S: Shader<V, U>
    {
        let v0 = shader.vertex(t0, uniform);
        let v1 = shader.vertex(t1, uniform);
        let v2 = shader.vertex(t2, uniform);
        macro_rules! as_isize { ($x:ident) => {{
            let t = $x.pos();
            Vec3(t.0 as isize, t.1 as isize, t.2 as isize)
        }}}
        let t0 = as_isize!(v0);
        let t1 = as_isize!(v1);
        let t2 = as_isize!(v2);
        let ((x0, y0), (x1, y1)) = self.clip(t0, t1, t2);

        for x in (x0..x1).chain(Some(x1)) {
            for y in (y0..y1).chain(Some(y1)) {
                let bc_screen = barycentric((t0.into(), t1.into(), t2.into()),
                                            Vec2(x as isize, y as isize));
                if bc_screen.0 < 0.0 || bc_screen.1 < 0.0 || bc_screen.2 < 0.0 {
                    continue;
                }

                let vert = <S::VOut as Vertex>::interpolate(bc_screen, &v0, &v1, &v2);
                if self.zbuf[(x as usize, y as usize)] < vert.pos().2 {
                    self.zbuf[(x as usize, y as usize)] = vert.pos().2;
                    self.color[(x as usize, y as usize)] = shader.fragment(vert, uniform);
                }
            }
        }
    }

    pub fn model<S, V, U>(&mut self, shader: &S, uniform: &U, model: &Model<V>)
        where V: Vertex + Copy, S: Shader<V, U>
    {
        for tri in &model.triangles {
            self.tri(shader, uniform,
                     model.vertices[tri[0] as usize],
                     model.vertices[tri[1] as usize],
                     model.vertices[tri[2] as usize]);
        }
    }

    fn clip(&self, t0: Vec3<isize>, t1: Vec3<isize>, t2: Vec3<isize>)
            -> ((isize, isize), (isize, isize))
    {
        use std::cmp::{min, max};
        let x0 = max(0, min(min(t0.0, min(t1.0, t2.0)), (self.width() - 1) as isize));
        let x1 = max(0, min(max(t0.0, max(t1.0, t2.0)), (self.width() - 1) as isize));
        let y0 = max(0, min(min(t0.1, min(t1.1, t2.1)), (self.height() - 1) as isize));
        let y1 = max(0, min(max(t0.1, max(t1.1, t2.1)), (self.height() - 1) as isize));
        ((x0, y0), (x1, y1))
    }
}

