use crate::constants::{D, MARGIN};
use crate::structs::Vec3;

pub fn serialize(render_data: &[char], width: usize, height: usize) -> String {
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

pub fn deserialize(frame: String) -> Vec<char> {
    frame.chars().filter(|c| *c != '\n').collect()
}

pub fn transform_vec3_to_base(vertices: &mut Vec<Vec3>) {
    let delta = D + MARGIN;

    for element in vertices {
        element.z += delta;
    }
}
