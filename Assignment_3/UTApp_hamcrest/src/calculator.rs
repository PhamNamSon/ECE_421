pub fn add(x: f64, y: f64) -> f64 {
    x + y
}

pub fn subtract(x: f64, y: f64) -> f64 {
    x - y
}

pub fn multiply(x: f64, y: f64) -> f64 {
    x * y
}

pub fn divide(x: f64, y: f64) -> f64 {
    x / y
}

pub fn get_square_root(x: f64) -> f64 {
    x.sqrt()
}

pub fn get_roots(a: f64, b: f64, c: f64) -> (f64, f64) {
    let discriminant = b.powi(2) - 4.0 * a * c;
    if discriminant < 0.0 {
        (f64::NAN, f64::NAN)
    } else {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        (root1, root2)
    }
}