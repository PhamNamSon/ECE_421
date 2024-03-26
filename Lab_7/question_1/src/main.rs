pub struct Point {
    x: i8,
    y: i8,
}

pub fn compute_euclidean_distance(p1: &Point, p2: &Point) -> f64 {
    let x_diff = p1.x - p2.x;
    let y_diff = p1.y - p2.y;
    ((x_diff * x_diff + y_diff * y_diff) as f64).sqrt()
}

fn main() {
}
