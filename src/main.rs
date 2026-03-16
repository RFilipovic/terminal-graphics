use std::io::{self, Write};

const FRAME_WIDTH: usize = 60;
const FRAME_HEIGHT: usize = 30;

struct FrameBuffer {
    width: usize,
    height: usize,
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
        println!("{}", serialize(render_data, FRAME_WIDTH, FRAME_HEIGHT));
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut out = io::stdout();
    let fb = FrameBuffer::new(FRAME_WIDTH, FRAME_HEIGHT);

    let mut render_data: Vec<char> = vec!['.'; fb.width * fb.height];

    set_pixel(&mut render_data, 59, 29, 'O');
    set_pixel(&mut render_data, 0, 0, 'O');

    write!(out, "\x1b[2J")?;
    out.flush()?;

    Ok(())
}
