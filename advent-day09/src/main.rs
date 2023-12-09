use std::io::{self, BufRead, Read};

fn main() {
    let (sum_last_extrapolated, sum_first_extrapolated) = process_input(io::stdin().lock());
    println!(
        "sum_last_extrapolated: {}, sum_first_extrapolated: {}",
        sum_last_extrapolated, sum_first_extrapolated
    );
}

fn process_input<R: Read>(reader: R) -> (i64, i64) {
    let mut sum_first_extrapolated: i64 = 0;
    let mut sum_last_extrapolated: i64 = 0;
    let buffered = io::BufReader::new(reader);
    for line_result in buffered.lines() {
        let line = line_result.unwrap();
        if line.trim().is_empty() {
            continue;
        }
        let (last, first) = extrapolate_sum(&line);
        sum_first_extrapolated += first;
        sum_last_extrapolated += last;
    }
    (sum_last_extrapolated, sum_first_extrapolated)
}

fn extrapolate_sum(input: &str) -> (i64, i64) {
    let mut numbers: Vec<i64> = Vec::new();
    for number in input.split_whitespace() {
        numbers.push(number.parse().unwrap());
    }
    let mut sets: Vec<Vec<i64>> = vec![numbers.clone()];
    let mut last_set = numbers;
    while last_set.iter().any(|&x| x != 0) {
        let differences = differences(&last_set);
        sets.push(differences.clone());
        last_set = differences;
    }
    let mut first = 0;
    let mut last = 0;
    sets.reverse();
    for set in sets {
        last += set[set.len() - 1];
        first = set[0] - first;
    }
    (last, first)
}

fn differences(numbers: &Vec<i64>) -> Vec<i64> {
    numbers
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extrapolate_1() {
        let input = "0 3 6 9 12 15";
        let result = extrapolate_sum(input);
        assert_eq!((18, -3), result);
    }

    #[test]
    fn test_extrapolate_2() {
        let input = "1 3 6 10 15 21";
        let result = extrapolate_sum(input);
        assert_eq!((28, 0), result);
    }

    #[test]
    fn test_extrapolate_3() {
        let input = "10 13 16 21 30 45";
        let result = extrapolate_sum(input);
        assert_eq!((68, 5), result);
    }

    #[test]
    fn test_process_input_advent_example_1() {
        let input = r#"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
            "#;
        let result = process_input(input.as_bytes());
        assert_eq!((114, 2), result);
    }
}
