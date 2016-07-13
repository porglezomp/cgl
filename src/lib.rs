// Copyright (c) 2016 by Caleb Jones <code@calebjones.net>
//
// Based on educational materials by Dmitry V. Sokolov released at
// https://github.com/ssloy/tinyrenderer/wiki

//! The CGL (Caleb's Graphics Library) was implemented as an educational
//! project, taking a deep dive into writing a software renderer in order to
//! better understand how they work. It's based on Dmitry V. Sokolov's excellent
//! ["tinyrenderer" lessons][tinyrenderer].
//!
//! [tinyrenderer]: https://github.com/ssloy/tinyrenderer/wiki
//!
//! # Priorities
//!
//! - *Clean code:* I want this to be a project that I can look back on in a few
//!   years and still be able to understand. It would be even better if other
//!   people could understand it as well!
//! - *No Dependencies:* I don't want to use other people's vector graphics or
//!   image loading libraries. I'm going to be stubborn and do it all myself.
//!
//! Some things that are non-priorities:
//!
//! - *Performance:* If you want performance, go look at hardware accelerated
//!   rendering.
//! - *Handling file formats robustly:* The only features of .obj or .bmp that I
//!   will make an effort to support are features that I actually end up using
//!   in my demos.
//!
//! # Examples
//!
//! ```rust,no_run
//! use self::cgl::{Color, Renderer, Obj, Shader, Vert, Vec3, Vec4, Mat4, write_bmp};
//! use std::fs::File;
//!
//! # fn load() -> Result<(), Box<::std::error::Error>> {
//! let model = try!(try!(Obj::from_file("suzanne.obj")).model());
//! let mut renderer = Renderer::with_dimensions(512, 512);
//! let matrix = Mat4::viewport(512, 512) * Mat4::perspective(1.0);
//!
//! renderer.model(&MyShader, &matrix, &model);
//!
//! let mut out_file = try!(File::create("suzanne.bmp"));
//! try!(write_bmp(renderer.image(), &mut out_file));
//! # Ok(()) }
//!
//! struct MyShader;
//!
//! impl Shader<Vert, Mat4<f32>> for MyShader {
//!     type VOut = Vert;
//!
//!     fn vertex(&self, vert: Vert, mat: &Mat4<f32>, pos: &mut Vec4<f32>) -> Vert {
//!         *pos = *mat * vert.pos.augment();
//!         vert
//!     }
//!
//!     fn fragment(&self, vert: Vert, _: &Mat4<f32>) -> Color {
//!         let c = Vec3(0.0f32, 1.0, 0.5).normalized()
//!             .dot(vert.norm.normalized());
//!         Color::rgb(200, 180, 140) * Color::float_rgb(c, c, c)
//!     }
//! }
//! ```

pub mod obj;
pub mod model;
pub mod bmp;
pub mod math;
pub mod image;
pub mod renderer;
pub mod shader;

pub use obj::Obj;
pub use model::{Model, Vertex, Vert};
pub use bmp::{read_bmp, write_bmp};
pub use math::{Vec2, Vec3, Vec4, Mat2, Mat3, Mat4};
pub use image::{Image, Color};
pub use renderer::Renderer;
pub use shader::Shader;
