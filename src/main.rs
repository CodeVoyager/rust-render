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
    //let model = draw_3d::Mesh::from_obj("/mnt/usb/Resources/models/obj/wolf/wolf.obj");
    let model = draw_3d::Mesh::from_obj("<PATH>");
    let camera = draw_3d::Vec3D::new(0.0, 0.0, 0.0);
    let light = draw_3d::Vec3D::new(0.0, 0.0, -1.0);
    // normalizing light
    let l = (light.x * light.x + light.y * light.y + light.z * light.z).sqrt();
    let light = draw_3d::Vec3D::new(light.x / l, light.y / l, light.z / l);
    let near: f32 = 0.1;
    let far: f32 = 1000.0;
    let fov: f32 = 90.0;
    let aspect_ratio: f32 = screen_height as f32 / screen_width as f32;
    let fov_rad: f32 = 1.0 / (fov * 0.5).tan();

    let mat_proj = transform::Mat4x4::mat_proj(
        aspect_ratio * fov_rad,       // x00
        fov_rad,                      // x11
        far / (far - near),           // x22
        1.0,                          // x23
        (-near * far) / (far - near), // x32
        0.0,                          // x33
    );

    let screen_width_half = screen_width as f32 * 0.5;
    let screen_height_half = screen_height as f32 * 0.5;
    let mut theta = 0.0;
    // HACK: pushing object further into space so my computer
    // does not melt
    let z_offset = 1500.0;

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
        let mut mat_rot_x = transform::Mat4x4::new_empty();
        let mut mat_rot_z = transform::Mat4x4::new_empty();

        theta += 1.0 * time_elapsed_frac;
        mat_rot_z.m[0][0] = theta.cos();
        mat_rot_z.m[0][1] = theta.sin();
        mat_rot_z.m[1][0] = -theta.sin();
        mat_rot_z.m[1][1] = theta.cos();
        mat_rot_z.m[2][2] = 1.0;
        mat_rot_z.m[3][3] = 1.0;

        mat_rot_x.m[0][0] = 1.0;
        mat_rot_x.m[1][1] = (theta * 0.5).cos();
        mat_rot_x.m[1][2] = (theta * 0.5).sin();
        mat_rot_x.m[2][1] = -(theta * 0.5).sin();
        mat_rot_x.m[2][2] = (theta * 0.5).cos();
        mat_rot_x.m[3][3] = 1.0;

        prev_sys_time = sys_time;

        canvas.set_draw_color(color_black);
        canvas.clear();

        let mut tris_to_rater: Vec<draw_3d::Triangle3D> = Vec::new();
        for i in 0..model.tris.len() {
            //println!("triangle {}", i);
            let mut tri_projected = draw_3d::Triangle3D::new_empty();
            let mut tri_rot_z = draw_3d::Triangle3D::new_empty();
            let mut tri_rot_zx = draw_3d::Triangle3D::new_empty();

            // Rotation
            tri_rot_z.p[0] = transform::mult_matrix_vector(&model.tris[i].p[0], &mat_rot_z);
            tri_rot_z.p[1] = transform::mult_matrix_vector(&model.tris[i].p[1], &mat_rot_z);
            tri_rot_z.p[2] = transform::mult_matrix_vector(&model.tris[i].p[2], &mat_rot_z);

            tri_rot_zx.p[0] = transform::mult_matrix_vector(&tri_rot_z.p[0], &mat_rot_x);
            tri_rot_zx.p[1] = transform::mult_matrix_vector(&tri_rot_z.p[1], &mat_rot_x);
            tri_rot_zx.p[2] = transform::mult_matrix_vector(&tri_rot_z.p[2], &mat_rot_x);

            // Translation
            let mut tri_translated = tri_rot_zx.clone();

            tri_translated.p[0].z += z_offset;
            tri_translated.p[1].z += z_offset;
            tri_translated.p[2].z += z_offset;

            let line1 = transform::get_line(&tri_translated.p[0], &tri_translated.p[1]);
            let line2 = transform::get_line(&tri_translated.p[0], &tri_translated.p[2]);
            let normal = transform::get_normal(&line1, &line2);
            if transform::get_dot_product(
                &normal,
                &draw_3d::Vec3D::new(
                    tri_translated.p[0].x - camera.x,
                    tri_translated.p[0].y - camera.y,
                    tri_translated.p[0].z - camera.z,
                ),
            ) < 0.0
            {
                // 3D -> 2D
                tri_projected.p[0] = transform::mult_matrix_vector(&tri_translated.p[0], &mat_proj);
                tri_projected.p[1] = transform::mult_matrix_vector(&tri_translated.p[1], &mat_proj);
                tri_projected.p[2] = transform::mult_matrix_vector(&tri_translated.p[2], &mat_proj);

                // Illumination
                let light_dp = transform::get_dot_product(&normal, &light);
                let color = Color::RGB(
                    (255.0 * light_dp) as u8,
                    (255.0 * light_dp) as u8,
                    (255.0 * light_dp) as u8,
                );

                tri_projected.color = Some(color);

                tri_projected.p[0].x += 1.0;
                tri_projected.p[0].y += 1.0;
                tri_projected.p[1].x += 1.0;
                tri_projected.p[1].y += 1.0;
                tri_projected.p[2].x += 1.0;
                tri_projected.p[2].y += 1.0;

                tri_projected.p[0].x *= screen_width_half;
                tri_projected.p[0].y *= screen_height_half;
                tri_projected.p[1].x *= screen_width_half;
                tri_projected.p[1].y *= screen_height_half;
                tri_projected.p[2].x *= screen_width_half;
                tri_projected.p[2].y *= screen_height_half;

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
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 120));
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 5));
    }
}
