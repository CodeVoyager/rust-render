use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use std::cmp;

pub const PIXEL_SIZE: i32 = 2;

#[derive(Debug)]
pub struct Triangle {
    a: Point,
    b: Point,
    c: Point,
}

impl Triangle {
    pub fn new(a: Point, b: Point, c: Point) -> Triangle {
        Triangle { a, b, c }
    }
}

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

