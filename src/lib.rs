use std::ops::{Index, IndexMut};

pub mod obj;
pub mod bmp;
pub mod math;

use math::{Vec2, Vec3, barycentric};

/// A type representing a BGR triple
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Color {
    pub b: u8,
    pub g: u8,
    pub r: u8,
}

impl Color {
    /// Initializes a color with the given RGB values
    pub fn rgb(r: u8, g: u8, b: u8) -> Self { Color { r: r, g: g, b: b } }
    /// The color of midnight
    pub fn black() -> Self { Default::default() }
    /// Like a beautiful dove
    pub fn white() -> Self { Color::rgb(255, 255, 255) }
    /// BLOOD!!!!
    pub fn red()   -> Self { Color::rgb(255, 0, 0) }
    /// Looks like plants, except way brighter
    pub fn green() -> Self { Color::rgb(0, 255, 0) }
    /// Somewhere between the color of the ocean and the sky
    pub fn blue()  -> Self { Color::rgb(0, 0, 255) }
}

/// Stores an image on the heap for drawing into
#[derive(Clone)]
pub struct Image<Pix> {
    pixels: Vec<Pix>,
    pub width: usize,
    pub height: usize,
}

impl<Pix> Image<Pix>
    where Pix: Clone + Copy + Default
{
    /// Creates a new `width` by `height` `Image` filled with the default value
    /// for `Pix`. This function is equivalent to calling `Image::filled(width,
    /// height, Default::default())`.
    pub fn with_dimensions(width: usize, height: usize) -> Self {
        Image::filled(width, height, Default::default())
    }

    /// Creates a new `width` by `height` `Image` where every pixel has the value `fill`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut zbuf = Image::filled(512, 512, std::f32::MIN);
    /// ```
    pub fn filled(width: usize, height: usize, fill: Pix) -> Self {
        let mut pixels = Vec::with_capacity(width * height);
        pixels.resize(width * height, fill);
        Image {
            pixels: pixels,
            width: width,
            height: height,
        }
    }

    /// Create a new image containing the data in `pixels`
    ///
    /// # Panics
    ///
    /// Panics if `pixels` doesn't have `width * height` elements
    pub fn with_pixels(width: usize, height: usize, pixels: &[Pix]) -> Self {
        assert_eq!(pixels.len(), width * height);
        Image {
            pixels: Vec::from(pixels),
            width: width,
            height: height,
        }
    }

    /// A view of the image data as bytes
    pub fn bytes(&self) -> &[u8] {
        let start: *const Pix = &self.pixels[0];
        let start: *const u8 = start as *const u8;
        unsafe {
            std::slice::from_raw_parts(start, self.pixels.len() * std::mem::size_of::<Pix>())
        }
    }

    /// Draw a line into the image using Bresenham's Line Drawing Algorithm
    pub fn line(&mut self, mut x0: isize, mut y0: isize,
                mut x1: isize, mut y1: isize, color: Pix)
    {
        let steep = (x0 - x1).abs() < (y0 - y1).abs();
        if steep {
            std::mem::swap(&mut x0, &mut y0);
            std::mem::swap(&mut x1, &mut y1);
        }
        if x0 > x1 {
            std::mem::swap(&mut x0, &mut x1);
            std::mem::swap(&mut y0, &mut y1);
        }
        let (x0, y0, x1, y1) = (x0, y0, x1, y1);
        let (dx, dy) = (x1 - x0, y1 - y0);
        let derror = dy.abs()*2;
        let yoff = if y1 > y0 { 1 } else { -1 };
        let mut error = 0;
        let mut y = y0;

        for x in (x0..x1).chain(Some(x1)) {
            if 0 <= x && x < self.width as isize &&
                    0 <= y && y < self.height as isize {
                if steep {
                    self[(y as usize, x as usize)] = color;
                } else {
                    self[(x as usize, y as usize)] = color;
                }
            }
            error += derror;
            if error > dx {
                y += yoff;
                error -= dx*2;
            }
        }
    }

    pub fn triangle(&mut self, t0: Vec2<isize>, t1: Vec2<isize>,
                    t2: Vec2<isize>, color: Pix)
    {
        use std::cmp::{min, max};
        let x0 = max(0, min(min(t0.0, min(t1.0, t2.0)), (self.width - 1) as isize));
        let x1 = max(0, min(max(t0.0, max(t1.0, t2.0)), (self.width - 1) as isize));
        let y0 = max(0, min(min(t0.1, min(t1.1, t2.1)), (self.height - 1) as isize));
        let y1 = max(0, min(max(t0.1, max(t1.1, t2.1)), (self.height - 1) as isize));
        for x in (x0..x1).chain(Some(x1)) {
            for y in (y0..y1).chain(Some(y1)) {
                let bc_screen = barycentric((t0, t1, t2), Vec2(x, y));
                if bc_screen.0 < 0.0 || bc_screen.1 < 0.0 || bc_screen.2 < 0.0 {
                    continue;
                }
                self[(x as usize, y as usize)] = color;
            }
        }
    }

    pub fn triangle3(&mut self, t0: Vec3<f32>, t1: Vec3<f32>, t2: Vec3<f32>,
                     zbuf: &mut Image<f32>, color: Pix)
    {
        assert_eq!((self.width, self.height), (zbuf.width, zbuf.height));
        fn world2screen<T>(image: &Image<T>, x: Vec3<f32>) -> Vec2<isize> {
            Vec2(((x.0 + 1.0) * 0.5 * image.width as f32) as isize,
                 ((-x.1 + 1.0) * 0.5 * image.height as f32) as isize)
        }

        let s0 = world2screen(self, t0);
        let s1 = world2screen(self, t1);
        let s2 = world2screen(self, t2);

        use std::cmp::{min, max};
        let x0 = max(0, min(min(s0.0, min(s1.0, s2.0)), (self.width - 1) as isize));
        let x1 = max(0, min(max(s0.0, max(s1.0, s2.0)), (self.width - 1) as isize));
        let y0 = max(0, min(min(s0.1, min(s1.1, s2.1)), (self.height - 1) as isize));
        let y1 = max(0, min(max(s0.1, max(s1.1, s2.1)), (self.height - 1) as isize));

        for x in (x0..x1).chain(Some(x1)) {
            for y in (y0..y1).chain(Some(y1)) {
                let bc_screen = barycentric((s0, s1, s2), Vec2(x, y));
                if bc_screen.0 < 0.0 || bc_screen.1 < 0.0 || bc_screen.2 < 0.0 {
                    continue;
                }
                let z = t0.2 * bc_screen.0 + t1.2 * bc_screen.1 + t2.2 * bc_screen.2;
                if zbuf[(x as usize, y as usize)] < z {
                    zbuf[(x as usize, y as usize)] = z;
                    self[(x as usize, y as usize)] = color;
                }
            }
        }
    }
}

impl<Pix> Index<(usize, usize)> for Image<Pix> {
    type Output = Pix;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        assert!(x < self.width);
        assert!(y < self.height);
        &self.pixels[y * self.width + x]
    }
}

impl<Pix> IndexMut<(usize, usize)> for Image<Pix> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        assert!(x < self.width);
        assert!(y < self.height);
        &mut self.pixels[y * self.width + x]
    }
}

#[cfg(test)]
mod tests {
    use super::{Color, Image};

    #[test]
    fn default_black() {
        assert_eq!(Color::default(), Color::rgb(0, 0, 0));
    }

    #[test]
    #[should_panic]
    fn incorrect_pixel_size() {
        Image::with_pixels(4, 4, &[Color::black(); 4]);
    }

    #[test]
    #[should_panic]
    fn out_of_bounds() {
        let im: Image<Color> = Image::with_dimensions(4, 4);
        im[(3, 5)];
    }

    #[test]
    fn set_colors() {
        let mut im = Image::with_dimensions(4, 4);
        im[(0, 0)] = Color::white();
        assert_eq!(im[(0, 0)], Color::white());
    }

    #[test]
    fn draw_line() {
        let mut im = Image::with_dimensions(4, 4);
        im.line(0, 0, 3, 0, Color::white());
        for x in 0..4 {
            assert_eq!(im[(x, 0)], Color::white());
            assert_eq!(im[(x, 1)], Color::black());
        }

        let mut im = Image::with_dimensions(4, 4);
        im.line(0, 0, 3, 3, Color::blue());
        for i in 0..4 {
            assert_eq!(im[(i, i)], Color::blue());
        }
        assert_eq!(im[(3, 0)], Color::black());
        assert_eq!(im[(0, 3)], Color::black());

        let mut im = Image::with_dimensions(4, 4);
        im.line(0, 0, 0, 3, Color::red());
        for y in 0..4 {
            assert_eq!(im[(0, y)], Color::red());
            assert_eq!(im[(1, y)], Color::black())
        }
    }
}
