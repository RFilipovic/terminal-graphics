use crate::constants::THETA;
use crate::structs::Vec3;

pub fn rotate_point(vertices: &mut Vec<Vec3>, axis: &char, direction: f32) {
    let theta = THETA * direction;
    let center = calculate_center(vertices);
    for vertex in vertices.iter_mut() {
        vertex.x -= center.x;
        vertex.y -= center.y;
        vertex.z -= center.z;
    }

    match axis {
        'x' => rotate_around_x(vertices, theta),
        'y' => rotate_around_y(vertices, theta),
        'z' => rotate_around_z(vertices, theta),
        _ => panic!("Wrong character passed to rotate_point"),
    }

    for vertex in vertices {
        vertex.x += center.x;
        vertex.y += center.y;
        vertex.z += center.z;
    }
}

fn rotate_around_x(vertices: &mut Vec<Vec3>, theta: f32) {
    for vertex in vertices {
        let new_y = vertex.y * theta.cos() - vertex.z * theta.sin();
        let new_z = vertex.y * theta.sin() + vertex.z * theta.cos();
        vertex.y = new_y;
        vertex.z = new_z;
    }
}

fn rotate_around_y(vertices: &mut Vec<Vec3>, theta: f32) {
    for vertex in vertices {
        let new_x = vertex.x * theta.cos() + vertex.z * theta.sin();
        let new_z = -vertex.x * theta.sin() + vertex.z * theta.cos();
        vertex.x = new_x;
        vertex.z = new_z;
    }
}

fn rotate_around_z(vertices: &mut Vec<Vec3>, theta: f32) {
    for vertex in vertices {
        let new_x = vertex.x * theta.cos() - vertex.y * theta.sin();
        let new_y = vertex.x * theta.sin() + vertex.y * theta.cos();
        vertex.x = new_x;
        vertex.y = new_y;
    }
}

fn calculate_center(vertices: &mut [Vec3]) -> Vec3 {
    let n = vertices.len() as f32;
    let (sum_x, sum_y, sum_z) = vertices.iter().fold((0.0, 0.0, 0.0), |(sx, sy, sz), v| {
        (sx + v.x, sy + v.y, sz + v.z)
    });
    Vec3 {
        x: sum_x / n,
        y: sum_y / n,
        z: sum_z / n,
    }
}
