use std::io::{self, BufRead, Read};

#[derive(Debug, PartialEq)]
struct PartNumber {
    x: u32,
    width: u32,
    y: u32,
    number: u32,
}

#[derive(Debug, PartialEq)]
struct Part {
    x: u32,
    y: u32,
}

fn main() {
    let sum = process_input(io::stdin().lock());
    println!("{}", sum);
}

fn process_input<R: Read>(reader: R) -> u32 {
    let buffered = io::BufReader::new(reader);
    for line_result in buffered.lines() {
        let line = line_result.unwrap();
        if line.trim().is_empty() {
            continue;
        }
    }
    0
}

fn process_line(parts: &mut Vec<PartNumber>, line: &str, y_pos: u32) {
    let mut current_number = String::new();
    let mut current_x = 0;
    let mut current_width = 0;
    for (x_pos, c) in line.chars().enumerate() {
        if c == '.' {
            if !current_number.is_empty() {
                let part = PartNumber {
                    x: current_x + 1 - current_width,
                    width: current_width,
                    y: y_pos,
                    number: current_number.parse::<u32>().unwrap(),
                };
                parts.push(part);
                current_number.clear();
                current_width = 0;
            }
        } else {
            current_number.push(c);
            current_width += 1;
            current_x = x_pos as u32;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let mut expected = Vec::new();
        expected.push(PartNumber {
            x: 0,
            width: 3,
            y: 0,
            number: 467,
        });
        expected.push(PartNumber {
            x: 5,
            width: 3,
            y: 0,
            number: 114,
        });
        let input = "467..114..";
        let mut parts: Vec<PartNumber> = Vec::new();
        process_line(&mut parts, input, 0);
        assert_eq!(expected, parts);
    }

    #[test]
    fn test_process_input_advent_input_1() {
        let input = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
        "#;
        let result = process_input(input.as_bytes());
        assert_eq!(4361, result);
    }
}
