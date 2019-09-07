//! Contains function and data structures used for
//! projection, matrix multiplication and similiar stuff
//!

use crate::draw_3d;

/// 4x4 Matrix
///
pub struct Mat4x4 {
    pub m: [[f32; 4]; 4],
}

impl Mat4x4 {
    pub fn new(
        x00: f32,
        x10: f32,
        x20: f32,
        x30: f32,
        x01: f32,
        x11: f32,
        x21: f32,
        x31: f32,
        x02: f32,
        x12: f32,
        x22: f32,
        x32: f32,
        x03: f32,
        x13: f32,
        x23: f32,
        x33: f32,
    ) -> Mat4x4 {
        Mat4x4 {
            m: [
                [x00, x10, x20, x30],
                [x01, x11, x21, x31],
                [x02, x12, x22, x32],
                [x03, x13, x23, x33],
            ],
        }
    }

    pub fn new_empty() -> Mat4x4 {
        Mat4x4::new(
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        )
    }

    /// Helper method for generating projection matrix
    ///
    pub fn mat_proj(fov_deg: f32, aspect_ratio: f32, far: f32, near: f32) -> Mat4x4 {
        let mut m = Mat4x4::new_empty();
        let fov_rad: f32 = 1.0 / (fov_deg).tan();

        m.m[0][0] = aspect_ratio * fov_rad;
        m.m[1][1] = fov_rad;
        m.m[2][2] = far / (far - near);
        m.m[2][3] = 1.0;
        m.m[3][2] = (-near * far) / (far - near);
        m
    }

    pub fn mat_rot_x(deg: &f32) -> Mat4x4 {
        let mut m = Mat4x4::new_empty();
        m.m[0][0] = 1.0;
        m.m[1][1] = deg.cos();
        m.m[1][2] = deg.sin();
        m.m[2][1] = -deg.sin();
        m.m[2][2] = deg.cos();
        m.m[3][3] = 1.0;
        m
    }

    pub fn mat_rot_z(deg: &f32) -> Mat4x4 {
        let mut m = Mat4x4::new_empty();
        m.m[0][0] = deg.cos();
        m.m[0][1] = deg.sin();
        m.m[1][0] = -deg.sin();
        m.m[1][1] = deg.cos();
        m.m[2][2] = 1.0;
        m.m[3][3] = 1.0;
        m
    }

    pub fn id() -> Mat4x4 {
        let mut m = Mat4x4::new_empty();
        m.m[0][0] = 1.0;
        m.m[1][1] = 1.0;
        m.m[2][2] = 1.0;
        m.m[3][3] = 1.0;
        m
    }
}

/// 4x4 Matrix multiplication function
/// Multiplies 3D vector over 4x4 matrix (fourth value is implied 1)
///
pub fn mult_matrix_vector(i: &draw_3d::Vec3D, m: &Mat4x4) -> draw_3d::Vec3D {
    let x = (i.x * m.m[0][0]) + (i.y * m.m[1][0]) + (i.z * m.m[2][0]) + (i.w * m.m[3][0]);
    let y = (i.x * m.m[0][1]) + (i.y * m.m[1][1]) + (i.z * m.m[2][1]) + (i.w * m.m[3][1]);
    let z = (i.x * m.m[0][2]) + (i.y * m.m[1][2]) + (i.z * m.m[2][2]) + (i.w * m.m[3][2]);
    let w = (i.x * m.m[0][3]) + (i.y * m.m[1][3]) + (i.z * m.m[2][3]) + (i.w * m.m[3][3]);

    if w != 0.0 {
        return draw_3d::Vec3D::new(x / w, y / w, z / w);
    }

    draw_3d::Vec3D::new(x, y, z)
}
