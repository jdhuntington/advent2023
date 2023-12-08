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

fn process_input<R: Read>(reader: R) -> (u32, u64) {
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
    let mut complex_steps = 0;
    if let Some(real_directions) = directions {
        simple_steps = compute_simple_steps(&real_directions, &map);
        complex_steps = compute_complex_steps(&real_directions, &map);
    }
    (simple_steps, complex_steps)
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
        if let Some(current_node) = map.get(current_node_name) {
            let direction = directions[(steps_taken as usize) % directions.len()];
            match direction {
                Direction::Left => current_node_name = &current_node.left,
                Direction::Right => current_node_name = &current_node.right,
            }
            steps_taken += 1;
        } else {
            return 0;
        }
    }
    steps_taken
}

fn compute_complex_steps(directions: &Vec<Direction>, map: &HashMap<String, MapNode>) -> u64 {
    let mut steps_taken: u64 = 0;
    let mut mod_steps_taken: usize = 0;
    let directions_length = directions.len();
    let mut current_nodes = map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| k.to_string())
        .collect::<Vec<String>>();
    let total_nodes: u8 = current_nodes.len() as u8;
    let mut nodes_in_target: u8 = 0;
    let mut next_nodes = Vec::new();
    let mut lcm = current_nodes.iter().map(|_| 0).collect::<Vec<u64>>();
    while nodes_in_target < total_nodes {
        let mut i: usize = 0;
        nodes_in_target = 0;
        next_nodes.clear();
        for current_node_name in &current_nodes {
            if let Some(current_node) = map.get(current_node_name) {
                let direction = directions[mod_steps_taken];
                let next_node = match direction {
                    Direction::Left => &current_node.left,
                    Direction::Right => &current_node.right,
                };
                if next_node.ends_with('Z') {
                    nodes_in_target += 1;
                    if lcm[i] == 0 {
                        lcm[i] = steps_taken;
                    }
                }
                next_nodes.push(next_node.to_string());
            } else {
                println!("current_node_name (aborting): {}", current_node_name);
                return 0;
            }
            i += 1;
        }

        // if all values in lcm are non-zero, then we can break early
        let mut all_non_zero = true;
        for value in &lcm {
            if *value == 0 {
                all_non_zero = false;
                break;
            }
        }

        std::mem::swap(&mut current_nodes, &mut next_nodes);
        steps_taken += 1;
        mod_steps_taken += 1;
        if mod_steps_taken == directions_length {
            mod_steps_taken = 0;
        }

        if steps_taken % 19900000 == 0 {
            println!("steps_taken: {} (mod: {})", steps_taken, mod_steps_taken);
        }

        if all_non_zero {
            break;
        }
    }

    println!("lcm: {:?}", lcm);

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
        assert_eq!((2, 2), result);
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
        assert_eq!((6, 6), result);
    }

    #[test]
    fn test_process_input_advent_example_3() {
        let input = r#"
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"#;
        let result = process_input(input.as_bytes());
        assert_eq!((0, 6), result);
    }
}
