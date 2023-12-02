use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let assets_dir = &args[1];
    println!("Assets dir: {}", assets_dir);
    run_day_1(assets_dir.to_owned());
}

fn run_day_1(assets_dir: String) {
    println!("Computing day 1...");

    const INPUT_DAY_1: &str = "/input_day_1.txt";

    let file_path = assets_dir + INPUT_DAY_1;

    println!("Reading file: {}", file_path);

    let mut sum: i32 = 0;

    let lines = read_lines(file_path).unwrap();

    for line in lines {
        let line: String = line.unwrap();

        let mut first_digit: i32 = -1;
        let mut last_digit: i32 = -1;

        // for char in line.chars() {
        let mut chars = line.chars();
        for i in 0..line.len() {
            let char = chars.next().unwrap();

            let current_digit;

            if char.is_digit(10) {
                // If digit we set the values
                current_digit = char.to_string().parse().expect("Failed to parse the digit");
            } else {
                // If not digit we check if it's a spelled number starting at i
                current_digit = is_numbers_spelled_with_letters(&line[i..]);
            }

            if current_digit > 0 {
                if first_digit == -1 {
                    first_digit = current_digit;
                }

                last_digit = current_digit;
            }
        }

        let number: i32 = (first_digit * 10) + last_digit;

        sum += number
    }

    println!("Result Day 1: {}", sum);
}

// If digit returns the corresponding number else returns zero
fn is_numbers_spelled_with_letters(substr: &str) -> i32 {
    let mut result: i32 = 0;

    let numbers: Vec<(&str, i32)> = vec![
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    for (name, number) in numbers {
        if substr.starts_with(name) {
            result = number;
            break;
        }
    }

    return result;
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
