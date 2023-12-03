fn main() {
    println!("Part 2!");

    let input = include_str!("../../input_p2/test.txt");

    let result = part2(input);

    println!("Result: {}", result);
}


fn part2(input: &str) -> String {

    let mut sum: i32 = 0;

    return sum.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = include_str!("../../input_p2/test.txt");

        let actual = part2(input);

        let expected = "0".to_string();

        assert_eq!(actual, expected);
    }
}
