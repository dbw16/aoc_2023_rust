use std::fs;

fn get_first_digit(line: &str) -> Option<u32> {
    line.chars().find(|c| c.is_digit(10)).and_then(|c| c.to_digit(10))
}

fn get_last_digit(line: &str) -> Option<u32> {
    line.chars().rev().find(|c| c.is_digit(10)).and_then(|c| c.to_digit(10))
}

fn part_1() -> u32 {
    let content = fs::read_to_string("day_1.txt").expect("Could not read file");
    let sum: u32 = content
        .lines()
        .map(|line| {
            let first_digit = get_first_digit(line).expect("No digit found");
            let last_digit = get_last_digit(line).expect("No digit found");

            first_digit * 10 + last_digit // Directly combine digits mathematically
        })
        .sum();

    sum
}

fn main() {
    let result = part_1();
    println!("Result: {}", result);
}
