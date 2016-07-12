use math::{Vec2, Vec3, barycentric};
use image::{Image, Color};

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
        use std::cmp::{min, max};
        let x0 = max(0, min(min(t0.0, min(t1.0, t2.0)), (self.width() - 1) as isize));
        let x1 = max(0, min(max(t0.0, max(t1.0, t2.0)), (self.width() - 1) as isize));
        let y0 = max(0, min(min(t0.1, min(t1.1, t2.1)), (self.height() - 1) as isize));
        let y1 = max(0, min(max(t0.1, max(t1.1, t2.1)), (self.height() - 1) as isize));

        for x in (x0..x1).chain(Some(x1)) {
            for y in (y0..y1).chain(Some(y1)) {
                let bc_screen = barycentric((t0.into(), t1.into(), t2.into()), Vec2(x, y));
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
}
