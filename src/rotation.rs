use crate::constants::THETA;
use crate::structs::Vec3;

pub fn rotate_point(vertices: &mut Vec<Vec3>, axis: &char) {
    match axis {
        'x' => rotate_around_x(vertices),
        'y' => rotate_around_y(vertices),
        'z' => rotate_around_z(vertices),
        _ => panic!("Wrong character passed to rotate_point"),
    }
}

fn rotate_around_x(vertices: &mut Vec<Vec3>) {
    for vertex in vertices {
        let new_y = vertex.y * THETA.cos() - vertex.z * THETA.sin();
        let new_z = vertex.y * THETA.sin() + vertex.z * THETA.cos();
        vertex.y = new_y;
        vertex.z = new_z;
    }
}

fn rotate_around_y(vertices: &mut Vec<Vec3>) {
    for vertex in vertices {
        let new_x = vertex.x * THETA.cos() + vertex.z * THETA.sin();
        let new_z = -vertex.x * THETA.sin() + vertex.z * THETA.cos();
        vertex.x = new_x;
        vertex.z = new_z;
    }
}

fn rotate_around_z(vertices: &mut Vec<Vec3>) {
    for vertex in vertices {
        let new_x = vertex.x * THETA.cos() - vertex.y * THETA.sin();
        let new_y = vertex.x * THETA.sin() + vertex.y * THETA.cos();
        vertex.x = new_x;
        vertex.y = new_y;
    }
}
