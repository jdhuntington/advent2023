use std::io::{self, BufRead, Read};

#[derive(Debug, PartialEq)]
struct PartNumber {
    x: u32,
    width: u32,
    y: u32,
    number: u32,
    real_part: bool,
}

#[derive(Debug, PartialEq)]
struct Part {
    x: u32,
    y: u32,
    symbol: char,
}

fn main() {
    let (sum, gear_ratios) = process_input(io::stdin().lock());
    println!("sum: {}, gear ratios: {}", sum, gear_ratios);
}

fn process_input<R: Read>(reader: R) -> (u32, u32) {
    let mut x_len = 0;
    let mut y_len = 0;
    let mut parts: Vec<Part> = Vec::new();
    let mut part_numbers: Vec<PartNumber> = Vec::new();
    let buffered = io::BufReader::new(reader);
    for line_result in buffered.lines() {
        let line = line_result.unwrap();
        if line.trim().is_empty() {
            continue;
        }
        process_line(&mut part_numbers, &mut parts, &line, y_len);
        y_len += 1;
        if x_len == 0 {
            x_len = line.len() as u32;
        }
    }
    let mut matrix: Vec<Vec<Option<usize>>> = Vec::new();
    while matrix.len() < y_len as usize {
        let mut row = Vec::new();
        while row.len() < x_len as usize {
            row.push(None);
        }
        matrix.push(row);
    }
    for (index, part_number) in part_numbers.iter().enumerate() {
        for x in part_number.x..(part_number.x + part_number.width) {
            matrix[part_number.y as usize][x as usize] = Some(index as usize);
        }
    }
    let mut gear_parts_sets: Vec<Vec<usize>> = Vec::new();
    for part in parts {
        let mut gear_parts_indices: Vec<usize> = Vec::new();

        let x = part.x;
        let y = part.y;
        for x_offset in -1..=1 {
            for y_offset in -1..=1 {
                if x_offset == 0 && y_offset == 0 {
                    continue;
                }
                let mut current_x = x as i32;
                let mut current_y = y as i32;

                current_x += x_offset;
                current_y += y_offset;
                if current_x < 0 || current_y < 0 {
                    continue;
                }
                let current_x = current_x as usize;
                let current_y = current_y as usize;
                if current_x >= matrix[0].len() || current_y >= matrix.len() {
                    continue;
                }
                if let Some(index) = matrix[current_y][current_x] {
                    part_numbers[index as usize].real_part = true;
                    if part.symbol == '*' && !gear_parts_indices.contains(&index) {
                        gear_parts_indices.push(index);
                    }
                }
            }
        }
        if !gear_parts_indices.is_empty() {
            gear_parts_sets.push(gear_parts_indices);
        }
    }
    let gear_ratios = gear_parts_sets
        .iter()
        .filter(|v| v.len() == 2)
        .map(|v| part_numbers[v[0]].number * part_numbers[v[1]].number)
        .sum::<u32>();
    let mut sum = 0;
    for part_number in part_numbers {
        if part_number.real_part {
            sum += part_number.number;
        }
    }
    (sum, gear_ratios)
}

fn process_line(part_numbers: &mut Vec<PartNumber>, parts: &mut Vec<Part>, line: &str, y_pos: u32) {
    let mut current_number = String::new();
    let mut current_x = 0;
    let mut current_width = 0;
    for (x_pos, c) in line.chars().enumerate() {
        if c == '.' {
            if !current_number.is_empty() {
                let part = PartNumber {
                    x: current_x + 1 - current_width,
                    real_part: false,
                    width: current_width,
                    y: y_pos,
                    number: current_number.parse::<u32>().unwrap(),
                };
                part_numbers.push(part);
                current_number.clear();
                current_width = 0;
            }
        } else if c.is_digit(10) {
            current_number.push(c);
            current_width += 1;
            current_x = x_pos as u32;
        } else {
            if !current_number.is_empty() {
                let part = PartNumber {
                    x: current_x + 1 - current_width,
                    real_part: false,
                    width: current_width,
                    y: y_pos,
                    number: current_number.parse::<u32>().unwrap(),
                };
                part_numbers.push(part);
                current_number.clear();
                current_width = 0;
            }
            let part = Part {
                x: x_pos as u32,
                y: y_pos,
                symbol: c,
            };
            parts.push(part);
        }
    }
    if !current_number.is_empty() {
        let part = PartNumber {
            x: current_x + 1 - current_width,
            real_part: false,
            width: current_width,
            y: y_pos,
            number: current_number.parse::<u32>().unwrap(),
        };
        part_numbers.push(part);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line1() {
        let mut expected = Vec::new();
        expected.push(PartNumber {
            x: 0,
            real_part: false,
            width: 3,
            y: 0,
            number: 467,
        });
        expected.push(PartNumber {
            x: 5,
            real_part: false,
            width: 3,
            y: 0,
            number: 114,
        });
        let input = "467..114..";
        let mut parts: Vec<PartNumber> = Vec::new();
        process_line(&mut parts, &mut Vec::new(), input, 0);
        assert_eq!(expected, parts);
    }

    #[test]
    fn test_parse_line2() {
        let mut expected = Vec::new();
        expected.push(PartNumber {
            x: 0,
            real_part: false,
            width: 1,
            y: 0,
            number: 1,
        });
        let input = "1";
        let mut parts: Vec<PartNumber> = Vec::new();
        process_line(&mut parts, &mut Vec::new(), input, 0);
        assert_eq!(expected, parts);
    }

    #[test]
    fn test_parse_line3() {
        let mut expected = Vec::new();
        expected.push(PartNumber {
            x: 0,
            real_part: false,
            width: 1,
            y: 0,
            number: 1,
        });
        expected.push(PartNumber {
            x: 2,
            real_part: false,
            width: 1,
            y: 0,
            number: 1,
        });
        expected.push(PartNumber {
            x: 4,
            real_part: false,
            width: 1,
            y: 0,
            number: 1,
        });
        let input = "1.1.1";
        let mut parts: Vec<PartNumber> = Vec::new();
        process_line(&mut parts, &mut Vec::new(), input, 0);
        assert_eq!(expected, parts);
    }

    #[test]
    fn test_parse_line4() {
        let mut expected = Vec::new();
        expected.push(PartNumber {
            x: 0,
            real_part: false,
            width: 5,
            y: 0,
            number: 12345,
        });
        let input = "12345";
        let mut parts: Vec<PartNumber> = Vec::new();
        process_line(&mut parts, &mut Vec::new(), input, 0);
        assert_eq!(expected, parts);
    }

    #[test]
    fn test_parse_line5() {
        let mut expected = Vec::new();
        expected.push(PartNumber {
            x: 0,
            real_part: false,
            width: 5,
            y: 0,
            number: 12345,
        });
        let input = "12345.";
        let mut parts: Vec<PartNumber> = Vec::new();
        process_line(&mut parts, &mut Vec::new(), input, 0);
        assert_eq!(expected, parts);
    }

    #[test]
    fn test_parse_line6() {
        let mut expected_parts: Vec<Part> = Vec::new();
        let expected_part_numbers: Vec<PartNumber> = Vec::new();
        expected_parts.push(Part {
            x: 3,
            y: 0,
            symbol: '$',
        });
        expected_parts.push(Part {
            x: 5,
            y: 0,
            symbol: '*',
        });
        let input = "...$.*....";
        let mut part_numbers: Vec<PartNumber> = Vec::new();
        let mut parts: Vec<Part> = Vec::new();
        process_line(&mut part_numbers, &mut parts, input, 0);
        assert_eq!(expected_parts, parts);
        assert_eq!(expected_part_numbers, part_numbers);
    }

    #[test]
    fn test_parse_line7() {
        let mut expected_parts: Vec<Part> = Vec::new();
        let mut expected_part_numbers: Vec<PartNumber> = Vec::new();
        expected_parts.push(Part {
            x: 3,
            y: 4,
            symbol: '*',
        });
        expected_part_numbers.push(PartNumber {
            x: 0,
            real_part: false,
            width: 3,
            y: 4,
            number: 617,
        });
        let input = "617*......";
        let mut part_numbers: Vec<PartNumber> = Vec::new();
        let mut parts: Vec<Part> = Vec::new();
        process_line(&mut part_numbers, &mut parts, input, 4);
        assert_eq!(expected_parts, parts);
        assert_eq!(expected_part_numbers, part_numbers);
    }

    #[test]
    fn test_parse_line8() {
        let mut expected_parts: Vec<Part> = Vec::new();
        let mut expected_part_numbers: Vec<PartNumber> = Vec::new();
        expected_parts.push(Part {
            x: 3,
            y: 4,
            symbol: '*',
        });
        expected_parts.push(Part {
            x: 7,
            y: 4,
            symbol: '*',
        });
        expected_part_numbers.push(PartNumber {
            x: 0,
            real_part: false,
            width: 3,
            y: 4,
            number: 617,
        });
        expected_part_numbers.push(PartNumber {
            x: 4,
            real_part: false,
            width: 3,
            y: 4,
            number: 617,
        });
        let input = "617*617*......";
        let mut part_numbers: Vec<PartNumber> = Vec::new();
        let mut parts: Vec<Part> = Vec::new();
        process_line(&mut part_numbers, &mut parts, input, 4);
        assert_eq!(expected_parts, parts);
        assert_eq!(expected_part_numbers, part_numbers);
    }

    #[test]
    fn test_process_input_advent_input_1() {
        let input = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
        "#;
        let result = process_input(input.as_bytes());
        assert_eq!((4361, 467835), result);
    }

    #[test]
    fn test_process_input_advent_input_2() {
        let input = r#"
.......358..........31.....339.....669.............598......328.....575......................447..650..............964...........692........
...............415..*.........@......*...627*...................945*.............144/.506............................*......514...*...150...
.........182..+.....873.756.......737........784..568....667..............258........./.........741...........707*....84........520.........
"#;
        let result = process_input(input.as_bytes());
        assert_eq!(
            (
                31 + 339
                    + 669
                    + 575
                    + 964
                    + 692
                    + 415
                    + 627
                    + 945
                    + 144
                    + 506
                    + 873
                    + 737
                    + 784
                    + 707
                    + 84
                    + 520,
                1995875
            ),
            result
        );
    }

    #[test]
    fn test_process_input_advent_input_3() {
        let input = r#"
    .....856...214..236....*.....159.%......738.....-......826....&.272.*.......36.....465.........../.....*...587.......*....*......548..699...
    .............*........36..........743.=.../...............*......*..424.................580.#...897.448....*.......833...633.....*...*......
    .............963......................542........734.....901...914..........843.............523..........818..................691.....833...
        "#;
        let result = process_input(input.as_bytes());
        assert_eq!(
            (
                214 + 738
                    + 826
                    + 272
                    + 587
                    + 548
                    + 699
                    + 36
                    + 743
                    + 424
                    + 897
                    + 448
                    + 833
                    + 633
                    + 963
                    + 542
                    + 901
                    + 914
                    + 523
                    + 818
                    + 691
                    + 833,
                2640017
            ),
            result
        );
    }

    #[test]
    fn test_process_input_advent_input_4() {
        let input = r#"
.*
36
    "#;
        let result = process_input(input.as_bytes());
        assert_eq!((36, 0), result);
    }
}
