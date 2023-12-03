fn main() {
    let input = include_str!("../../input.txt");

    let bag_load = CubesSet {
        red: 12,
        green: 13,
        blue: 14,
    };
    let sum = run_day_1_part_1(input.to_owned(), bag_load);
    println!("Result Day 2 part 1: {}", sum);
}

#[derive(Debug)]
struct CubesSet {
    red: i32,
    green: i32,
    blue: i32,
}

fn run_day_1_part_1(all_games: String, bag_load: CubesSet) -> i32 {
    let mut sum = 0;

    'games_loop: for game in all_games.lines() {
        let game = game.trim();

        let tokens = game.split(":").collect::<Vec<&str>>();

        // Get game id
        let game_id: i32 = tokens[0].split_whitespace().collect::<Vec<&str>>()[1]
            .trim()
            .parse()
            .unwrap();

        let mut sets: Vec<CubesSet> = Vec::new();

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

            sets.push(set);
        }

        // Check if the game is possible or not
        for set in sets {
            if set.red > bag_load.red || set.green > bag_load.green || set.blue > bag_load.blue {
                continue 'games_loop;
            }
        }

        sum += game_id;
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_1_test_input() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let bag_load = CubesSet {
            red: 12,
            green: 13,
            blue: 14,
        };

        let result = run_day_1_part_1(input.to_string(), bag_load);
        assert_eq!(result, 8);
    }

    #[test]
    fn day_1_full_input() {
        let input = include_str!("../../input.txt");

        let bag_load = CubesSet {
            red: 12,
            green: 13,
            blue: 14,
        };
        let result = run_day_1_part_1(input.to_owned(), bag_load);
        assert_eq!(result, 2369);
    }
}
