//! For parsing and representing a model in the [wavefront obj] format.
//!
//! [wavefront obj]: http://en.wikipedia.org/wiki/Wavefront_.obj_file

use std::io::{self, BufRead, BufReader};
use std::fmt::{self, Display, Formatter};
use std::error;
use std::num;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    FloatParse(num::ParseFloatError),
    IntParse(num::ParseIntError),
    ObjParse,
}

impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => write!(fmt, "IO error: {}", err),
            Error::FloatParse(ref err) => write!(fmt, "Float parse error: {}", err),
            Error::IntParse(ref err) => write!(fmt, "Int parse error: {}", err),
            Error::ObjParse => write!(fmt, "OBJ parse error"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref err) => err.description(),
            Error::FloatParse(ref err) => err.description(),
            Error::IntParse(ref err) => err.description(),
            Error::ObjParse => "Error parsing OBJ file",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref err) => Some(err),
            Error::FloatParse(ref err) => Some(err),
            Error::IntParse(ref err) => Some(err),
            Error::ObjParse => None,
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

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Vertex(pub f32, pub f32, pub f32);

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Normal(pub f32, pub f32, pub f32);

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct TexCoord(pub f32, pub f32);

#[derive(Debug, Clone, PartialEq)]
pub struct Model {
    pub vertices: Vec<Vertex>,
    pub normals: Vec<Normal>,
    pub texture: Vec<TexCoord>,
    /// Indices into the `vertices` array
    pub triangles: Vec<[u32; 3]>,
}

impl Model {
    pub fn from_str(input: &str) -> Result<Self, Error> {
        let reader = BufReader::new(input.as_bytes());
        Model::from_reader(reader)
    }

    pub fn from_reader<R: BufRead>(read: R) -> Result<Self, Error> {
        let mut vertices = Vec::new();
        let mut triangles = Vec::new();
        let mut texture = Vec::new();
        let mut normals = Vec::new();
        for line in read.lines() {
            let line = try!(line);
            let mut v = line.split_whitespace();
            let pattern = v.next();
            match pattern {
                Some("v") => {
                    let v: Vec<_> = v.filter_map(|x| x.parse().ok()).collect();
                    if v.len() != 3 {
                        return Err(Error::ObjParse);
                    }
                    vertices.push(Vertex(v[0], v[1], v[2]));
                }
                Some("vt") => {
                    let v: Vec<_> = v.filter_map(|x| x.parse().ok()).collect();
                    if v.len() < 2 || v.len() > 3 {
                        return Err(Error::ObjParse);
                    }
                    texture.push(TexCoord(v[0], v[1]));
                }
                Some("vn") => {
                    let v: Vec<_> = v.filter_map(|x| x.parse().ok()).collect();
                    if v.len() != 3 {
                        return Err(Error::ObjParse);
                    }
                    normals.push(Normal(v[0], v[1], v[2]));
                }
                Some("f") => {
                    let parts: Vec<_> = v.map(|x| x.split('/').collect::<Vec<_>>()).collect();
                    if parts.len() != 3 {
                        return Err(Error::ObjParse);
                    }
                    let mut verts = [0; 3];
                    for (i, group) in parts.iter().enumerate() {
                        verts[i] = try!(group[0].parse::<u32>()) - 1;
                    }
                    triangles.push(verts);
                }
                Some("#") | Some("vp") | None => continue,
                Some("s") | Some("g") => continue,
                Some(_) => return Err(Error::ObjParse),
            }
        }
        Ok(Model {
            vertices: vertices,
            normals: normals,
            texture: texture,
            triangles: triangles,
        })
    }
}

#[cfg(test)]
mod test {
    use super::{Vertex, Model};

    #[test]
    fn read_vertex() {
        let model = Model::from_str("v 0.5 -0.25 1.0
v 1.0 1.0 1.0");
        assert_eq!(model.unwrap(), Model {
            vertices: vec![
                Vertex(0.5, -0.25, 1.0),
                Vertex(1.0, 1.0, 1.0),
            ],
            normals: Vec::new(),
            texture: Vec::new(),
            triangles: vec![],
        });
    }
}
