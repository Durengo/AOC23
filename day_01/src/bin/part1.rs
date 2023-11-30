fn main() {
    println!("Part 1!");
}

fn part1(input: &str) -> String {
    return String::from("Part 1!");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_part1() {
        assert_eq!(part1(""), "Part 1!");
    }
}