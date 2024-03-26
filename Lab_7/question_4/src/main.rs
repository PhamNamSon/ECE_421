use std::cmp::max;

#[repr(C)]
pub struct Point {
    x: i8,
    y: i8,
}

extern {
    pub fn abs(i: i32) -> i32;
}
    

pub fn compute_chebyshev_distance_c(p1: &Point, p2: &Point) -> i32 {
    let x_diff = p1.x - p2.x;
    let y_diff = p1.y - p2.y;
    unsafe {
        max(abs(x_diff as i32), abs(y_diff as i32))
    }
}

fn main() {
}
    