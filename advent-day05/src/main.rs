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
    #[derive(PartialEq)]
    enum ParseState {
        Seeds,
        ExpectMapStartOrEof,
        ExpectMappingBlankOrEof,
    }
    let buffered = io::BufReader::new(reader);

    let mut state = ParseState::Seeds;
    for line_result in buffered.lines() {
        let line = line_result.unwrap();
        if state == ParseState::Seeds {
            if line.trim().is_empty() {
                continue;
            }
            let seeds = process_seeds(&line);
            state = ParseState::ExpectMapStartOrEof;
            continue;
        }
    }
    (0, 0)
}

fn process_map_header(line: &str) -> Result<&str, &str> {
    if !line.ends_with(" map:") {
        return Err("invalid line");
    }
    let line = line.trim_end_matches(" map:");
    Ok(line)
}

fn process_mappings(line: &str) -> Result<(u32, u32, u32), &str> {
    let mut parts = line.split_whitespace();
    if parts.clone().count() != 3 {
        return Err("invalid line");
    }

    let first = parts.next().ok_or("No first part found")?;
    let second = parts.next().ok_or("No second part found")?;
    let third = parts.next().ok_or("No third part found")?;

    let first = first.parse::<u32>().map_err(|_| "Invalid number")?;
    let second = second.parse::<u32>().map_err(|_| "Invalid number")?;
    let third = third.parse::<u32>().map_err(|_| "Invalid number")?;

    Ok((first, second, third))
}

fn process_seeds(line: &str) -> Result<Vec<u32>, &str> {
    if !line.starts_with("seeds: ") {
        return Err("invalid line");
    }
    let line = line.trim_start_matches("seeds: ");

    let mut seeds = Vec::new();
    for seed in line.split_whitespace() {
        let seed = seed.parse::<u32>().map_err(|_| "Invalid number")?;
        seeds.push(seed);
    }
    Ok(seeds)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_map_header_success() {
        assert_eq!(
            Ok("temperature-to-humidity"),
            process_map_header("temperature-to-humidity map:")
        )
    }

    #[test]
    fn test_parse_map_header_failure() {
        assert!(process_map_header("incorrect header").is_err());
    }

    #[test]
    fn test_parse_mappings_success() {
        assert_eq!(Ok((0, 69, 1)), process_mappings("0 69 1"));
    }

    #[test]
    fn test_parse_mappings_failure() {
        assert!(process_mappings("0 1").is_err());
        assert!(process_mappings("0 1 a").is_err());
        assert!(process_mappings("100 100 100a").is_err());
    }

    #[test]
    fn test_parse_seeds_success() {
        assert_eq!(
            Ok(vec![79, 14, 55, 13]),
            process_seeds("seeds: 79 14 55 13")
        );
    }

    #[test]
    fn test_parse_seeds_failure() {
        assert!(process_seeds("seeds 79 14 55 13").is_err());
        assert!(process_seeds("79 14 55 13").is_err());
        assert!(process_seeds("seeds:").is_err());
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