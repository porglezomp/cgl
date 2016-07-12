extern crate cgl;

use cgl::{Image, Color, Vec3, write_bmp};

use std::fs::File;

fn main() {
    // Create a black image and a z-buffer
    let mut image = Image::with_dimensions(512, 512);
    let mut zbuf = Image::filled(512, 512, std::f32::MIN);

    // Draw the floor at z = -1, in a pale cream color
    image.triangle3(Vec3(-1.0, 1.0, -1.0), Vec3(1.0, 1.0, -1.0),
                    Vec3(1.0, -1.0, -1.0), &mut zbuf, Color::rgb(255, 240, 230));
    image.triangle3(Vec3(1.0, -1.0, -1.0), Vec3(-1.0, -1.0, -1.0),
                    Vec3(-1.0, 1.0, -1.0), &mut zbuf, Color::rgb(255, 240, 230));

    // Draw the three intersecting triangles
    image.triangle3(Vec3(-0.7, 0.0, -0.5), Vec3(0.7, 0.5, 0.5),
                    Vec3(0.7, -0.5, 0.5), &mut zbuf, Color::red());
    image.triangle3(Vec3(-0.2, 0.0, 0.7), Vec3(0.4, 0.9, -0.5),
                    Vec3(0.4, -0.9, -0.5), &mut zbuf, Color::blue());
    image.triangle3(Vec3(0.2, 0.0, 0.3), Vec3(-0.5, 0.7, 0.4),
                    Vec3(-0.5, -0.7, 0.4), &mut zbuf, Color::green());

    // Save the image to an output file
    let mut output = File::create("demo/demo3.bmp")
        .expect("Should create demo3.bmp");
    write_bmp(&image, &mut output)
        .expect("Should save image");
}
