//! Contains function and data structures used for
//! projection, m.multiplication and similiar stuff
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

    pub fn mat_rot_y(deg: &f32) -> Mat4x4 {
        let mut m = Mat4x4::new_empty();
        m.m[0][0] = deg.cos();
        m.m[0][2] = deg.sin();
        m.m[2][0] = -deg.sin();
        m.m[1][1] = 1.0;
        m.m[2][2] = deg.cos();
        m.m[3][3] = 1.0;
        m
    }

    pub fn mat_trans(x: f32, y: f32, z: f32) -> Mat4x4 {
        let mut m = Mat4x4::id();
        m.m[3][0] = x;
        m.m[3][1] = y;
        m.m[3][2] = z;
        m
    }

    pub fn point_at(pos: &draw_3d::Vec3D, target: &draw_3d::Vec3D, up: &draw_3d::Vec3D) -> Mat4x4 {
        let forward = target.sub(pos).normalize();
        let up_forward_dp = up.dot_product(&forward);
        let new_up = up
            .sub(&forward.mul(&draw_3d::Vec3D::new(
                up_forward_dp,
                up_forward_dp,
                up_forward_dp,
            )))
            .normalize();
        let right = new_up.cross_product(&forward);
        let mut m = Mat4x4::new_empty();

        m.m[0][0] = right.x;
        m.m[0][1] = right.y;
        m.m[0][2] = right.z;
        m.m[0][3] = 0.0;
        m.m[1][0] = new_up.x;
        m.m[1][1] = new_up.y;
        m.m[1][2] = new_up.z;
        m.m[1][3] = 0.0;
        m.m[2][0] = forward.x;
        m.m[2][1] = forward.y;
        m.m[2][2] = forward.z;
        m.m[2][3] = 0.0;
        m.m[3][0] = pos.x;
        m.m[3][1] = pos.y;
        m.m[3][2] = pos.z;
        m.m[3][3] = 1.0;

        m
    }

    pub fn to_look_at(&self) -> Mat4x4 {
        let mut new_mat = Mat4x4::new_empty();

        new_mat.m[0][0] = self.m[0][0];
        new_mat.m[0][1] = self.m[1][0];
        new_mat.m[0][2] = self.m[2][0];
        new_mat.m[0][3] = 0.0;
        new_mat.m[1][0] = self.m[0][1];
        new_mat.m[1][1] = self.m[1][1];
        new_mat.m[1][2] = self.m[2][1];
        new_mat.m[1][3] = 0.0;
        new_mat.m[2][0] = self.m[0][2];
        new_mat.m[2][1] = self.m[1][2];
        new_mat.m[2][2] = self.m[2][2];
        new_mat.m[2][3] = 0.0;
        new_mat.m[3][0] = -(self.m[3][0] * new_mat.m[0][0]
            + self.m[3][1] * new_mat.m[1][0]
            + self.m[3][2] * new_mat.m[2][0]);
        new_mat.m[3][1] = -(self.m[3][0] * new_mat.m[0][1]
            + self.m[3][1] * new_mat.m[1][1]
            + self.m[3][2] * new_mat.m[2][1]);
        new_mat.m[3][2] = -(self.m[3][0] * new_mat.m[0][2]
            + self.m[3][1] * new_mat.m[1][2]
            + self.m[3][2] * new_mat.m[2][2]);
        new_mat.m[3][3] = 1.0;

        new_mat
    }

    pub fn id() -> Mat4x4 {
        let mut m = Mat4x4::new_empty();
        m.m[0][0] = 1.0;
        m.m[1][1] = 1.0;
        m.m[2][2] = 1.0;
        m.m[3][3] = 1.0;
        m
    }

    pub fn mul(&self, other: &Mat4x4) -> Mat4x4 {
        let mut m = Mat4x4::new_empty();

        for c in 0..4 {
            for r in 0..4 {
                m.m[r][c] = self.m[r][0] * other.m[0][c]
                    + self.m[r][1] * other.m[1][c]
                    + self.m[r][2] * other.m[2][c]
                    + self.m[r][3] * other.m[3][c]
            }
        }

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

    draw_3d::Vec3D { x, y, z, w }
}
