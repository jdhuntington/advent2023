use std::io::{self, BufRead, Read};

#[derive(Debug, PartialEq)]
struct RaceRecord {
    time: u32,
    record: u32,
}

fn main() {
    let (mult_result, question) = process_input(io::stdin().lock());
    println!("mult_result: {}, ?: {}", mult_result, question);
}

fn process_input<R: Read>(reader: R) -> (u32, u32) {
    let buffered = io::BufReader::new(reader);
    let mut line1: Option<String> = None;
    let mut line2: Option<String> = None;
    for line_result in buffered.lines() {
        let line = line_result.unwrap();
        if line.trim().is_empty() {
            continue;
        }
        if line1.is_none() {
            line1 = Some(line);
            continue;
        }
        if line2.is_none() {
            line2 = Some(line);
            continue;
        }
        panic!("Unexpected line: {}", line);
    }
    let scenarios = parse_race_lines(&line1.unwrap(), &line2.unwrap());

    (compute_mult_result(scenarios), 0)
}

fn compute_mult_result(scenarios: Vec<RaceRecord>) -> u32 {
    let mut result = 1;
    for scenario in scenarios {
        let winning_scenarios = compute_winning_scenarios(scenario);
        result *= winning_scenarios.len() as u32;
    }
    result
}

fn parse_race_lines(line1: &str, line2: &str) -> Vec<RaceRecord> {
    let mut result = Vec::new();
    let mut time_iter = line1.split_whitespace();
    let mut record_iter = line2.split_whitespace();
    let time_prefix = time_iter.next();
    if time_prefix.is_none() || time_prefix.unwrap() != "Time:" {
        panic!("Expected \"Time:\" prefix");
    }
    let record_prefix = record_iter.next();
    if record_prefix.is_none() || record_prefix.unwrap() != "Distance:" {
        panic!("Expected \"Distance:\" prefix");
    }
    loop {
        let time = time_iter.next();
        let record = record_iter.next();
        if time.is_none() || record.is_none() {
            break;
        }
        let time = time.unwrap().parse::<u32>().unwrap();
        let record = record.unwrap().parse::<u32>().unwrap();
        result.push(RaceRecord { time, record });
    }
    result
}

fn compute_winning_scenarios(input: RaceRecord) -> Vec<u32> {
    let mut result = Vec::new();
    for i in 0..=(input.time) {
        let charge_time = i;
        let run_time = input.time - charge_time;
        let distance = charge_time * run_time;
        if distance > input.record {
            result.push(charge_time);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_race_lines() {
        let expected = vec![
            RaceRecord { time: 7, record: 9 },
            RaceRecord {
                time: 15,
                record: 40,
            },
            RaceRecord {
                time: 30,
                record: 200,
            },
        ];
        let actual = parse_race_lines("Time:      7  15   30", "Distance:  9  40  200");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_compute_winning_scenarios() {
        let input = RaceRecord { time: 7, record: 9 };
        let expected = vec![2, 3, 4, 5];
        let actual = compute_winning_scenarios(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_process_input_advent_example_1() {
        let input = r#"
Time:      7  15   30
Distance:  9  40  200
            "#;
        let result = process_input(input.as_bytes());
        assert_eq!((288, 0), result);
    }
}
