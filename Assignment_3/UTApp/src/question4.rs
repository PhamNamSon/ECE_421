use rand::prelude::*;

// Note this useful idiom: importing names from outer (for mod tests) scope.
use super::*;

#[test]
pub fn test_basic_roots() {
    let a = 1.0;
    let b = -2.0;
    let c = -3.0;
    assert_eq!(calculator::get_roots(a, b, c), (3.0, -1.0));
}
#[test]
pub fn test_single_root() {
    let a = 1.0;
    let b = -2.0;
    let c = 1.0;
    assert_eq!(calculator::get_roots(a, b, c), (1.0, 1.0));
}
#[test]
pub fn test_random_solvable_quadratic() {
    let mut rng = thread_rng();
    let mut a: f64 = rng.gen(); // random number in range [0, 1)
    while a == 0.0 {
        a = rng.gen();
    }
    let b: f64 = rng.gen();
    let c: f64 = rng.gen();
    let discriminant = b.powi(2) - 4.0 * a * c;
    if discriminant >= 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        assert_eq!(calculator::get_roots(a, b, c), (root1, root2));
    }
}
#[test]
pub fn test_random_non_solvable_quadratic() {
    let mut rng = thread_rng();
    let mut a: f64 = rng.gen(); // random number in range [0, 1)
    while a == 0.0 {
        a = rng.gen();
    }
    let b: f64 = rng.gen();
    let c: f64 = rng.gen();
    let discriminant = b.powi(2) - 4.0 * a * c;
    if discriminant < 0.0 {
        assert!(calculator::get_roots(a, b, c).0.is_nan() && calculator::get_roots(a, b, c).1.is_nan());
    }
}
