use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    let assets_dir = &args[1];
    run_day_1(assets_dir.to_owned());
}

fn run_day_1(assets_dir: String) {
    println!("Computing day 1...");

    const INPUT_DAY_1: &str = "/input_day_1.txt";

    let file_path = assets_dir + INPUT_DAY_1;

    println!("Reading file: {}", file_path);

    let mut sum: u32 = 0;

    let lines = read_lines(file_path).unwrap();

    for line in lines {
        let line = line.unwrap();

        let mut first_digit: char = 'a';
        let mut last_digit: char = 'a';

        for char in line.chars() {
            if char.is_digit(10) {
                if !first_digit.is_digit(10) {
                    first_digit = char;
                }

                last_digit = char;
            }
        }

        if !first_digit.is_digit(10) || !last_digit.is_digit(10) {
            eprintln!("Found line with no digits in challenge day 1 input");
            std::process::exit(1);
        }

        let mut number_str: String = first_digit.to_string();
        number_str.push(last_digit);

        let number: u32 = number_str.parse().expect("Failed to parse a number");
        // println!("Number: {}", number);

        sum += number
    }

    println!("Result Day 1: {}", sum);
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
