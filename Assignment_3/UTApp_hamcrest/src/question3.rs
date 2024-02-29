use hamcrest2::prelude::*;
use rand::prelude::*;

// Note this useful idiom: importing names from outer (for mod tests) scope.
use super::*;

#[test]
pub fn test_random_positive_square_root() {
    let mut rng = thread_rng();
    let x: f64 = rng.gen(); // random number in range [0, 1)
    assert_that!(calculator::get_square_root(x), close_to(x.sqrt(), 0.001));
}

// This test is not possible to implement using hamcrest2
#[test]
pub fn test_random_negitive_square_root() {
    let mut rng = thread_rng();
    let x: f64 = rng.gen(); // random number in range [0, 1)
    let result = calculator::get_square_root(-x);
    assert!(result.is_nan());
}

#[test]
pub fn test_square_root_of_zero() {
    assert_that!(calculator::get_square_root(0.0), close_to(0.0, 0.001));
}

#[test]
pub fn test_square_root_of_one() {
    assert_that!(calculator::get_square_root(1.0), close_to(1.0, 0.001));
}
