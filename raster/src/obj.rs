//! For parsing and representing a model in the [Wavefront .obj] format.
//!
//! [Wavefront .obj]: http://en.wikipedia.org/wiki/Wavefront_.obj_file
//!
//! The .obj format is a multi-indexed format, which means that a single vertex
//! can have a different index for its position, texture, and normals. As such,
//! it's slightly more annoying to work with directly while rendering, so in
//! order to render models you should first convert them to a [`Model`] with the
//! [`model()`] method.
//!
//! [`Model`]: ../model/struct.Model.html
//! [`model()`]: struct.Obj.html#method.model

use std::io::{self, BufRead, BufReader};
use std::fmt::{self, Display, Formatter};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::convert::AsRef;
use std::error;
use std::num;

use model::{Model, Vert};
use cgl_math::{Vec2, Vec3, Mat4};

pub type Vertex = Vec3<f32>;
pub type Normal = Vec3<f32>;
pub type TexCoord = Vec2<f32>;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct VertexIndex {
    pub vertex: i32,
    pub texture: Option<i32>,
    pub normal: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
/// Represents a Wavefront .obj model.
pub struct Obj {
    pub vertices: Vec<Vertex>,
    pub normals: Vec<Normal>,
    pub texture: Vec<TexCoord>,
    /// Indices into the `vertices` array
    pub triangles: Vec<[u32; 3]>,
    pub components: Vec<Vec<VertexIndex>>,
}

impl Obj {
    pub fn from_str(input: &str) -> Result<Self, Error> {
        let reader = BufReader::new(input.as_bytes());
        Obj::from_reader(reader)
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let file = try!(File::open(path));
        let reader = BufReader::new(file);
        Obj::from_reader(reader)
    }

    pub fn from_reader<R: BufRead>(read: R) -> Result<Self, Error> {
        let mut vertices = Vec::new();
        let mut triangles = Vec::new();
        let mut texture = Vec::new();
        let mut normals = Vec::new();
        let mut components = Vec::new();
        for (n, line) in read.lines().enumerate() {
            let line_num = n + 1;
            let line = try!(line);
            let mut v = line.split_whitespace();
            let pattern = v.next();
            match pattern {
                Some("v") => {
                    let v: Vec<_> = try!(v.map(|x| x.parse()).collect());
                    if v.len() != 3 {
                        return Err(Error::ObjParse(line_num));
                    }
                    vertices.push(Vec3(v[0], v[1], v[2]));
                }
                Some("vt") => {
                    let v: Vec<_> = try!(v.map(|x| x.parse()).collect());
                    if v.len() < 2 || v.len() > 3 {
                        return Err(Error::ObjParse(line_num));
                    }
                    texture.push(Vec2(v[0], v[1]));
                }
                Some("vn") => {
                    let v: Vec<_> = try!(v.map(|x| x.parse()).collect());
                    if v.len() != 3 {
                        return Err(Error::ObjParse(line_num));
                    }
                    normals.push(Vec3(v[0], v[1], v[2]));
                }
                Some("f") => {
                    let v: Vec<_> = try!(v.map(|v| parse_vertex(v).and_then(|v| {
                        normalize_indices(v, vertices.len(),
                                          texture.len(), normals.len())
                    })).collect());
                    if v.len() < 3 {
                        return Err(Error::ObjParse(line_num));
                    }
                    triangles.push([v[0].vertex as u32,
                                    v[1].vertex as u32,
                                    v[2].vertex as u32]);
                    components.push(v);
                }
                Some("#") | Some("vp") | None => continue,
                Some("s") | Some("g") => continue,
                Some(_) => return Err(Error::ObjParse(line_num)),
            }
        }
        Ok(Obj {
            vertices: vertices,
            normals: normals,
            texture: texture,
            triangles: triangles,
            components: components,
        })
    }

    pub fn model(&self) -> Result<Model<Vert>, String> {
        let mut unique_vert = HashMap::new();
        let mut vertices = Vec::new();
        let mut triangles = Vec::with_capacity(self.triangles.len());
        for face in &self.components {
            if face.len() > 3 {
                return Err(format!("{} components, expected 3", face.len()));
            }
            for index_set in face {
                if !unique_vert.contains_key(index_set) {
                    let vert = Vert {
                        pos: self.vertices[index_set.vertex as usize],
                        tex: match index_set.texture {
                            Some(idx) => self.texture[idx as usize],
                            None => Vec2(0.0, 0.0),
                        },
                        norm: match index_set.normal {
                            Some(idx) => self.normals[idx as usize],
                            None => Vec3(0.0, 0.0, 0.0),
                        },
                    };
                    unique_vert.insert(index_set, vertices.len());
                    vertices.push(vert);
                }
            }
            triangles.push([unique_vert[&face[0]],
                            unique_vert[&face[1]],
                            unique_vert[&face[2]]]);
        }
        Ok(Model {
            vertices: vertices,
            triangles: triangles,
        })
    }

    pub fn transform(&self, matrix: Mat4<f32>) -> (Vec<Vec3<isize>>, &Vec<[u32; 3]>) {
        let vertices = self.vertices.iter().map(|v| {
            let w = (matrix * v.augment()).retro_project();
            Vec3(w.0 as isize, w.1 as isize, w.2 as isize)
        }).collect();
        (vertices, &self.triangles)
    }
}

// Faces in a Wavefront OBJ file look approximately like:
//
//     f 1/5/1 2/6/2 3/7/3
//
// This function is concerned with parsing a single one of the space-separated
// chunks of indices that can be found after the f. They come in several forms:
// 1. Vertex: f v1 v2 v3 ...
// 2. Vertex + texture: f v1/vt1 v2/vt2 v3/vt3 ...
// 3. Vertex + texture + normal: f v1/vt1/vn1 v2/vt2/vn2 v3/vt3/vn3 ...
// 4. Vertex + normal: f v1//vn1 v2//vn2 v3//vn3 ...
// Each type needs to be handled separately
fn parse_vertex(vert_str: &str) -> Result<VertexIndex, Error> {
    let parts = vert_str.split('/').take(3).collect::<Vec<_>>();
    let vertex = try!(parts[0].parse());
    match parts.len() {
        1 => Ok(VertexIndex {
            vertex: vertex,
            texture: None,
            normal: None,
        }),
        2 => Ok(VertexIndex {
            vertex: vertex,
            texture: Some(try!(parts[1].parse())),
            normal: None,
        }),
        3 if parts[1] == "" => Ok(VertexIndex {
            vertex: vertex,
            texture: None,
            normal: Some(try!(parts[2].parse())),
        }),
        3 => Ok(VertexIndex {
            vertex: vertex,
            texture: Some(try!(parts[1].parse())),
            normal: Some(try!(parts[2].parse())),
        }),
        _ => Err(Error::ObjParse(0)),
    }
}

/// This transforms the indices in an OBJ file into a more useful format for
/// models. First, it will subtract one from all indices since OBJ indices are
/// one-based, while most code is going to be zero-based. Second, it will make
/// all negative indices relative to the corresponding data. A value of -1 in a
/// slot in an OBJ file refers to the most recently defined component, so for
/// example `-1/-1/-1` would indicate that the vertex should use the most recent
/// value for each of the position, texture, and normal.
///
/// # Examples
///
/// ```rust
/// # use cgl::obj::{VertexIndex, normalize_indices};
/// # let vertices: Vec<()> = Vec::new();
/// # let texture: Vec<()> = Vec::new();
/// # let normals: Vec<()> = Vec::new();
/// # let v = VertexIndex { vertex: 1, texture: Some(-1), normal: None };
/// normalize_indices(v, vertices.len(), texture.len(), normals.len());
/// ```
pub fn normalize_indices(i: VertexIndex, vert: usize, tex: usize, norm: usize)
                         -> Result<VertexIndex, Error>
{
    fn fix(val: i32, reference: usize) -> Result<i32, Error> {
        match val {
            0 => Err(Error::ObjParse(0)),
            val if val < 0 => {
                let val = reference as i32 + val;
                if val < 0 {
                    Err(Error::ObjParse(0))
                } else {
                    Ok(val)
                }
            }
            val => Ok(val - 1),
        }
    }

    Ok(VertexIndex {
        vertex: try!(fix(i.vertex, vert)),
        normal: match i.normal {
            Some(n) => Some(try!(fix(n, norm))),
            None => None,
        },
        texture: match i.texture {
            Some(t) => Some(try!(fix(t, tex))),
            None => None,
        },
    })
}

#[cfg(test)]
mod test {
    use super::{Obj, VertexIndex, normalize_indices};
    use math::Vec3;

    #[test]
    fn read_vertex() {
        let model = Obj::from_str(
"v 0.5 -0.25 1.0
v 1.0 1.0 1.0");
        assert_eq!(model.unwrap(), Obj {
            vertices: vec![
                Vec3(0.5, -0.25, 1.0),
                Vec3(1.0, 1.0, 1.0),
            ],
            normals: Vec::new(),
            texture: Vec::new(),
            triangles: Vec::new(),
            components: Vec::new(),
        });
    }

    #[test]
    fn test_normalize_indices() {
        let simple_index = VertexIndex {
            vertex: 1,
            normal: None,
            texture: None
        };
        let index = normalize_indices(simple_index, 8, 2, 1);
        assert_eq!(index.expect("simple_index"), VertexIndex {
            vertex: 0,
            normal: None,
            texture: None,
        });

        let complex_index = VertexIndex {
            vertex: 3,
            normal: Some(8),
            texture: Some(5),
        };
        let index = normalize_indices(complex_index, 1, 2, 3);
        assert_eq!(index.expect("complex_index"), VertexIndex {
            vertex: 2,
            normal: Some(7),
            texture: Some(4),
        });

        let relative_index = VertexIndex {
            vertex: -1,
            normal: Some(-3),
            texture: Some(-2),
        };
        let index = normalize_indices(relative_index, 5, 5, 5);
        assert_eq!(index.expect("relative_index"), VertexIndex {
            vertex: 4,
            normal: Some(2),
            texture: Some(3),
        });

        let wrong_relative = VertexIndex {
            vertex: -3,
            normal: None,
            texture: None,
        };
        let index = normalize_indices(wrong_relative, 1, 1, 2);
        assert!(index.is_err(), "The vertex is an invalid relative value");

        let wrong_relative2 = VertexIndex {
            vertex: 1,
            normal: Some(-1),
            texture: None,
        };
        let index = normalize_indices(wrong_relative2, 0, 0, 0);
        assert!(index.is_err(), "The normal is an invalid relative value");

        let zero_index = VertexIndex {
            vertex: 1,
            normal: None,
            texture: Some(0),
        };
        let index = normalize_indices(zero_index, 0, 0, 0);
        assert!(index.is_err(), "A zero index is invalid");
    }
}

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    FloatParse(num::ParseFloatError),
    IntParse(num::ParseIntError),
    ObjParse(usize),
}

impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => write!(fmt, "IO error: {}", err),
            Error::FloatParse(ref err) => write!(fmt, "Float parse error: {}", err),
            Error::IntParse(ref err) => write!(fmt, "Int parse error: {}", err),
            Error::ObjParse(0) => write!(fmt, "OBJ parse error on unknown line"),
            Error::ObjParse(n) => write!(fmt, "OBJ parse error on line {}", n),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref err) => err.description(),
            Error::FloatParse(ref err) => err.description(),
            Error::IntParse(ref err) => err.description(),
            Error::ObjParse(_) => "Error parsing OBJ file",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref err) => Some(err),
            Error::FloatParse(ref err) => Some(err),
            Error::IntParse(ref err) => Some(err),
            Error::ObjParse(_) => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self { Error::Io(error) }
}

impl From<num::ParseFloatError> for Error {
    fn from(error: num::ParseFloatError) -> Self { Error::FloatParse(error) }
}

impl From<num::ParseIntError> for Error {
    fn from(error: num::ParseIntError) -> Self { Error::IntParse(error) }
}
