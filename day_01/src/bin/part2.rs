fn main() {
    println!("Part 2!");
}

fn part2(input: &str) -> String {
    return String::from("Part 2!");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_part2() {
        assert_eq!(part2(""), "Part 2!");
    }
}