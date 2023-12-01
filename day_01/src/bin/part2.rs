fn main() {
    println!("Part 2!");

    let input = include_str!("../../input_p1/input.txt");
    let result = part2(input);

    println!("Result: {}", result);
}

fn part2(input: &str) -> String {
    println!("Input:\n{}", input);

    let mut sum = 0;

    for line in input.lines() {
        // we only need the first and the last digit within a line
        let mut digit_count = 0;
        let mut first_digit = 0;
        let mut last_digit = 0;

        for c in line.chars() {
            if c.is_digit(10) {
                digit_count += 1;

                if digit_count == 1 {
                    first_digit = c.to_digit(10).unwrap();
                }

                if digit_count >= 2 {
                    last_digit = c.to_digit(10).unwrap();
                }
            }

            // Do not break, because we need to count the digits and get the last digit
        }

        if digit_count == 1 {
            last_digit = first_digit;
        }

        // println!("F1: {} | F2: {}\n", first_digit, last_digit);

        let combined_digits = std::fmt::format(format_args!("{}{}", first_digit, last_digit));
        // println!("Combined: {}", combined_digits);

        sum += combined_digits.parse::<i32>().unwrap();
    }

    let result = sum.to_string();
    return result.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_p1/test.txt");

        let actual = part2(input);
        let expected = "142".to_string();

        assert_eq!(actual, expected);
    }
}
