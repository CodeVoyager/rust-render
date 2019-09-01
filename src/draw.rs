//! Part of the project used for drawing 2D shapes on a screen
//!
//!


use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use std::cmp;

pub const PIXEL_SIZE: i32 = 1;

/// Triangle shape
///
#[derive(Debug, Clone)]
pub struct Triangle {
    a: Point,
    b: Point,
    c: Point,
    pub color: Option<Color>,
}

impl Triangle {
    pub fn new(a: Point, b: Point, c: Point) -> Triangle {
        Triangle { a, b, c, color: None }
    }
}

/// My own API for putting pixel on a screen
///
/// This is discourged by SDL2 rust binding author(s). I do it
/// because:
/// 1. This is fun/educational project
/// 2. Pixel in memory can be presented by more than one physical/canvas pixel
///
pub fn pixel(point: Point, color: Color, canvas: &mut WindowCanvas) {
    canvas.set_draw_color(color);
    if PIXEL_SIZE == 1 {
        canvas.draw_point(point).unwrap();
    } else {
        for dx in 0..PIXEL_SIZE {
            for dy in 0..PIXEL_SIZE {
                canvas
                    .draw_point(Point::new(
                        point.x * PIXEL_SIZE + dx,
                        point.y * PIXEL_SIZE + dy,
                    ))
                    .unwrap();
            }
        }
    }
}

pub fn line(point_a: Point, point_b: Point, color: Color, canvas: &mut WindowCanvas) {
    canvas.set_draw_color(color);

    let dx = point_a.x - point_b.x;
    let dy = point_a.y - point_b.y;

    if dx == 0 {
        let y1 = cmp::min(point_a.y, point_b.y);
        let y2 = cmp::max(point_a.y, point_b.y);
        for y in y1..=y2 {
            pixel(Point::new(point_a.x, y), color, canvas);
        }

        return;
    }

    if dy == 0 {
        let x1 = cmp::min(point_a.x, point_b.x);
        let x2 = cmp::max(point_a.x, point_b.x);
        for x in x1..=x2 {
            pixel(Point::new(x, point_a.y), color, canvas);
        }
        return;
    }

    let dx = point_b.x - point_a.x;
    let dy = point_b.y - point_a.y;

    if dx.abs() >= dy.abs() {
        let mut y = point_a.y as f32 + 0.5;
        let dly = dy as f32 / dx as f32;

        if dx > 0 {
            let x_start = point_a.x;

            for x in x_start..=point_b.x {
                pixel(Point::new(x, y.floor() as i32), color, canvas);
                y += dly
            }
        } else {
            let mut x_start = point_a.x;
            loop {
                pixel(Point::new(x_start, y.floor() as i32), color, canvas);
                y -= dly;
                x_start -= 1;
                if x_start < point_b.x {
                    break;
                }
            }
        }
    } else {
        let mut x = point_a.x as f32 + 0.5;
        let dlx = dx as f32 / dy as f32;

        if dy > 0 {
            let y_start = point_a.y;

            for y in y_start..=point_b.y {
                pixel(Point::new(x.floor() as i32, y), color, canvas);
                x += dlx
            }
        } else {
            let mut y_start = point_a.y;

            loop {
                pixel(Point::new(x.floor() as i32, y_start), color, canvas);
                x -= dlx;
                y_start -= 1;
                if y_start < point_b.y {
                    break;
                }
            }
        }
    }
}

pub fn triangle(triangle: Triangle, color: Color, canvas: &mut WindowCanvas) {
    line(triangle.a, triangle.b, color, canvas);
    line(triangle.b, triangle.c, color, canvas);
    line(triangle.a, triangle.c, color, canvas);
}

pub fn filled_triangle(t: Triangle, color: Color, canvas: &mut WindowCanvas) {
    let mut ps = [t.a, t.b, t.c];

    ps.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

    let [p1, p2, p3] = ps;

    if p2.y == p3.y {
        filled_flat_bottom_triangle(Triangle::new(p1, p2, p3), color, canvas);
    } else if p1.y == p2.y {
        filled_flat_top_triangle(Triangle::new(p3, p1, p2), color, canvas);
    } else {
        let x4 =
            p1.x + (((p2.y - p1.y) as f32 / (p3.y - p1.y) as f32) * (p3.x - p1.x) as f32) as i32;
        let p4 = Point::new(x4, p2.y);

        filled_flat_bottom_triangle(Triangle::new(p1, p2, p4), color, canvas);
        filled_flat_top_triangle(Triangle::new(p3, p2, p4), color, canvas);
    }
}

/// Draws flat bottom triangle
///
/// Assumptions:
/// t.a - top of a triangle
/// t.b.y == t.c.y
///
pub fn filled_flat_bottom_triangle(t: Triangle, color: Color, canvas: &mut WindowCanvas) {
    let inv_slope_1 = (t.b.x - t.a.x) as f32 / (t.b.y - t.a.y) as f32;
    let inv_slope_2 = (t.c.x - t.a.x) as f32 / (t.c.y - t.a.y) as f32;
    let mut curr_x_1 = t.a.x as f32;
    let mut curr_x_2 = t.a.x as f32;
    let mut scan_line_y = t.a.y;

    while scan_line_y <= t.b.y {
        line(
            Point::new(curr_x_1 as i32, scan_line_y),
            Point::new(curr_x_2 as i32, scan_line_y),
            color,
            canvas,
        );

        curr_x_1 += inv_slope_1;
        curr_x_2 += inv_slope_2;
        scan_line_y += 1;
    }
}

/// Draws flat top triangle
///
/// Assumptions:
/// t.a - bottom of a triangle
/// t.b.y == t.c.y
///
pub fn filled_flat_top_triangle(t: Triangle, color: Color, canvas: &mut WindowCanvas) {
    let inv_slope_1 = (t.a.x - t.b.x) as f32 / (t.a.y - t.b.y) as f32;
    let inv_slope_2 = (t.a.x - t.c.x) as f32 / (t.a.y - t.c.y) as f32;
    let mut curr_x_1 = t.a.x as f32;
    let mut curr_x_2 = t.a.x as f32;
    let mut scan_line_y = t.a.y;

    while scan_line_y >= t.b.y {
        line(
            Point::new(curr_x_1 as i32, scan_line_y),
            Point::new(curr_x_2 as i32, scan_line_y),
            color,
            canvas,
        );

        curr_x_1 -= inv_slope_1;
        curr_x_2 -= inv_slope_2;
        scan_line_y -= 1;
    }
}
