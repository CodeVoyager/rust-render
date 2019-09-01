//! This module is my own API for drawing 2D shapes on a screen
//!
//!

use crate::draw;
use sdl2::rect::Point;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;
use sdl2::pixels::Color;

/// Vertex vector
///
#[derive(Clone, Copy, Debug)]
pub struct Vec3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3D {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3D {
        Vec3D { x, y, z }
    }
}

/// Represents triangle in 3D space
///
#[derive(Clone, Copy, Debug)]
pub struct Triangle3D {
    pub p: [Vec3D; 3],
    pub color: Option<Color>,
}

impl Triangle3D {
    pub fn new(a: Vec3D, b: Vec3D, c: Vec3D) -> Triangle3D {
        Triangle3D { p: [a, b, c], color: None }
    }

    pub fn new_empty() -> Triangle3D {
        Triangle3D::new(
            Vec3D::new(0.0, 0.0, 0.0),
            Vec3D::new(0.0, 0.0, 0.0),
            Vec3D::new(0.0, 0.0, 0.0),
        )
    }

    /// Converts 3D triangle to 2D version
    ///
    /// basically converts verticies f32 to i32
    /// version and copies color information
    ///
    pub fn to_2d(&self) -> draw::Triangle {
        let mut t = draw::Triangle::new(
            Point::new(self.p[0].x.round() as i32, self.p[0].y.round() as i32),
            Point::new(self.p[1].x.round() as i32, self.p[1].y.round() as i32),
            Point::new(self.p[2].x.round() as i32, self.p[2].y.round() as i32),
        );
        t.color = self.color;

        t
    }
}

/// Represents 3D object
///
pub struct Mesh {
    pub tris: Vec<Triangle3D>,
}

impl Mesh {
    pub fn new(tris: Vec<Triangle3D>) -> Mesh {
        Mesh { tris }
    }

    /// Reads mesh object from OBJ file into memory
    ///
    pub fn from_obj(path: &str) -> Mesh {
        let file = File::open(path);
        let mut tris: Vec<Triangle3D> = Vec::new();
        match file {
            Ok(content) => {
                let reader = BufReader::new(content);
                let mut vs: Vec<Vec3D> = Vec::new();
                let mut x: f32;
                let mut y: f32;
                let mut z: f32;
                let mut f1: i32;
                let mut f2: i32;
                let mut f3: i32;
                for res_line in reader.lines() {
                    match res_line {
                        Ok(line) => {
                            let words: Vec<&str> = line.split(" ").collect();
                            if words.len() > 0 && (words[0] == "f" || words[0] == "v") {
                                if words[0] == "v" {
                                    x = words[1].parse().unwrap();
                                    y = words[2].parse().unwrap();
                                    z = words[3].parse().unwrap();

                                    vs.push(Vec3D::new(x, y, z));
                                }
                                // TODO: Better version for reading faces
                                // TODO: Better memory efficient version of faces storing
                                if words[0] == "f" {
                                    let fpart: Vec<&str> = words[1].split("/").collect();
                                    f1 = fpart[0].parse().unwrap();
                                    let fpart: Vec<&str> = words[2].split("/").collect();
                                    f2 = fpart[0].parse().unwrap();
                                    let fpart: Vec<&str> = words[3].split("/").collect();
                                    f3 = fpart[0].parse().unwrap();
                                    tris.push(Triangle3D::new(vs[f1 as usize - 1], vs[f2 as usize - 1], vs[f3 as usize - 1]));
                                }
                            }
                        }
                        Err(err) => {
                            println!("{:?}", err);
                            break;
                        }
                    }
                }
                Mesh::new(tris)
            }
            Err(_) => Mesh::new(Vec::new()),
        }
    }
}
