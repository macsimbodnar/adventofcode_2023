fn main() {
    let input = include_str!("../../input.txt");

    let sum = run_day_2_part_2(input.to_owned());
    println!("Result Day 2 part 2: {}", sum);
}

#[derive(Debug)]
struct CubesSet {
    red: i32,
    green: i32,
    blue: i32,
}

fn run_day_2_part_2(all_games: String) -> i32 {
    let mut sum = 0;

    for game in all_games.lines() {
        let game = game.trim();

        let tokens = game.split(":").collect::<Vec<&str>>();

        let mut min_required_cube_set = CubesSet {
            red: 0,
            green: 0,
            blue: 0,
        };

        // Parsing cubes subsets
        let game_result_subsets = tokens[1].split(";");
        for game_result in game_result_subsets {
            // Split colors
            let color_results = game_result.trim().split(",");
            let mut set = CubesSet {
                red: 0,
                green: 0,
                blue: 0,
            };

            for color_result in color_results {
                let color_result = color_result.trim();

                let color_tokens = color_result
                    .trim()
                    .split_whitespace()
                    .collect::<Vec<&str>>();

                let number_of_cubes: i32 = color_tokens[0].trim().parse().unwrap();

                match color_tokens[1].trim() {
                    "red" => set.red = number_of_cubes,
                    "green" => set.green = number_of_cubes,
                    "blue" => set.blue = number_of_cubes,
                    _ => (),
                }
            }

            if set.red > min_required_cube_set.red {
                min_required_cube_set.red = set.red;
            }

            if set.green > min_required_cube_set.green {
                min_required_cube_set.green = set.green;
            }

            if set.blue > min_required_cube_set.blue {
                min_required_cube_set.blue = set.blue;
            }
        }

        sum += min_required_cube_set.red * min_required_cube_set.green * min_required_cube_set.blue;
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_2_test_input() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let result = run_day_2_part_2(input.to_string());
        assert_eq!(result, 2286);
    }

    #[test]
    fn day_2_full_input() {
        let input = include_str!("../../input.txt");

        let result = run_day_2_part_2(input.to_owned());
        assert_eq!(result, 66363);
    }
}
