use sdl2::rect::Point;
use crate::draw;

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

#[derive(Clone, Copy, Debug)]
pub struct Triangle3D {
    pub p: [Vec3D; 3],
}

impl Triangle3D {
    pub fn new(a: Vec3D, b: Vec3D, c: Vec3D) -> Triangle3D {
        Triangle3D { p: [a, b, c] }
    }

    pub fn new_empty() -> Triangle3D {
        Triangle3D::new(
            Vec3D::new(0.0, 0.0, 0.0),
            Vec3D::new(0.0, 0.0, 0.0),
            Vec3D::new(0.0, 0.0, 0.0),
        )
    }

    pub fn to_2d (&self) -> draw::Triangle {
        draw::Triangle::new(
            Point::new(self.p[0].x.round() as i32, self.p[0].y.round() as i32),
            Point::new(self.p[1].x.round() as i32, self.p[1].y.round() as i32),
            Point::new(self.p[2].x.round() as i32, self.p[2].y.round() as i32),
        )
    }
}

pub struct Mesh {
    pub tris: Vec<Triangle3D>,
}

impl Mesh {
    pub fn new(tris: Vec<Triangle3D>) -> Mesh {
        Mesh { tris }
    }
}
