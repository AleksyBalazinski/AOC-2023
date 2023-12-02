use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = match args.len() {
        2.. => &args[1],
        _ => "input.txt",
    };
    let part: i32 = match args.len() {
        3.. => args[2]
            .parse::<i32>()
            .expect("2nd argument must be a number"),
        _ => 1,
    };

    let lines = read_lines(&filename);

    let calibration_value = match part {
        1 => calibration_value_pt1(&lines),
        _ => calibration_value_pt2(&lines),
    };

    println!("{calibration_value}");
}

fn calibration_value_pt1(lines: &[String]) -> i32 {
    let mut acc = 0;
    let mut number_str = String::new();
    for line in lines {
        match first_and_last_digit(&line) {
            Some((first, last)) => {
                number_str.clear();

                number_str.push(first);
                number_str.push(last);

                acc += number_str
                    .parse::<i32>()
                    .expect("This string doesn't represent a number");
            }
            None => continue,
        }
    }

    acc
}

fn calibration_value_pt2(lines: &[String]) -> i32 {
    let mut acc = 0;
    let mut number_str = String::new();

    for line in lines {
        match first_and_last(&line) {
            Some((first, last)) => {
                number_str.clear();

                number_str.push(first);
                number_str.push(last);

                acc += number_str
                    .parse::<i32>()
                    .expect("This string doesn't represent a number");
            }
            None => continue,
        }
    }

    acc
}

fn first_and_last(line: &str) -> Option<(char, char)> {
    let mut first = '\0';

    for (i, c) in line.chars().enumerate() {
        match get_numeric_char(c, i, line) {
            Some(x) => {
                first = x;
                break;
            }
            None => continue,
        }
    }

    let mut last = '\0';
    for (i, c) in line.chars().rev().enumerate() {
        match get_numeric_char(c, line.len() - i - 1, line) {
            Some(x) => {
                last = x;
                break;
            }
            None => continue,
        }
    }

    if first == '\0' {
        return None;
    }

    Some((first, last))
}

fn get_numeric_char(c: char, i: usize, line: &str) -> Option<char> {
    if c.is_numeric() {
        return Some(c);
    } else {
        return find_spelled_out_from(i, line);
    }
}

fn find_spelled_out_from(start: usize, s: &str) -> Option<char> {
    let digits = HashMap::from([
        ("zero", '0'),
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    for key in digits.keys() {
        if try_find_from(start, s, key) {
            return Some(digits[key]);
        }
    }

    return None;
}

fn try_find_from(start: usize, haystack: &str, needle: &str) -> bool {
    if start + needle.len() > haystack.len() {
        return false;
    }
    return haystack[start..start + needle.len()].eq(needle);
}

fn first_and_last_digit(line: &str) -> Option<(char, char)> {
    let mut first: char = '\0';
    let mut last: char = '\0';
    for c in line.chars() {
        if c.is_numeric() {
            if first == '\0' {
                first = c;
                last = c;
            } else {
                last = c;
            }
        }
    }

    if first == '\0' {
        None
    } else {
        Some((first, last))
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).expect("Couldn't read from file");
    let mut result = Vec::new();

    for line in contents.lines() {
        result.push(line.to_string());
    }

    result
}
