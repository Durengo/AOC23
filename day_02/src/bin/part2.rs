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
    return input.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_p2/test.txt");

        let actual = part2(input);
        let expected = "".to_string();

        assert_eq!(actual, expected);
    }
}
