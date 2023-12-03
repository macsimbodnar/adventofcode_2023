fn main() {
    let input = include_str!("../../input.txt");
    let sum = run_day_1_part_1(input.to_owned());
    println!("Result Day 1 part 1: {}", sum);
}

fn run_day_1_part_1(lines: String) -> i32 {
    let mut sum: i32 = 0;

    for line in lines.lines() {
        let mut first_digit: i32 = -1;
        let mut last_digit: i32 = -1;

        let line = line.trim();
        for char in line.chars() {
            let mut current_digit = 0;

            if char.is_digit(10) {
                // If digit we set the values
                current_digit = char.to_string().parse().expect("Failed to parse the digit");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_1_test_input() {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"
            .to_string();

        let result = run_day_1_part_1(input);

        assert_eq!(result, 142);
    }

    #[test]
    fn day_1_full_input() {
        let input = include_str!("../../input.txt");
        let result = run_day_1_part_1(input.to_string());

        assert_eq!(result, 54953);
    }
}
