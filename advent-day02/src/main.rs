use std::collections::HashMap;
use std::io::{self, BufRead, Read};

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, PartialEq)]
struct Reveal {
    cubes: HashMap<Color, u32>,
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    power: u32,
    reveals: Vec<Reveal>,
}

fn main() {
    let (id_sum, power_sum) = process_input(io::stdin().lock());
    println!("id_sum: {}, power_sum: {}", id_sum, power_sum);
}

fn process_input<R: Read>(reader: R) -> (u32, u32) {
    let mut max_reveals = HashMap::new();
    max_reveals.insert(Color::Green, 13);
    max_reveals.insert(Color::Red, 12);
    max_reveals.insert(Color::Blue, 14);
    let reveals = Reveal { cubes: max_reveals };
    let buffered = io::BufReader::new(reader);
    let mut id_result = 0;
    let mut power_sum = 0;
    for line_result in buffered.lines() {
        let line = line_result.unwrap();
        if line.trim().is_empty() {
            continue;
        }
        let game = game_from_line(&line);
        if valid_game(&reveals, &game) {
            id_result += game.id;
        }
        power_sum += game.power;
    }
    (id_result, power_sum)
}

fn valid_game(max_reveals: &Reveal, game: &Game) -> bool {
    if game.reveals.is_empty() {
        return true;
    }
    for reveal in &game.reveals {
        for (color, count) in &reveal.cubes {
            if let Some(max_count) = max_reveals.cubes.get(color) {
                if count > max_count {
                    return false;
                }
            } else {
                return false;
            }
        }
    }
    true
}

fn game_from_line(input: &str) -> Game {
    let mut max_count = HashMap::new();
    max_count.insert(Color::Green, 0);
    max_count.insert(Color::Red, 0);
    max_count.insert(Color::Blue, 0);

    let parts: Vec<&str> = input.split(": ").collect();
    let id_part = parts[0];
    let id = id_part
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let reveals_part = parts[1];
    let reveals_strings: Vec<&str> = reveals_part.split("; ").collect();

    let mut reveals = Vec::new();
    for reveal_str in reveals_strings {
        let mut reveal = Reveal {
            cubes: HashMap::new(),
        };
        for color_count in reveal_str.split(", ") {
            let parts: Vec<&str> = color_count.split_whitespace().collect();
            let count = parts[0].parse::<u32>().unwrap();
            let color = match parts[1] {
                "red" => Color::Red,
                "green" => Color::Green,
                "blue" => Color::Blue,
                _ => panic!("Unknown color"),
            };
            reveal.cubes.insert(color, count);
            let entry = max_count.entry(color).or_insert(0);
            if count > *entry {
                *entry = count;
            }
        }
        reveals.push(reveal);
    }

    let mut power = 1;
    for &count in max_count.values() {
        power *= count;
    }

    Game { id, reveals, power }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_from_line() {
        let mut game1_reveals = Vec::new();
        let mut reveal1 = HashMap::new();
        reveal1.insert(Color::Blue, 3);
        reveal1.insert(Color::Red, 4);
        game1_reveals.push(Reveal { cubes: reveal1 });
        let mut reveal2 = HashMap::new();
        reveal2.insert(Color::Red, 1);
        reveal2.insert(Color::Green, 2);
        reveal2.insert(Color::Blue, 6);
        game1_reveals.push(Reveal { cubes: reveal2 });
        let mut reveal3 = HashMap::new();
        reveal3.insert(Color::Green, 2);
        game1_reveals.push(Reveal { cubes: reveal3 });

        let game1 = Game {
            id: 1,
            power: 48,
            reveals: game1_reveals,
        };
        assert_eq!(
            game_from_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            game1
        );
    }

    #[test]
    fn test_game_from_line2() {
        let mut game4_reveals = Vec::new();
        let mut reveal1 = HashMap::new();
        reveal1.insert(Color::Green, 1);
        reveal1.insert(Color::Red, 3);
        reveal1.insert(Color::Blue, 6);
        game4_reveals.push(Reveal { cubes: reveal1 });
        let mut reveal2 = HashMap::new();
        reveal2.insert(Color::Green, 3);
        reveal2.insert(Color::Red, 6);
        game4_reveals.push(Reveal { cubes: reveal2 });
        let mut reveal3 = HashMap::new();
        reveal3.insert(Color::Green, 3);
        reveal3.insert(Color::Blue, 15);
        reveal3.insert(Color::Red, 14);
        game4_reveals.push(Reveal { cubes: reveal3 });
        let game4 = Game {
            id: 4,
            power: 630,
            reveals: game4_reveals,
        };
        assert_eq!(
            game_from_line(
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
            ),
            game4
        );
    }

    #[test]
    fn test_valid_game1() {
        let mut max_reveals = HashMap::new();
        max_reveals.insert(Color::Green, 13);
        max_reveals.insert(Color::Red, 12);
        max_reveals.insert(Color::Blue, 14);
        let game1 = game_from_line("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        assert_eq!(valid_game(&Reveal { cubes: max_reveals }, &game1), true);
    }

    #[test]
    fn test_valid_game3() {
        let mut max_reveals = HashMap::new();
        max_reveals.insert(Color::Green, 13);
        max_reveals.insert(Color::Red, 12);
        max_reveals.insert(Color::Blue, 14);
        let game3 = game_from_line(
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        );
        assert_eq!(valid_game(&Reveal { cubes: max_reveals }, &game3), false);
    }

    #[test]
    fn test_process_input_advent_input_1() {
        let input = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;
        let result = process_input(input.as_bytes());
        assert_eq!((8, 2286), result);
    }
}
