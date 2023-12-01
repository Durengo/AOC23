use std::collections::HashMap;

fn main() {
    println!("Part 2!");

    let input = include_str!("../../input_p2/input.txt");
    let result = part2(input);

    println!("Result: {}", result);
}

// Now we also need to check for digits written out as words.
// It is also possible to have digits and word combinations.
// We will use a hash map to find the word-digit convert it into a number.
fn part2(input: &str) -> String {
    let word_to_digit = HashMap::from([
        ("zero", 0),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut sum = 0;

    for line in input.lines() {
        let mut first_digit = -1;
        let mut last_digit = -1;
        let mut current_char = 0;
        let mut digit_count = 0;

        // println!("Current Line: {}", line);

        while current_char < line.len() {
            // We must take into account that once we reach the end of the line, we cannot take more than 5 characters
            // Take a slice of the maximum possible length of a digit-word
            // Then create a modifier to add to have a full word-digit
            let mut modifier = 5;
            if current_char + 5 > line.len() {
                // We need to allow the modifier to take the remaining characters (up to 5) but not more than the line length
                modifier = line.len() - current_char;
            }
            let slice = &line[current_char..current_char + modifier];

            // println!("Slice: {} | Current Char: {}, Modifier: {}", slice, current_char, modifier);

            // Check against the hashmap
            for digit_word in word_to_digit.iter() {
                // If slice contains a digit then break, but only if it's not the first character
                // Possible optimization to use starts with?
                if slice.contains(&digit_word.1.to_string()) {
                    let c = slice.chars().next().unwrap();
                    if c.is_digit(10) {
                        // digit found, add to first_digit or last_digit
                        if digit_count == 0 {
                            first_digit = c.to_digit(10).unwrap() as i32;
                        }
                        if digit_count >= 1 {
                            last_digit = c.to_digit(10).unwrap() as i32;
                        }
                        digit_count += 1;
                        break;
                    }
                }

                // We need to check the contains condition from the first char aka make sure that we don't find a matching word in the middle of the string
                if slice.starts_with(digit_word.0) {
                    // digit-word found, add to first_digit or last_digit
                    if digit_count == 0 {
                        first_digit = *digit_word.1;
                    }
                    if digit_count >= 1 {
                        last_digit = *digit_word.1;
                    }
                    digit_count += 1;
                    break;
                }
            }

            // println!("P1: {} | P2: {}", first_digit, last_digit);

            current_char += 1;
        }

        if last_digit == -1 && first_digit != -1 {
            last_digit = first_digit;
        }

        if first_digit != -1 && last_digit != -1 {
            let combined_digits = format!("{}{}", first_digit, last_digit);
            sum += combined_digits.parse::<i32>().unwrap();
        } else {
            panic!("Something went wrong!");
        }
    }
    return sum.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_p2/test.txt");

        let actual = part2(input);
        let expected = "281".to_string();

        assert_eq!(actual, expected);
    }
}
