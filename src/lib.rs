/// A type representing an RGB triple
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8);

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
}
