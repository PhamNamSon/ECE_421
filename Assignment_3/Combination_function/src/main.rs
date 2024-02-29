use std::io;


fn combination_function(a_str: String, b_str: String) -> u32 {
    let a = a_str.trim().parse::<u32>().expect("Failed to parse 'a'");
    let b = b_str.trim().parse::<u32>().expect("Failed to parse 'b'");
    
    if a < b {
        panic!("'a' must not be smaller than 'b'");
    }

    factorial(a) / (factorial(b) * factorial(a - b))
}

fn factorial(n: u32) -> u32 {
    (1..=n).product()
}

fn main() {
    println!("Enter the value of a: ");
    let mut a_str = String::new();
    io::stdin().read_line(&mut a_str).expect("Failed to read line");

    println!("Enter the value of b: ");
    let mut b_str = String::new();
    io::stdin().read_line(&mut b_str).expect("Failed to read line");

    let result = combination_function(a_str.trim().to_string(), b_str.trim().to_string());
    println!("C({},{}) = {}", a_str.trim(), b_str.trim(), result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_a_not_smaller_than_b() {
        combination_function("3".to_string(), "4".to_string());
    }

    #[test]
    #[should_panic]
    fn test_both_a_and_b_are_positive() {
        combination_function("-3".to_string(), "4".to_string());
    }
}
