use hamcrest2::prelude::*;
use rand::prelude::*;

// Note this useful idiom: importing names from outer (for mod tests) scope.
use super::*;

#[test]
pub fn basic_multiply() {
    assert_that!(calculator::multiply(1.0, 2.0), close_to(2.0, 0.001));
}

#[test]
pub fn multiply_negative_number() {
    assert_that!(calculator::multiply(-1.0, 2.0), close_to(-2.0, 0.001));
}

#[test]
pub fn multiply_random_numbers() {
    let mut rng = thread_rng();
    let x: f64 = rng.gen(); // random number in range [0, 1)
    let y: f64 = rng.gen();
    assert_that!(calculator::multiply(x, y), close_to(x * y, 0.001));
}

#[test]
pub fn basic_divide() {
    assert_that!(calculator::divide(4.0, 2.0), close_to(2.0, 0.001));
}

#[test]
pub fn divide_negative_number() {
    assert_that!(calculator::divide(-4.0, 2.0), close_to(-2.0, 0.001));
}

#[test]
pub fn divide_random_numbers() {
    let mut rng = thread_rng();
    let x: f64 = rng.gen(); // random number in range [0, 1)
    let mut y: f64 = rng.gen();
    while y == 0.0 {
        y = rng.gen();
    }
    assert_that!(calculator::divide(x, y), close_to(x / y, 0.001));
}
