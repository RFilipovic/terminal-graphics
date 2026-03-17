use std::io::{self, Write};
use std::thread;
use std::time::Duration;

mod constants;
mod helpers;
mod perspective_projection;
mod structs;
mod writer;

use constants::{FRAME_HEIGHT, FRAME_WIDTH};
use helpers::serialize;
use structs::*;
use writer::{draw_line, set_pixel};

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

    let point0 = Point { x: 59, y: 29 };
    let point1 = Point { x: 0, y: 0 };
    draw_line(point0, point1, &mut render_data);

    let point0 = Point { x: 59, y: 0 };
    let point1 = Point { x: 0, y: 29 };
    draw_line(point0, point1, &mut render_data);

    println!("{}", serialize(&render_data, FRAME_WIDTH, FRAME_HEIGHT));

    /*
    let cube_vertices = vec![
        Vec3 {
            x: -1.0,
            y: -1.0,
            z: -1.0,
        },
        Vec3 {
            x: 1.0,
            y: -1.0,
            z: -1.0,
        },
        Vec3 {
            x: 1.0,
            y: 1.0,
            z: -1.0,
        },
        Vec3 {
            x: -1.0,
            y: 1.0,
            z: -1.0,
        },
        Vec3 {
            x: -1.0,
            y: -1.0,
            z: 1.0,
        },
        Vec3 {
            x: 1.0,
            y: -1.0,
            z: 1.0,
        },
        Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
        Vec3 {
            x: -1.0,
            y: 1.0,
            z: 1.0,
        },
    ];
    */

    Ok(())
}
