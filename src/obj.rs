//! For parsing and representing a model in the [wavefront obj] format.
//! [wavefront obj]: http://en.wikipedia.org/wiki/Wavefront_.obj_file

use std::io::{self, BufRead, BufReader};
use std::fmt::{self, Display, Formatter};
use std::error;

#[derive(Debug)]

pub enum Error {
    IoError(io::Error),
    ParseError,
}

impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::IoError(_) => "io error",
            Error::ParseError => "parse error",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::IoError(ref err) => Some(err),
            Error::ParseError => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IoError(error)
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct Vertex(pub f32, pub f32, pub f32);

#[derive(Debug, Clone, PartialEq)]
struct Model {
    pub vertices: Vec<Vertex>,
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
        for line in read.lines() {
            let line = try!(line);
            if line.starts_with('v') {
                let v: Vec<_> = line.split_whitespace().skip(1)
                    .filter_map(|x| x.parse().ok()).collect();
                if v.len() != 3 {
                    return Err(Error::ParseError);
                }
                vertices.push(Vertex(v[0], v[1], v[2]));
            }
            println!("{:?}", line);
        }
        Ok(Model {
            vertices: vertices,
            triangles: Vec::new(),
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
            triangles: vec![],
        });
    }
}
