use std::collections::HashMap;
use std::io::{self, BufRead, Read};

fn main() {
    let result = process_input(io::stdin().lock());
    println!("{}", result);
}

fn process_input<R: Read>(reader: R) -> i32 {
    let string_map = build_string_map();
    let buffered = io::BufReader::new(reader);
    let mut result = 0;
    for line_result in buffered.lines() {
        let line = line_result.unwrap();
        if let Some(line_value) = number_from_string(&string_map, &line) {
            result += line_value;
        }
    }
    result
}

fn number_from_string(string_map: &HashMap<String, i32>, input: &str) -> Option<i32> {
    let mut current_str = String::new();
    let mut first_digit = None;
    let mut last_digit = 0;
    for c in input.chars() {
        current_str.push(c);

        if let Some(digit) = c.to_digit(10) {
            last_digit = digit as i32;
            if first_digit.is_none() {
                first_digit = Some(last_digit);
            }
        } else if let Some((_key, &value)) = string_map
            .iter()
            .find(|&(key, _)| current_str.ends_with(key))
        {
            last_digit = value;
            if first_digit.is_none() {
                first_digit = Some(last_digit);
            }
        }
    }
    first_digit.map(|fd| fd * 10 + last_digit)
}

fn build_string_map() -> HashMap<String, i32> {
    let mut map = HashMap::new();
    map.insert("one".to_string(), 1);
    map.insert("two".to_string(), 2);
    map.insert("three".to_string(), 3);
    map.insert("four".to_string(), 4);
    map.insert("five".to_string(), 5);
    map.insert("six".to_string(), 6);
    map.insert("seven".to_string(), 7);
    map.insert("eight".to_string(), 8);
    map.insert("nine".to_string(), 9);
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_from_string_simple() {
        assert_eq!(number_from_string(&build_string_map(), "12"), Some(12));
    }

    #[test]
    fn test_number_from_string_1_character() {
        assert_eq!(number_from_string(&build_string_map(), "2x3"), Some(23));
    }

    #[test]
    fn test_number_from_string_alpha_numbers() {
        assert_eq!(
            number_from_string(&build_string_map(), "twothree"),
            Some(23)
        );
        assert_eq!(
            number_from_string(&build_string_map(), "twoxthree"),
            Some(23)
        );
        assert_eq!(
            number_from_string(&build_string_map(), "two1nine"),
            Some(29)
        );
        assert_eq!(
            number_from_string(&build_string_map(), "eightwothree"),
            Some(83)
        );
        assert_eq!(
            number_from_string(&build_string_map(), "abcone2threexyz"),
            Some(13)
        );
        assert_eq!(
            number_from_string(&build_string_map(), "xtwone3four"),
            Some(24)
        );
        assert_eq!(
            number_from_string(&build_string_map(), "4nineeightseven2"),
            Some(42)
        );
        assert_eq!(
            number_from_string(&build_string_map(), "zoneight234"),
            Some(14)
        );
        assert_eq!(
            number_from_string(&build_string_map(), "7pqrstsixteen"),
            Some(76)
        );
    }

    #[test]
    fn test_number_from_string_tricky() {
        assert_eq!(number_from_string(&build_string_map(), "twone"), Some(21));
    }

    #[test]
    fn test_number_from_string_advent_input() {
        assert_eq!(number_from_string(&build_string_map(), "1abc2"), Some(12));
        assert_eq!(
            number_from_string(&build_string_map(), "pqr3stu8vwx"),
            Some(38)
        );
        assert_eq!(
            number_from_string(&build_string_map(), "a1b2c3d4e5f"),
            Some(15)
        );
        assert_eq!(
            number_from_string(&build_string_map(), "treb7uchet"),
            Some(77)
        );
    }

    #[test]
    fn test_process_input_advent_input_1() {
        let input = r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#;

        let result = process_input(input.as_bytes());
        assert_eq!(142, result);
    }

    #[test]
    fn test_process_input_advent_input_2() {
        let input = r#"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#;

        let result = process_input(input.as_bytes());
        assert_eq!(281, result);
    }
}
