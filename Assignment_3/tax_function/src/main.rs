use std::io;
use std::num::ParseFloatError;


fn read_number(prompt: &str) -> f64 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(ParseFloatError { .. }) => {
                println!("Please enter a valid number");
                continue;
            }
        }
    }
}

fn tax_function(income: f64) -> f64 {
    if income < 10000.0 {
        income
    } else if income >= 10000.0 && income < 50000.0 {
        income - (income * 0.1)
    } else if income >= 50000.0 && income < 100000.0 {
        income - (income * 0.2)
    } else if income >= 100000.0 && income < 1000000.0 {
        income - (income * 0.3)
    } else {
        income - (income * 0.4)
    }
}

fn main() {
    let income = read_number("Enter your income: ");
    println!("Your taxed income is: {}", tax_function(income as f64));
}


