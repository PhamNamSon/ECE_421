use rand::prelude::*;

// Note this useful idiom: importing names from outer (for mod tests) scope.
use super::*;

#[test]
pub fn basic_add() {
    assert_eq!(calculator::add(1.0, 2.0), 3.0);
}
#[test]
pub fn add_negative_number() {
	 assert_eq!(calculator::add(-1.0, 2.0), 1.0);
}
#[test]
pub fn add_random_numbers() {
	let mut rng = thread_rng();
    let x: f64 = rng.gen(); // random number in range [0, 1)
    let y: f64 = rng.gen();
    assert_eq!(calculator::add(x, y), x+y);
}

#[test]
pub fn basic_subtract() {
	assert_eq!(calculator::subtract(4.0, 2.0), 2.0);
}
#[test]
pub fn subtract_negative_number() {
    assert_eq!(calculator::subtract(-3.0, 2.0), -5.0);
}
#[test]
pub fn subtract_random_numbers() {
    let mut rng = thread_rng();
    let x: f64 = rng.gen(); // random number in range [0, 1)
    let y: f64 = rng.gen();
    assert_eq!(calculator::subtract(x, y), x-y);
}
