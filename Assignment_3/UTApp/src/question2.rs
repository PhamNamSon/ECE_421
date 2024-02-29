use rand::prelude::*;

// Note this useful idiom: importing names from outer (for mod tests) scope.
use super::*;

#[test]
pub fn basic_multiply() {
    assert_eq!(calculator::multiply(1.0, 2.0), 2.0);
}
#[test]
pub fn multiply_negative_number() {
    assert_eq!(calculator::multiply(-1.0, 2.0), -2.0);
}
#[test]
pub fn multiply_random_numbers() {
    let mut rng = thread_rng();
    let x: f64 = rng.gen(); // random number in range [0, 1)
    let y: f64 = rng.gen();
    assert_eq!(calculator::multiply(x, y), x*y);
}

#[test]
pub fn basic_divide() {
    assert_eq!(calculator::divide(4.0, 2.0), 2.0);
}
#[test]
pub fn divide_negative_number() {
    assert_eq!(calculator::divide(-4.0, 2.0), -2.0);
}
#[test]
pub fn divide_random_numbers() {
    let mut rng = thread_rng();
    let x: f64 = rng.gen(); // random number in range [0, 1)
    let mut y: f64 = rng.gen();
    while y == 0.0 {
        y = rng.gen();
    }
    assert_eq!(calculator::divide(x, y), x/y);
}
