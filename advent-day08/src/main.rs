use std::io::{self, BufRead, Read};

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

fn main() {
    let (simple_steps, complex_steps) = process_input(io::stdin().lock());
    println!(
        "simple_steps: {}, complex_steps: {}",
        simple_steps, complex_steps
    );
}

fn process_input<R: Read>(reader: R) -> (u32, u32) {
    let buffered = io::BufReader::new(reader);
    for line_result in buffered.lines() {
        let line = line_result.unwrap();
        if line.trim().is_empty() {
            continue;
        }
    }

    (0, 0)
}

fn parse_directions(input: &str) -> Vec<Direction> {
    let mut directions = Vec::new();
    for c in input.chars() {
        match c {
            'L' => directions.push(Direction::Left),
            'R' => directions.push(Direction::Right),
            _ => panic!("unexpected direction: {}", c),
        }
    }
    directions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_directions() {
        let input = "RL";
        let result = parse_directions(input);
        assert_eq!(vec![Direction::Right, Direction::Left], result);
    }

    #[test]
    fn test_process_input_advent_example_1() {
        let input = r#"
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
            "#;
        let result = process_input(input.as_bytes());
        assert_eq!((2, 0), result);
    }

    #[test]
    fn test_process_input_advent_example_2() {
        let input = r#"
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
            "#;
        let result = process_input(input.as_bytes());
        assert_eq!((6, 0), result);
    }
}
