use std::io::{self, BufRead, Read};

fn main() {
    let (sum, scratchcard_count) = process_input(io::stdin().lock());
    println!("sum: {}, scratchcard_count: {}", sum, scratchcard_count);
}

fn process_input<R: Read>(reader: R) -> (u32, u32) {
    let buffered = io::BufReader::new(reader);
    let mut sum = 0;
    let mut scratchcard_count = 0;
    let mut multipliers: Vec<u32> = Vec::new();
    for line_result in buffered.lines() {
        let line = line_result.unwrap();
        if line.trim().is_empty() {
            continue;
        }
        let mut current_multiplier = 1;

        if let Some(_first_element) = multipliers.get(0) {
            let first_element = multipliers.remove(0);
            current_multiplier += first_element;
        }
        scratchcard_count += current_multiplier;
        let (card_value, counts) = process_line(&line);
        for i in 1..=(counts as usize) {
            if i > multipliers.len() {
                multipliers.push(0);
            }
            multipliers[i - 1] += current_multiplier;
        }
        sum += card_value;
    }
    (sum, scratchcard_count)
}

fn process_line(line: &str) -> (u32, u32) {
    let mut parts = line.split(":");
    if parts.clone().count() != 2 {
        panic!("invalid line: {}", line);
    }
    let _card = parts.next().expect("No card part found");
    let numbers_part = parts.next().expect("No numbers part found");
    let mut number_part_segments = numbers_part.split("|");
    if number_part_segments.clone().count() != 2 {
        panic!("invalid line: {}", line);
    }
    let winners = number_part_segments
        .next()
        .expect("No winners found")
        .split_whitespace()
        .map(|s| s.parse::<u32>().expect("Invalid number"))
        .collect::<Vec<u32>>();
    let mine = number_part_segments
        .next()
        .expect("No 'my numbers' found")
        .split_whitespace()
        .map(|s| s.parse::<u32>().expect("Invalid number"))
        .collect::<Vec<u32>>();
    let winner_count = mine.iter().filter(|&m| winners.contains(m)).count();
    if winner_count == 0 {
        (0, 0)
    } else {
        (1 << winner_count - 1, winner_count as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_1() {
        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let score = process_line(line);
        assert_eq!((8, 4), score);
    }

    #[test]
    fn test_parse_line_2() {
        let line = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
        let score = process_line(line);
        assert_eq!((2, 2), score);
    }

    #[test]
    fn test_parse_line_3() {
        let line = "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
        let score = process_line(line);
        assert_eq!((2, 2), score);
    }

    #[test]
    fn test_parse_line_4() {
        let line = "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83";
        let score = process_line(line);
        assert_eq!((1, 1), score);
    }

    #[test]
    fn test_parse_line_5() {
        let line = "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36";
        let score = process_line(line);
        assert_eq!((0, 0), score);
    }

    #[test]
    fn test_parse_line_6() {
        let line = "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let score = process_line(line);
        assert_eq!((0, 0), score);
    }

    #[test]
    fn test_process_input_sample() {
        let input = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "#;
        let result = process_input(input.as_bytes());
        assert_eq!((13, 30), result);
    }
}
