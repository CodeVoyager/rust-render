//! Project featuring 3D render written in pure Rust
//! Written from scratch
//! Fun/educational project
//!

#![crate_name = "rust_renderer"]

extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::cmp::Ordering;
use std::f32;
use std::time::Duration;
use std::time::SystemTime;

pub mod draw;
pub mod draw_3d;
pub mod transform;

fn main() {
    let color_black: Color = Color::RGB(0, 0, 0);
    let color_red: Color = Color::RGB(255, 0, 0);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let screen_width = 1300;
    let screen_height = 760;

    let window = video_subsystem
        .window(
            "Rust Renderer",
            (draw::PIXEL_SIZE * screen_width) as u32,
            (draw::PIXEL_SIZE * screen_height) as u32,
        )
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(color_black);
    canvas.clear();

    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let model = draw_3d::Mesh::from_obj("<PATH>");
    let camera = draw_3d::Vec3D {
        ..Default::default()
    };
    let light = (draw_3d::Vec3D {
        z: -1.0,
        ..Default::default()
    })
    .normalize();
    let near: f32 = 0.1;
    let far: f32 = 1000.0;
    let fov: f32 = 90.0;
    let aspect_ratio: f32 = screen_height as f32 / screen_width as f32;
    let mat_proj = transform::Mat4x4::mat_proj(fov, aspect_ratio, far, near);

    let screen_width_half = screen_width as f32 * 0.5;
    let screen_height_half = screen_height as f32 * 0.5;
    let mut theta = 0.0;
    // HACK: pushing object further into space so my computer
    // does not melt
    let z_offset = 800.0;

    let mut prev_sys_time = SystemTime::now();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        let sys_time = SystemTime::now();
        let time_elapsed = sys_time.duration_since(prev_sys_time);
        let time_elapsed_frac = match time_elapsed {
            Ok(duration) => duration.as_millis() as f32 / 1000.0,
            Err(_) => 0.0,
        };
        theta += 1.0 * time_elapsed_frac;
        let mat_rot_x = transform::Mat4x4::mat_rot_x(&(theta * 0.5));
        let mat_rot_z = transform::Mat4x4::mat_rot_z(&theta);
        let mat_rot_y = transform::Mat4x4::mat_rot_z(&(theta * 0.3));
        let mat_trans = transform::Mat4x4::mat_trans(0.0, 0.0, 16.0 + z_offset);
        let mat_world = mat_rot_z.mul(&mat_rot_x).mul(&mat_trans).mul(&mat_rot_y);
        let view_offset = draw_3d::Vec3D::new(1.0, 1.0, 0.0);
        let screen_offset = draw_3d::Vec3D::new(screen_width_half, screen_height_half, 1.0);

        prev_sys_time = sys_time;

        canvas.set_draw_color(color_black);
        canvas.clear();

        let mut tris_to_rater: Vec<draw_3d::Triangle3D> = Vec::new();
        for i in 0..model.tris.len() {
            //println!("triangle {}", i);
            let mut tri_projected = draw_3d::Triangle3D::new_empty();
            let mut tri_translated = draw_3d::Triangle3D::new_empty();

            // Rotation
            for v in 0..3 {
                tri_translated.p[v] =
                    transform::mult_matrix_vector(&model.tris[i].p[v], &mat_world);
            }
            let line1 = tri_translated.p[1].sub(&tri_translated.p[0]);
            let line2 = tri_translated.p[2].sub(&tri_translated.p[0]);

            let normal = line1.cross_product(&line2).normalize();
            if normal.dot_product(&tri_translated.p[0].sub(&camera)) < 0.0 {
                // 3D -> 2D
                for v in 0..3 {
                    tri_projected.p[v] =
                        transform::mult_matrix_vector(&tri_translated.p[v], &mat_proj)
                            .add(&view_offset)
                            .mul(&screen_offset);
                }
                // Illumination
                let light_dp = normal.dot_product(&light);
                let shade = (255.0 * light_dp) as u8;
                let color = Color::RGB(shade, shade, shade);
                tri_projected.color = Some(color);
                tris_to_rater.push(tri_projected);
            }
        }

        tris_to_rater.sort_by(|a, b| {
            let z0 = (a.p[0].z + a.p[1].z + a.p[2].z) / 3.0;
            let z1 = (b.p[0].z + b.p[1].z + b.p[2].z) / 3.0;
            if z0 < z1 {
                return Ordering::Greater;
            } else if z0 == z1 {
                return Ordering::Equal;
            } else {
                return Ordering::Less;
            }
        });

        for t in &mut tris_to_rater.into_iter() {
            // Drawing
            draw::filled_triangle(t.to_2d(), t.color.unwrap(), &mut canvas);

            // Wireframe for debugging
            draw::triangle(t.to_2d(), color_red, &mut canvas);
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 120));
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 5));
    }
}
