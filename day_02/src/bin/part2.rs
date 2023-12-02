fn main() {
    println!("Part 2!");

    let input = include_str!("../../input_p2/input.txt");

    let result = part2(input);

    println!("Result: {}", result);
}

#[derive(Clone)]
struct Game {
    id: i32,
    red: Vec<i32>,
    green: Vec<i32>,
    blue: Vec<i32>,
}

impl Game {
    fn print(&self) {
        let mut counts = String::new();
        counts.push_str("Red:\n");
        for i in 0..self.red.len() {
            if i == self.red.len() - 1 {
                counts.push_str(format!("{}\n", self.red[i]).as_str());
            } else {
                counts.push_str(format!("{}, ", self.red[i]).as_str());
            }
        }

        counts.push_str("Green:\n");
        for i in 0..self.green.len() {
            if i == self.green.len() - 1 {
                counts.push_str(format!("{}\n", self.green[i]).as_str());
            } else {
                counts.push_str(format!("{}, ", self.green[i]).as_str());
            }
        }

        counts.push_str("Blue:\n");
        for i in 0..self.blue.len() {
            if i == self.blue.len() - 1 {
                counts.push_str(format!("{}\n", self.blue[i]).as_str());
            } else {
                counts.push_str(format!("{}, ", self.blue[i]).as_str());
            }
        }

        println!("Game:\nID: {}\n{}", self.id, counts);
    }
}

struct MinimumRequired {
    red: i32,
    green: i32,
    blue: i32,
}

impl MinimumRequired {
    fn print(&self) {
        println!(
            "Minimum Required:\nRed Count: {}\nGreen Count: {}\nBlue Count: {}",
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
            red: Vec::new(),
            green: Vec::new(),
            blue: Vec::new(),
        };

        // Each line is in such a format: "Game 1: x color1, y color2, z color3; x color1, y color2; z color3"
        // Colors type can repeat as well as their count or the count might be different, so we need to make sure to add the correct counts of each color type appropriately
        // The colors are separated by a comma and a space, so we can split the line by ", " and then split each color by " " to get the color and its count
        // But the colors are also in sets, so we need to split the line by "; " first to get the sets and then split each set by ", " to get the colors
        // We will use a hashmap to store the color and its count (defined earlier)

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
                    game.red.push(color_count[0].parse::<i32>().unwrap());
                } else if color_count[1] == "green" {
                    game.green.push(color_count[0].parse::<i32>().unwrap());
                } else if color_count[1] == "blue" {
                    game.blue.push(color_count[0].parse::<i32>().unwrap());
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

fn run_algorithm(games: &Vec<Game>) -> Vec<MinimumRequired> {
    let mut minimum_cubes: Vec<MinimumRequired> = Vec::new();
    let mut red = -1;
    let mut green = -1;
    let mut blue = -1;

    // Iterate over the games and check if the current game's counts are less than the maximum allowed counts
    // If they are, we add them to a vector of game ids and then check the next game
    // If they are not, we check the next game
    for game in games {
        // game.print();

        for i in 0..game.red.len() {
            if red == -1 {
                red = game.red[i];
            }

            let previous = red;
            red = game.red[i];

            if previous > red {
                red = previous;
            }
        }
        for i in 0..game.green.len() {
            if green == -1 {
                green = game.green[i];
            }

            let previous = green;
            green = game.green[i];

            if previous > green {
                green = previous;
            }
        }
        for i in 0..game.blue.len() {
            if blue == -1 {
                blue = game.blue[i];
            }

            let previous = blue;
            blue = game.blue[i];

            if previous > blue {
                blue = previous;
            }
        }

        minimum_cubes.push(MinimumRequired {
            red: red,
            green: green,
            blue: blue,
        });

        red = -1;
        green = -1;
        blue = -1;
    }

    return minimum_cubes;
}

fn part2(input: &str) -> String {
    // Parse the input and get the games
    let games: Vec<Game> = parse_games(input);

    // for game in &games {
    //     game.print();
    // }

    // Since the Ids are already sorted, we can proceed
    // Now we need to find a game combination that satisfies the maximum allowed counts
    // We do this by iterating over the games and checking if the current game's counts are less than the maximum allowed counts
    // If they are, we add them to a vector of game ids and then check the next game
    // If they are not, we check the next game

    let minimum_cubes: Vec<MinimumRequired> = run_algorithm(&games);

    // for minimum in &minimum_cubes {
    //     minimum.print();
    // }

    // Now we need to add the game ids together and return the result
    let mut sum: i32 = 0;
    for minimum in &minimum_cubes {
        sum += minimum.red * minimum.green * minimum.blue;
    }

    // return input.to_string();
    return sum.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_p2/test.txt");

        let actual = part2(input);

        let expected = "8".to_string();

        assert_eq!(actual, expected);
    }
}
