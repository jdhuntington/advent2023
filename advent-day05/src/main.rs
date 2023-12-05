use std::io::{self, BufRead, Read};

#[derive(Debug, PartialEq)]
struct GardenMap {
    name: String,
    mappings: Vec<(u32, u32, u32)>,
}

fn main() {
    let (lowest, _x) = process_input(io::stdin().lock());
    println!("lowest: {}", lowest);
}

fn process_input<R: Read>(reader: R) -> (u32, u32) {
    let buffered = io::BufReader::new(reader);
    let lowest = 0;
    for line_result in buffered.lines() {
        let line = line_result.unwrap();
        if line.trim().is_empty() {
            continue;
        }
    }
    (lowest, 0)
}

fn process_map_header(line: &str) -> Result<&str, &str> {
    // make sure line ends with " map:"
    if !line.ends_with(" map:") {
        return Err("invalid line");
    }
    // remove " map:" from the end
    let line = line.trim_end_matches(" map:");
    Ok(line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_map_header_success() {
        let name = process_map_header("temperature-to-humidity map:");
        assert_eq!(Ok("temperature-to-humidity"), name)
    }

    #[test]
    fn test_parse_map_header_failure() {
        let result = process_map_header("incorrect header");
        assert!(result.is_err());
    }

    //     #[test]
    //     fn test_parse_map_1() {
    //         let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
    //         let score = process_line(line);
    //         assert_eq!((8, 4), score);
    //     }

    #[test]
    fn test_process_input_advent_example_1() {
        let input = r#"
    seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48

    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15

    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4

    water-to-light map:
    88 18 7
    18 25 70

    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13

    temperature-to-humidity map:
    0 69 1
    1 0 69

    humidity-to-location map:
    60 56 37
    56 93 4
            "#;

        let result = process_input(input.as_bytes());
        assert_eq!((35, 0), result);
    }
}
