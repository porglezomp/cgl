extern crate cgl;

mod demo;

fn main() {
    let image = demo::load(7);
    demo::save(&image, 8);
}
