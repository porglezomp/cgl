use std::ops::{Index, IndexMut};

pub mod obj;
pub mod bmp;
pub mod math;

use math::Vec2;

/// A type representing an RGB triple
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    /// The color of midnight
    pub fn black() -> Self { Default::default() }
    /// Like a beautiful dove
    pub fn white() -> Self { Color(255, 255, 255) }
    /// BLOOD!!!!
    pub fn red()   -> Self { Color(255, 0, 0) }
    /// Looks like plants, except way brighter
    pub fn green() -> Self { Color(0, 255, 0) }
    /// Somewhere between the color of the ocean and the sky
    pub fn blue()  -> Self { Color(0, 0, 255) }
}

/// Stores an image on the heap for drawing into
#[derive(Clone)]
pub struct Image {
    pixels: Vec<Color>,
    pub width: usize,
    pub height: usize,
}

impl Image {
    /// Creates a new `width` by `height` black `Image`.
    pub fn with_dimensions(width: usize, height: usize) -> Self {
        let mut pixels = Vec::with_capacity(width * height);
        pixels.resize(width * height, Default::default());
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
    pub fn with_pixels(width: usize, height: usize, pixels: &[Color]) -> Self {
        assert_eq!(pixels.len(), width * height);
        Image {
            pixels: Vec::from(pixels),
            width: width,
            height: height,
        }
    }

    /// A view of the image data as bytes
    pub fn bytes(&self) -> &[u8] {
        let start: *const Color = &self.pixels[0];
        let start: *const u8 = start as *const u8;
        unsafe {
            std::slice::from_raw_parts(start, self.pixels.len() * std::mem::size_of::<Color>())
        }
    }

    /// Draw a line into the image using Bresenham's Line Drawing Algorithm
    pub fn line(&mut self, mut x0: isize, mut y0: isize,
                mut x1: isize, mut y1: isize, color: Color)
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

    pub fn triangle(&mut self, mut t0: Vec2<isize>, mut t1: Vec2<isize>,
                    mut t2: Vec2<isize>, color: Color)
    {
        if t0.1 > t1.1 { std::mem::swap(&mut t0, &mut t1); }
        if t0.1 > t2.1 { std::mem::swap(&mut t0, &mut t2); }
        if t1.1 > t2.1 { std::mem::swap(&mut t1, &mut t2); }

        let long_dx = t2.0 - t0.0;
        let long_dy = t2.1 - t0.1;
        if long_dy == 0 { return; }
        let long_xoff = if t2.0 > t0.0 { 1 } else { -1 };
        let mut long_x = t0.0;
        let mut long_err = 0;

        let short_dx = (t1.0 - t0.0).abs();
        let short_dy = t1.1 - t0.1;
        let short_xoff = if t1.0 > t0.0 { 1 } else { -1 };
        let mut short_x = t0.0;
        let mut short_err = 0;
        for y in t0.1..t1.1 {
            if 0 <= y && y < self.height as isize {
                let (a, b) = if short_x < long_x { (short_x, long_x) } else { (long_x, short_x) };
                let a = std::cmp::max(a, 0);
                let b = std::cmp::min(b, (self.width - 1) as isize);
                for x in (a..b).chain(Some(b)) {
                    self[(x as usize, y as usize)] = color;
                }
            }
            long_err += long_dx;
            while long_err >= long_dy {
                long_x += long_xoff;
                long_err -= long_dy;
            }
            short_err += short_dx;
            while short_err >= short_dy {
                short_x += short_xoff;
                short_err -= short_dy;
            }
        }
        let short_dx = (t2.0 - t1.0).abs();
        let short_dy = t2.1 - t1.1;
        if short_dy == 0 { return; }
        let short_xoff = if t2.0 > t1.0 { 1 } else { -1 };
        let mut short_x = t1.0;
        let mut short_err = 0;
        for y in (t1.1..t2.1).chain(Some(t2.1)) {
            if 0 <= y && y < self.height as isize {
                let (a, b) = if short_x < long_x { (short_x, long_x) } else { (long_x, short_x) };
                let a = std::cmp::max(a, 0);
                let b = std::cmp::min(b, (self.width - 1) as isize);
                for x in (a..b).chain(Some(b)) {
                    self[(x as usize, y as usize)] = color;
                }
            }
            long_err += long_dx;
            while long_err >= long_dy {
                long_x += long_xoff;
                long_err -= long_dy;
            }
            short_err += short_dx;
            while short_err >= short_dy {
                short_x += short_xoff;
                short_err -= short_dy;
            }
        }
    }
}

impl Index<(usize, usize)> for Image {
    type Output = Color;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        assert!(x < self.width);
        assert!(y < self.height);
        &self.pixels[y * self.width + x]
    }
}

impl IndexMut<(usize, usize)> for Image {
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
        assert_eq!(<Color as Default>::default(), Color(0, 0, 0));
    }

    #[test]
    #[should_panic]
    fn incorrect_pixel_size() {
        Image::with_pixels(4, 4, &[Default::default(); 4]);
    }

    #[test]
    #[should_panic]
    fn out_of_bounds() {
        let im = Image::with_dimensions(4, 4);
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
