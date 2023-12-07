use std::io::{self, BufRead, Read};

struct Hand {
    human_friendly: String,
    rankable: u32,
}

struct Bid {
    hand: Hand,
    wager: u32,
}

fn main() {
    let (winning_sum, question) = process_input(io::stdin().lock());
    println!("winning_sum: {}, ?: {}", winning_sum, question);
}

fn process_input<R: Read>(reader: R) -> (u64, u64) {
    let buffered = io::BufReader::new(reader);
    for line_result in buffered.lines() {
        let line = line_result.unwrap();
        if line.trim().is_empty() {
            continue;
        }
    }
    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_input_advent_example_1() {
        let input = r#"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
            "#;
        let result = process_input(input.as_bytes());
        assert_eq!((6440, 0), result);
    }
}
