use std::cmp::max;

pub struct Point {
    x: i8,
    y: i8,
}

pub fn compute_chebyshev_distance(p1: &Point, p2: &Point) -> i32 {
    let x_diff = p1.x - p2.x;
    let y_diff = p1.y - p2.y;
    max(x_diff.abs(), y_diff.abs()) as i32
}

fn main() {
}
    