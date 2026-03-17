use crate::constants::D;
use crate::structs::{Point, Vec3};

pub fn perspective_projection(vertices: Vec<Vec3>) -> Vec<Point> {
    let mut projections: Vec<Point> = Vec::new();

    for element in vertices {
        let proj_x = (element.x * D) / element.z;
        let proj_y = (element.y * D) / element.z;
        projections.push(Point {
            x: proj_x,
            y: proj_y,
        });
    }
    projections
}
