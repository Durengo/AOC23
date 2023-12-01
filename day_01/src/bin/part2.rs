use std::collections::HashMap;
use lazy_static::lazy_static;

// Store digit words in a hashmap
lazy_static! {
    static ref PRIVILEGES: HashMap<&'static str, i32> = {
        let mut digit_words = HashMap::new();
        digit_words.insert("zero", 0);
        digit_words.insert("one", 1);
        digit_words.insert("two", 2);
        digit_words.insert("three", 3);
        digit_words.insert("four", 4);
        digit_words.insert("five", 5);
        digit_words.insert("six", 6);
        digit_words.insert("seven", 7);
        digit_words.insert("eight", 8);
        digit_words.insert("nine", 9);

        digit_words
    };
}

fn main() {
    println!("Part 2!");

    let input = include_str!("../../input_p2/test.txt");
    let result = part2(input);

    println!("Result: {}", result);
}

// Now we also need to check for digits written out as words.
// It is also possible to have digits and word combinations.
// We will use a hash map to find the word-form of a digit and convert it into a number.

fn part2(input: &str) -> String {
    println!("Input:\n{}", input);

    let mut sum = 0;
    let mut buffer = String::new();

    for line in input.lines() {
        // we only need the first and the last digit within a line
        let mut digit_count = 0;
        let mut first_digit = 0;
        let mut last_digit = 0;
        let mut current_start = 0;

        buffer.clear();
        buffer = line.to_string();
        // buffer = line.to_string();
        println!("Line: {}", line);

        // Try to split the line into sections of words and digits using the hashmap
        // We need to sample not character by character, but word by word - using the hashmap
        let mut times = 0;
        while buffer.len() > 0 {
            for word in PRIVILEGES.iter() {
                // let mut len = word.0.len();

                if word.0.len() > buffer.len() {
                    continue;
                }

                let slice = (&buffer[0..word.0.len()]).to_string();

                println!("Buffer: {} | Word: {}", slice.as_str(), word.0);

                if slice.contains(word.0) {
                    digit_count += 1;

                    if digit_count == 1 {
                        first_digit = *word.1;
                    }

                    if digit_count >= 2 {
                        last_digit = *word.1;
                    }

                    // Remove the word from the buffer, but only the first instance of it
                    // buffer = buffer.replace(word.0, "");
                    println!("Removing slice: {}", slice.as_str());
                    buffer = buffer.replacen(word.0, "", 1);
                    break;
                }
            }

            println!("Buffer after: {}", buffer.as_str());

            // Now check specifically for digits
            let slice: String = (&buffer[0..1]).to_string();
            println!("Slice 2: {}", slice.as_str());
            if slice.parse::<i32>().is_ok() {
                digit_count += 1;

                if digit_count == 1 {
                    first_digit = slice.parse::<i32>().unwrap();
                }

                if digit_count >= 2 {
                    last_digit = slice.parse::<i32>().unwrap();
                }

                // Remove the word from the buffer
                buffer = buffer.replacen(slice.as_str(), "", 1);
                println!("Removing slice: {}", slice.as_str());
            }

            times += 1;
            if times > 1 {
                buffer = (&buffer[1..]).to_string();
                println!("Buffer end loop: {}", buffer.as_str());
                times = 0;
            }
        }

        // for c in line.chars() {
        //     buffer.push(c);

        //     println!("Buffer: {}", buffer.as_str());

        //     // Check if the buffer contains a digit word
        //     if PRIVILEGES.contains_key(buffer.as_str()) {
        //         digit_count += 1;

        //         if digit_count == 1 {
        //             first_digit = *PRIVILEGES.get(buffer.as_str()).unwrap();
        //         }

        //         if digit_count >= 2 {
        //             last_digit = *PRIVILEGES.get(buffer.as_str()).unwrap();
        //         }

        //         println!("Found digit word: {}", buffer.as_str());

        //         buffer.clear();
        //     }

        //     // Check if the buffer contains a digit
        //     if buffer.parse::<i32>().is_ok() {
        //         digit_count += 1;

        //         if digit_count == 1 {
        //             first_digit = buffer.parse::<i32>().unwrap();
        //         }

        //         if digit_count >= 2 {
        //             last_digit = buffer.parse::<i32>().unwrap();
        //         }

        //         println!("Found digit: {}", buffer.as_str());

        //         buffer.clear();
        //     }

        //     // Clear the buffer if non of the above conditions are met, we need to make sure not to reset the buffer if we are in the middle of a word
        //     // But if the word is a random set of characters, we need to clear the buffer

        // }

        // let words: Vec<&str> = line.split_whitespace().collect();

        // for word in words {
        //     // Check if the word is a digit
        //     if word.parse::<i32>().is_ok() {
        //         digit_count += 1;

        //         if digit_count == 1 {
        //             first_digit = word.parse::<i32>().unwrap();
        //         }

        //         if digit_count >= 2 {
        //             last_digit = word.parse::<i32>().unwrap();
        //         }
        //     }

        //     // Check if the word is a digit written out as a word
        //     if PRIVILEGES.contains_key(word) {
        //         digit_count += 1;

        //         if digit_count == 1 {
        //             first_digit = *PRIVILEGES.get(word).unwrap();
        //         }

        //         if digit_count >= 2 {
        //             last_digit = *PRIVILEGES.get(word).unwrap();
        //         }
        //     }
        // }

        // if digit_count == 1 {
        //     last_digit = first_digit;
        // }

        println!("F1: {} | F2: {}\n", first_digit, last_digit);

        // let combined_digits = std::fmt::format(format_args!("{}{}", first_digit, last_digit));
        // // println!("Combined: {}", combined_digits);

        // sum += combined_digits.parse::<i32>().unwrap();
    }

    // let result = sum.to_string();
    let result = "";
    return result.to_string();
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
