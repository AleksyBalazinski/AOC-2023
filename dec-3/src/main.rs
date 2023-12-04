use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = match args.len() {
        2.. => &args[1],
        _ => "input.txt",
    };

    let lines = read_lines(filename);
    let sum_of_part_numbers = sum_part_numbers(&lines);
    println!("sum of part numbers: {sum_of_part_numbers}");
}

fn sum_part_numbers(lines: &[String]) -> i32 {
    let line_length = lines[0].len();
    let empty_line = get_empty_line(line_length);
    let mut acc = 0;
    for i in 0..lines.len() {
        let upper_line = if i == 0 { &empty_line } else { &lines[i - 1] };
        let current_line = &lines[i];
        let lower_line = if i == line_length - 1 {
            &empty_line
        } else {
            &lines[i + 1]
        };

        acc += add_part_numbers_in_line(
            upper_line.as_bytes(),
            current_line.as_bytes(),
            lower_line.as_bytes(),
        );
    }
    acc
}

fn add_part_numbers_in_line(upper_line: &[u8], current_line: &[u8], lower_line: &[u8]) -> i32 {
    let mut selected = false;
    let mut number_start = usize::MAX;
    let mut number_bytes: Vec<u8> = Vec::new();
    let mut acc = 0;
    for i in 0..current_line.len() {
        if is_numeric(current_line[i]) {
            if number_start == usize::MAX {
                number_start = i;
            }
            number_bytes.push(current_line[i]);
            if is_symbol(upper_line[i]) || is_symbol(lower_line[i]) {
                selected = true;
            }
        }

        if (!is_numeric(current_line[i]) && !number_bytes.is_empty())
            || (is_numeric(current_line[i]) && i == current_line.len() - 1)
        {
            selected = selected
                || is_symbol(current_line[i])
                || is_symbol(upper_line[i])
                || is_symbol(lower_line[i]);

            if number_start != 0 {
                selected = selected
                    || is_symbol(current_line[number_start - 1])
                    || is_symbol(upper_line[number_start - 1])
                    || is_symbol(lower_line[number_start - 1]);
            }

            if selected {
                acc += get_token_value(number_bytes);
            }
            number_bytes = Vec::new();
            number_start = usize::MAX;
            selected = false;
        }
    }

    acc
}

fn get_token_value(literal: Vec<u8>) -> i32 {
    String::from_utf8(literal).unwrap().parse().unwrap()
}

fn is_numeric(c: u8) -> bool {
    c >= b'0' && c <= b'9'
}

fn is_symbol(c: u8) -> bool {
    !is_numeric(c) && c != b'.'
}

fn get_empty_line(line_length: usize) -> String {
    ".".repeat(line_length)
}

fn read_lines(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).expect("Couldn't read from file");
    let mut result = Vec::new();

    for line in contents.lines() {
        result.push(line.to_string());
    }

    result
}
