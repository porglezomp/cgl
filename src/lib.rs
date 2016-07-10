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
//! 1. *Clean code:* I want this to be a project that I can look back on in a few
//!    years and still be able to understand. It would be even better if other
//!    people could understand it as well!
//! 2. *No Dependencies:* I don't want to use other people's vector graphics or
//!    image loading libraries. I'm going to be stubborn and do it all myself.
//!
//! Some things that are non-priorities:
//!
//! 1. *Performance:* If you want performance, go look at hardware accelerated
//!    rendering.
//! 2. *Handling file formats robustly:* The only features of .obj or .bmp that I
//!    will make an effort to support are features that I actually end up using
//!    in my demos.
//!
//! # Examples
//!
//! The [`obj`] module lets you load models for rendering use
//!
//! ```rust
//! use std::fs::File;
//! use std::io::BufReader;
//! use cgl::obj::Model;
//!
//! let model_file = File::open("suzanne.obj").unwrap();
//! let model = Model::from_reader(BufReader::new(model_file)).unwrap();
//! ```
//!
//! [`obj`]: obj/index.html

pub mod obj;
pub mod bmp;
pub mod math;
pub mod image;

pub use obj::Model;
pub use bmp::write_bmp;
pub use math::{Vec2, Vec3, Vec4};
pub use image::{Image, Color};
