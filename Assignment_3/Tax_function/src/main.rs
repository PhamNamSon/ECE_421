use std::io;


fn tax_function(income_str: String) -> i64 {
    let income: i64 = income_str.parse().expect("Please enter a valid number");
    if income < 0 {
        panic!("Income cannot be negative");
    } else if income < 10000 {
        income
    } else if income >= 10000 && income < 50000 {
        income - (income * 10 / 100)
    } else if income >= 50000 && income < 100000 {
        income - (income * 20 / 100)
    } else if income >= 100000 && income < 1000000 {
        income - (income * 30 / 100)
    } else {
        income - (income * 40 / 100)
    }
}

fn main() {
    println!("Enter your income: ");
    let mut income_str = String::new();
    io::stdin().read_line(&mut income_str).expect("Failed to read line");
    let taxed_income = tax_function(income_str.trim().to_string());
    println!("Your taxed income is: {}", taxed_income);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_income_is_a_negative_number() {
        tax_function("-1".to_string());
    }

    #[test]
    #[should_panic]
    fn test_income_is_not_an_integer() {
        tax_function("1.5".to_string());
    }

    #[test]
    #[should_panic]
    fn test_income_is_not_a_number() {
        tax_function("abc".to_string());

    }
}
