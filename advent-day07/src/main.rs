use std::cmp::Ordering;
use std::io::{self, BufRead, Read};

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    human_friendly: String,
    rankable: u32,
    wild_rankable: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct Bid {
    hand: Hand,
    wager: u32,
}

impl Bid {
    fn new(line: &str) -> Self {
        let mut parts = line.split_whitespace();
        let hand = parts.next().expect("No hand found");
        let wager = parts
            .next()
            .expect("No wager found")
            .parse::<u32>()
            .expect("Invalid wager");

        Bid {
            hand: Hand::new(hand),
            wager,
        }
    }

    fn sort_by_basic_rank(a: &Bid, b: &Bid) -> Ordering {
        a.hand.rankable.cmp(&b.hand.rankable)
    }

    fn sort_by_wild_rank(a: &Bid, b: &Bid) -> Ordering {
        a.hand.wild_rankable.cmp(&b.hand.wild_rankable)
    }
}

impl Hand {
    fn new(human_friendly: &str) -> Self {
        let cards: Vec<u8> = human_friendly
            .chars()
            .map(|c| match c {
                '2' => 0,
                '3' => 1,
                '4' => 2,
                '5' => 3,
                '6' => 4,
                '7' => 5,
                '8' => 6,
                '9' => 7,
                'T' => 8,
                'J' => 9,
                'Q' => 10,
                'K' => 11,
                'A' => 12,
                _ => panic!("Invalid card character: {}", c),
            })
            .collect();

        Hand {
            human_friendly: human_friendly.to_string(),
            rankable: Hand::get_rankable(cards.clone()),
            wild_rankable: Hand::get_wild_rankable(cards),
        }
    }

    fn get_rankable(cards: Vec<u8>) -> u32 {
        let mut rank = Hand::get_ranking(cards.clone());
        for card in cards {
            rank = (rank << 4) | (card as u32);
        }
        rank
    }

    fn get_wild_rankable(cards: Vec<u8>) -> u32 {
        let adjusted_cards: Vec<u8> = cards
            .iter()
            .map(|&card| match card {
                0 => 1,
                1 => 2,
                2 => 3,
                3 => 4,
                4 => 5,
                5 => 6,
                6 => 7,
                7 => 8,
                8 => 9,
                9 => 0,
                _ => card,
            })
            .collect();
        let mut rank = Hand::get_wild_ranking(adjusted_cards.clone());
        for card in adjusted_cards {
            rank = (rank << 4) | (card as u32);
        }
        rank
    }

    fn get_wild_ranking(cards: Vec<u8>) -> u32 {
        let mut frequency_map = [0; 13];
        let mut wild_card_count = 0;
        for card in cards {
            if card == 0 {
                wild_card_count += 1;
                continue;
            }
            frequency_map[card as usize] += 1;
        }
        frequency_map.sort_unstable();
        frequency_map.reverse();
        frequency_map[0] += wild_card_count;
        if frequency_map[0] == 5 {
            return 6;
        }
        if frequency_map[0] == 4 {
            return 5;
        }
        if frequency_map[0] == 3 && frequency_map[1] == 2 {
            return 4;
        }
        if frequency_map[0] == 3 {
            return 3;
        }
        if frequency_map[0] == 2 && frequency_map[1] == 2 {
            return 2;
        }
        if frequency_map[0] == 2 {
            return 1;
        }
        0
    }

    fn get_ranking(cards: Vec<u8>) -> u32 {
        let mut frequency_map = [0; 13];
        for card in cards {
            frequency_map[card as usize] += 1;
        }
        frequency_map.sort_unstable();
        frequency_map.reverse();
        if frequency_map[0] == 5 {
            return 6;
        }
        if frequency_map[0] == 4 {
            return 5;
        }
        if frequency_map[0] == 3 && frequency_map[1] == 2 {
            return 4;
        }
        if frequency_map[0] == 3 {
            return 3;
        }
        if frequency_map[0] == 2 && frequency_map[1] == 2 {
            return 2;
        }
        if frequency_map[0] == 2 {
            return 1;
        }
        0
    }
}

fn main() {
    let (winning_sum, jokers_wild) = process_input(io::stdin().lock());
    println!("winning_sum: {}, jokers_wild: {}", winning_sum, jokers_wild);
}

fn process_input<R: Read>(reader: R) -> (u32, u32) {
    let buffered = io::BufReader::new(reader);
    let mut bids: Vec<Bid> = Vec::new();
    for line_result in buffered.lines() {
        let line = line_result.unwrap();
        if line.trim().is_empty() {
            continue;
        }
        bids.push(Bid::new(&line));
    }
    bids.sort_by(Bid::sort_by_basic_rank);
    let mut winning_sum = 0;
    for (i, bid) in bids.iter().enumerate() {
        winning_sum += bid.wager * ((i + 1) as u32);
    }

    bids.sort_by(Bid::sort_by_wild_rank);
    let mut wild_sum = 0;
    for (i, bid) in bids.iter().enumerate() {
        wild_sum += bid.wager * ((i + 1) as u32);
    }
    (winning_sum, wild_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bid_new_1() {
        let line = "32T3K 765";
        let parsed_hand = Hand::new("32T3K");
        assert_eq!(
            Bid {
                hand: parsed_hand,
                wager: 765,
            },
            Bid::new(line)
        );
    }

    #[test]
    fn test_hand_new_1() {
        let hand = Hand::new("32T3K");
        assert_eq!(
            Hand {
                human_friendly: "32T3K".to_string(),
                rankable: 1116187,
                wild_rankable: 1116187,
            },
            hand
        );
    }

    #[test]
    fn test_get_ranking() {
        assert_eq!(0, Hand::get_ranking(vec![0, 1, 2, 3, 4]));
        assert_eq!(1, Hand::get_ranking(vec![0, 1, 2, 3, 3]));
    }

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
        assert_eq!((6440, 5905), result);
    }
}
