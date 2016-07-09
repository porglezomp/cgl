use std::ops::{Index, IndexMut};

/// A type representing an RGB triple
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    pub fn black() -> Self { Default::default() }
    pub fn white() -> Self { Color(255, 255, 255) }
    pub fn red()   -> Self { Color(255, 0, 0) }
    pub fn green() -> Self { Color(0, 255, 0) }
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
}

impl Index<(usize, usize)> for Image {
    type Output = Color;
    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        assert!(row < self.width);
        assert!(col < self.height);
        &self.pixels[row * self.width + col]
    }
}

impl IndexMut<(usize, usize)> for Image {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        assert!(row < self.width);
        assert!(col < self.height);
        &mut self.pixels[row* self.width + col]
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
}
