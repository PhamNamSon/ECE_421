use std::io;
use std::cmp::max;

pub struct Point {
    x: i8,
    y: i8,
}

fn read_point() -> Point {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let parsed: Result<Vec<i8>, _> = input.trim().split_whitespace()
            .map(|x| x.parse::<i8>())
            .collect();

        match parsed {
            Ok(coords) if coords.len() == 2 => return Point { x: coords[0], y: coords[1] },
            _ => println!("Invalid input. Please type two numbers separated by a space."),
        }
    }
}

pub fn compute_euclidean_distance(p1: &Point, p2: &Point) -> f64 {
    let x_diff = p1.x - p2.x;
    let y_diff = p1.y - p2.y;
    ((x_diff * x_diff + y_diff * y_diff) as f64).sqrt()
}

pub fn compute_manhattan_distance(p1: &Point, p2: &Point) -> i32 {
    let x_diff = (p1.x as i32 - p2.x as i32).abs();
    let y_diff = (p1.y as i32 - p2.y as i32).abs();
    x_diff + y_diff
}

pub fn compute_chebyshev_distance(p1: &Point, p2: &Point) -> i32 {
    let x_diff = p1.x - p2.x;
    let y_diff = p1.y - p2.y;
    max(x_diff.abs(), y_diff.abs()) as i32
}

fn main() {
    loop {
        println!("Enter: ");
        println!("1: Euclidean distance");
        println!("2: Manhattan distance");
        println!("3: Chebyshev distance");
        println!("4: Exit");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = choice.trim().parse().expect("Please type a number!");
        if choice == 4 {
            println!("Exiting...");
            break;
        }
        println!("Enter the coordinates of the first point:");
        let p1 = read_point();
        println!("Enter the coordinates of the second point:");
        let p2 = read_point();
        match choice {
            1 => println!("Euclidean distance: {}", compute_euclidean_distance(&p1, &p2)),
            2 => println!("Manhattan distance: {}", compute_manhattan_distance(&p1, &p2)),
            3 => println!("Chebyshev distance: {}", compute_chebyshev_distance(&p1, &p2)),
            _ => println!("Invalid choice!"),
        }
    }
}