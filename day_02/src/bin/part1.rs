fn main() {
    println!("Part 1!");

    let input = include_str!("../../input_p1/input.txt");
    let result = part1(input);

    println!("Result: {}", result);
}

fn part1(input: &str) -> String {
    return input.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../../input_p1/test.txt");

        let actual = part1(input);
        let expected = "".to_string();

        assert_eq!(actual, expected);
    }
}
