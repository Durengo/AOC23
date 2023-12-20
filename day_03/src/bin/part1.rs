use std::fmt;

fn main() {
    println!("Part 1!");

    let input = include_str!("../../input_p1/test.txt");

    let result = algorithm(input);

    println!("Result: {}", result);
}

// We will use this datatype to store the coordinates of symbols
struct Symbol {
    x: i32,
    y: i32,
}

impl Symbol {
    fn new(x: i32, y: i32) -> Symbol {
        Symbol { x, y }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut str = "";
        fmt.write_fmt(format_args!("({}, {})", self.x, self.y))?;
        Ok(())
    }
}

fn algorithm(input: &str) -> String {
    let mut sum: i32 = 0;

    // We will run the algorithm in such a way
    // We will take 3 lines at a time and go to the next line
    // We need to find a symbol (anything that is not a '.' char or a digit)
    // Then we need to check diagonally, anti-diagonally, horizontally and vertically for digits
    // If we find a digit, we also need to get all the digits that are next to it
    // Then we combine those digits into a number and add it to the sum
    // And finally replace those digits with a '.' char so that we don't count them again

    // Assign the input to a buffer
    let mut input_buffer: Vec<&str> = Vec::new();
    for line in input.lines() {
        input_buffer.push(line);
    }

    // We need to keep a buffer for the 3 lines
    let mut line_buffer: Vec<&str> = Vec::new();

    // We also need a buffer to keep the track of subsequent digits
    let mut digit_buffer: Vec<char> = Vec::new();

    // We need an index to keep track of which was the last line we put in the buffer
    let mut index: usize = 0;

    // First we assign the buffer to the 3 lines
    let mut end_of_buffer: bool = false;

    while !end_of_buffer {
        // Do not go out of bounds
        if index + 2 >= input_buffer.len() {
            end_of_buffer = true;
            break;
        }

        // Assign the 3 lines to the buffer
        line_buffer.push(input_buffer[index]);
        line_buffer.push(input_buffer[index + 1]);
        line_buffer.push(input_buffer[index + 2]);
        index += 1;

        println!("Index: {} | Line Buffer: {:?}", index, line_buffer);

        // We need to find a symbol
        let mut symbols: Vec<Symbol> = Vec::new();

        for i in 0..3 {
            let line = line_buffer[i];

            for j in 0..line.len() {
                let symbol = line.chars().nth(j).unwrap();

                // We found a symbol
                if symbol != '.' && !symbol.is_digit(10) {
                    symbols.push(Symbol {
                        x: i as i32,
                        y: j as i32,
                    });
                }
            }
        }

        for symbol in symbols {
            println!("Symbol: {}", symbol);
        }

        // Make sure to clear the buffer for the next iteration
        line_buffer.clear();
    }

    return sum.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algorithm() {
        let input = include_str!("../../input_p1/test.txt");

        let actual = algorithm(input);

        let expected = "0".to_string();

        assert_eq!(actual, expected);
    }
}
