use hamcrest2::prelude::*;
use rand::prelude::*;

// Note this useful idiom: importing names from outer (for mod tests) scope.
use super::*;

#[test]
pub fn basic_add() {
    assert_that!(calculator::add(1.0, 2.0), close_to(3.0, 0.001));
}

#[test]
pub fn add_negative_number() {
    assert_that!(calculator::add(-1.0, 2.0), close_to(1.0, 0.001));
}

#[test]
pub fn add_random_numbers() {
    let mut rng = thread_rng();
    let x: f64 = rng.gen(); // random number in range [0, 1)
    let y: f64 = rng.gen();
    assert_that!(calculator::add(x, y), close_to(x + y, 0.001));
}

#[test]
pub fn basic_subtract() {
    assert_that!(calculator::subtract(4.0, 2.0), close_to(2.0, 0.001));
}

#[test]
pub fn subtract_negative_number() {
    assert_that!(calculator::subtract(-3.0, 2.0), close_to(-5.0, 0.001));
}

#[test]
pub fn subtract_random_numbers() {
    let mut rng = thread_rng();
    let x: f64 = rng.gen(); // random number in range [0, 1)
    let y: f64 = rng.gen();
    assert_that!(calculator::subtract(x, y), close_to(x - y, 0.001));
}
