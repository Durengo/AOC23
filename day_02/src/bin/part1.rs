use std::collections::HashMap;

// Current algorithm returns 480 for input, which is not correct.

fn main() {
    println!("Part 1!");

    let input = include_str!("../../input_p1/input.txt");

    let maxAllowed: MaximumAllowed = MaximumAllowed {
        red: 12,
        green: 13,
        blue: 14,
    };

    let result = part1(input, &maxAllowed);

    println!("Result: {}", result);
}

struct Game {
    id: i32,
    red: i32,
    green: i32,
    blue: i32,
}

impl Game {
    fn print(&self) {
        println!(
            "Game:\nID: {}\nRed Count: {}\nGreen Count: {}\nBlue Count: {}",
            self.id,
            self.red,
            self.green,
            self.blue
        );
    }
}

struct MaximumAllowed {
    red: i32,
    green: i32,
    blue: i32,
}

impl MaximumAllowed {
    fn print(&self) {
        println!(
            "Maximum Allowed:\nRed Count: {}\nGreen Count: {}\nBlue Count: {}",
            self.red,
            self.green,
            self.blue
        );
    }
}

// Here we will parse the input and just instantiate our vector of games
fn parse_games(input: &str) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();

    for line in input.lines() {
        // println!("Line: {}", line);

        let mut game = Game {
            id: 0,
            red: 0,
            green: 0,
            blue: 0,
        };

        // Each line is in such a format: "Game 1: x color1, y color2, z color3; x color1, y color2; z color3"
        // Colors type can repeat as well as their count or the count might be different, so we need to make sure to add the correct counts of each color type appropriately
        // The colors are separated by a comma and a space, so we can split the line by ", " and then split each color by " " to get the color and its count
        // But the colors are also in sets, so we need to split the line by "; " first to get the sets and then split each set by ", " to get the colors
        // We will use a hashmap to store the color and its count (defined earlier)
        let mut color_count = HashMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0),
        ]);

        // Split the line into two parts: the game id and the colors
        let mut line_parts: Vec<&str> = line.split(":").collect();

        // Remove the whitespace after the ':' character
        line_parts[1] = line_parts[1].trim_start();

        // for part in &line_parts {
        //     println!("Part: {}", part);
        // }
        // Get the game id
        let game_id_digits: String = line_parts[0]
            .chars()
            .filter(|c| c.is_digit(10))
            .collect();
        game.id = game_id_digits.parse::<i32>().unwrap();

        // Get the colors
        let colors: Vec<&str> = line_parts[1].split("; ").collect();

        for colors in &colors {
            // println!("Colors: {}", colors);

            let color_count: Vec<&str> = colors.split(", ").collect();

            for color_count in &color_count {
                // println!("Color Count: {}", color_count);

                let color_count: Vec<&str> = color_count.split(" ").collect();

                // Check against the hashmap and add the color count to the hashmap
                if color_count[1] == "red" {
                    game.red += color_count[0].parse::<i32>().unwrap();
                } else if color_count[1] == "green" {
                    game.green += color_count[0].parse::<i32>().unwrap();
                } else if color_count[1] == "blue" {
                    game.blue += color_count[0].parse::<i32>().unwrap();
                } else {
                    panic!("Invalid color!");
                }
            }
        }

        //game.print();
        games.push(game);
    }

    return games;
}

fn run_algorithm(games: &Vec<Game>, maxAllowed: &MaximumAllowed) -> Vec<i32> {
    let mut game_ids: Vec<i32> = Vec::new();

    // Iterate over the games and check if the current game's counts are less than the maximum allowed counts
    // If they are, we add them to a vector of game ids and then check the next game
    // If they are not, we check the next game
    // If we reach the end of the games and we still don't have a combination that satisfies the maximum allowed counts, we panic, but we must recheck the games again
    for game in games {
        if
            game.red <= maxAllowed.red &&
            game.green <= maxAllowed.green &&
            game.blue <= maxAllowed.blue
        {
            game_ids.push(game.id);
        }
    }

    return game_ids;
}

fn part1(input: &str, maxAllowed: &MaximumAllowed) -> String {
    // Parse the input and get the games
    let mut games: Vec<Game> = parse_games(input);
    for game in &games {
        game.print();
    }

    // Since the Ids are already sorted, we can proceed
    // Now we need to find a game combination that satisfies the maximum allowed counts
    // We do this by iterating over the games and checking if the current game's counts are less than the maximum allowed counts
    // If they are, we add them to a vector of game ids and then check the next game
    // If they are not, we check the next game
    // If we reach the end of the games and we still don't have a combination that satisfies the maximum allowed counts, we panic, but we must recheck the games again

    let game_ids: Vec<i32> = run_algorithm(&games, &maxAllowed);

    println!("Game Ids: {:?}", game_ids);

    // Now we need to add the game ids together and return the result
    let mut sum: i32 = 0;
    for game_id in game_ids {
        sum += game_id;
    }

    // return input.to_string();
    return sum.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_p1/test.txt");

        let maxAllowed: MaximumAllowed = MaximumAllowed {
            red: 12,
            green: 13,
            blue: 14,
        };

        let actual = part1(input, &maxAllowed);

        let expected = "8".to_string();

        assert_eq!(actual, expected);
    }
}
