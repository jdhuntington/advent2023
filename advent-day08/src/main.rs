use std::collections::HashMap;
use std::io::{self, BufRead, Read};

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
struct MapNode {
    name: String,
    left: String,
    right: String,
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
    let mut directions: Option<Vec<Direction>> = None;
    let mut map = HashMap::new();
    for line_result in buffered.lines() {
        let line = line_result.unwrap();
        if line.trim().is_empty() {
            continue;
        }
        if directions.is_none() {
            directions = Some(parse_directions(&line));
        } else {
            let map_node = parse_map_row(&line);
            map.insert(map_node.name.clone(), map_node);
        }
    }

    let mut simple_steps = 0;
    if let Some(real_directions) = directions {
        simple_steps = compute_simple_steps(&real_directions, &map);
    }
    (simple_steps, 0)
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

fn parse_map_row(input: &str) -> MapNode {
    let mut parts = input.split(" = ");
    let name = parts.next().unwrap();
    let mut directions = parts.next().unwrap().split(", ");
    let left = directions.next().unwrap();
    let left = &left[1..];
    let right = directions.next().unwrap();
    // remove ending paren from right
    let right = &right[..right.len() - 1];
    MapNode {
        name: name.to_string(),
        left: left.to_string(),
        right: right.to_string(),
    }
}

fn compute_simple_steps(directions: &Vec<Direction>, map: &HashMap<String, MapNode>) -> u32 {
    let mut steps_taken: u32 = 0;
    let mut current_node_name = "AAA";
    while current_node_name != "ZZZ" {
        let current_node = map.get(current_node_name).unwrap();
        let direction = directions[(steps_taken as usize) % directions.len()];
        match direction {
            Direction::Left => current_node_name = &current_node.left,
            Direction::Right => current_node_name = &current_node.right,
        }
        steps_taken += 1;
    }
    steps_taken
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
    fn test_parse_map_row() {
        let input = "AAA = (BBB, CCC)";
        let result: MapNode = parse_map_row(input);
        assert_eq!(
            MapNode {
                name: "AAA".to_string(),
                left: "BBB".to_string(),
                right: "CCC".to_string(),
            },
            result
        );
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
