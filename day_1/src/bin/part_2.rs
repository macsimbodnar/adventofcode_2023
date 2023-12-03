fn main() {
    let input = include_str!("../../input.txt");
    let sum = run_day_1_part_2(input.to_owned());
    println!("Result Day 1 part 1: {}", sum);
}

fn run_day_1_part_2(lines: String) -> i32 {
    let mut sum: i32 = 0;

    for line in lines.lines() {
        let mut first_digit: i32 = -1;
        let mut last_digit: i32 = -1;

        let line = line.trim();
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

    return sum;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_1_test_input() {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"
            .to_string();

        let result = run_day_1_part_2(input);

        assert_eq!(result, 281);
    }

    #[test]
    fn day_1_full_input() {
        let input = include_str!("../../input.txt");
        let result = run_day_1_part_2(input.to_string());

        assert_eq!(result, 53868);
    }
}
