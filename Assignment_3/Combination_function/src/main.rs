use std::io;
use std::num::ParseIntError;


fn read_number(prompt: &str) -> u32 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse::<u32>() {
            Ok(num) => return num,
            Err(ParseIntError { .. }) => {
                println!("Please enter a valid number");
                continue;
            }
        }
    }
}

fn combination_function(a: u32, b: u32) -> Option<u32> {
    if a < b {
        return None;
    }
    Some(factorial(a) / (factorial(b) * factorial(a - b)))
}

fn factorial(n: u32) -> u32 {
    if n == 0 {
        return 1;
    }
    n * factorial(n - 1)
}

fn main() {
    let a = read_number("Enter the value of a: ");
    let b = read_number("Enter the value of b: ");

    match combination_function(a, b) {
        Some(result) => println!("C({},{}) = {}", a, b, result),
        None => println!("The value of a must be greater than b")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_is_not_smaller_than_b() {
        assert_eq!(combination_function(5,3), Some(10));
    }

    #[test]
    fn test_both_a_and_b_are_positive() {
        assert_eq!(combination_function(5,2), Some(10));
    }
}
