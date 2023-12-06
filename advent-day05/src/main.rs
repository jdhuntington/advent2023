use std::collections::HashMap;
use std::io::{self, BufRead, Read};

#[derive(Debug, PartialEq)]
struct GardenMap {
    name: String,
    mappings: Vec<(u32, u32, u32)>,
}

impl GardenMap {
    fn follow(&self, input: u32) -> u32 {
        for &(destination_start, source_start, range) in &self.mappings {
            if input >= source_start && input < source_start + range {
                return destination_start + (input - source_start);
            }
        }
        input
    }
}

fn main() {
    let (lowest, lowest_range) = process_input(io::stdin().lock());
    println!(
        "lowest (simple): {}, lowest (range): {}",
        lowest, lowest_range
    );
}

fn process_input<R: Read>(reader: R) -> (u32, u32) {
    #[derive(PartialEq)]
    enum ParseState {
        Seeds,
        ExpectMapStartOrEof,
        ExpectMappingBlankOrEof,
    }
    let buffered = io::BufReader::new(reader);
    let mut seeds: Vec<u32> = Vec::new();
    let mut current_garden_map: Option<GardenMap> = None;
    let mut garden_maps: HashMap<String, GardenMap> = HashMap::new();
    let mut state = ParseState::Seeds;
    for line_result in buffered.lines() {
        let line = line_result.unwrap();

        if state == ParseState::Seeds {
            if line.trim().is_empty() {
                continue;
            }

            let seed_result = process_seeds(&line).unwrap_or_else(|err| {
                panic!("Error processing seeds: {}", err);
            });
            seeds = seed_result;
            state = ParseState::ExpectMapStartOrEof;
            continue;
        }

        if state == ParseState::ExpectMapStartOrEof {
            if line.trim().is_empty() {
                continue;
            }

            let map_name = process_map_header(&line).unwrap().to_string();
            current_garden_map = Some(GardenMap {
                name: map_name,
                mappings: Vec::new(),
            });

            state = ParseState::ExpectMappingBlankOrEof;
            continue;
        }

        if state == ParseState::ExpectMappingBlankOrEof {
            if line.trim().is_empty() {
                state = ParseState::ExpectMapStartOrEof;
                garden_maps.insert(
                    current_garden_map.as_ref().unwrap().name.clone(),
                    current_garden_map.take().unwrap(),
                );
                current_garden_map = None;
                continue;
            }

            let mapping = process_mappings(&line).unwrap();
            current_garden_map.as_mut().unwrap().mappings.push(mapping);
            continue;
        }
    }
    if let Some(garden_map) = current_garden_map {
        garden_maps.insert(garden_map.name.clone(), garden_map);
    }

    let chain = vec![
        garden_maps.get("seed-to-soil").unwrap(),
        garden_maps.get("soil-to-fertilizer").unwrap(),
        garden_maps.get("fertilizer-to-water").unwrap(),
        garden_maps.get("water-to-light").unwrap(),
        garden_maps.get("light-to-temperature").unwrap(),
        garden_maps.get("temperature-to-humidity").unwrap(),
        garden_maps.get("humidity-to-location").unwrap(),
    ];

    let simple_seed_results = seeds.iter().map(|seed| follow_chain(&chain, *seed));
    let simple_lowest = simple_seed_results.min().unwrap();

    let mut range_results = Vec::new();
    seeds.chunks(2).for_each(|chunk| {
        println!("chunk: {:?}", chunk);
        let first = chunk[0];
        let second = chunk[1];
        for i in first..(first + second) {
            let result = follow_chain(&chain, i);
            range_results.push(result);
        }
    });
    let range_lowest = range_results.iter().min().unwrap();
    (simple_lowest, *range_lowest)
}

fn process_map_header(line: &str) -> Result<&str, &str> {
    if !line.ends_with(" map:") {
        return Err("invalid line");
    }
    let line = line.trim_end_matches(" map:");
    Ok(line)
}

fn process_mappings(line: &str) -> Result<(u32, u32, u32), String> {
    let mut parts = line.split_whitespace();
    if parts.clone().count() != 3 {
        return Err("invalid line".to_string());
    }

    let first = parts.next().ok_or("No first part found")?;
    let second = parts.next().ok_or("No second part found")?;
    let third = parts.next().ok_or("No third part found")?;

    let first = first.parse::<u32>().map_err(|_| "Invalid number")?;
    let second = second.parse::<u32>().map_err(|_| "Invalid number")?;
    let third = third.parse::<u32>().map_err(|_| "Invalid number")?;

    Ok((first, second, third))
}

fn process_seeds(line: &str) -> Result<Vec<u32>, String> {
    if !line.starts_with("seeds: ") {
        let message = format!("process_seeds: Invalid line: {} (expected seeds: )", line);
        return Err(message);
    }
    let line = line.trim_start_matches("seeds: ");

    let mut seeds = Vec::new();
    for seed in line.split_whitespace() {
        let seed = seed.parse::<u32>().map_err(|_| "Invalid number")?;
        seeds.push(seed);
    }
    Ok(seeds)
}

fn follow_chain(chain: &[&GardenMap], input: u32) -> u32 {
    let mut current = input;
    for garden_map in chain {
        current = garden_map.follow(current);
    }
    current
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

    #[test]
    fn test_map_lookups() {
        let garden_map = GardenMap {
            name: "test".to_string(),
            mappings: vec![(50, 98, 2), (52, 50, 48)],
        };
        assert_eq!(50, garden_map.follow(98));
        assert_eq!(51, garden_map.follow(99));
        assert_eq!(100, garden_map.follow(100));
        assert_eq!(52, garden_map.follow(50));
        assert_eq!(55, garden_map.follow(53));
        assert_eq!(10, garden_map.follow(10));
    }

    #[test]
    fn test_map_chain() {
        let garden_map_0 = GardenMap {
            name: "test".to_string(),
            mappings: vec![(20, 30, 40)],
        };
        let garden_map_1 = GardenMap {
            name: "test".to_string(),
            mappings: vec![(50, 0, 50)],
        };
        assert_eq!(21, garden_map_0.follow(garden_map_0.follow(31))); // precondition
        assert_eq!(71, garden_map_1.follow(garden_map_0.follow(21))); // precondition
        let chain = vec![&garden_map_0, &garden_map_1];
        assert_eq!(71, follow_chain(&chain, 31));
    }

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
        assert_eq!((35, 46), result);
    }
}
