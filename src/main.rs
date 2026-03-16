use std::io::{self, Write};
use std::thread;
use std::time::Duration;

const FRAME_WIDTH: usize = 60;
const FRAME_HEIGHT: usize = 30;

struct FrameBuffer {
    width: usize,
    height: usize,
}

struct Point {
    x: usize,
    y: usize,
}

impl FrameBuffer {
    fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

fn serialize(render_data: &[char], width: usize, height: usize) -> String {
    let mut frame = "".to_string();

    for i in 0..height {
        //frame.push_str("        ");
        for j in 0..width {
            frame.push(render_data[i * width + j]);
        }
        frame.push('\n');
    }
    frame
}

/*
fn deserialize(frame: String) -> Vec<char> {
    frame.chars().filter(|c| *c != '\n').collect()
}
*/

fn set_pixel(render_data: &mut [char], x: usize, y: usize, ch: char) {
    if x >= FRAME_WIDTH || y >= FRAME_HEIGHT {
        println!("x must be less that FRAME_WIDTH, y must be less than FRAME_HEIGHT")
    } else {
        render_data[y * FRAME_WIDTH + x] = ch;
    }
}

fn draw_line(point0: Point, point1: Point, render_data: &mut [char]) {
    let mut x0 = point0.x as isize;
    let mut y0 = point0.y as isize;
    let x1 = point1.x as isize;
    let y1 = point1.y as isize;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = dx - dy;
    loop {
        set_pixel(render_data, x0 as usize, y0 as usize, '#');
        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }
        if e2 < dx {
            err += dx;
            y0 += sy;
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut out = io::stdout();
    let fb = FrameBuffer::new(FRAME_WIDTH, FRAME_HEIGHT);

    let mut render_data: Vec<char> = vec![' '; fb.width * fb.height];
    /*
        loop {
            write!(out, "\x1b[2J")?;
            out.flush()?;

            write!(out, "\x1b[H")?;
            out.flush()?;

            println!("{}", serialize(&render_data, FRAME_WIDTH, FRAME_HEIGHT));
        }
    */

    let point0 = Point { x: 29, y: 29 };
    let point1 = Point { x: 0, y: 0 };
    draw_line(point0, point1, &mut render_data);

    let point0 = Point { x: 59, y: 0 };
    let point1 = Point { x: 0, y: 29 };
    draw_line(point0, point1, &mut render_data);

    println!("{}", serialize(&render_data, FRAME_WIDTH, FRAME_HEIGHT));

    Ok(())
}
